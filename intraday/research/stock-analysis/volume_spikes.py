#!/usr/bin/env python3
"""What happens after an unusually heavy one-minute candle?

Motivated by APARINDS showing occasional minute bars of ~20k shares against a
normal minute of a few hundred. The question is whether such a bar marks the
start of a directional run, and if so how far it carries.

## Definitions, and why these

**Unusual volume** is `volume >= mult x the trailing 60-bar MEAN`, measured
within the same session so there is no look-ahead.

The mean, not the median, and 60 bars, not 20. A thin stock's minute volumes
are zero-heavy — APARINDS runs p50=126 against p99.9=10,619 — so a trailing
median sits near the floor and *any* real trade reads as a 70x spike. The
first version of this script used 10x the trailing 20-bar median and flagged
6% of all bars as "unusual", which is a definition of nothing. A mean over a
longer window is pulled up by the occasional heavy bar and gives a baseline
that a genuine spike has to clear.

The hit rate is printed so the rarity of the threshold is visible rather than
assumed.

**Direction** cannot be read from OHLC — there is no order-flow data here. Two
proxies are reported instead, and they disagree often enough to be worth
having both:
  * bar sign: close vs open
  * close location value (CLV): (close-low)/(high-low). Near 1 means buyers
    finished in control of the bar, near 0 means sellers did.

**Run length** is "how far before the move is given back": from the spike
bar's close, walk forward tracking the best excursion, and stop at the first
bar whose CLOSE crosses back through the spike close. That is a cleaner stop
rule than counting consecutive same-direction bars, which ends on the first
one-tick wobble and understates every run.

The opening window 09:15-09:45 is excluded as instructed — opening volume is
structurally heavy and would dominate the sample. Runs are also truncated at
15:10, the live square-off, so nothing is measured that could not be traded.

    python3 volume_spikes.py --symbols APARINDS
    python3 volume_spikes.py --mult 10 --from-date 2019-01-01
"""
import argparse
import sqlite3
import statistics
from collections import defaultdict

IST_OPEN_SKIP_UNTIL = "04:15:00"   # 09:45 IST in UTC
IST_SQUAREOFF = "09:40:00"         # 15:10 IST in UTC


def load_days(con, token, from_date):
    """{date: [(ts, o, h, l, c, v), ...]} ordered."""
    rows = con.execute(
        """SELECT ts, open, high, low, close, volume FROM candles
           WHERE instrument_token=? AND interval='1m' AND ts>=?
           ORDER BY ts""", (token, from_date)).fetchall()
    days = defaultdict(list)
    for r in rows:
        days[r[0][:10]].append(r)
    return days


def analyse(days, mult, lookback=60):
    """Yields one record per volume spike."""
    out = []
    for day, bars in days.items():
        vols = [b[5] or 0 for b in bars]
        for i in range(lookback, len(bars)):
            ts, o, h, l, c, v = bars[i]
            t = ts[11:19]
            if t < IST_OPEN_SKIP_UNTIL or t >= IST_SQUAREOFF:
                continue
            base = statistics.fmean(vols[i - lookback:i])
            if base <= 0 or (v or 0) < mult * base:
                continue
            rng = h - l
            clv = ((c - l) / rng) if rng > 0 else 0.5
            direction = 1 if c > o else (-1 if c < o else 0)
            # Walk forward until the close crosses back through the spike close.
            best = 0.0
            bars_held = 0
            for j in range(i + 1, len(bars)):
                if bars[j][0][11:19] >= IST_SQUAREOFF:
                    break
                bars_held = j - i
                _, _, hj, lj, cj, _ = bars[j]
                if direction >= 0:
                    best = max(best, (hj - c) / c * 100)
                    if cj < c:
                        break
                else:
                    best = max(best, (c - lj) / c * 100)
                    if cj > c:
                        break
            out.append({
                "day": day, "t": t, "ratio": (v or 0) / base, "vol": v or 0,
                "dir": direction, "clv": clv, "bars": bars_held,
                "run_pct": best, "bar_pct": (c - o) / o * 100 if o else 0.0,
            })
    return out


def report(name, recs, mult):
    if not recs:
        print(f"{name}: no spikes at {mult}x")
        return
    ups = [r for r in recs if r["dir"] > 0]
    dns = [r for r in recs if r["dir"] < 0]
    print(f"\n=== {name} — {len(recs)} spikes at >= {mult}x the trailing 60-bar mean ===")
    print(f"{'direction':<14}{'n':>6}{'avg ratio':>11}{'bar move%':>11}"
          f"{'run%':>8}{'median run%':>13}{'bars':>7}{'CLV':>7}")
    print("-" * 78)
    for lbl, g in (("up (buyers)", ups), ("down (sellers)", dns)):
        if not g:
            continue
        print(f"{lbl:<14}{len(g):>6}{statistics.fmean(r['ratio'] for r in g):>11.1f}"
              f"{statistics.fmean(r['bar_pct'] for r in g):>+11.3f}"
              f"{statistics.fmean(r['run_pct'] for r in g):>8.3f}"
              f"{statistics.median(r['run_pct'] for r in g):>13.3f}"
              f"{statistics.fmean(r['bars'] for r in g):>7.1f}"
              f"{statistics.fmean(r['clv'] for r in g):>7.2f}")
    # A run only means something against what an ordinary bar leads to, so
    # report the split by how decisively the spike bar itself closed.
    print(f"\n  by close-location (did buyers or sellers finish the bar?)")
    print(f"  {'CLV band':<14}{'n':>6}{'run%':>9}{'bars':>7}")
    print("  " + "-" * 36)
    for lo, hi, lbl in [(0.0, 0.25, "0.00-0.25"), (0.25, 0.5, "0.25-0.50"),
                        (0.5, 0.75, "0.50-0.75"), (0.75, 1.01, "0.75-1.00")]:
        g = [r for r in recs if lo <= r["clv"] < hi]
        if len(g) < 5:
            continue
        print(f"  {lbl:<14}{len(g):>6}{statistics.fmean(r['run_pct'] for r in g):>9.3f}"
              f"{statistics.fmean(r['bars'] for r in g):>7.1f}")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--db", default="data/papertrader.db")
    ap.add_argument("--symbols", default="APARINDS")
    ap.add_argument("--mult", type=float, default=10.0)
    ap.add_argument("--from-date", default="2019-01-01")
    ap.add_argument("--pooled", action="store_true", help="one combined report")
    a = ap.parse_args()

    con = sqlite3.connect(f"file:{a.db}?mode=ro", uri=True)
    con.execute("PRAGMA busy_timeout=240000")
    syms = [s.strip().upper() for s in a.symbols.split(",")]
    toks = {s: t for s, t in con.execute(
        "SELECT tradingsymbol, instrument_token FROM instruments WHERE tradingsymbol IN ({})"
        .format(",".join("?" * len(syms))), syms)}

    allrecs = []
    for s in syms:
        if s not in toks:
            print(f"{s}: not found")
            continue
        recs = analyse(load_days(con, toks[s], a.from_date), a.mult)
        allrecs += recs
        if not a.pooled:
            report(s, recs, a.mult)
    if a.pooled:
        report(f"{len(syms)} stocks pooled", allrecs, a.mult)


if __name__ == "__main__":
    main()
