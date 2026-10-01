#!/usr/bin/env python3
"""Walk-forward test of the stock-selection rule.

A single in-sample/held-out split is not enough on this dataset: an effect
can clear it because one strong year happened to land on the right side of
the cut (the 11:00 entry window did exactly that). This re-selects at the
start of every year using only data available then, trades that basket for
the following year, and reports each year separately.

Compares selection with and without a liquidity floor, because the floor is
what would let the book scale past ~Rs1-2L — and the question is whether it
costs edge to get that capacity.

Usage:
    python3 walkforward.py
    python3 walkforward.py --topn 10 --min-turnover 4500000
"""
import argparse
import collections
import csv
import sqlite3

BREAKEVEN_R = 0.061
DATA = "research/long-baseline/analysis/short_yearly_110.tsv"


def load(path):
    d = collections.defaultdict(dict)
    for r in csv.DictReader(open(path), delimiter="\t"):
        d[r["sym"]][int(r["yr"])] = (int(r["n"]), float(r["avg_r"]),
                                     float(r["win_pct"]))
    return d


def agg(years_data, years, idx=1):
    v = [years_data[y] for y in years if y in years_data]
    n = sum(x[0] for x in v)
    return (n, sum(x[0] * x[idx] for x in v) / n) if n else (0, 0.0)


def select(d, train_years, topn, min_turn, turnover, min_trades):
    """Rank by Avg R over train_years; require every year non-negative."""
    cands = []
    for sym, yrs in d.items():
        if min_turn and turnover.get(sym, 0) < min_turn:
            continue
        vals = [yrs.get(y) for y in train_years]
        present = [v for v in vals if v is not None]
        if len(present) < len(train_years):
            continue                      # not tradeable for the whole window
        if any(v[0] < min_trades for v in present):
            continue                      # too thin to rank on
        if any(v[1] < 0 for v in present):
            continue                      # a losing year disqualifies
        n, r = agg(yrs, train_years)
        cands.append((sym, r))
    cands.sort(key=lambda x: -x[1])
    return [s for s, _ in cands[:topn]]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--topn", type=int, default=10)
    ap.add_argument("--min-turnover", type=float, default=4_500_000)
    ap.add_argument("--min-trades", type=int, default=15)
    ap.add_argument("--db", default="data/papertrader.db")
    a = ap.parse_args()

    db = sqlite3.connect(a.db)
    turnover = {s: t for s, t in db.execute(
        "SELECT i.tradingsymbol, AVG(c.close*c.volume) FROM candles c "
        "JOIN instruments i ON i.instrument_token=c.instrument_token "
        "WHERE c.ts>='2026-06-01' GROUP BY i.tradingsymbol") if t}
    d = load(DATA)

    print("=" * 78)
    print(f"WALK-FORWARD: re-select each year, trade the next, top {a.topn}")
    print("Selection uses only years strictly before the test year.")
    print("=" * 78)

    for label, minturn in (("no liquidity floor", 0),
                           (f"turnover >= Rs{a.min_turnover/1e6:.1f}M/min", a.min_turnover)):
        print(f"\n### {label}")
        print(f"  {'test yr':<9}{'picks':>6}{'trades':>8}{'Avg R':>9}"
              f"{'xBE':>7}{'net':>9}   overlap w/ prev")
        prev = set()
        tot_n = 0
        tot_r = 0.0
        for test in range(2022, 2027):
            train = list(range(2019, test))
            picks = select(d, train, a.topn, minturn, turnover, a.min_trades)
            if not picks:
                print(f"  {test:<9}{'0':>6}   (none qualified)")
                continue
            n = 0
            s = 0.0
            for sym in picks:
                tn, tr = agg(d[sym], [test])
                n += tn
                s += tn * tr
            if not n:
                continue
            r = s / n
            tot_n += n
            tot_r += s
            ov = len(set(picks) & prev)
            prev = set(picks)
            print(f"  {test:<9}{len(picks):>6}{n:>8}{r:>9.3f}"
                  f"{r/BREAKEVEN_R:>6.2f}x{r-BREAKEVEN_R:>+9.3f}   {ov}/{len(picks)}")
        if tot_n:
            allr = tot_r / tot_n
            print(f"  {'POOLED':<9}{'':>6}{tot_n:>8}{allr:>9.3f}"
                  f"{allr/BREAKEVEN_R:>6.2f}x{allr-BREAKEVEN_R:>+9.3f}")

    print("\n" + "-" * 78)
    print("Every test year above is genuinely out of sample for its own basket.")
    print("Judge on consistency across years, not the pooled number — one strong")
    print("year can carry a pooled figure while the rule itself is worthless.")


if __name__ == "__main__":
    main()
