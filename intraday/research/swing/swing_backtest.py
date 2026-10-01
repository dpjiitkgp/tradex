#!/usr/bin/env python3
"""Multi-day holds on daily bars: stop, max hold, or setup broken.

Deliberately separate from the Rust engine. `PaperEngine` is intraday-shaped —
its session boundary squares off anything still open, which is correct there
and fatal here. Rather than loosen a rule that exists to stop overnight
carries, swing research runs on its own simulator, in Python, where iteration
is cheap and a mistake cannot reach the live book.

## What the simulator is careful about

**Gaps.** The single biggest difference from intraday, and the easiest thing
to fake. A stop at 95 does not fill at 95 when the stock opens at 91 — it
fills at 91. Every exit here checks the OPEN first and only then the intraday
range, so a gap through the stop costs what it really costs.

**No look-ahead.** A signal on day D is acted on at day D+1's open. Nothing
decides using a bar it could not have seen.

**Costs by instrument.** A delivery round trip is ~5x an intraday one, almost
all of it STT. See costs.py; it is passed through here so a result is always
net of the real cost of the instrument it would have traded on.

## Exits, in the order checked each day

  1. gap through the stop     -> exit at the open
  2. stop touched intraday    -> exit at the stop
  3. setup broken             -> exit at the close
  4. max hold reached         -> exit at the close

    python3 swing_backtest.py --direction long --instrument cnc
    python3 swing_backtest.py --max-hold 5,10,20 --stop 2,3,5
"""
import argparse
import itertools
import math
import sqlite3
import statistics
import sys

sys.path.insert(0, __file__.rsplit("/", 1)[0])
from costs import CNC_DELIVERY, STOCK_FUTURES, MIS_INTRADAY  # noqa: E402

DB = "research/swing/data/daily.sqlite"
SIGMA_GUESS = 1.4  # per-trade R sigma, for standard errors until measured here
INSTRUMENTS = {"cnc": CNC_DELIVERY, "futures": STOCK_FUTURES, "mis": MIS_INTRADAY}


def load(con, symbol):
    """Daily bars for one symbol, oldest first."""
    return con.execute(
        "SELECT d, open, high, low, close, volume FROM daily "
        "WHERE tradingsymbol=? ORDER BY d", (symbol,)).fetchall()


def ema(values, period):
    """Standard EMA, seeded with a simple mean, matching indicators/src/ema.rs
    so a daily result is comparable with the intraday work."""
    out = [None] * len(values)
    k = 2.0 / (period + 1.0)
    if len(values) < period:
        return out
    prev = statistics.fmean(values[:period])
    out[period - 1] = prev
    for i in range(period, len(values)):
        prev = (values[i] - prev) * k + prev
        out[i] = prev
    return out


def signals_ema_cross(bars, direction, fast=10, slow=30):
    """Reference setup: a daily EMA cross, the daily analogue of the intraday
    trigger.

    A PLACEHOLDER, not a recommendation — it exists so the machinery has
    something to run and so results are comparable with the intraday book's
    own logic. Replace with the real setup when it is defined.

    Returns indices of signal days. Entry happens on the NEXT day's open.
    """
    closes = [b[4] for b in bars]
    f, s = ema(closes, fast), ema(closes, slow)
    out = []
    for i in range(1, len(bars)):
        if None in (f[i], s[i], f[i - 1], s[i - 1]):
            continue
        was_above = f[i - 1] > s[i - 1]
        now_above = f[i] > s[i]
        if was_above == now_above:
            continue
        if (direction == "long" and now_above) or (direction == "short" and not now_above):
            out.append(i)
    return out


def signals_random(bars, direction, n, seed):
    """Random entry days — the control that decides whether a setup has any
    information in it.

    Indian equities rose a great deal over 2019-2026, so a LONG strategy that
    holds longer captures more of that drift. A sweep whose net R rises
    monotonically with hold length looks like an edge and may be nothing but
    market exposure. The only way to tell is to give a rule with no
    information the same stops, the same holds and the same instruments, and
    see what it earns.

    If the real setup does not clearly beat this, it has told you nothing.
    """
    import random
    rng = random.Random(seed)
    lo, hi = 35, len(bars) - 2   # leave room for warmup and for the hold
    if hi <= lo:
        return []
    return sorted(rng.sample(range(lo, hi), min(n, hi - lo)))


def setup_broken(bars, i, entry_idx, direction):
    """Has the setup failed, as of day `i`'s close?

    Definition: price has closed back through the signal day's own extreme —
    for a long, below the signal day's low. That is the level the setup
    asserted would hold, so closing past it is the setup being wrong rather
    than merely unprofitable. Checked on the close, never intraday, so a wick
    does not eject a position that finished the day fine.
    """
    sig = bars[entry_idx - 1]
    close = bars[i][4]
    return close < sig[3] if direction == "long" else close > sig[2]


def run_trade(bars, entry_idx, direction, stop_pct, max_hold, use_setup_break):
    """Simulates one trade. Returns (r_multiple, exit_reason, days_held)."""
    if entry_idx >= len(bars):
        return None
    entry = bars[entry_idx][1]  # next day's OPEN — no look-ahead
    if entry <= 0:
        return None
    sign = 1.0 if direction == "long" else -1.0
    stop = entry * (1 - stop_pct) if direction == "long" else entry * (1 + stop_pct)

    def r_of(exit_px):
        return sign * (exit_px - entry) / (entry * stop_pct)

    for held in range(1, max_hold + 1):
        i = entry_idx + held - 1
        if i >= len(bars):
            break
        _, o, h, l, c, _ = bars[i]

        # 1. Gapped through the stop. The open is the fill, not the stop.
        gapped = (direction == "long" and o <= stop) or (direction == "short" and o >= stop)
        if gapped and held > 1:
            return r_of(o), "gap_through_stop", held
        # On the entry day the open IS the entry, so a gap cannot pre-date it.

        # 2. Stop touched during the day.
        touched = (direction == "long" and l <= stop) or (direction == "short" and h >= stop)
        if touched:
            return r_of(stop), "stop", held

        # 3. The setup has failed, judged on the close.
        if use_setup_break and held > 1 and setup_broken(bars, i, entry_idx, direction):
            return r_of(c), "setup_broken", held

        # 4. Time.
        if held == max_hold:
            return r_of(c), "max_hold", held

    # Ran out of data — treat as unresolved rather than inventing an exit.
    return None


def summarise(rs, cost_r):
    n = len(rs)
    if n == 0:
        return None
    gross = statistics.fmean(rs)
    wins = [r for r in rs if r > 0]
    return {
        "n": n, "gross": gross, "net": gross - cost_r,
        "win": 100.0 * len(wins) / n,
        "avg_win": statistics.fmean(wins) if wins else 0.0,
        "avg_loss": statistics.fmean([r for r in rs if r <= 0]) or 0.0,
        "se": SIGMA_GUESS / math.sqrt(n),
    }


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--db", default=DB)
    ap.add_argument("--direction", choices=["long", "short"], default="long")
    ap.add_argument("--instrument", choices=list(INSTRUMENTS), default="cnc")
    ap.add_argument("--stop", default="2,3,5", help="stop %% of entry")
    ap.add_argument("--max-hold", default="5,10,20", help="trading days")
    ap.add_argument("--no-setup-break", action="store_true")
    ap.add_argument("--symbols", help="comma-separated; default every symbol present")
    ap.add_argument("--from-date", default="2019-01-01")
    ap.add_argument("--notional", type=float, default=500_000.0)
    a = ap.parse_args()

    model = INSTRUMENTS[a.instrument]
    if a.direction == "short" and not model.can_hold_short:
        sys.exit(
            f"A multi-day SHORT cannot be held on {model.name}.\n  {model.note}\n"
            "  Use --instrument futures, or --direction long."
        )

    con = sqlite3.connect(f"file:{a.db}?mode=ro", uri=True)
    syms = ([s.strip().upper() for s in a.symbols.split(",")] if a.symbols else
            [r[0] for r in con.execute(
                "SELECT DISTINCT tradingsymbol FROM daily ORDER BY 1")])
    stops = [float(x) / 100 for x in a.stop.split(",")]
    holds = [int(x) for x in a.max_hold.split(",")]

    series = {s: [b for b in load(con, s) if b[0] >= a.from_date] for s in syms}
    series = {s: b for s, b in series.items() if len(b) > 60}
    print(f"{model.name} | {a.direction} | {len(series)} symbols | from {a.from_date}")
    print(f"entry: daily EMA 10/30 cross (PLACEHOLDER setup), next-open fill")
    print(f"setup-break exit: {'off' if a.no_setup_break else 'close back through the signal bar extreme'}\n")

    # The random column is not optional and not a flag. Long-only backtests on
    # 2019-2026 Indian equities all look good, because the index roughly
    # tripled — and a longer hold captures more of that drift, which reads as
    # a rising "edge" with hold length. Random entries on the same symbols,
    # stops and holds measure exactly that drift, so `vs rnd` is the only
    # column in this table that could be a signal.
    print(f"{'stop':>6}{'hold':>6}{'trades':>8}{'net R':>8}{'random':>8}{'vs rnd':>8}"
          f"{'t':>6}{'win%':>7}{'avg win':>9}{'avg loss':>10}{'days':>6}")
    print("-" * 88)
    for stop_pct, hold in itertools.product(stops, holds):
        cost_r = model.breakeven_r(a.notional, stop_pct)
        rs, rnd, reasons, days = [], [], {}, []
        for sym, bars in series.items():
            sigs = signals_ema_cross(bars, a.direction)
            for si in sigs:
                res = run_trade(bars, si + 1, a.direction, stop_pct, hold,
                                not a.no_setup_break)
                if res is None:
                    continue
                r, why, held = res
                rs.append(r)
                reasons[why] = reasons.get(why, 0) + 1
                days.append(held)
            for si in signals_random(bars, a.direction, len(sigs), hash(sym) & 0xffff):
                res = run_trade(bars, si + 1, a.direction, stop_pct, hold,
                                not a.no_setup_break)
                if res:
                    rnd.append(res[0])
        s = summarise(rs, cost_r)
        if not s:
            continue
        rn = statistics.fmean(rnd) - cost_r if rnd else float("nan")
        edge = s["net"] - rn
        se = SIGMA_GUESS * math.sqrt(1 / len(rs) + 1 / max(len(rnd), 1))
        print(f"{stop_pct*100:>5.1f}%{hold:>6}{s['n']:>8}{s['net']:>+8.3f}{rn:>+8.3f}"
              f"{edge:>+8.3f}{edge/se:>+6.2f}{s['win']:>7.1f}"
              f"{s['avg_win']:>+9.2f}{s['avg_loss']:>+10.2f}{statistics.fmean(days):>6.1f}")
    print("\n`vs rnd` is the result. `net R` alone is mostly market drift.")

    # Exit-reason mix for the middle configuration, since it says more about
    # what the rules are actually doing than any single average.
    mid_stop, mid_hold = stops[len(stops) // 2], holds[len(holds) // 2]
    rs, reasons = [], {}
    for sym, bars in series.items():
        for si in signals_ema_cross(bars, a.direction):
            res = run_trade(bars, si + 1, a.direction, mid_stop, mid_hold,
                            not a.no_setup_break)
            if res:
                rs.append(res[0])
                reasons[res[1]] = reasons.get(res[1], 0) + 1
    print(f"\nExit mix at stop {mid_stop*100:.1f}% / hold {mid_hold}d:")
    for why, c in sorted(reasons.items(), key=lambda kv: -kv[1]):
        print(f"  {why:<20}{c:>6}  ({100*c/len(rs):>4.1f}%)")
    print(f"\ncost of this instrument at {mid_stop*100:.1f}% stop: "
          f"{model.breakeven_r(a.notional, mid_stop):.3f} R per round trip")


if __name__ == "__main__":
    main()
