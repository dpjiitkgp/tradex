#!/usr/bin/env python3
"""Did the book's losing quarters coincide with a falling market?

The book is SHORT-only, so the naive expectation is that it profits when the
index falls. If the losses instead cluster in down quarters, that inverts the
intuition and says something about what the strategy is actually capturing.

Establishes the fact only. No treatment, no filter, no conclusion about what
to do — the point is to see whether any relationship exists before reasoning
about one.

Per quarter it reports:
  * the book's Avg R and trade count
  * NIFTY 50's quarterly return (close-to-close over the quarter)
  * mean India VIX (volatility regime)

and then the correlation across quarters, with the caveat that ~31 quarters is
a small sample for a correlation and one outlier quarter can carry it.

Usage:
    python3 quarterly_regime.py --run 110
    python3 quarterly_regime.py --run 110 --symbols-from config/tune30_live.toml
"""
import argparse
import sqlite3
import statistics
import sys
import tomllib

DB = "data/papertrader.db"
NIFTY, VIX = 256265, 264969


def quarter_of(datestr):
    y, m = int(datestr[:4]), int(datestr[5:7])
    return f"{y}Q{(m - 1) // 3 + 1}"


def book_quarters(con, run_id, symbols):
    """(quarter) -> (n, avg_r) for the book's closed trades."""
    placeholders = ",".join("?" * len(symbols))
    rows = con.execute(
        f"""SELECT date(t.entry_ts), t.r_multiple
            FROM trades t JOIN instruments i ON i.instrument_token = t.instrument_token
            WHERE t.run_id = ? AND t.r_multiple IS NOT NULL
              AND i.tradingsymbol IN ({placeholders})""",
        [run_id, *symbols],
    ).fetchall()
    by_q = {}
    for d, r in rows:
        by_q.setdefault(quarter_of(d), []).append(r)
    return {q: (len(v), statistics.fmean(v)) for q, v in by_q.items()}


def index_quarters(con, token):
    """(quarter) -> (first_close, last_close, mean_close) from 1m bars.

    Uses the first and last bar of each quarter rather than a daily series,
    because that is the granularity actually cached here.
    """
    rows = con.execute(
        """SELECT date(ts), ts, close FROM candles
           WHERE instrument_token = ? AND interval = '1m' ORDER BY ts""",
        (token,),
    ).fetchall()
    agg = {}
    for d, ts, close in rows:
        q = quarter_of(d)
        cur = agg.get(q)
        if cur is None:
            agg[q] = [close, close, [close]]
        else:
            cur[1] = close
            cur[2].append(close)
    return {q: (v[0], v[1], statistics.fmean(v[2])) for q, v in agg.items()}


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--run", type=int, default=110)
    ap.add_argument("--db", default=DB)
    ap.add_argument("--symbols-from", default="config/tune30_live.toml")
    ap.add_argument("--breakeven", type=float, default=0.061)
    a = ap.parse_args()

    cfg = tomllib.load(open(a.symbols_from, "rb"))
    symbols = [s["symbol"] for s in cfg["stock"]]

    con = sqlite3.connect(a.db)
    con.execute("PRAGMA busy_timeout=20000")
    book = book_quarters(con, a.run, symbols)
    nifty = index_quarters(con, NIFTY)
    vix = index_quarters(con, VIX)

    quarters = sorted(q for q in book if q in nifty)
    if not quarters:
        sys.exit("no overlapping quarters — is the index data cached?")

    print(f"run {a.run}, {len(symbols)} stocks, breakeven {a.breakeven} R\n")
    print(f"{'quarter':<9}{'trades':>7}{'avg R':>9}{'net R':>9}"
          f"{'NIFTY %':>10}{'VIX':>7}   market")
    print("-" * 66)

    rows = []
    for q in quarters:
        n, r = book[q]
        o, c, _ = nifty[q]
        ret = (c - o) / o * 100
        v = vix[q][2] if q in vix else float("nan")
        # Deliberately coarse buckets: with ~31 quarters, finer ones would be
        # reading noise.
        label = "DOWN" if ret < -2 else ("UP" if ret > 2 else "flat")
        rows.append((q, n, r, ret, v, label))
        print(f"{q:<9}{n:>7}{r:>+9.3f}{r - a.breakeven:>+9.3f}"
              f"{ret:>+10.1f}{v:>7.1f}   {label}")

    print()
    for label in ("DOWN", "flat", "UP"):
        sel = [x for x in rows if x[5] == label]
        if not sel:
            continue
        n = sum(x[1] for x in sel)
        # Trade-weighted, not a mean of quarterly means: quarters differ a lot
        # in trade count and an unweighted average would let a thin quarter
        # count as much as a busy one.
        avg = sum(x[1] * x[2] for x in sel) / n
        beat = sum(1 for x in sel if x[2] > a.breakeven)
        print(f"{label:<5} quarters={len(sel):<3} trades={n:<6} "
              f"avg R={avg:+.4f}  net={avg - a.breakeven:+.4f}  "
              f"beat breakeven in {beat}/{len(sel)}")

    xs = [x[3] for x in rows]
    ys = [x[2] for x in rows]
    if len(xs) > 2:
        print(f"\ncorrelation(NIFTY quarterly %, book Avg R) = "
              f"{statistics.correlation(xs, ys):+.3f}  over {len(xs)} quarters")
        vs = [(x[4], x[2]) for x in rows if x[4] == x[4]]
        if len(vs) > 2:
            print(f"correlation(mean VIX, book Avg R)          = "
                  f"{statistics.correlation([v for v, _ in vs], [r for _, r in vs]):+.3f}")
    print("\nNote: these 25 stocks were selected on trailing performance, so the LEVEL of")
    print("Avg R here is optimistic. The question asked is about the RELATIONSHIP to the")
    print("market, which selection bias distorts less — but does not leave untouched.")


if __name__ == "__main__":
    main()
