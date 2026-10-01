#!/usr/bin/env python3
"""Can a volume spike be scalped with a tight stop?

Enters at the spike bar's close in the spike's own direction, then walks
forward bar by bar until either the target or the stop is touched. Reports win
rate and expectancy net of the real MIS round-trip cost.

## The two things that make or break this

**Costs dominate at tight stops.** The MIS round trip is 0.0446% of notional,
which is 0.30 R against a 0.15% stop versus 0.06 R against the current book's
0.75%. A scalp therefore needs roughly five times the per-R edge just to break
even. That is the binding constraint, not signal.

**A spike has to beat an ordinary bar.** Every spike is measured against
CONTROL bars drawn from the same stock, the same session and the same minute
of the day, excluded from being spikes themselves. Without that, "price moved
0.08% after a spike" says nothing — ordinary bars also lead somewhere.

## Where this is deliberately pessimistic

When a single bar's range contains both the stop and the target, there is no
way to know from OHLC which was touched first, so the STOP is assumed. Tight
stops make that case common, and assuming the target would turn a losing
system into a winning one on an artefact.

Impact is NOT modelled and matters more here than anywhere else in the
project: a scalp pays the spread on every trade and takes many more trades, so
the square-root impact estimate of ~0.06 R at median position size is a per-
trade charge against a per-trade edge of the same order.

    python3 spike_scalp.py --symbols APARINDS,IDEA,YESBANK,NMDC
"""
import argparse
import random
import sqlite3
import statistics
from collections import defaultdict

SKIP_UNTIL = "04:15:00"    # 09:45 IST
SQUAREOFF = "09:40:00"     # 15:10 IST
COST_PCT = 223 / 5e5 * 100  # MIS round trip as % of notional


def load_days(con, token, from_date):
    days = defaultdict(list)
    for r in con.execute(
        """SELECT ts, open, high, low, close, volume FROM candles
           WHERE instrument_token=? AND interval='1m' AND ts>=? ORDER BY ts""",
            (token, from_date)):
        days[r[0][:10]].append(r)
    return days


AMBIG = {"n": 0, "tot": 0}   # how often a bar contains both levels


def first_touch(bars, i, direction, stop_pct, target_pct, ambiguous="stop"):
    """Outcome of entering at bars[i]'s close. Returns +target/-stop in %, or
    None if the session ends first (treated as a discard, not a win)."""
    entry = bars[i][4]
    if entry <= 0:
        return None
    if direction > 0:
        stop, tgt = entry * (1 - stop_pct / 100), entry * (1 + target_pct / 100)
    else:
        stop, tgt = entry * (1 + stop_pct / 100), entry * (1 - target_pct / 100)
    for j in range(i + 1, len(bars)):
        if bars[j][0][11:19] >= SQUAREOFF:
            return None
        _, _, h, l, _, _ = bars[j]
        if direction > 0:
            hit_stop, hit_tgt = l <= stop, h >= tgt
        else:
            hit_stop, hit_tgt = h >= stop, l <= tgt
        if hit_stop and hit_tgt:
            AMBIG["n"] += 1
            AMBIG["tot"] += 1
            return -stop_pct if ambiguous == "stop" else target_pct
        if hit_stop or hit_tgt:
            AMBIG["tot"] += 1
        if hit_stop:
            return -stop_pct
        if hit_tgt:
            return target_pct
    return None


def collect(days, mult, lookback=60, seed=1):
    """(spikes, controls) as lists of (bars, index, direction)."""
    rng = random.Random(seed)
    spikes, controls = [], []
    for bars in days.values():
        vols = [b[5] or 0 for b in bars]
        elig, isspike = [], set()
        for i in range(lookback, len(bars)):
            t = bars[i][0][11:19]
            if t < SKIP_UNTIL or t >= SQUAREOFF:
                continue
            base = statistics.fmean(vols[i - lookback:i])
            if base <= 0:
                continue
            elig.append(i)
            if (vols[i] or 0) >= mult * base:
                isspike.add(i)
        for i in isspike:
            o, c = bars[i][1], bars[i][4]
            d = 1 if c > o else (-1 if c < o else 0)
            if d:
                spikes.append((bars, i, d))
        # One control per spike, same day, same hour, never itself a spike.
        pool = [i for i in elig if i not in isspike]
        for i in isspike:
            hh = bars[i][0][11:13]
            same_hr = [j for j in pool if bars[j][0][11:13] == hh] or pool
            if not same_hr:
                continue
            j = rng.choice(same_hr)
            o, c = bars[j][1], bars[j][4]
            d = 1 if c > o else (-1 if c < o else 0)
            if d:
                controls.append((bars, j, d))
    return spikes, controls


def score(cases, stop_pct, target_pct, ambiguous="stop"):
    res = [first_touch(b, i, d, stop_pct, target_pct, ambiguous) for b, i, d in cases]
    res = [r for r in res if r is not None]
    if len(res) < 30:
        return None
    wins = sum(1 for r in res if r > 0)
    gross = statistics.fmean(res)
    return len(res), 100 * wins / len(res), gross, gross - COST_PCT


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--db", default="data/papertrader.db")
    ap.add_argument("--symbols", default="APARINDS")
    ap.add_argument("--mult", type=float, default=20.0)
    ap.add_argument("--from-date", default="2019-01-01")
    a = ap.parse_args()

    con = sqlite3.connect(f"file:{a.db}?mode=ro", uri=True)
    con.execute("PRAGMA busy_timeout=240000")
    syms = [s.strip().upper() for s in a.symbols.split(",")]
    toks = dict(con.execute(
        "SELECT tradingsymbol, instrument_token FROM instruments WHERE tradingsymbol IN ({})"
        .format(",".join("?" * len(syms))), syms))

    spikes, controls = [], []
    for s in syms:
        if s not in toks:
            continue
        sp, ct = collect(load_days(con, toks[s], a.from_date), a.mult)
        spikes += sp
        controls += ct
    print(f"{len(syms)} stocks | {len(spikes)} spikes at >={a.mult}x | "
          f"{len(controls)} matched controls")
    print(f"cost {COST_PCT:.4f}% of notional per round trip; stop-first on "
          f"ambiguous bars\n")
    print(f"{'stop':>6}{'target':>8}{'n':>7}{'win%':>7}{'gross%':>9}{'net%':>9}"
          f"{'| ctrl n':>9}{'win%':>7}{'net%':>9}{'edge%':>8}")
    print("-" * 80)
    for stop, tgt in [(0.10, 0.10), (0.15, 0.15), (0.15, 0.30), (0.20, 0.20),
                      (0.20, 0.40), (0.25, 0.50), (0.35, 0.35), (0.35, 0.70)]:
        s = score(spikes, stop, tgt)
        c = score(controls, stop, tgt)
        if not s or not c:
            continue
        print(f"{stop:>5.2f}%{tgt:>7.2f}%{s[0]:>7}{s[1]:>7.1f}{s[2]:>+9.4f}"
              f"{s[3]:>+9.4f}{c[0]:>9}{c[1]:>7.1f}{c[3]:>+9.4f}{s[3]-c[3]:>+8.4f}")
    print("\n`edge%` is spike minus control. Anything else in this table is")
    print("partly just what an ordinary bar does.")


if __name__ == "__main__":
    main()
