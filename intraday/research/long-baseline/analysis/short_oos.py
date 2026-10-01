#!/usr/bin/env python3
"""Does the SHORT-side selection hold up out of sample?

Run 110 (tune30 mrm-3.0/trail-0.0075) is the run the live 18 were picked
from. Same test as the long side: select on 2022-2024, measure on
2025-2026. Then, separately, track the 18 stocks actually deployed live --
their in-sample Avg R is what the live sizing assumptions rest on.
"""
import csv, collections, os, sys
from pathlib import Path

PATH = Path(__file__).with_name("short_yearly_110.tsv")
LIVE18 = ["APARINDS", "AWL", "BAJAJHFL", "COCHINSHIP", "DIXON", "DMART",
          "HAL", "INDUSINDBK", "IREDA", "ITCHOTELS", "JIOFIN", "JSWINFRA",
          "LICI", "LINDEINDIA", "LLOYDSME", "LTM", "MAZDOCK", "VOLTAS"]
COST_R = 0.047 / 0.75          # round-trip cost expressed in R units

rows = list(csv.DictReader(open(PATH), delimiter="\t"))
data = collections.defaultdict(dict)
for r in rows:
    data[r["sym"]][int(r["yr"])] = (
        int(r["n"]), float(r["avg_r"]), float(r["win_pct"]))

years = sorted({int(r["yr"]) for r in rows})
print(f"run 110 covers {years[0]}..{years[-1]}  ({len(data)} symbols)\n")

IS = [y for y in (2022, 2023, 2024) if y in years]
OOS = [y for y in (2025, 2026) if y in years]

def agg(yrs, window, idx=1):
    v = [yrs[y] for y in window if y in yrs]
    n = sum(x[0] for x in v)
    return (n, sum(x[0] * x[idx] for x in v) / n) if n else (0, 0.0)

univ_n = univ_s = 0
for sym, yrs in data.items():
    n, r = agg(yrs, OOS)
    univ_n += n; univ_s += n * r
univ_oos = univ_s / univ_n if univ_n else 0.0

print("=" * 78)
print("SHORT SIDE: select on 2022-2024, measure on 2025-2026")
print("=" * 78)
cands = []
for sym, yrs in data.items():
    vals = [yrs.get(y) for y in IS]
    if any(v is None or v[0] < 20 for v in vals) or any(v[1] < 0 for v in vals):
        continue
    n_i, r_i = agg(yrs, IS)
    n_o, r_o = agg(yrs, OOS)
    cands.append((sym, n_i, r_i, n_o, r_o))
cands.sort(key=lambda x: -x[2])

print(f"{'N':>5}{'IS AvgR':>10}{'OOS AvgR':>11}{'univ OOS':>11}"
      f"{'edge':>9}{'OOS net':>10}{'held':>9}")
for topn in (15, 18, 30, len(cands)):
    sel = cands[:topn]
    if not sel:
        continue
    tn_i, tn_o = sum(s[1] for s in sel), sum(s[3] for s in sel)
    is_r = sum(s[1] * s[2] for s in sel) / tn_i
    oos_r = sum(s[3] * s[4] for s in sel) / tn_o if tn_o else 0.0
    held = sum(1 for s in sel if s[4] > 0)
    print(f"{len(sel):>5}{is_r:>10.3f}{oos_r:>11.3f}{univ_oos:>11.3f}"
          f"{oos_r - univ_oos:>+9.3f}{oos_r - COST_R:>+10.3f}{held:>6}/{len(sel)}")

print(f"\ncost = {COST_R:.3f} R per trade; 'OOS net' is out-of-sample Avg R "
      f"after costs.\n")

print("=" * 78)
print("THE 18 STOCKS CURRENTLY DEPLOYED LIVE")
print("=" * 78)
print(f"{'SYM':<13}{'IS n':>7}{'IS AvgR':>9}{'OOS n':>7}{'OOS AvgR':>10}"
      f"{'net':>8}  verdict")
tot_i = tot_o = 0
si = so = 0.0
missing = []
for sym in LIVE18:
    if sym not in data:
        missing.append(sym); continue
    n_i, r_i = agg(data[sym], IS)
    n_o, r_o = agg(data[sym], OOS)
    tot_i += n_i; si += n_i * r_i
    tot_o += n_o; so += n_o * r_o
    v = "held" if r_o - COST_R > 0 else ("thin" if r_o > 0 else "FAILED")
    print(f"{sym:<13}{n_i:>7}{r_i:>9.3f}{n_o:>7}{r_o:>10.3f}"
          f"{r_o - COST_R:>+8.3f}  {v}")
if missing:
    print(f"\n(no run-110 rows for: {', '.join(missing)})")
pi = si / tot_i if tot_i else 0.0
po = so / tot_o if tot_o else 0.0
print(f"\n{'PORTFOLIO':<13}{tot_i:>7}{pi:>9.3f}{tot_o:>7}{po:>10.3f}"
      f"{po - COST_R:>+8.3f}")
print(f"\nuniverse OOS Avg R over the same period: {univ_oos:.3f}")
print(f"portfolio edge vs universe, out of sample: {po - univ_oos:+.3f}")
