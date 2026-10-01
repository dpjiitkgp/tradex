#!/usr/bin/env python3
"""Detailed 3-year-window walk-forward of the stock-selection method.

For each 3-year training window, selects every stock meeting BOTH
  win rate > 48%   and   Avg R > 0.200
over that window, then measures how exactly those stocks performed in the
FOLLOWING year — which is data the selection never saw.

Writes a markdown report. Every figure is trade-weighted:
    Avg R over a set = sum(trades_i * avg_r_i) / sum(trades_i)
never a simple mean of per-stock or per-year averages.

Avg R is GROSS. Costs are ~Rs206 per round trip against a 0.75% risk unit,
so breakeven is 0.061 R; "net" columns subtract that.
"""
import argparse
import csv
import sqlite3

BREAKEVEN_R = 0.061
MIN_WIN = 48.0
MIN_R = 0.200
MIN_TRADES = 20          # a window figure on <20 trades is not a rank, it is noise
DATA = "research/long-baseline/analysis/short_yearly_110.tsv"
WINDOWS = [(2019, 2021, 2022), (2020, 2022, 2023), (2021, 2023, 2024),
           (2022, 2024, 2025), (2023, 2025, 2026)]


def load(path):
    d = {}
    for r in csv.DictReader(open(path), delimiter="\t"):
        d.setdefault(r["sym"], {})[int(r["yr"])] = (
            int(r["n"]), float(r["avg_r"]), float(r["win_pct"]))
    return d


def agg(years_data, years):
    """Trade-weighted (n, Avg R, win%) over `years`."""
    v = [years_data[y] for y in years if y in years_data]
    n = sum(x[0] for x in v)
    if n == 0:
        return 0, 0.0, 0.0
    return (n,
            sum(x[0] * x[1] for x in v) / n,
            sum(x[0] * x[2] for x in v) / n)


def qualifies(yrs, train, per_year, min_win, min_r, min_trades, min_trades_yr):
    """Window-average rule, or the stricter every-single-year rule.

    The average rule lets one exceptional year carry two mediocre ones —
    which is how a stock can pass on a 3-year mean and then fail
    immediately. The per-year rule demands the edge was present in EVERY
    training year, so it selects for consistency rather than for a good run.
    """
    if not per_year:
        n, r, w = agg(yrs, train)
        return n >= min_trades and w > min_win and r > min_r, (n, r, w)
    for y in train:
        if y not in yrs:
            return False, (0, 0.0, 0.0)
        n, r, w = yrs[y]
        if n < min_trades_yr or w <= min_win or r <= min_r:
            return False, (0, 0.0, 0.0)
    return True, agg(yrs, train)


def main():
    global MIN_WIN, MIN_R
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", default="research/stock-analysis/walkforward_3y_detail.md")
    ap.add_argument("--data", default=DATA)
    ap.add_argument("--per-year", action="store_true",
                    help="require the thresholds in EVERY training year, not on the window average")
    ap.add_argument("--min-win", type=float, default=MIN_WIN)
    ap.add_argument("--min-r", type=float, default=MIN_R)
    ap.add_argument("--min-trades-yr", type=int, default=15,
                    help="minimum trades in each individual year (per-year mode)")
    a = ap.parse_args()
    MIN_WIN, MIN_R = a.min_win, a.min_r

    d = load(a.data)
    L = []
    w = L.append
    w("# 3-year window walk-forward — detailed\n")
    if a.per_year:
        w(f"Selection rule: **win rate > {MIN_WIN}% AND Avg R > {MIN_R} in EVERY ONE "
          f"of the 3 training years** (min {a.min_trades_yr} trades per year).\n")
        w("This is the strict form — a stock must have shown the edge in each "
          "year separately, so one exceptional year cannot carry two weak ones.\n")
    else:
        w(f"Selection rule: **win rate > {MIN_WIN}%** AND **Avg R > {MIN_R}** over the "
          f"3-year training window (average), minimum {MIN_TRADES} trades.\n")
    w("Each basket is then measured on the **following year only** — data the "
      "selection never saw.\n")
    w(f"Avg R is gross. Breakeven is **{BREAKEVEN_R} R** (~Rs206 per round trip "
      "against a 0.75% risk unit); `net` subtracts it.\n")
    w("All aggregates are trade-weighted, never a simple mean of averages.\n")

    grand_n = 0
    grand_s = 0.0
    summary = []

    for y0, y1, test in WINDOWS:
        train = list(range(y0, y1 + 1))
        picks = []
        for sym, yrs in d.items():
            ok, (n, r, win) = qualifies(yrs, train, a.per_year, MIN_WIN, MIN_R,
                                        MIN_TRADES, a.min_trades_yr)
            if ok:
                picks.append((sym, n, r, win))
        picks.sort(key=lambda x: -x[2])

        w(f"\n## Train {y0}-{y1}  →  test {test}\n")
        if not picks:
            w("_No stock met both thresholds in this window._\n")
            summary.append((f"{y0}-{y1}", test, 0, 0, None, None))
            continue

        w(f"**{len(picks)} stocks qualified.**\n")
        w(f"| stock | train trades | train Avg R | train win% | "
          f"{test} trades | {test} Avg R | {test} win% | held up? |")
        w("|---|---|---|---|---|---|---|---|")
        tn = 0
        ts = 0.0
        held = 0
        scored = 0
        for sym, n, r, win in picks:
            tn_, tr_, tw_ = agg(d[sym], [test])
            if tn_ == 0:
                w(f"| {sym} | {n} | {r:.3f} | {win:.1f} | — | — | — | no trades |")
                continue
            tn += tn_
            ts += tn_ * tr_
            scored += 1
            ok = tr_ > BREAKEVEN_R
            held += ok
            w(f"| {sym} | {n} | {r:.3f} | {win:.1f} | {tn_} | **{tr_:.3f}** | "
              f"{tw_:.1f} | {'yes' if ok else 'NO'} |")
        if tn:
            br = ts / tn
            grand_n += tn
            grand_s += ts
            w(f"\n**Basket in {test}: {tn} trades, Avg R {br:.4f} "
              f"({br/BREAKEVEN_R:.2f}x breakeven), net {br-BREAKEVEN_R:+.4f}**  ")
            w(f"Stocks that beat cost in {test}: **{held}/{scored}**\n")
            summary.append((f"{y0}-{y1}", test, len(picks), tn, br, held / scored))

    w("\n## Summary\n")
    w("| train | test | picks | test trades | test Avg R | xBE | net | stocks beating cost |")
    w("|---|---|---|---|---|---|---|---|")
    for tr, test, npick, tn, br, frac in summary:
        if br is None:
            w(f"| {tr} | {test} | 0 | — | — | — | — | — |")
        else:
            w(f"| {tr} | {test} | {npick} | {tn} | {br:.4f} | {br/BREAKEVEN_R:.2f}x | "
              f"{br-BREAKEVEN_R:+.4f} | {frac*100:.0f}% |")
    if grand_n:
        g = grand_s / grand_n
        w(f"\n**Pooled across every test year: {grand_n} trades, Avg R {g:.4f} "
          f"({g/BREAKEVEN_R:.2f}x breakeven), net {g-BREAKEVEN_R:+.4f} per trade.**\n")
        w(f"At ~200 live trades/year and 1 R = Rs3,740 (today's actual sizing: "
          f"238 MAZDOCK @ Rs2,096 = Rs4.99L notional), that net is "
          f"**Rs{(g-BREAKEVEN_R)*3740*200:,.0f}/year on Rs1L**.\n")
        w("\nSlippage is NOT included — the backtest fills at candle prices.\n")

    open(a.out, "w").write("\n".join(L) + "\n")
    print("\n".join(L))
    print(f"\n[written to {a.out}]")


if __name__ == "__main__":
    main()
