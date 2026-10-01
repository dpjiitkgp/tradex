#!/usr/bin/env python3
"""How long should the training window be?

For each window length L, re-selects a basket using ONLY the L years before
the test year, then trades it for that year. Repeats for every test year the
data allows. A 1-year window gives the most test years (7) because it needs
the least history.

Two selection rules are compared, since the 3-year work showed they behave
very differently:
  * top-N ranking     - fixed basket size, take the best N by Avg R
  * threshold         - every stock clearing win% and Avg R bars (size varies)

Avg R is gross; breakeven is 0.061 R. `net` subtracts it.
"""
import argparse
import csv

BREAKEVEN_R = 0.061
DATA = "research/long-baseline/analysis/short_yearly_110.tsv"


def load(path):
    d = {}
    for r in csv.DictReader(open(path), delimiter="\t"):
        d.setdefault(r["sym"], {})[int(r["yr"])] = (
            int(r["n"]), float(r["avg_r"]), float(r["win_pct"]))
    return d


def agg(yrs, years):
    v = [yrs[y] for y in years if y in yrs]
    n = sum(x[0] for x in v)
    if n == 0:
        return 0, 0.0, 0.0
    return n, sum(x[0]*x[1] for x in v)/n, sum(x[0]*x[2] for x in v)/n


def select(d, train, mode, topn, min_win, min_r, min_trades):
    c = []
    for sym, yrs in d.items():
        n, r, w = agg(yrs, train)
        if n < min_trades or w < min_win:
            continue
        if mode == "threshold" and r <= min_r:
            continue
        c.append((sym, r))
    c.sort(key=lambda x: -x[1])
    return [s for s, _ in c[:topn]] if mode == "topn" else [s for s, _ in c]


def run(d, lb, mode, topn, min_win, min_r, min_trades, verbose):
    tn = 0
    ts = 0.0
    ok = yrs = 0
    picks_seen = []
    rows = []
    for test in range(2020, 2027):
        train = list(range(test - lb, test))
        if train[0] < 2019:
            continue
        picks = select(d, train, mode, topn, min_win, min_r, min_trades)
        if not picks:
            rows.append((test, 0, 0, None))
            continue
        n = 0
        s = 0.0
        for sym in picks:
            a, b, _ = agg(d[sym], [test])
            n += a
            s += a*b
        if not n:
            continue
        r = s/n
        tn += n
        ts += s
        yrs += 1
        ok += r > BREAKEVEN_R
        picks_seen.append(len(picks))
        rows.append((test, len(picks), n, r))
    if verbose:
        for test, np_, n, r in rows:
            if r is None:
                print(f"    {test}: {np_:>3} picks   (none qualified)")
            else:
                print(f"    {test}: {np_:>3} picks {n:>5} trades  AvgR {r:>7.3f}"
                      f"  {r/BREAKEVEN_R:>5.2f}x  net {r-BREAKEVEN_R:>+7.3f}")
    p = ts/tn if tn else 0.0
    avgp = sum(picks_seen)/len(picks_seen) if picks_seen else 0
    return p, ok, yrs, tn, avgp


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--topn", type=int, default=15)
    ap.add_argument("--min-win", type=float, default=48.0)
    ap.add_argument("--min-r", type=float, default=0.200)
    ap.add_argument("--min-trades", type=int, default=20)
    ap.add_argument("--verbose", action="store_true")
    a = ap.parse_args()
    d = load(DATA)

    for mode, label in (("topn", f"TOP {a.topn} BY AVG R (win% >= {a.min_win})"),
                        ("threshold", f"THRESHOLD (win% > {a.min_win}, Avg R > {a.min_r})")):
        print("=" * 78)
        print(label)
        print("=" * 78)
        print(f"{'window':<10}{'test yrs':>9}{'avg picks':>11}{'trades':>9}"
              f"{'pooled AvgR':>13}{'xBE':>7}{'net':>9}{'beat cost':>11}")
        print("-" * 78)
        for lb in (1, 2, 3, 4, 5):
            if a.verbose:
                print(f"  {lb}-year window:")
            p, ok, yrs, tn, avgp = run(d, lb, mode, a.topn, a.min_win,
                                       a.min_r, a.min_trades, a.verbose)
            if not yrs:
                continue
            print(f"{str(lb)+'-year':<10}{yrs:>9}{avgp:>11.1f}{tn:>9}"
                  f"{p:>13.4f}{p/BREAKEVEN_R:>6.2f}x{p-BREAKEVEN_R:>+9.4f}"
                  f"{(str(ok)+'/'+str(yrs)):>11}")
        print()


if __name__ == "__main__":
    main()
