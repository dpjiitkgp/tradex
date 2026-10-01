#!/usr/bin/env python3
"""Per-period detail for any half-year lookback, re-selected every 6 months.

Shows the actual basket chosen at each rebalance and how each stock did in
the following 6 months — the data the selection never saw.

    python3 detail_report.py --lookback 4    # 2-year lookback
    python3 detail_report.py --lookback 2    # 1-year lookback

Avg R is gross; breakeven 0.061 R.
"""
import argparse
import csv

BREAKEVEN_R = 0.061
DATA = "research/long-baseline/analysis/short_halfyear_110.tsv"
# Indices are cached as market-context series but are not tradeable stocks;
# their bar counts are large enough to pass research's filters, so they must
# be excluded explicitly here too.
NOT_STOCKS = {"NIFTY 50", "INDIA VIX"}


def load(path):
    d, periods = {}, set()
    for r in csv.DictReader(open(path), delimiter="\t"):
        if r["sym"] in NOT_STOCKS:
            continue
        d.setdefault(r["sym"], {})[r["period"]] = (
            int(r["n"]), float(r["avg_r"]), float(r["win_pct"]))
        periods.add(r["period"])
    return d, sorted(periods)


def agg(ps, keys):
    v = [ps[k] for k in keys if k in ps]
    n = sum(x[0] for x in v)
    if n == 0:
        return 0, 0.0, 0.0
    return n, sum(x[0]*x[1] for x in v)/n, sum(x[0]*x[2] for x in v)/n


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--lookback", type=int, default=4, help="in half-years")
    ap.add_argument("--topn", type=int, default=15)
    ap.add_argument("--min-win", type=float, default=48.0)
    ap.add_argument("--min-trades", type=int, default=10)
    ap.add_argument("--out", default=None)
    ap.add_argument("--data", default=DATA)
    ap.add_argument("--min-turnover", type=float, default=0,
                    help="exclude stocks below this Rs/min traded value")
    ap.add_argument("--unit", default="half-year",
                    help="what one period is, for labelling")
    a = ap.parse_args()
    lb = a.lookback
    out = a.out or f"research/stock-analysis/detail_lookback_{lb}{a.unit[0]}.md"

    d, periods = load(a.data)
    if a.min_turnover:
        # A position this book cannot exit cleanly is worse than no position:
        # thin names both drag returns and cap how much capital can be deployed.
        import sqlite3
        _db = sqlite3.connect('data/papertrader.db')
        _t = {s_: t_ for s_, t_ in _db.execute(
            'SELECT i.tradingsymbol, AVG(c.close*c.volume) FROM candles c '
            'JOIN instruments i ON i.instrument_token=c.instrument_token '
            "WHERE c.ts>='2026-06-01' GROUP BY i.tradingsymbol") if t_}
        d = {k: v for k, v in d.items() if _t.get(k, 0) >= a.min_turnover}
    L = []
    w = L.append
    span = f"{lb/2:g}-year" if a.unit == "half-year" else f"{lb}-{a.unit}"
    w(f"# Walk-forward detail — {span} lookback, rebalanced every {a.unit}\n")
    w(f"At each rebalance: top {a.topn} by Avg R over the previous {lb} {a.unit}s "
      f"(win% >= {a.min_win}, >= {a.min_trades} trades), then traded for the next "
      f"{a.unit} only.\n")
    w(f"Avg R is gross; breakeven **{BREAKEVEN_R} R**. Indices excluded.\n")

    gn = 0
    gs = 0.0
    summary = []
    for i in range(lb, len(periods)):
        train, test = periods[i-lb:i], periods[i]
        # Universe-wide training Avg R: every stock's trades over the window,
        # not just the ones selected. This is a regime read — "was the strategy
        # working at all" — rather than a statement about which names look best.
        un = 0
        us = 0.0
        for ps in d.values():
            n_, r_, _ = agg(ps, train)
            un += n_
            us += n_ * r_
        uni = us / un if un else 0.0
        c = []
        for sym, ps in d.items():
            n, r, win = agg(ps, train)
            if n >= a.min_trades and win >= a.min_win:
                c.append((sym, n, r, win))
        c.sort(key=lambda x: -x[2])
        picks = c[:a.topn]
        if not picks:
            continue
        w(f"\n## Train {train[0]}–{train[-1]}  →  test {test}\n")
        w(f"| stock | train n | train Avg R | train win% | {test} n | "
          f"{test} Avg R | {test} win% | held? |")
        w("|---|---|---|---|---|---|---|---|")
        tn = 0
        ts = 0.0
        held = scored = 0
        for sym, n, r, win in picks:
            a_, b_, c_ = agg(d[sym], [test])
            if a_ == 0:
                w(f"| {sym} | {n} | {r:.3f} | {win:.1f} | — | — | — | no trades |")
                continue
            tn += a_
            ts += a_*b_
            scored += 1
            ok = b_ > BREAKEVEN_R
            held += ok
            w(f"| {sym} | {n} | {r:.3f} | {win:.1f} | {a_} | **{b_:.3f}** | "
              f"{c_:.1f} | {'yes' if ok else 'NO'} |")
        if tn:
            br = ts/tn
            gn += tn
            gs += ts
            w(f"\n**{test}: {tn} trades, Avg R {br:.4f} ({br/BREAKEVEN_R:.2f}x BE), "
              f"net {br-BREAKEVEN_R:+.4f} — {held}/{scored} stocks beat cost**\n")
            summary.append((train[0], train[-1], test, tn, br, held, scored,
                            uni, sum(x[1]*x[2] for x in picks)/sum(x[1] for x in picks)))

    w("\n## Summary\n")
    w("`universe train Avg R` is every stock's trades over the training window, "
      "not just the 15 picked — a read on whether the strategy was working at "
      "all going in, as opposed to which stocks looked best.\n")
    w("| train | test | universe train Avg R | picks train Avg R | trades | "
      "Avg R | xBE | net | stocks beating cost |")
    w("|---|---|---|---|---|---|---|---|---|")
    for t0, t1, test, tn, br, held, scored, uni, ptr in summary:
        w(f"| {t0}–{t1} | {test} | {uni:.4f} | {ptr:.4f} | {tn} | {br:.4f} | "
          f"{br/BREAKEVEN_R:.2f}x | {br-BREAKEVEN_R:+.4f} | {held}/{scored} |")
    if gn:
        g = gs/gn
        beat = sum(1 for s in summary if s[4] > BREAKEVEN_R)
        w(f"\n**Pooled: {gn} trades, Avg R {g:.4f} ({g/BREAKEVEN_R:.2f}x BE), "
          f"net {g-BREAKEVEN_R:+.4f} per trade. "
          f"Periods beating cost: {beat}/{len(summary)}.**\n")
        w(f"\nAt ~200 live trades/year and 1 R = Rs3,740, that net is "
          f"**Rs{(g-BREAKEVEN_R)*3740*200:,.0f}/year on Rs1L** (before slippage).\n")

    open(out, "w").write("\n".join(L) + "\n")
    print("\n".join(L))
    print(f"\n[written to {out}]")


if __name__ == "__main__":
    main()
