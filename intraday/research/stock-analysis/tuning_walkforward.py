#!/usr/bin/env python3
"""Does per-stock parameter tuning survive walk-forward?

The live book assigns each stock its own (MRM, trail) chosen from a 12-point
grid on all available history. That is 2 fitted values per stock on ~300
trades, where the standard error is ~0.081 R and the entire grid spans about
0.05 R — so the tuning cannot resolve what it claims to, and picking the best
of 12 per stock should yield a large apparent gain from noise alone.

This tests it honestly. At each quarterly rebalance:

  * SHARED   — every stock trades one config, itself chosen on training data
  * TUNED    — each stock trades the config that was best for IT in training
  * SHRUNK   — per-stock choice only when that stock has enough training
               trades to distinguish configs; otherwise fall back to shared

...then all three are scored on the NEXT quarter, which none of them saw.

If TUNED does not beat SHARED out of sample, the 45 fitted values in the live
config are decoration at best.
"""
import argparse
import collections
import csv
import math

BREAKEVEN_R = 0.061
SIGMA = 1.4
DATA = "research/long-baseline/analysis/grid_quarterly.tsv"
# run id -> (MRM, trail), from the tune30 grid labels
GRID = {106: (1.0, 0.0025), 107: (1.0, 0.0075), 108: (1.0, 0.015),
        109: (3.0, 0.0025), 110: (3.0, 0.0075), 111: (3.0, 0.015),
        112: (6.0, 0.0025), 113: (6.0, 0.0075), 114: (6.0, 0.015),
        115: (10.0, 0.0025), 116: (10.0, 0.0075), 117: (10.0, 0.015)}


def load(path):
    """(run, sym, period) -> (n, avg_r, win)"""
    d = {}
    periods, syms = set(), set()
    for r in csv.DictReader(open(path), delimiter="\t"):
        run = int(r["run_id"])
        d[(run, r["sym"], r["period"])] = (int(r["n"]), float(r["avg_r"]),
                                           float(r["win_pct"]))
        periods.add(r["period"])
        syms.add(r["sym"])
    return d, sorted(periods), sorted(syms)


def agg(d, run, sym, keys):
    v = [d[(run, sym, k)] for k in keys if (run, sym, k) in d]
    n = sum(x[0] for x in v)
    if n == 0:
        return 0, 0.0, 0.0
    return n, sum(x[0]*x[1] for x in v)/n, sum(x[0]*x[2] for x in v)/n


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--data", default=DATA)
    ap.add_argument("--topn", type=int, default=25)
    ap.add_argument("--lookback", type=int, default=8, help="quarters")
    ap.add_argument("--min-win", type=float, default=48.0)
    ap.add_argument("--min-trades", type=int, default=15)
    ap.add_argument("--shrink-min", type=int, default=150,
                    help="training trades a stock needs before its own config is trusted")
    a = ap.parse_args()

    d, periods, syms = load(a.data)
    runs = sorted(GRID)
    print(f"{len(runs)} grid configs x {len(syms)} stocks x {len(periods)} quarters\n")

    out = {k: [0, 0.0, 0, 0] for k in ("shared", "tuned", "shrunk")}  # n, sum, beats, tests
    per_q = []
    tuned_used = shrunk_used = shrunk_total = 0

    for i in range(a.lookback, len(periods)):
        train, test = periods[i-a.lookback:i], periods[i]

        # 1. the shared config: best single config across the whole universe in training
        best_run, best_r = None, None
        for run in runs:
            tot_n = tot_s = 0
            for s in syms:
                n, r, _ = agg(d, run, s, train)
                tot_n += n
                tot_s += n*r
            if tot_n:
                v = tot_s/tot_n
                if best_r is None or v > best_r:
                    best_run, best_r = run, v

        # 2. selection uses the shared config's training record (same for all arms,
        #    so any difference between arms is the tuning, not the stock picking)
        cands = []
        for s in syms:
            n, r, w = agg(d, best_run, s, train)
            if n >= a.min_trades and w >= a.min_win:
                cands.append((s, r))
        cands.sort(key=lambda x: -x[1])
        picks = [s for s, _ in cands[:a.topn]]
        if not picks:
            continue

        row = {"q": test, "shared_run": GRID[best_run]}
        for arm in ("shared", "tuned", "shrunk"):
            n_tot = s_tot = 0
            for s in picks:
                if arm == "shared":
                    run = best_run
                else:
                    br, bv, bn = None, None, 0
                    for run_ in runs:
                        n_, r_, _ = agg(d, run_, s, train)
                        if n_ and (bv is None or r_ > bv):
                            br, bv, bn = run_, r_, n_
                    if arm == "tuned":
                        run = br or best_run
                        if run != best_run:
                            tuned_used += 1
                    else:  # shrunk
                        shrunk_total += 1
                        if bn >= a.shrink_min and br:
                            run = br
                            if run != best_run:
                                shrunk_used += 1
                        else:
                            run = best_run
                tn, tr, _ = agg(d, run, s, [test])
                n_tot += tn
                s_tot += tn*tr
            if n_tot:
                v = s_tot/n_tot
                out[arm][0] += n_tot
                out[arm][1] += s_tot
                out[arm][3] += 1
                out[arm][2] += v > BREAKEVEN_R
                row[arm] = v
        per_q.append(row)

    print(f"{'arm':<10}{'net Avg R':>11}{'trades':>9}{'beat cost':>12}{'Rs/yr on 1L':>14}")
    print("-"*58)
    res = {}
    for arm in ("shared", "tuned", "shrunk"):
        n, s, beat, t = out[arm]
        net = s/n - BREAKEVEN_R
        res[arm] = (net, n)
        print(f"{arm:<10}{net:>+11.4f}{n:>9}{f'{beat}/{t}':>12}"
              f"{net*3740*200:>13,.0f}")
    ns, nn = res["shared"]
    for arm in ("tuned", "shrunk"):
        v, n = res[arm]
        se = math.sqrt(SIGMA**2/n + SIGMA**2/nn)
        print(f"\n{arm} minus shared: {v-ns:+.4f}  (SE {se:.4f}, t={(v-ns)/se:+.2f}) "
              f"-> {'significant' if abs((v-ns)/se) > 1.96 else 'NOT significant'}")
    print(f"\nper-stock config differed from shared in {tuned_used} stock-quarters (tuned)")
    print(f"shrunk trusted the per-stock choice {shrunk_used}/{shrunk_total} times "
          f"(>= {a.shrink_min} training trades)")

    print(f"\n{'quarter':<10}{'shared':>10}{'tuned':>10}{'shrunk':>10}   shared config")
    print("-"*62)
    for r in per_q:
        m, tr = r["shared_run"]
        print(f"{r['q']:<10}{r.get('shared',0):>10.3f}{r.get('tuned',0):>10.3f}"
              f"{r.get('shrunk',0):>10.3f}   MRM {m} / trail {tr}")


if __name__ == "__main__":
    main()
