#!/usr/bin/env python3
"""Selection window x refresh cadence, always against a random control.

The hypothesis: if the selection's edge comes from a volatility or regime
property rather than durable stock-picking skill, a SHORT trailing window
should beat a long one — two years averages across regimes, three months
tracks the current one.

An earlier sweep found window length and refresh rate to be noise, but that
was measured without a control, so it could not distinguish "the window does
not matter" from "the selection does not work and nothing about it matters."
Every cell here reports both arms.

## Reading the output

  strat      what the real entry earns on that selected book
  random     what a coin flip earns on the SAME book
  entry      strat - random, the entry's marginal contribution
  sel        random_picked - random_universe, the selection's contribution
             measured on the arm that has no signal at all

`sel` is the honest measure of selection value: if a coin flip gains from a
selection rule, that rule is identifying a property of the stock rather than
a place where this particular signal happens to work.

## Sample size warning

A three-month window holds roughly a tenth the trades of a two-year one, so a
fixed `min_trades` of 15 would silently reject almost every candidate at short
windows and quietly turn the comparison into one about eligibility rather than
about window length. The threshold therefore scales with the window; the
`cands` column reports how many stocks actually qualified so a binding filter
is visible rather than hidden.

    python3 selection_sweep.py --strategy-run 110 --random-run 263
"""
import argparse
import math
import sqlite3
import statistics

BREAKEVEN = 0.061
SIGMA = 1.4
EXCLUDE = {"PIIND", "NIFTY 50", "INDIA VIX"}


def month_of(datestr):
    return int(datestr[:4]) * 12 + int(datestr[5:7]) - 1


def mname(m):
    return f"{m // 12}-{m % 12 + 1:02d}"


def load(con, run_id):
    """(symbol, month) -> [r_multiple]"""
    out = {}
    for s, d, r in con.execute(
        """SELECT i.tradingsymbol, date(t.entry_ts), t.r_multiple
           FROM trades t JOIN instruments i ON i.instrument_token = t.instrument_token
           WHERE t.run_id = ? AND t.r_multiple IS NOT NULL""", (run_id,)):
        if s in EXCLUDE:
            continue
        out.setdefault((s, month_of(d)), []).append(r)
    return out


def agg(data, sym, months):
    rs = [r for m in months for r in data.get((sym, m), [])]
    if not rs:
        return 0, 0.0, 0.0
    return len(rs), statistics.fmean(rs), 100.0 * sum(1 for r in rs if r > 0) / len(rs)


def walk(S, R, syms, months, window, refresh, min_trades, min_win, topn):
    """Returns pooled trade lists and the mean candidate count."""
    ps, pr, us, ur, cand_counts = [], [], [], [], []
    start = months[0] + window
    m = start
    while m <= months[-1]:
        train = list(range(m - window, m))
        test = list(range(m, min(m + refresh, months[-1] + 1)))
        cands = []
        for s in syms:
            n, r, w = agg(S, s, train)
            if n >= min_trades and w >= min_win:
                cands.append((s, r))
        cand_counts.append(len(cands))
        cands.sort(key=lambda x: -x[1])
        picks = [s for s, _ in cands[:topn]]
        if len(picks) >= 5:
            ps += [r for s in picks for mm in test for r in S.get((s, mm), [])]
            pr += [r for s in picks for mm in test for r in R.get((s, mm), [])]
            us += [r for s in syms for mm in test for r in S.get((s, mm), [])]
            ur += [r for s in syms for mm in test for r in R.get((s, mm), [])]
        m += refresh
    return ps, pr, us, ur, (statistics.fmean(cand_counts) if cand_counts else 0)


def t_of(a, b):
    d = statistics.fmean(a) - statistics.fmean(b)
    se = SIGMA * math.sqrt(1 / len(a) + 1 / len(b))
    return d, d / se


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--db", default="data/papertrader.db")
    ap.add_argument("--strategy-run", type=int, default=110)
    ap.add_argument("--random-run", type=int, required=True)
    ap.add_argument("--topn", type=int, default=25)
    ap.add_argument("--min-win", type=float, default=48.0)
    ap.add_argument("--min-trades-per-quarter", type=float, default=5.0,
                    help="scaled by window length; 5/quarter ~= the original 15 at 1 year")
    ap.add_argument("--windows", default="3,6,12,24", help="months")
    ap.add_argument("--refresh", default="1,3", help="months")
    a = ap.parse_args()

    con = sqlite3.connect(a.db)
    con.execute("PRAGMA busy_timeout=60000")
    S, R = load(con, a.strategy_run), load(con, a.random_run)
    syms = sorted({s for s, _ in S})
    months = sorted({m for _, m in S})

    print(f"strategy run {a.strategy_run} | random run {a.random_run} | "
          f"{len(syms)} symbols | {len(months)} months | PIIND excluded")
    print(f"top {a.topn}, win% >= {a.min_win}, min trades scaled at "
          f"{a.min_trades_per_quarter}/quarter of window\n")

    print(f"{'window':>7}{'refresh':>9}{'cands':>7}{'trades':>8}"
          f"{'strat':>9}{'random':>9}{'entry':>8}{'t':>6}{'sel':>9}{'t':>6}")
    print("-" * 80)
    rows = []
    for w in [int(x) for x in a.windows.split(",")]:
        mt = max(3, int(a.min_trades_per_quarter * w / 3))
        for rf in [int(x) for x in a.refresh.split(",")]:
            ps, pr, us, ur, cands = walk(S, R, syms, months, w, rf, mt,
                                         a.min_win, a.topn)
            if len(ps) < 500 or len(pr) < 500:
                print(f"{w:>6}m{rf:>8}m{cands:>7.0f}{len(ps):>8}   too few trades")
                continue
            ent, t_ent = t_of(ps, pr)
            sel, t_sel = t_of(pr, ur)
            rows.append((w, rf, statistics.fmean(ps) - BREAKEVEN, ent, t_ent, sel, t_sel))
            print(f"{w:>6}m{rf:>8}m{cands:>7.0f}{len(ps):>8}"
                  f"{statistics.fmean(ps)-BREAKEVEN:>+9.4f}"
                  f"{statistics.fmean(pr)-BREAKEVEN:>+9.4f}"
                  f"{ent:>+8.4f}{t_ent:>+6.2f}{sel:>+9.4f}{t_sel:>+6.2f}")

    if not rows:
        return
    print("\nBest by SELECTION value (the random arm's gain — no signal involved):")
    for w, rf, net, ent, te, sel, ts in sorted(rows, key=lambda r: -r[5])[:4]:
        print(f"  window {w:>2}m / refresh {rf}m: sel {sel:+.4f} (t {ts:+.2f}), "
              f"book net {net:+.4f}")
    print("\nBest by ENTRY value:")
    for w, rf, net, ent, te, sel, ts in sorted(rows, key=lambda r: -r[3])[:4]:
        print(f"  window {w:>2}m / refresh {rf}m: entry {ent:+.4f} (t {te:+.2f})")
    print("\nIf short windows win on `sel`, the selection is tracking a regime.")
    print("If nothing separates them, it is picking something slow-moving.")


if __name__ == "__main__":
    main()
