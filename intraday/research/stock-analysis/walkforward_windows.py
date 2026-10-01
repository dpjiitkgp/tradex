#!/usr/bin/env python3
"""Walk-forward the LIVE selection method over rolling training windows.

Reproduces exactly how the live 15 were chosen — rank by Avg R over the
training window, require win% >= 48 and a minimum trade count — then trades
that basket for the FOLLOWING year only, and repeats.

  train 2019-2023 -> test 2024
  train 2020-2024 -> test 2025
  train 2021-2025 -> test 2026        (5-year rolling; also 4y and 3y)

Every test year is genuinely out of sample for its own basket: nothing from
the test year, or after it, influences selection.

The bar is the 0.061 R breakeven, not zero. A basket at +0.03 R lost money.
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


def agg(yrs, window, idx=1):
    v = [yrs[y] for y in window if y in yrs]
    n = sum(x[0] for x in v)
    return (n, sum(x[0] * x[idx] for x in v) / n) if n else (0, 0.0)


def select(d, train, topn, min_win, min_trades, min_turn, turnover):
    """The live method: rank by Avg R over `train`, with a win-rate floor."""
    cands = []
    for sym, yrs in d.items():
        if min_turn and turnover.get(sym, 0) < min_turn:
            continue
        n, r = agg(yrs, train)
        if n < min_trades:
            continue
        _, w = agg(yrs, train, idx=2)
        if w < min_win:
            continue
        cands.append((sym, r))
    cands.sort(key=lambda x: -x[1])
    return [s for s, _ in cands[:topn]]


def run(d, turnover, lookback, topn, min_win, min_trades, min_turn, label):
    print(f"\n### {label}")
    print(f"  {'train window':<16}{'test':<7}{'picks':>6}{'trades':>8}"
          f"{'Avg R':>9}{'xBE':>7}{'net':>9}")
    tn = 0
    ts = 0.0
    ok = yrs = 0
    for test in range(2022, 2027):
        train = list(range(test - lookback, test))
        if train[0] < 2019:
            continue
        picks = select(d, train, topn, min_win, min_trades, min_turn, turnover)
        if not picks:
            print(f"  {str(train[0])+'-'+str(train[-1]):<16}{test:<7}"
                  f"{0:>6}   (nothing qualified)")
            continue
        n = 0
        s = 0.0
        for sym in picks:
            a, b = agg(d[sym], [test])
            n += a
            s += a * b
        if not n:
            continue
        r = s / n
        tn += n
        ts += s
        yrs += 1
        ok += r > BREAKEVEN_R
        print(f"  {str(train[0])+'-'+str(train[-1]):<16}{test:<7}{len(picks):>6}"
              f"{n:>8}{r:>9.3f}{r/BREAKEVEN_R:>6.2f}x{r-BREAKEVEN_R:>+9.3f}")
    if yrs:
        p = ts / tn
        print(f"  {'POOLED':<23}{'':>6}{tn:>8}{p:>9.3f}{p/BREAKEVEN_R:>6.2f}x"
              f"{p-BREAKEVEN_R:>+9.3f}   -> beat cost in {ok}/{yrs} years")
    return (ts / tn if tn else 0.0), ok, yrs


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--topn", type=int, default=15)
    ap.add_argument("--min-win", type=float, default=48.0)
    ap.add_argument("--min-trades", type=int, default=20)
    ap.add_argument("--db", default="data/papertrader.db")
    a = ap.parse_args()

    db = sqlite3.connect(a.db)
    turnover = {s: t for s, t in db.execute(
        "SELECT i.tradingsymbol, AVG(c.close*c.volume) FROM candles c "
        "JOIN instruments i ON i.instrument_token=c.instrument_token "
        "WHERE c.ts>='2026-06-01' GROUP BY i.tradingsymbol") if t}
    d = load(DATA)

    print("=" * 76)
    print(f"WALK-FORWARD OF THE LIVE SELECTION METHOD")
    print(f"top {a.topn} by Avg R, win% >= {a.min_win}, >= {a.min_trades} trades")
    print(f"breakeven = {BREAKEVEN_R} R")
    print("=" * 76)

    summary = []
    for lb in (5, 4, 3):
        r, ok, yrs = run(d, turnover, lb, a.topn, a.min_win, a.min_trades, 0,
                         f"rolling {lb}-year window — LIVE METHOD (no liquidity floor)")
        summary.append((f"{lb}y  live method", r, ok, yrs))
    for lb in (5, 4, 3):
        r, ok, yrs = run(d, turnover, lb, a.topn, a.min_win, a.min_trades, 4_500_000,
                         f"rolling {lb}-year window — plus Rs4.5M/min liquidity floor")
        summary.append((f"{lb}y  + liquidity", r, ok, yrs))

    print("\n" + "=" * 76)
    print(f"{'variant':<22}{'pooled Avg R':>14}{'xBE':>7}{'years beating cost':>21}")
    print("-" * 76)
    for lbl, r, ok, yrs in summary:
        print(f"{lbl:<22}{r:>14.3f}{r/BREAKEVEN_R:>6.2f}x{(str(ok)+'/'+str(yrs)):>21}")


if __name__ == "__main__":
    main()
