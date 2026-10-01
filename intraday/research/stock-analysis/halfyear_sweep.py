#!/usr/bin/env python3
"""Walk-forward at HALF-YEAR granularity.

Re-selects the basket every 6 months using the previous N half-years, then
trades it for the next 6 months. Compared against the same sweep at yearly
granularity, this answers whether re-selecting more often helps — the yearly
sweep already showed shorter windows beating longer ones.

Caution built into the output: a 6-month window gives each stock only ~15
trades to rank on, where the standard error is 1.4/sqrt(15) = 0.36 R. At that
point the ranking is mostly noise, so `avg picks` and the per-period spread
matter as much as the pooled figure.

Avg R is gross; breakeven 0.061 R.
"""
import argparse
import csv
import math

BREAKEVEN_R = 0.061
SIGMA = 1.4
DATA = "research/long-baseline/analysis/short_halfyear_110.tsv"


NOT_STOCKS = {"NIFTY 50", "INDIA VIX"}


def load(path):
    d = {}
    periods = set()
    for r in csv.DictReader(open(path), delimiter="\t"):
        if r["sym"] in NOT_STOCKS:
            continue
        d.setdefault(r["sym"], {})[r["period"]] = (
            int(r["n"]), float(r["avg_r"]), float(r["win_pct"]))
        periods.add(r["period"])
    return d, sorted(periods)


def agg(yrs, keys):
    v = [yrs[k] for k in keys if k in yrs]
    n = sum(x[0] for x in v)
    if n == 0:
        return 0, 0.0, 0.0
    return n, sum(x[0]*x[1] for x in v)/n, sum(x[0]*x[2] for x in v)/n


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--topn", type=int, default=15)
    ap.add_argument("--min-win", type=float, default=48.0)
    ap.add_argument("--min-trades", type=int, default=10)
    ap.add_argument("--verbose", action="store_true")
    ap.add_argument("--data", default=DATA)
    ap.add_argument("--lookbacks", default="1,2,3,4,6,8,10",
                    help="comma-separated lookbacks, in periods")
    ap.add_argument("--unit", default="half-yr",
                    help="label for one period, e.g. month")
    a = ap.parse_args()

    d, periods = load(a.data)
    print(f"periods available: {len(periods)}  ({periods[0]} .. {periods[-1]})")
    print(f"selection: top {a.topn} by Avg R, win% >= {a.min_win}, "
          f">= {a.min_trades} trades in the training window\n")
    print(f"{'lookback':<12}{'tests':>7}{'avg picks':>11}{'trades':>9}"
          f"{'pooled AvgR':>13}{'xBE':>7}{'net':>9}{'beat cost':>11}{'median n/stock':>16}")
    print("-" * 95)

    for lb in [int(x) for x in a.lookbacks.split(',')]:
        tn = 0
        ts = 0.0
        ok = tests = 0
        picks_n = []
        per_stock_n = []
        rows = []
        for i in range(lb, len(periods)):
            train = periods[i-lb:i]
            test = periods[i]
            c = []
            for sym, ps in d.items():
                n, r, w = agg(ps, train)
                if n < a.min_trades or w < a.min_win:
                    continue
                per_stock_n.append(n)
                c.append((sym, r))
            c.sort(key=lambda x: -x[1])
            picks = [s for s, _ in c[:a.topn]]
            if not picks:
                continue
            n = 0
            s = 0.0
            for sym in picks:
                x, y, _ = agg(d[sym], [test])
                n += x
                s += x*y
            if not n:
                continue
            r = s/n
            tn += n
            ts += s
            tests += 1
            ok += r > BREAKEVEN_R
            picks_n.append(len(picks))
            rows.append((test, len(picks), n, r))
        if not tests:
            continue
        p = ts/tn
        med = sorted(per_stock_n)[len(per_stock_n)//2] if per_stock_n else 0
        if a.verbose and lb in (1, 2):
            for test, np_, n, r in rows:
                print(f"    {test}: {np_:>3} picks {n:>5} trades  AvgR {r:>7.3f}"
                      f"  net {r-BREAKEVEN_R:>+7.3f}")
        lbl = f"{lb} {a.unit}"
        print(f"{lbl:<12}{tests:>7}{sum(picks_n)/len(picks_n):>11.1f}{tn:>9}"
              f"{p:>13.4f}{p/BREAKEVEN_R:>6.2f}x{p-BREAKEVEN_R:>+9.4f}"
              f"{(str(ok)+'/'+str(tests)):>11}{med:>16}")

    print(f"\nmedian n/stock is the trade count each ranking decision rests on.")
    print(f"standard error at that n is 1.4/sqrt(n) — compare it to the ~0.24 R")
    print(f"spread you are trying to rank on before trusting a short window.")


if __name__ == "__main__":
    main()
