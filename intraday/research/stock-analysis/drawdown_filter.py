#!/usr/bin/env python3
"""Should we skip shorts when the stock is already X% below yesterday's close?

Hypothesis: by the time a stock has fallen X% from the previous close, the
move is exhausted and shorting it is late. Tested at X = 1, 1.5, 2, 2.5, 3.

The decisive number is NOT the Avg R of what survives the filter — that
always looks better by construction, because removing any below-average
trade raises the mean. It is the Avg R of the trades the filter REMOVES,
judged against:

  * 0.061 R  - the cost of trading. Removing trades below this pays.
  * ~0.24 R  - the book average, i.e. the opportunity cost of the single
               position slot. Removing trades ABOVE this is actively
               harmful: the freed slot goes to a worse trade.

Every filter that survives is then broken down by year, because on this
dataset an effect can clear an in-sample/held-out split purely because one
strong year landed on the right side of the cut.
"""
import argparse
import collections
import csv
import math
import sqlite3
from pathlib import Path

BREAKEVEN_R = 0.061
SIGMA = 1.4
# Trade export this script consumes; see the docstring for the query that
# produces it. Override with --data.
DATA = Path(__file__).with_name("gapdown_trades.tsv")


def prev_closes(db, token):
    """IST trading date -> that day's close, from cached minute bars."""
    rows = db.execute(
        "SELECT ts, close FROM candles WHERE instrument_token=? AND interval='1m' ORDER BY ts",
        (token,)).fetchall()
    last = {}
    for ts, c in rows:
        last[ts[:10]] = c          # ordered, so the final write per date is its close
    days = sorted(last)
    return {d: last[days[i - 1]] for i, d in enumerate(days) if i > 0}


def stats(v):
    n = len(v)
    if not n:
        return 0, 0.0, 0.0
    return n, sum(r for r, _ in v) / n, 100.0 * sum(1 for _, p in v if p > 0) / n


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--data", default=DATA)
    ap.add_argument("--db", default="data/papertrader.db")
    a = ap.parse_args()

    db = sqlite3.connect(a.db)
    raw = list(csv.DictReader(open(a.data), delimiter="\t"))
    pc_cache = {}
    rows = []
    for x in raw:
        try:
            tok = int(x["tok"]); ep = float(x["ep"])
            r = float(x["r"]); p = float(x["p"]); d = x["d"]
        except (ValueError, KeyError):
            continue
        if tok not in pc_cache:
            pc_cache[tok] = prev_closes(db, tok)
        pc = pc_cache[tok].get(d)
        if not pc:
            continue
        rows.append((x["sym"], d, (ep - pc) / pc * 100.0, r, p))

    n, base_r, base_w = stats([(r, p) for *_, r, p in rows])
    print("=" * 78)
    print(f"ALREADY-DOWN FILTER — 15-stock short book, {n} trades "
          f"({len(raw)-1-n} dropped: no previous close)")
    print(f"baseline Avg R {base_r:.4f} ({base_r/BREAKEVEN_R:.2f}x BE)  win {base_w:.1f}%")
    print("=" * 78)

    print(f"\nDistribution of entry vs previous close:")
    bands = [(-99, -3, "<-3%"), (-3, -2, "-3..-2"), (-2, -1, "-2..-1"),
             (-1, 0, "-1..0"), (0, 1, "0..+1"), (1, 99, ">+1%")]
    for lo, hi, lbl in bands:
        v = [(r, p) for _, _, g, r, p in rows if lo <= g < hi]
        if v:
            bn, br, bw = stats(v)
            print(f"  {lbl:<10}{bn:>7} trades{br:>9.3f} Avg R{bw:>7.1f}% win{br/BREAKEVEN_R:>7.2f}x BE")

    print(f"\n{'skip if down more than':<24}{'removed':>8}{'REMOVED AvgR':>14}"
          f"{'kept':>7}{'kept AvgR':>11}{'dR':>9}   verdict")
    print("-" * 82)
    survivors = []
    for x in (1.0, 1.5, 2.0, 2.5, 3.0):
        rem = [(r, p) for _, _, g, r, p in rows if g <= -x]
        kep = [(r, p) for _, _, g, r, p in rows if g > -x]
        rn, rr, _ = stats(rem)
        kn, kr, _ = stats(kep)
        if rn < 15:
            print(f"  {'-'+str(x)+'%':<22}{rn:>8}{'(too few to judge)':>14}")
            continue
        if rr < BREAKEVEN_R:
            verdict = "PAYS — removed trades are below cost"
            survivors.append(x)
        elif rr > base_r:
            verdict = "HARMFUL — removes better-than-average trades"
        else:
            verdict = "marginal — removed trades still beat cost"
        print(f"  {'-'+str(x)+'%':<22}{rn:>8}{rr:>14.3f}{kn:>7}{kr:>11.4f}"
              f"{kr-base_r:>+9.4f}   {verdict}")

    if not survivors:
        print("\nNo threshold removes trades that are below the cost of trading.")
        print("On this evidence the filter should not be applied at any X.")
        return

    print(f"\n{'='*78}\nBY-YEAR CHECK for thresholds that looked worth keeping\n{'='*78}")
    for x in survivors:
        print(f"\n--- skip if down more than {x}% ---")
        print(f"  {'year':<7}{'removed n':>11}{'removed AvgR':>14}   (below "
              f"{BREAKEVEN_R} = filter pays that year)")
        by = collections.defaultdict(list)
        for _, d, g, r, p in rows:
            if g <= -x:
                by[d[:4]].append((r, p))
        good = tot = 0
        for y in sorted(by):
            yn, yr, _ = stats(by[y])
            if yn < 10:
                continue
            tot += 1
            good += yr < BREAKEVEN_R
            print(f"  {y:<7}{yn:>11}{yr:>14.3f}   {'pays' if yr < BREAKEVEN_R else 'costs'}")
        if tot:
            print(f"  -> filter paid in {good}/{tot} years"
                  f"{'  (consistent)' if good == tot else '  (INCONSISTENT — likely noise)'}")


if __name__ == "__main__":
    main()
