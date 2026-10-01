#!/usr/bin/env python3
"""Test the ACTUAL selection procedure: zero negative years, then rank by
Avg R and take the best N -- optionally with a win-rate floor. Measured
out of sample on 2025-2026 against the universe over the same period."""
import csv, collections
from pathlib import Path

PATH = Path(__file__).with_name("long_yearly_all.tsv")
RUNS = {215: "ma_alignment_confirmed", 216: "ma_alignment",
        217: "trend_confluence", 218: "strong_rally"}
rows = list(csv.DictReader(open(PATH), delimiter="\t"))
data = collections.defaultdict(dict)
for r in rows:
    data[(int(r["run_id"]), r["sym"])][int(r["yr"])] = (
        int(r["n"]), float(r["avg_r"]), float(r["win_pct"]))

IS, OOS = [2022, 2023, 2024], [2025, 2026]

def agg(yrs, years, idx=1):
    v = [yrs[y] for y in years if y in yrs]
    n = sum(x[0] for x in v)
    return (n, sum(x[0] * x[idx] for x in v) / n) if n else (0, 0.0)

def run_sel(run, topn, win_floor):
    cands, univ_n, univ_s = [], 0, 0.0
    for (rid, sym), yrs in data.items():
        if rid != run:
            continue
        n_o, r_o = agg(yrs, OOS)
        if n_o:
            univ_n += n_o; univ_s += n_o * r_o
        vals = [yrs.get(y) for y in IS]
        if any(v is None or v[0] < 20 for v in vals) or any(v[1] < 0 for v in vals):
            continue
        n_i, r_i = agg(yrs, IS)
        _, w_i = agg(yrs, IS, idx=2)
        if w_i < win_floor:
            continue
        cands.append((sym, n_i, r_i, n_o, r_o))
    cands.sort(key=lambda x: -x[2])
    sel = cands[:topn]
    if not sel:
        return None
    tn_i, tn_o = sum(s[1] for s in sel), sum(s[3] for s in sel)
    is_r = sum(s[1] * s[2] for s in sel) / tn_i
    oos_r = sum(s[3] * s[4] for s in sel) / tn_o if tn_o else 0.0
    u_r = univ_s / univ_n if univ_n else 0.0
    held = sum(1 for s in sel if s[4] > 0)
    return len(sel), is_r, oos_r, u_r, oos_r - u_r, held

print("=" * 82)
print("THE ACTUAL PROCEDURE: zero neg years -> rank by Avg R -> take best N")
print("Selected on 2022-2024, measured on 2025-2026")
print("=" * 82)
for floor, ftxt in ((0.0, "no win floor"), (44.0, "win% >= 44"), (48.0, "win% >= 48")):
    print(f"\n### {ftxt}")
    print(f"{'strategy':<24}{'N':>4}{'IS AvgR':>9}{'OOS AvgR':>10}"
          f"{'univ OOS':>10}{'edge':>8}{'held':>9}")
    for topn in (15, 30):
        for run, name in RUNS.items():
            res = run_sel(run, topn, floor)
            if not res:
                print(f"{name+' top'+str(topn):<24}{'-':>4}{'(none qualify)':>37}")
                continue
            n, is_r, oos_r, u_r, edge, held = res
            print(f"{name+' top'+str(topn):<24}{n:>4}{is_r:>9.3f}{oos_r:>10.3f}"
                  f"{u_r:>10.3f}{edge:>+8.3f}{held:>6}/{n}")
