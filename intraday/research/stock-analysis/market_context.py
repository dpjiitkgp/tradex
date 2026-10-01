#!/usr/bin/env python3
"""Does wider-market context predict which trades work?

Conditions the book's trades on India VIX (volatility regime) and, when
available, NIFTY 50 (index direction//strength) — both computed from data
that existed BEFORE the entry bar, so anything found here is usable live.

Pooled across all stocks in the run by default, because per-stock buckets
are far too small to separate a real effect from sigma ~1.4 R noise. Always
reports a held-out period alongside the in-sample one: in this dataset
several large in-sample effects have vanished under that check.

Usage:
    python3 market_context.py --runs 229 --split-date 2024-01-01
    python3 market_context.py --runs 229,230 --min-bucket 200
"""
import argparse
import collections
import sqlite3
import statistics
import sys

BREAKEVEN_R = 0.061


def ist_date(ts):
    """UTC timestamp string -> IST trading date."""
    hh = int(ts[11:13]) + 5
    mm = int(ts[14:16]) + 30
    if mm >= 60:
        hh += 1
    if hh >= 24:
        # past IST midnight — belongs to the next calendar day, but no NSE
        # session bar can land there, so this only guards malformed rows
        return None
    return ts[:10]


def daily_series(db, symbol):
    """date -> (open, close, high, low) for an index, from its minute bars."""
    tok = db.execute(
        "SELECT instrument_token FROM instruments WHERE tradingsymbol=?", (symbol,)
    ).fetchone()
    if not tok:
        return {}
    rows = db.execute(
        "SELECT ts, open, high, low, close FROM candles "
        "WHERE instrument_token=? ORDER BY ts", (tok[0],)
    ).fetchall()
    agg = collections.defaultdict(list)
    for ts, o, h, l, c in rows:
        d = ist_date(ts)
        if d:
            agg[d].append((ts, o, h, l, c))
    out = {}
    for d, bars in agg.items():
        bars.sort()
        out[d] = (bars[0][1], bars[-1][4],
                  max(b[2] for b in bars), min(b[3] for b in bars))
    return out


def band(v, edges, labels):
    if v is None:
        return None
    for e, l in zip(edges, labels):
        if v < e:
            return l
    return labels[-1]


def report(title, rows, keyfn, min_n, split_date, order=None):
    """Per-bucket Avg R, shown in-sample and held-out side by side."""
    groups = collections.defaultdict(lambda: ([], []))
    for date, r, win in rows:
        k = keyfn(date)
        if k is None:
            continue
        which = 0 if (not split_date or date < split_date) else 1
        groups[k][which].append((r, win))
    keys = order if order else sorted(groups)
    keys = [k for k in keys if k in groups
            and len(groups[k][0]) + len(groups[k][1]) >= min_n]
    if not keys:
        print(f"\n{title}\n  (no bucket reached min-bucket={min_n})")
        return
    print(f"\n{title}")
    print(f"  {'bucket':<16}{'IS n':>7}{'IS AvgR':>9}"
          f"{'OOS n':>8}{'OOS AvgR':>10}{'OOS xBE':>9}{'win%':>7}")
    for k in keys:
        a, b = groups[k]
        ar = sum(x[0] for x in a) / len(a) if a else float("nan")
        br = sum(x[0] for x in b) / len(b) if b else float("nan")
        bw = 100.0 * sum(x[1] for x in b) / len(b) if b else float("nan")
        print(f"  {str(k):<16}{len(a):>7}{ar:>9.3f}"
              f"{len(b):>8}{br:>10.3f}{br / BREAKEVEN_R:>8.2f}x{bw:>7.1f}")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--runs", required=True, help="comma-separated run ids")
    ap.add_argument("--split-date", default="2024-01-01")
    ap.add_argument("--min-bucket", type=int, default=150)
    ap.add_argument("--db", default="data/papertrader.db")
    a = ap.parse_args()
    runs = [int(x) for x in a.runs.split(",")]

    db = sqlite3.connect(a.db)
    vix = daily_series(db, "INDIA VIX")
    nifty = daily_series(db, "NIFTY 50")
    if not vix:
        sys.exit("no INDIA VIX data — run: backfill --indices \"INDIA VIX\"")
    print(f"VIX days: {len(vix)}  ({min(vix)}..{max(vix)})")
    print(f"NIFTY days: {len(nifty)}" + (f"  ({min(nifty)}..{max(nifty)})" if nifty else "  — NOT CACHED, skipping index buckets"))

    # PIIND (token 6191105) is excluded: Kite serves cross-contaminated bars
    # for it and re-fetching reproduces them, so its trades carry impossible
    # R-multiples. It is 0.53% of trades yet drags a universe-wide aggregate
    # from about +0.05 to -2.27. The [universe].exclude config keeps it out of
    # research runs, but raw SQL like this still sees it.
    q = ("SELECT entry_ts, r_multiple, pnl_pct FROM trades WHERE run_id IN (%s) "
         "AND r_multiple IS NOT NULL AND instrument_token <> 6191105"
         % ",".join("?" * len(runs)))
    rows = []
    for ts, r, pnl in db.execute(q, runs):
        d = ist_date(ts)
        if d:
            rows.append((d, r, 1 if pnl > 0 else 0))
    if not rows:
        sys.exit("no trades for those runs")

    dates = sorted({d for d, _, _ in rows})
    ix = {d: i for i, d in enumerate(dates)}
    n = len(rows)
    ar = sum(x[1] for x in rows) / n
    print(f"\n{'=' * 70}")
    print(f"runs {runs} — {n} trades — baseline Avg R {ar:.3f} "
          f"({ar / BREAKEVEN_R:.2f}x breakeven)")
    print(f"held out from {a.split_date}")
    print("=" * 70)

    # VIX level on the PREVIOUS session's close — known before the open, so a
    # live engine could act on it. Using the same day's close would peek.
    vdates = sorted(vix)
    vix_ix = {d: i for i, d in enumerate(vdates)}

    def prev_vix(date):
        i = vix_ix.get(date)
        if i is None or i == 0:
            return None
        return vix[vdates[i - 1]][1]

    lv = [v for v in (prev_vix(d) for d in dates) if v]
    if lv:
        qs = statistics.quantiles(lv, n=4)
        print(f"\nVIX quartile cuts: {qs[0]:.1f} / {qs[1]:.1f} / {qs[2]:.1f}")
        labels = [f"1 calm <{qs[0]:.0f}", f"2 {qs[0]:.0f}-{qs[1]:.0f}",
                  f"3 {qs[1]:.0f}-{qs[2]:.0f}", f"4 fearful >{qs[2]:.0f}"]
        report("VIX LEVEL (previous close)", rows,
               lambda d: band(prev_vix(d), qs + [1e9], labels),
               a.min_bucket, a.split_date, order=labels)

    def vix_chg(date):
        i = vix_ix.get(date)
        if i is None or i < 2:
            return None
        a_, b_ = vix[vdates[i - 1]][1], vix[vdates[i - 2]][1]
        return (a_ - b_) / b_ * 100.0 if b_ else None

    report("VIX 1-DAY CHANGE (into the session)", rows,
           lambda d: band(vix_chg(d), [-5, -1, 1, 5, 1e9],
                          ["VIX -5%+", "-5..-1", "flat", "1..5", "VIX +5%+"]),
           a.min_bucket, a.split_date,
           order=["VIX -5%+", "-5..-1", "flat", "1..5", "VIX +5%+"])

    if nifty:
        ndates = sorted(nifty)
        nix = {d: i for i, d in enumerate(ndates)}

        def nifty_prev(date):
            i = nix.get(date)
            if i is None or i < 2:
                return None
            c1, c2 = nifty[ndates[i - 1]][1], nifty[ndates[i - 2]][1]
            return (c1 - c2) / c2 * 100.0 if c2 else None

        def nifty_gap(date):
            i = nix.get(date)
            if i is None or i == 0:
                return None
            o, pc = nifty[date][0], nifty[ndates[i - 1]][1]
            return (o - pc) / pc * 100.0 if pc else None

        report("NIFTY PREVIOUS DAY", rows,
               lambda d: band(nifty_prev(d), [-1, -0.3, 0.3, 1, 1e9],
                              ["N -1%+", "-1..-0.3", "flat", "0.3..1", "N +1%+"]),
               a.min_bucket, a.split_date,
               order=["N -1%+", "-1..-0.3", "flat", "0.3..1", "N +1%+"])
        report("NIFTY OPENING GAP", rows,
               lambda d: band(nifty_gap(d), [-0.5, -0.15, 0.15, 0.5, 1e9],
                              ["gap -0.5%+", "-0.5..-0.15", "flat",
                               "0.15..0.5", "gap +0.5%+"]),
               a.min_bucket, a.split_date,
               order=["gap -0.5%+", "-0.5..-0.15", "flat", "0.15..0.5", "gap +0.5%+"])

    print(f"\n{'-' * 70}")
    print("Judge on the OOS column only. A bucket is worth acting on if its")
    print("held-out Avg R is clearly below breakeven (skip it) or clearly")
    print(f"above the book average of ~{ar:.2f} (concentrate there) — and if the")
    print("in-sample and held-out figures agree. They often do not.")


if __name__ == "__main__":
    main()
