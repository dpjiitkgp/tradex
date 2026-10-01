#!/usr/bin/env python3
"""Runs every standard swing setup through one harness and ranks them.

All ten setups see the same symbols, the same dates, the same stop, the same
maximum hold and the same cost model, so the only thing that differs is the
entry idea.

Each is scored against a RANDOM-ENTRY control on the same symbols and the same
number of trades. That column is the whole point: Indian equities roughly
tripled over 2019-2026, so any long-only rule held for days will show a
positive net R whether or not it knows anything. `vs rnd` is the only number
here that could be a signal; `net R` on its own is mostly market drift.

Out-of-sample and per-year splits are printed for the leaders, because a
pooled edge that lives in one year is not an edge — the standard that
eliminated fourteen of fourteen filters in the intraday work.

    python3 compare_setups.py
    python3 compare_setups.py --stop 3 --max-hold 10 --split-date 2023-01-01
"""
import argparse
import math
import sqlite3
import statistics
import sys

sys.path.insert(0, __file__.rsplit("/", 1)[0])
from costs import CNC_DELIVERY, STOCK_FUTURES, MIS_INTRADAY  # noqa: E402
from setups import SETUPS  # noqa: E402
from swing_backtest import load, run_trade, signals_random  # noqa: E402

DB = "research/swing/data/daily.sqlite"
SIGMA = 1.4
INSTRUMENTS = {"cnc": CNC_DELIVERY, "futures": STOCK_FUTURES, "mis": MIS_INTRADAY}


def collect(series, fn, direction, stop, hold, brk):
    """Runs one setup across every symbol. Returns (trades, reasons, dates)."""
    rs, rnd, reasons, dates, held = [], [], {}, [], []
    for sym, bars in series.items():
        try:
            sigs = fn(bars)
        except Exception:
            sigs = []
        for si in sigs:
            res = run_trade(bars, si + 1, direction, stop, hold, brk)
            if res is None:
                continue
            rs.append(res[0])
            reasons[res[1]] = reasons.get(res[1], 0) + 1
            dates.append(bars[si + 1][0])
            held.append(res[2])
        for si in signals_random(bars, direction, len(sigs), hash(sym) & 0xFFFF):
            res = run_trade(bars, si + 1, direction, stop, hold, brk)
            if res:
                rnd.append(res[0])
    return rs, rnd, reasons, dates, held


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--db", default=DB)
    ap.add_argument("--instrument", choices=list(INSTRUMENTS), default="cnc")
    ap.add_argument("--stop", type=float, default=3.0, help="percent")
    ap.add_argument("--max-hold", type=int, default=10, help="trading days")
    ap.add_argument("--no-setup-break", action="store_true")
    ap.add_argument("--from-date", default="2019-01-01")
    ap.add_argument("--split-date", default="2024-01-01")
    ap.add_argument("--notional", type=float, default=500_000.0)
    a = ap.parse_args()

    model = INSTRUMENTS[a.instrument]
    stop = a.stop / 100
    cost = model.breakeven_r(a.notional, stop)
    brk = not a.no_setup_break

    con = sqlite3.connect(f"file:{a.db}?mode=ro", uri=True)
    syms = [r[0] for r in con.execute(
        "SELECT DISTINCT tradingsymbol FROM daily ORDER BY 1")]
    series = {s: [b for b in load(con, s) if b[0] >= a.from_date] for s in syms}
    series = {s: b for s, b in series.items() if len(b) > 260}

    print(f"{model.name} | LONG | {len(series)} symbols | from {a.from_date}")
    print(f"stop {a.stop}% | max hold {a.max_hold}d | "
          f"setup-break exit {'off' if a.no_setup_break else 'on'} | "
          f"cost {cost:.3f} R per round trip\n")

    print(f"{'setup':<17}{'kind':<14}{'trades':>8}{'net R':>8}{'random':>8}"
          f"{'vs rnd':>8}{'t':>6}{'win%':>7}{'days':>6}")
    print("-" * 82)

    results = []
    for name, (fn, kind) in SETUPS.items():
        rs, rnd, reasons, dates, held = collect(series, fn, "long", stop, a.max_hold, brk)
        if len(rs) < 100:
            print(f"{name:<17}{kind:<14}{len(rs):>8}   too few trades to judge")
            continue
        net = statistics.fmean(rs) - cost
        rn = statistics.fmean(rnd) - cost if rnd else float("nan")
        edge = net - rn
        se = SIGMA * math.sqrt(1 / len(rs) + 1 / max(len(rnd), 1))
        win = 100.0 * sum(1 for r in rs if r > 0) / len(rs)
        results.append((name, kind, rs, rnd, dates, net, rn, edge, edge / se))
        print(f"{name:<17}{kind:<14}{len(rs):>8}{net:>+8.3f}{rn:>+8.3f}"
              f"{edge:>+8.3f}{edge/se:>+6.2f}{win:>7.1f}"
              f"{statistics.fmean(held):>6.1f}")

    if not results:
        return
    results.sort(key=lambda r: -r[7])
    print("\nRanked by edge over random:")
    for name, kind, rs, rnd, dates, net, rn, edge, t in results:
        mark = "  <-- worth a look" if t > 2.0 else ""
        print(f"  {name:<17}{edge:>+7.3f} R   t {t:>+5.2f}{mark}")

    # Out of sample, on the leaders only.
    print(f"\nOUT OF SAMPLE (entries from {a.split_date}) — top 4 by pooled edge")
    print(f"{'setup':<17}{'trades':>8}{'net R':>8}{'random':>8}{'vs rnd':>8}{'t':>6}")
    print("-" * 55)
    rnd_by_date = None
    for name, kind, rs, rnd, dates, net, rn, edge, t in results[:4]:
        oos = [r for r, d in zip(rs, dates) if d >= a.split_date]
        if len(oos) < 50:
            print(f"{name:<17}{len(oos):>8}   too few out of sample")
            continue
        # The control is date-agnostic (random entries are uniform over the
        # window), so its pooled mean is the right comparison here.
        o_net = statistics.fmean(oos) - cost
        se = SIGMA * math.sqrt(1 / len(oos) + 1 / max(len(rnd), 1))
        print(f"{name:<17}{len(oos):>8}{o_net:>+8.3f}{rn:>+8.3f}"
              f"{o_net-rn:>+8.3f}{(o_net-rn)/se:>+6.2f}")

    # Year by year for the single leader.
    name, kind, rs, rnd, dates, net, rn, edge, t = results[0]
    print(f"\nYEAR BY YEAR — {name} (net R, and edge over the random control)")
    print(f"{'year':<8}{'trades':>8}{'net R':>9}{'vs rnd':>9}")
    print("-" * 34)
    years = sorted({d[:4] for d in dates})
    for y in years:
        sel = [r for r, d in zip(rs, dates) if d[:4] == y]
        if len(sel) < 30:
            continue
        yn = statistics.fmean(sel) - cost
        print(f"{y:<8}{len(sel):>8}{yn:>+9.3f}{yn-rn:>+9.3f}")
    print("\nA setup worth pursuing beats random pooled, out of sample, and in")
    print("most years — not in one of the three.")


if __name__ == "__main__":
    main()
