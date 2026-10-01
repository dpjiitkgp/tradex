#!/usr/bin/env python3
"""Does demanding a steeper VWAP slope select better trades, or just fewer?

The live book requires only that VWAP is flat-or-falling over 20 bars
(`min_slope_pct = 0.0`). Each variant here raises that bar. Because the
conditions nest — a trade whose VWAP fell 0.3% also satisfies "fell at least
0.1%" — every stricter variant's trades are a subset of the baseline's.

That nesting is what makes the honest test possible. A filter is only worth
having if the trades it REMOVES are worse than the ones it keeps. Comparing
kept-vs-kept flatters any filter that happens to cut sample in a lucky place;
comparing kept-vs-removed cannot.

So for each threshold this reports:

    kept     — what you would have traded
    removed  — what the filter threw away
    edge     — removed minus kept, with a standard error

A filter earns its place when `removed` is clearly worse than `kept`, and when
that holds out of sample and year by year. Eleven earlier entry filters failed
exactly one of those checks.

Usage:
    python3 slope_threshold_sweep.py
    python3 slope_threshold_sweep.py --split-date 2023-01-01
"""
import argparse
import math
import sqlite3
import statistics
import tomllib

BREAKEVEN = 0.061
SIGMA = 1.4
BASELINE_LABEL = "tune30/mrm-3.0-trail-0.0075%"


def run_ids(con, like):
    return [r[0] for r in con.execute(
        "SELECT id FROM runs WHERE label LIKE ? ORDER BY id", (like,)).fetchall()]


def trades_of(con, run_id, symbols):
    """{(token, entry_ts): r_multiple} — the key identifies the same trade
    across runs, which is what makes the set difference meaningful."""
    ph = ",".join("?" * len(symbols))
    rows = con.execute(
        f"""SELECT t.instrument_token, t.entry_ts, t.r_multiple
            FROM trades t JOIN instruments i ON i.instrument_token = t.instrument_token
            WHERE t.run_id = ? AND t.r_multiple IS NOT NULL
              AND i.tradingsymbol IN ({ph})""",
        [run_id, *symbols],
    ).fetchall()
    return {(tok, ts): r for tok, ts, r in rows}


def summarise(rs):
    if not rs:
        return None
    n = len(rs)
    return n, statistics.fmean(rs), SIGMA / math.sqrt(n)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--db", default="data/papertrader.db")
    ap.add_argument("--symbols-from", default="config/tune30_live.toml")
    ap.add_argument("--split-date", default="2024-01-01")
    a = ap.parse_args()

    syms = [s["symbol"] for s in tomllib.load(open(a.symbols_from, "rb"))["stock"]]
    con = sqlite3.connect(a.db)
    con.execute("PRAGMA busy_timeout=30000")

    base_ids = run_ids(con, BASELINE_LABEL)
    if not base_ids:
        raise SystemExit(f"no baseline run matching {BASELINE_LABEL!r}")
    base = trades_of(con, base_ids[-1], syms)

    variants = []
    for tag, pct in [("min005", 0.05), ("min010", 0.10), ("min020", 0.20),
                     ("min035", 0.35), ("min050", 0.50)]:
        ids = run_ids(con, f"slope-{tag}-%")
        if ids:
            variants.append((tag, pct, trades_of(con, ids[-1], syms)))

    if not variants:
        raise SystemExit("no slope-* runs found yet — is the sweep still going?")

    b = summarise(list(base.values()))
    print(f"baseline (slope <= 0, the live book): {b[0]} trades, "
          f"avg R {b[1]:+.4f}, net {b[1] - BREAKEVEN:+.4f}\n")

    print(f"{'threshold':<12}{'kept':>7}{'kept R':>9}{'removed':>9}{'rem R':>9}"
          f"{'edge':>9}{'SE':>7}{'t':>7}  verdict")
    print("-" * 80)
    for tag, pct, kept_map in variants:
        kept = list(kept_map.values())
        removed = [r for k, r in base.items() if k not in kept_map]
        ks, rs = summarise(kept), summarise(removed)
        if not ks or not rs:
            continue
        # Positive edge = the filter removed worse trades than it kept.
        edge = ks[1] - rs[1]
        se = SIGMA * math.sqrt(1 / ks[0] + 1 / rs[0])
        verdict = ("helps (sig)" if edge / se > 1.96 else
                   "helps (ns)" if edge > 0 else
                   "HURTS (sig)" if edge / se < -1.96 else "hurts (ns)")
        print(f"VWAP<=-{pct:.2f}%{'':<3}{ks[0]:>7}{ks[1]:>+9.3f}{rs[0]:>9}{rs[1]:>+9.3f}"
              f"{edge:>+9.3f}{se:>7.3f}{edge / se:>+7.2f}  {verdict}")

    # Out of sample: the same comparison on trades after the split only.
    print(f"\nOUT OF SAMPLE (entries from {a.split_date})")
    print(f"{'threshold':<12}{'kept':>7}{'kept R':>9}{'removed':>9}{'rem R':>9}{'edge':>9}{'t':>7}")
    print("-" * 62)
    for tag, pct, kept_map in variants:
        kept = [r for k, r in kept_map.items() if k[1] >= a.split_date]
        removed = [r for k, r in base.items() if k not in kept_map and k[1] >= a.split_date]
        ks, rs = summarise(kept), summarise(removed)
        if not ks or not rs:
            continue
        edge = ks[1] - rs[1]
        se = SIGMA * math.sqrt(1 / ks[0] + 1 / rs[0])
        print(f"VWAP<=-{pct:.2f}%{'':<3}{ks[0]:>7}{ks[1]:>+9.3f}{rs[0]:>9}{rs[1]:>+9.3f}"
              f"{edge:>+9.3f}{edge / se:>+7.2f}")

    # Year by year on the most promising threshold, because a pooled edge that
    # lives in one year is not an edge.
    print("\nYEAR BY YEAR — edge (kept minus removed) per threshold")
    years = sorted({k[1][:4] for k in base})
    header = "year  " + "".join(f"{t:>12}" for t, _, _ in variants)
    print(header)
    print("-" * len(header))
    for y in years:
        cells = []
        for tag, pct, kept_map in variants:
            kept = [r for k, r in kept_map.items() if k[1][:4] == y]
            removed = [r for k, r in base.items() if k not in kept_map and k[1][:4] == y]
            if len(kept) < 20 or len(removed) < 20:
                cells.append(f"{'-':>12}")
                continue
            cells.append(f"{statistics.fmean(kept) - statistics.fmean(removed):>+12.3f}")
        print(f"{y:<6}" + "".join(cells))
    print("\nA threshold worth adopting is positive in most years, not just pooled.")


if __name__ == "__main__":
    main()
