#!/usr/bin/env python3
"""Per-stock trade forensics for the papertrader short book.

Takes one stock's trades from a backtest run and slices them by market
context — entry time, gap, prior-session direction, intraday extension,
volatility, volume — to find conditions under which the edge reliably
fails. Those conditions are filter candidates: skipping them raises Avg R
and win rate without touching the entry logic itself.

The bar that matters is NOT "is this bucket profitable" but "is it above
the cost of trading". At ~Rs4.5L notional against a 0.75% stop that is
0.061 R, so a bucket at +0.03 R is a losing bucket after charges.

Usage:
    python3 analyze_stock.py --symbol HAL --run 219
    python3 analyze_stock.py --symbol HAL --run 219 --min-bucket 15

Reads data/papertrader.db relative to the repo root.
"""
import argparse
import collections
import sqlite3
import statistics
import sys

BREAKEVEN_R = 0.061  # see research notes: 0.0457% notional cost / 0.75% risk


def load_candles(db, token):
    """All 1m bars for one instrument, grouped by IST trading date.

    Timestamps are stored UTC; +5:30 shifts them to IST so a 'day' matches
    a trading session rather than splitting one across two UTC dates.
    """
    rows = db.execute(
        "SELECT ts, open, high, low, close, volume FROM candles "
        "WHERE instrument_token=? AND interval='1m' ORDER BY ts", (token,)
    ).fetchall()
    days = collections.defaultdict(list)
    for ts, o, h, l, c, v in rows:
        hh = int(ts[11:13]) + 5
        mm = int(ts[14:16]) + 30
        if mm >= 60:
            mm -= 60
            hh += 1
        date = ts[:10]
        if hh >= 24:  # IST shift rolled past midnight — next calendar day
            hh -= 24
        days[date].append((hh * 60 + mm, o, h, l, c, v))
    for d in days:
        days[d].sort()
    return days


def daily_summary(days):
    """date -> (open, high, low, close, total_volume), in date order."""
    out = {}
    for d, bars in days.items():
        out[d] = (
            bars[0][1],
            max(b[2] for b in bars),
            min(b[3] for b in bars),
            bars[-1][4],
            sum(b[5] for b in bars),
        )
    return out


def pct(a, b):
    return (a - b) / b * 100.0 if b else 0.0


def features(trade, days, daily, dates, date_ix):
    """Context known at entry time — nothing here may peek past the entry bar."""
    ts, entry_px, direction = trade
    date = ts[:10]
    if date not in days:
        return None
    hh = int(ts[11:13]) + 5
    mm = int(ts[14:16]) + 30
    if mm >= 60:
        mm -= 60
        hh += 1
    entry_min = hh * 60 + mm

    bars = days[date]
    upto = [b for b in bars if b[0] <= entry_min]
    if len(upto) < 5:
        return None
    day_open = bars[0][1]
    hi = max(b[2] for b in upto)
    lo = min(b[3] for b in upto)

    i = date_ix.get(date)
    prev = daily[dates[i - 1]] if i and i > 0 else None
    prev5 = daily[dates[i - 5]] if i and i >= 5 else None

    vols = [b[5] for b in upto]
    recent = vols[-15:]
    base = statistics.mean(vols) if vols else 0

    return {
        "date": date,
        "entry_min": entry_min,
        "dow": ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"][
            __import__("datetime").date.fromisoformat(date).weekday()],
        # gap: today's open vs yesterday's close
        "gap_pct": pct(day_open, prev[3]) if prev else None,
        # yesterday's own close-to-close move
        "prev_day_pct": pct(prev[3], daily[dates[i - 2]][3])
        if i and i > 1 else None,
        # 5-session trend into today
        "ret5_pct": pct(prev[3], prev5[3]) if prev5 else None,
        # how far price has already travelled from the open when we enter
        "ext_from_open_pct": pct(entry_px, day_open),
        # realised range so far — a volatility proxy available at entry
        "range_so_far_pct": (hi - lo) / day_open * 100.0 if day_open else None,
        # is the entry near the day's extreme so far?
        "pos_in_range": (entry_px - lo) / (hi - lo) if hi > lo else 0.5,
        # burst of activity vs the day's own average
        "vol_ratio": (statistics.mean(recent) / base) if base and recent else None,
    }


def bucket_report(name, rows, keyfn, min_n, order=None):
    """Group trades by keyfn and print Avg R / win% / vs-breakeven per bucket."""
    g = collections.defaultdict(list)
    for f, r, win in rows:
        k = keyfn(f)
        if k is not None:
            g[k].append((r, win))
    keys = order if order else sorted(g)
    keys = [k for k in keys if k in g and len(g[k]) >= min_n]
    if not keys:
        return
    print(f"\n{name}")
    print(f"  {'bucket':<18}{'n':>6}{'Avg R':>9}{'win%':>8}{'x BE':>8}   verdict")
    for k in keys:
        v = g[k]
        n = len(v)
        ar = sum(x[0] for x in v) / n
        wr = 100.0 * sum(x[1] for x in v) / n
        ratio = ar / BREAKEVEN_R
        verdict = ("SKIP" if ar < 0 else "below cost" if ratio < 1.0
                   else "thin" if ratio < 1.5 else "keep")
        print(f"  {str(k):<18}{n:>6}{ar:>9.3f}{wr:>8.1f}{ratio:>7.2f}x   {verdict}")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--symbol", required=True)
    ap.add_argument("--run", type=int, required=True)
    ap.add_argument("--min-bucket", type=int, default=12,
                    help="suppress buckets with fewer trades than this")
    ap.add_argument("--db", default="data/papertrader.db")
    ap.add_argument("--split-date", default=None,
                    help="YYYY-MM-DD; trades on/after this date are held out "
                         "to verify the filters generalise")
    a = ap.parse_args()

    db = sqlite3.connect(a.db)
    tok = db.execute("SELECT instrument_token FROM instruments WHERE tradingsymbol=?",
                     (a.symbol,)).fetchone()
    if not tok:
        sys.exit(f"unknown symbol {a.symbol}")
    tok = tok[0]

    trades = db.execute(
        "SELECT entry_ts, entry_price, direction, r_multiple, pnl_pct FROM trades "
        "WHERE run_id=? AND instrument_token=? AND r_multiple IS NOT NULL "
        "ORDER BY entry_ts", (a.run, tok)).fetchall()
    if not trades:
        sys.exit(f"no trades for {a.symbol} in run {a.run}")

    days = load_candles(db, tok)
    daily = daily_summary(days)
    dates = sorted(daily)
    date_ix = {d: i for i, d in enumerate(dates)}

    rows = []
    for ts, px, direction, r, pnl in trades:
        f = features((ts, px, direction), days, daily, dates, date_ix)
        if f:
            rows.append((f, r, 1 if pnl > 0 else 0))

    n = len(rows)
    ar = sum(x[1] for x in rows) / n
    wr = 100.0 * sum(x[2] for x in rows) / n
    print("=" * 74)
    print(f"{a.symbol} — run {a.run} — {n} trades")
    print(f"baseline: Avg R {ar:.3f}   win {wr:.1f}%   "
          f"{ar / BREAKEVEN_R:.2f}x breakeven ({BREAKEVEN_R} R)")
    print("=" * 74)

    def tod(f):
        m = f["entry_min"]
        return f"{m // 60:02d}:{(m % 60) // 15 * 15:02d}"

    def band(v, edges, labels):
        if v is None:
            return None
        for e, l in zip(edges, labels):
            if v < e:
                return l
        return labels[-1]

    bucket_report("ENTRY TIME (15-min)", rows, tod, a.min_bucket)
    bucket_report("DAY OF WEEK", rows, lambda f: f["dow"], a.min_bucket,
                  order=["Mon", "Tue", "Wed", "Thu", "Fri"])
    bucket_report("OVERNIGHT GAP", rows,
                  lambda f: band(f["gap_pct"], [-1, -0.3, 0.3, 1, 99],
                                 ["gap<-1%", "-1..-0.3", "flat", "0.3..1", "gap>1%"]),
                  a.min_bucket,
                  order=["gap<-1%", "-1..-0.3", "flat", "0.3..1", "gap>1%"])
    bucket_report("PREVIOUS DAY MOVE", rows,
                  lambda f: band(f["prev_day_pct"], [-1.5, -0.5, 0.5, 1.5, 99],
                                 ["prev<-1.5", "-1.5..-0.5", "prev flat",
                                  "0.5..1.5", "prev>1.5"]),
                  a.min_bucket,
                  order=["prev<-1.5", "-1.5..-0.5", "prev flat", "0.5..1.5", "prev>1.5"])
    bucket_report("LAST 5 SESSIONS", rows,
                  lambda f: band(f["ret5_pct"], [-4, -1, 1, 4, 999],
                                 ["5d<-4%", "-4..-1", "5d flat", "1..4", "5d>4%"]),
                  a.min_bucket,
                  order=["5d<-4%", "-4..-1", "5d flat", "1..4", "5d>4%"])
    bucket_report("EXTENSION FROM OPEN AT ENTRY", rows,
                  lambda f: band(f["ext_from_open_pct"], [-1.5, -0.5, 0.5, 1.5, 99],
                                 ["<-1.5%", "-1.5..-0.5", "near open",
                                  "0.5..1.5", ">1.5%"]),
                  a.min_bucket,
                  order=["<-1.5%", "-1.5..-0.5", "near open", "0.5..1.5", ">1.5%"])
    bucket_report("DAY RANGE SO FAR (volatility)", rows,
                  lambda f: band(f["range_so_far_pct"], [0.8, 1.5, 2.5, 99],
                                 ["quiet <0.8%", "0.8-1.5%", "1.5-2.5%", "wild >2.5%"]),
                  a.min_bucket,
                  order=["quiet <0.8%", "0.8-1.5%", "1.5-2.5%", "wild >2.5%"])
    bucket_report("ENTRY POSITION IN DAY RANGE", rows,
                  lambda f: band(f["pos_in_range"], [0.25, 0.5, 0.75, 1.01],
                                 ["bottom 25%", "lower mid", "upper mid", "top 25%"]),
                  a.min_bucket,
                  order=["bottom 25%", "lower mid", "upper mid", "top 25%"])
    bucket_report("VOLUME BURST vs DAY AVG", rows,
                  lambda f: band(f["vol_ratio"], [0.7, 1.0, 1.5, 99],
                                 ["<0.7x", "0.7-1.0x", "1.0-1.5x", ">1.5x"]),
                  a.min_bucket,
                  order=["<0.7x", "0.7-1.0x", "1.0-1.5x", ">1.5x"])

    print(f"\n{'-' * 74}")
    print("verdicts: SKIP = negative | below cost = <0.061 R | thin = <1.5x | keep")
    print("A filter is only worth applying if the bucket is BOTH clearly bad AND")
    print("big enough to matter — check n before acting on any single row.")

    filter_impact(rows, ar, wr, n)

    if a.split_date:
        print(f"\n{'=' * 74}")
        print(f"OUT-OF-SAMPLE CHECK — rules held out at {a.split_date}")
        print(f"{'=' * 74}")
        print("The rules above were read off the WHOLE sample, so their gain is")
        print("in-sample by construction. The only question that matters is")
        print("whether the same rules still help on data not used to pick them.")
        early = [x for x in rows if x[0]["date"] < a.split_date]
        late = [x for x in rows if x[0]["date"] >= a.split_date]
        for label, part in (("in-sample  (before)", early), ("HELD OUT   (after)", late)):
            if not part:
                continue
            bn, br, bw = _stats(part)
            kept = [x for x in part if not any(p(x[0]) for _, p in CANDIDATES)]
            kn, kr, kw = _stats(kept)
            print(f"\n  {label}   {bn} trades")
            print(f"    {'':<12}{'n':>6}{'Avg R':>9}{'win%':>8}{'x BE':>8}")
            print(f"    {'unfiltered':<12}{bn:>6}{br:>9.3f}{bw:>8.1f}"
                  f"{br / BREAKEVEN_R:>7.2f}x")
            print(f"    {'filtered':<12}{kn:>6}{kr:>9.3f}{kw:>8.1f}"
                  f"{kr / BREAKEVEN_R:>7.2f}x")
            print(f"    {'change':<12}{kn - bn:>6}{kr - br:>+9.3f}{kw - bw:>+8.1f}")
        print("\n  If the held-out change is near zero or negative, the rules are")
        print("  fitting this stock's noise and must not go into the live config.")


# Candidate skip-rules. Each is (label, predicate) where the predicate is
# True for trades the filter would REMOVE. Conditions are deliberately
# simple and computable at entry time — anything the live engine couldn't
# evaluate before placing the order is useless here.
CANDIDATES = [
    ("gap down > 1%", lambda f: f["gap_pct"] is not None and f["gap_pct"] < -1.0),
    ("already -1.5% from open",
     lambda f: f["ext_from_open_pct"] is not None and f["ext_from_open_pct"] < -1.5),
    ("volume burst > 1.0x",
     lambda f: f["vol_ratio"] is not None and f["vol_ratio"] > 1.0),
]


def _stats(rows):
    if not rows:
        return 0, 0.0, 0.0
    n = len(rows)
    return n, sum(x[1] for x in rows) / n, 100.0 * sum(x[2] for x in rows) / n


def filter_impact(rows, base_r, base_w, base_n):
    """What each candidate skip-rule would have bought, alone and combined.

    Reported as the effect on the trades that REMAIN, since that is what
    would actually be traded. A filter that improves Avg R but removes half
    the trades is usually not worth it — fewer trades means the same fixed
    research/attention overhead spread over less profit, and a noisier
    live estimate — so the kept-share is printed alongside.
    """
    print(f"\n{'=' * 74}")
    print("FILTER IMPACT — effect on the trades that remain")
    print(f"{'=' * 74}")
    print(f"  {'rule':<28}{'removed':>9}{'kept':>7}{'Avg R':>9}{'win%':>8}{'dR':>8}{'dWin':>7}")
    print(f"  {'(baseline)':<28}{'-':>9}{base_n:>7}{base_r:>9.3f}{base_w:>8.1f}"
          f"{'-':>8}{'-':>7}")

    for label, pred in CANDIDATES:
        kept = [x for x in rows if not pred(x[0])]
        k, r, w = _stats(kept)
        if k == base_n:
            continue
        print(f"  {label:<28}{base_n - k:>9}{k:>7}{r:>9.3f}{w:>8.1f}"
              f"{r - base_r:>+8.3f}{w - base_w:>+7.1f}")

    kept = [x for x in rows if not any(p(x[0]) for _, p in CANDIDATES)]
    k, r, w = _stats(kept)
    print(f"  {'ALL COMBINED':<28}{base_n - k:>9}{k:>7}{r:>9.3f}{w:>8.1f}"
          f"{r - base_r:>+8.3f}{w - base_w:>+7.1f}")
    print(f"\n  kept {k}/{base_n} trades ({100.0 * k / base_n:.0f}%), "
          f"{r / BREAKEVEN_R:.2f}x breakeven")
    print("  NOTE: these rules were chosen by looking at this stock's own")
    print("  results, so this is an in-sample figure. Confirm on a held-out")
    print("  period before trusting it — see the skill's validation step.")


if __name__ == "__main__":
    main()
