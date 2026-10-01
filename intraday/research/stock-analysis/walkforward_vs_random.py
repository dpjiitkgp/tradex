#!/usr/bin/env python3
"""The decisive test: does the walk-forward book beat a coin flip?

Everything this project has claimed rests on one unexamined assumption — that
the entry signal carries information. Measured against a random-entry control
on the 25 currently-selected stocks, it beat random by +0.038 R pooled but was
*worse* than random in five of eight years, with all of its value in 2024-26.
That is also the window those 25 stocks were selected from, so the result is
uninterpretable: "the strategy works" and "the stocks were picked in hindsight"
predict exactly the same thing.

This separates them. At each quarterly rebalance the universe is re-selected
using only trailing data, exactly as the live rule does, and then BOTH arms —
the real entry and the coin flip — trade that same selected book over the
following quarter. Nothing selects on data it then scores.

Two questions, answered separately:

  1. Does the ENTRY add value on a walk-forward-selected book?
     -> strategy vs random, both on the same quarterly picks.
  2. Does the SELECTION add value at all?
     -> selected picks vs the whole universe, within each arm.

If (1) holds pre-2024 as well as after, the strategy survives. If it does not,
the edge was the selection, and the entry signal is decoration.

    python3 walkforward_vs_random.py --strategy-run 110 --random-run <id>
"""
import argparse
import math
import sqlite3
import statistics

BREAKEVEN = 0.061
SIGMA = 1.4


def quarter(datestr):
    y, m = int(datestr[:4]), int(datestr[5:7])
    return y * 4 + (m - 1) // 3          # sortable integer quarter


def qname(q):
    return f"{q // 4}Q{q % 4 + 1}"


def per_stock_quarter(con, run_id):
    """(symbol, quarter) -> list of r_multiples."""
    rows = con.execute(
        """SELECT i.tradingsymbol, date(t.entry_ts), t.r_multiple
           FROM trades t JOIN instruments i ON i.instrument_token = t.instrument_token
           WHERE t.run_id = ? AND t.r_multiple IS NOT NULL""",
        (run_id,),
    ).fetchall()
    out = {}
    for sym, d, r in rows:
        out.setdefault((sym, quarter(d)), []).append(r)
    return out


def agg(data, sym, quarters):
    rs = [r for q in quarters for r in data.get((sym, q), [])]
    if not rs:
        return 0, 0.0, 0.0
    return (len(rs), statistics.fmean(rs),
            100.0 * sum(1 for r in rs if r > 0) / len(rs))


def t_of(a, b):
    if not a or not b:
        return None
    d = statistics.fmean(a) - statistics.fmean(b)
    se = SIGMA * math.sqrt(1 / len(a) + 1 / len(b))
    return d, se, d / se


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--db", default="data/papertrader.db")
    ap.add_argument("--strategy-run", type=int, default=110)
    ap.add_argument("--random-run", type=int, required=True)
    ap.add_argument("--topn", type=int, default=25)
    ap.add_argument("--lookback", type=int, default=8, help="quarters of trailing data")
    ap.add_argument("--min-win", type=float, default=48.0)
    ap.add_argument("--min-trades", type=int, default=15)
    a = ap.parse_args()

    con = sqlite3.connect(a.db)
    con.execute("PRAGMA busy_timeout=60000")
    strat = per_stock_quarter(con, a.strategy_run)
    rand = per_stock_quarter(con, a.random_run)

    syms = sorted({s for s, _ in strat})
    quarters = sorted({q for _, q in strat})
    print(f"strategy run {a.strategy_run} | random run {a.random_run}")
    print(f"{len(syms)} symbols, {len(quarters)} quarters, "
          f"top {a.topn} by trailing {a.lookback}Q Avg R, win% >= {a.min_win}\n")

    # Walk forward: select on [i-lookback, i), score on i.
    # Per-quarter trade lists are retained so the same pooled numbers can be
    # re-sliced by era without re-deriving the selection.
    per_q = []          # (test_quarter, picked_strat, picked_rand, univ_strat, univ_rand)
    for i in range(a.lookback, len(quarters)):
        train = quarters[i - a.lookback:i]
        test = quarters[i]

        cands = []
        for s in syms:
            n, r, w = agg(strat, s, train)
            if n >= a.min_trades and w >= a.min_win:
                cands.append((s, r))
        cands.sort(key=lambda x: -x[1])
        picks = [s for s, _ in cands[:a.topn]]
        if len(picks) < 5:
            continue

        ps = [r for s in picks for r in strat.get((s, test), [])]
        pr = [r for s in picks for r in rand.get((s, test), [])]
        us = [r for s in syms for r in strat.get((s, test), [])]
        ur = [r for s in syms for r in rand.get((s, test), [])]
        if not (ps and pr):
            continue
        per_q.append((test, ps, pr, us, ur))

    def pool(idx, lo=None, hi=None):
        return [r for row in per_q
                if (lo is None or row[0] >= lo) and (hi is None or row[0] < hi)
                for r in row[idx]]

    def show(label, s, r):
        res = t_of(s, r)
        if not res:
            print(f"{label:<34}  insufficient data")
            return
        d, se, t = res
        print(f"{label:<34}{len(s):>8}{statistics.fmean(s)-BREAKEVEN:>+10.4f}"
              f"{statistics.fmean(r)-BREAKEVEN:>+10.4f}{d:>+9.4f}{se:>7.4f}{t:>+7.2f}")

    CUT = 2024 * 4   # quarter index for 2024Q1

    print("QUESTION 1 - does the ENTRY beat a coin flip on the selected book?")
    print(f"{'':<34}{'trades':>8}{'strat':>10}{'random':>10}{'edge':>9}{'SE':>7}{'t':>7}")
    print("-" * 85)
    show("pooled, all quarters", pool(1), pool(2))
    # The pre-2024 half is the half no hindsight could reach: the live 25 were
    # ranked on trailing data as of 2026, so if the entry only works after
    # 2024 the edge is the selection and not the signal.
    show("  quarters before 2024", pool(1, hi=CUT), pool(2, hi=CUT))
    show("  quarters from 2024", pool(1, lo=CUT), pool(2, lo=CUT))

    print("\nQUESTION 2 - does the SELECTION add value, within each arm?")
    print(f"{'':<34}{'trades':>8}{'picked':>10}{'all 249':>10}{'edge':>9}{'SE':>7}{'t':>7}")
    print("-" * 85)
    show("strategy: picked vs whole universe", pool(1), pool(3))
    show("random:   picked vs whole universe", pool(2), pool(4))

    print("\nPER QUARTER (net R)")
    print(f"{'quarter':<9}{'trades':>8}{'strategy':>10}{'random':>9}{'edge':>9}{'all-univ':>10}")
    print("-" * 56)
    pos = 0
    for q, ps, pr, us, _ in per_q:
        sv = statistics.fmean(ps) - BREAKEVEN
        rv = statistics.fmean(pr) - BREAKEVEN
        pos += (sv - rv) > 0
        print(f"{qname(q):<9}{len(ps):>8}{sv:>+10.3f}{rv:>+9.3f}{sv-rv:>+9.3f}"
              f"{statistics.fmean(us)-BREAKEVEN:>+10.3f}")
    print(f"\nentry beat random in {pos}/{len(per_q)} quarters")


if __name__ == "__main__":
    main()
