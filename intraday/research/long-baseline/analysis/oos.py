#!/usr/bin/env python3
"""Does long-side consistency selection actually generalize?

Select stocks on 2022-2024 using the same rule as the short side (zero
negative years, min sample), then measure them on 2025-2026 data they were
never selected on. Compare against the universe average over the same
out-of-sample period -- that difference is the only thing that says whether
the selection carries information or is fitting noise.
"""
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

IS = [2022, 2023, 2024]     # selection window
OOS = [2025, 2026]          # untouched by selection

def agg(yrs, years):
    v = [yrs[y] for y in years if y in yrs]
    n = sum(x[0] for x in v)
    if n == 0:
        return 0, 0.0
    return n, sum(x[0] * x[1] for x in v) / n

print("=" * 76)
print("OUT-OF-SAMPLE TEST: select on 2022-2024, measure on 2025-2026")
print("=" * 76)
print(f"{'strategy':<24}{'picked':>7}{'IS AvgR':>9}{'OOS AvgR':>10}"
      f"{'univ OOS':>10}{'edge':>8}{'held':>7}")

for run, name in RUNS.items():
    picked, univ_n, univ_sum = [], 0, 0.0
    for (rid, sym), yrs in data.items():
        if rid != run:
            continue
        n_o, r_o = agg(yrs, OOS)
        if n_o:                       # universe baseline over the same period
            univ_n += n_o
            univ_sum += n_o * r_o
        vals = [yrs.get(y) for y in IS]
        if any(v is None or v[0] < 20 for v in vals):
            continue
        if any(v[1] < 0 for v in vals):
            continue
        n_i, r_i = agg(yrs, IS)
        picked.append((sym, n_i, r_i, n_o, r_o))
    if not picked:
        print(f"{name:<24}{0:>7}{'-':>9}{'-':>10}{'-':>10}{'-':>8}{'-':>7}")
        continue
    tn_i = sum(p[1] for p in picked)
    tn_o = sum(p[3] for p in picked)
    is_r = sum(p[1] * p[2] for p in picked) / tn_i
    oos_r = sum(p[3] * p[4] for p in picked) / tn_o if tn_o else 0.0
    u_r = univ_sum / univ_n if univ_n else 0.0
    held = sum(1 for p in picked if p[4] > 0)
    print(f"{name:<24}{len(picked):>7}{is_r:>9.3f}{oos_r:>10.3f}"
          f"{u_r:>10.3f}{oos_r - u_r:>8.3f}{held:>4}/{len(picked)}")

print("\n'edge' = how much the picked basket beat the whole universe out of")
print("sample. 'held' = how many picks stayed positive out of sample.\n")

print("=" * 76)
print("Per-stock detail, ma_alignment (best pool)")
print("=" * 76)
print(f"{'SYM':<13}{'IS n':>7}{'IS AvgR':>9}{'OOS n':>7}{'OOS AvgR':>10}  verdict")
det = []
for (rid, sym), yrs in data.items():
    if rid != 216:
        continue
    vals = [yrs.get(y) for y in IS]
    if any(v is None or v[0] < 20 for v in vals) or any(v[1] < 0 for v in vals):
        continue
    n_i, r_i = agg(yrs, IS)
    n_o, r_o = agg(yrs, OOS)
    det.append((sym, n_i, r_i, n_o, r_o))
for sym, n_i, r_i, n_o, r_o in sorted(det, key=lambda x: -x[2]):
    print(f"{sym:<13}{n_i:>7}{r_i:>9.3f}{n_o:>7}{r_o:>10.3f}  "
          f"{'held' if r_o > 0 else 'FAILED'}")
