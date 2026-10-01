#!/usr/bin/env python3
"""Does the book lose when the wider market is trending UP?

The quarterly view suggested it does, but 32 quarters is a thin sample and the
quarters interleave with a selection-bias era effect. This conditions each
TRADE on the market's trailing trend instead, which turns 32 observations into
thousands and — because the trend is measured strictly before the entry bar —
produces something that could actually be used live.

Every number here uses only NIFTY closes from days STRICTLY BEFORE the entry
date. No same-day information, so nothing found is hindsight.

Reports each trend bucket in-sample and out-of-sample, plus a split by era,
because the 25-stock book was selected on trailing performance and its recent
years are flattered. A regime effect worth believing has to survive both.

Usage:
    python3 market_trend_conditioning.py --run 110
    python3 market_trend_conditioning.py --run 110 --lookback 10
"""
import argparse
import sqlite3
import statistics
import tomllib

NIFTY = 256265
BREAKEVEN = 0.061
SIGMA = 1.4  # per-trade R standard deviation, for standard errors


def nifty_daily(con):
    """date -> that day's last NIFTY close, from the cached 1m bars."""
    rows = con.execute(
        """SELECT date(ts) d, close FROM candles
           WHERE instrument_token = ? AND interval = '1m' ORDER BY ts""",
        (NIFTY,),
    ).fetchall()
    out = {}
    for d, c in rows:
        out[d] = c  # later rows overwrite, leaving the day's last close
    return out


def trailing_return(daily, dates_sorted, entry_date, lookback):
    """NIFTY % change over the `lookback` sessions ending the day BEFORE entry.

    Returns None when there is not enough history, rather than a partial
    window — a short window would read as a small move and pollute the flat
    bucket.
    """
    import bisect

    i = bisect.bisect_left(dates_sorted, entry_date)
    if i - lookback < 0 or i == 0:
        return None
    prev = dates_sorted[i - 1]
    past = dates_sorted[i - 1 - lookback]
    a, b = daily[past], daily[prev]
    return (b - a) / a * 100 if a else None


def stats(rs):
    n = len(rs)
    if n == 0:
        return None
    avg = statistics.fmean(rs)
    se = SIGMA / (n**0.5)
    win = 100.0 * sum(1 for r in rs if r > 0) / n
    return n, avg, avg - BREAKEVEN, se, win


def show(title, buckets, order):
    print(f"\n{title}")
    print(f"{'trend bucket':<18}{'trades':>8}{'avg R':>9}{'net R':>9}{'SE':>8}{'win%':>7}{'t vs BE':>9}")
    print("-" * 68)
    for k in order:
        s = stats(buckets.get(k, []))
        if not s:
            continue
        n, avg, net, se, win = s
        print(f"{k:<18}{n:>8}{avg:>+9.3f}{net:>+9.3f}{se:>8.3f}{win:>7.1f}{net / se:>+9.2f}")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--run", type=int, default=110)
    ap.add_argument("--db", default="data/papertrader.db")
    ap.add_argument("--symbols-from", default="config/tune30_live.toml")
    ap.add_argument("--lookback", type=int, default=20, help="sessions of NIFTY trend")
    ap.add_argument("--split-date", default="2024-01-01")
    a = ap.parse_args()

    syms = [s["symbol"] for s in tomllib.load(open(a.symbols_from, "rb"))["stock"]]
    con = sqlite3.connect(a.db)
    con.execute("PRAGMA busy_timeout=30000")
    ph = ",".join("?" * len(syms))
    trades = con.execute(
        f"""SELECT date(t.entry_ts) d, t.r_multiple
            FROM trades t JOIN instruments i ON i.instrument_token = t.instrument_token
            WHERE t.run_id = ? AND t.r_multiple IS NOT NULL
              AND i.tradingsymbol IN ({ph})
            ORDER BY t.entry_ts""",
        [a.run, *syms],
    ).fetchall()

    daily = nifty_daily(con)
    dates = sorted(daily)

    # Thresholds in percent over the lookback window. Coarse on purpose: three
    # buckets on thousands of trades, rather than deciles on hundreds.
    def bucket(r):
        if r is None:
            return None
        if r <= -2.0:
            return "falling (<-2%)"
        if r >= 2.0:
            return "rising (>+2%)"
        return "flat (-2..+2%)"

    order = ["falling (<-2%)", "flat (-2..+2%)", "rising (>+2%)"]
    allb, ins, oos, early, late = ({} for _ in range(5))
    skipped = 0
    for d, r in trades:
        tr = trailing_return(daily, dates, d, a.lookback)
        k = bucket(tr)
        if k is None:
            skipped += 1
            continue
        allb.setdefault(k, []).append(r)
        (ins if d < a.split_date else oos).setdefault(k, []).append(r)
        (early if d < "2024-01-01" else late).setdefault(k, []).append(r)

    print(f"run {a.run} | {len(syms)} stocks | NIFTY trailing {a.lookback} sessions, "
          f"measured before the entry day")
    print(f"{len(trades)} trades, {skipped} skipped for insufficient NIFTY history")
    print(f"breakeven {BREAKEVEN} R; t vs BE = net / SE (|t| > 2 is meaningful)")

    show("ALL TRADES", allb, order)
    show(f"IN SAMPLE (before {a.split_date})", ins, order)
    show(f"OUT OF SAMPLE (from {a.split_date})", oos, order)
    show("2019-2023 only (before the selection window flatters everything)", early, order)
    show("2024-2026 only", late, order)


if __name__ == "__main__":
    main()
