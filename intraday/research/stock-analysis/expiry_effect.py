#!/usr/bin/env python3
"""Does the short book perform differently on F&O expiry days?

Hypothesis: expiry sessions behave differently (pinning, unwinding, higher
index-driven volatility), so excluding them might raise Avg R.

Classifies every trade date as:
  * monthly expiry  - last Thursday of the month (stock F&O expiry)
  * weekly expiry   - any other Thursday (index weekly expiry)
  * normal          - everything else

Reported against the 0.061 R breakeven, and broken down BY YEAR, because a
single in-sample/held-out split is not enough on this dataset: an effect can
clear it purely because one strong year lands on the right side of the cut.
If the yearly sign alternates, it is noise.

Usage: python3 expiry_effect.py [--data <tsv>]
"""
import argparse
import collections
import csv
import datetime as dt
from pathlib import Path

BREAKEVEN_R = 0.061


def last_thursday(year, month):
    d = dt.date(year, month, 1)
    nxt = dt.date(year + (month == 12), (month % 12) + 1, 1)
    d = nxt - dt.timedelta(days=1)
    while d.weekday() != 3:          # 3 = Thursday
        d -= dt.timedelta(days=1)
    return d


def classify(date_str):
    d = dt.date.fromisoformat(date_str)
    if d.weekday() != 3:
        return "normal"
    return "monthly expiry" if d == last_thursday(d.year, d.month) else "weekly expiry"


def stats(rows):
    n = len(rows)
    if not n:
        return 0, 0.0, 0.0
    return (n,
            sum(r for r, _ in rows) / n,
            100.0 * sum(1 for _, p in rows if p > 0) / n)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--data",
                    default=Path(__file__).with_name("expiry_trades.tsv"))
    a = ap.parse_args()

    rows = []
    for x in csv.DictReader(open(a.data), delimiter="\t"):
        try:
            rows.append((x["d"], float(x["r"]), float(x["p"]), x["sym"]))
        except (ValueError, KeyError):
            continue
    if not rows:
        raise SystemExit("no trades loaded")

    buckets = collections.defaultdict(list)
    for d, r, p, _ in rows:
        buckets[classify(d)].append((r, p))

    all_n, all_r, all_w = stats([(r, p) for _, r, p, _ in rows])
    print("=" * 76)
    print(f"EXPIRY-DAY EFFECT — 15-stock short book, {all_n} trades")
    print(f"baseline Avg R {all_r:.4f}  ({all_r/BREAKEVEN_R:.2f}x breakeven)  win {all_w:.1f}%")
    print("=" * 76)
    print(f"\n{'bucket':<18}{'trades':>8}{'share':>8}{'Avg R':>9}{'win%':>8}{'xBE':>7}")
    for k in ("normal", "weekly expiry", "monthly expiry"):
        n, r, w = stats(buckets[k])
        if not n:
            continue
        print(f"{k:<18}{n:>8}{100.0*n/all_n:>7.1f}%{r:>9.4f}{w:>8.1f}{r/BREAKEVEN_R:>6.2f}x")

    # What excluding each bucket would leave behind — the decision-relevant view,
    # since what matters is the Avg R of the trades you would still take.
    print(f"\n{'if you exclude...':<26}{'kept':>8}{'Avg R':>9}{'dR':>9}{'xBE':>7}")
    base_all = [(r, p) for _, r, p, _ in rows]
    for excl in (("monthly expiry",), ("weekly expiry",), ("weekly expiry", "monthly expiry")):
        kept = [(r, p) for d, r, p, _ in rows if classify(d) not in excl]
        n, r, _ = stats(kept)
        label = "all Thursdays" if len(excl) == 2 else excl[0]
        print(f"{label:<26}{n:>8}{r:>9.4f}{r-all_r:>+9.4f}{r/BREAKEVEN_R:>6.2f}x")

    # The decisive check: is the gap stable, or one year carrying it?
    print(f"\n{'year':<8}{'normal n':>10}{'normal R':>10}{'expiry n':>10}"
          f"{'expiry R':>10}{'diff':>9}")
    print("-" * 57)
    by_year = collections.defaultdict(lambda: {"normal": [], "expiry": []})
    for d, r, p, _ in rows:
        k = "normal" if classify(d) == "normal" else "expiry"
        by_year[d[:4]][k].append((r, p))
    flips = []
    for y in sorted(by_year):
        nn, nr, _ = stats(by_year[y]["normal"])
        en, er, _ = stats(by_year[y]["expiry"])
        if nn < 20 or en < 20:
            continue
        diff = nr - er
        flips.append(diff)
        print(f"{y:<8}{nn:>10}{nr:>10.3f}{en:>10}{er:>10.3f}{diff:>+9.3f}")
    if flips:
        pos = sum(1 for f in flips if f > 0)
        print(f"\nnormal beat expiry in {pos}/{len(flips)} years.")
        print("Consistent sign across years = real; alternating = noise.")


if __name__ == "__main__":
    main()
