#!/usr/bin/env python3
"""Long-side consistency selection, mirroring the short-side methodology:
require every year in the window to be non-negative on Avg R, with a
minimum sample per year so a 3-trade year can't qualify a stock."""
import csv, collections, sys
from pathlib import Path

PATH = Path(__file__).with_name("long_yearly_all.tsv")
RUNS = {215: "ma_alignment_confirmed", 216: "ma_alignment",
        217: "trend_confluence", 218: "strong_rally"}

rows = list(csv.DictReader(open(PATH), delimiter="\t"))
data = collections.defaultdict(dict)  # (run, sym) -> {yr: (n, avg_r, win)}
for r in rows:
    data[(int(r["run_id"]), r["sym"])][int(r["yr"])] = (
        int(r["n"]), float(r["avg_r"]), float(r["win_pct"]))

def survivors(run, years, min_n_per_year):
    out = []
    for (rid, sym), yrs in data.items():
        if rid != run:
            continue
        vals = [yrs.get(y) for y in years]
        if any(v is None or v[0] < min_n_per_year for v in vals):
            continue                      # missing or too-thin year
        if any(v[1] < 0 for v in vals):
            continue                      # a negative year disqualifies
        tot_n = sum(v[0] for v in vals)
        # trade-weighted, so a stock isn't flattered by a tiny stellar year
        w_r = sum(v[0] * v[1] for v in vals) / tot_n
        w_w = sum(v[0] * v[2] for v in vals) / tot_n
        out.append((sym, tot_n, w_r, w_w, min(v[1] for v in vals)))
    return sorted(out, key=lambda x: -x[2])

W_FULL = list(range(2019, 2027))
W_POST = list(range(2022, 2027))

print("=" * 78)
print("SURVIVOR COUNTS  (zero negative years, >=20 trades in every year)")
print("=" * 78)
print(f"{'strategy':<26}{'2019-2026':>12}{'2022-2026':>12}")
for run, name in RUNS.items():
    a = len(survivors(run, W_FULL, 20))
    b = len(survivors(run, W_POST, 20))
    print(f"{name:<26}{a:>12}{b:>12}")

for run, name in RUNS.items():
    for label, win in (("2019-2026", W_FULL), ("2022-2026", W_POST)):
        s = survivors(run, win, 20)
        if not s:
            continue
        print(f"\n--- {name} / {label} : {len(s)} survivors (top 15) ---")
        print(f"{'SYM':<13}{'TRADES':>8}{'AVG R':>9}{'WIN%':>8}{'WORST YR R':>12}")
        for sym, n, r, w, worst in s[:15]:
            print(f"{sym:<13}{n:>8}{r:>9.3f}{w:>8.1f}{worst:>12.3f}")
