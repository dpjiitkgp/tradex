# Does the selection window matter? — 6m vs 24m vs everything between

The live book (`config/tune30_live.toml`) selects on a **24-month** trailing
window, re-ranked quarterly. The committed detail report was a **6-month**
window. This note compares them properly, because the two reports as generated
are not comparable and the difference is not small.

All runs: strategy run 417, random control run 263, both at the achievable
**15:10** square-off. Top 25 by Avg R, `n >= 15`, win% >= 48, no liquidity
floor. 1 R = Rs.3,750. Breakeven 0.061 R, already subtracted from every `net`.

## Why the two reports can't be compared as written

A 24-month window cannot select anything until it has 24 months of history, so
its first test quarter is **2020-12**. The 6-month report starts testing in
**2019-06**. The 6m report therefore contains 2019 (+0.0880 net R, its best
year) and the first COVID year, and the 24m report does not.

`--first-test YYYY-MM` now forces a common start so this cannot happen by
accident again.

| | 6m report as written | 24m report as written |
|---|---|---|
| first test quarter | 2019Q2 | 2020Q4 |
| trades | 8573 | 6621 |
| net R | +0.0269 | +0.0133 |
| entry edge | +0.0287 | +0.0124 |

Half of that gap is just the two extra good years.

## Matched: both forced to start 2020-12

23 quarters, 2020Q4..2026Q3.

| window | trades | net R | random | entry | quarters > 0 |
|---|---|---|---|---|---|
| 6m | 6717 | +0.0194 | -0.0139 | +0.0334 | 15/24 |
| 24m | 6621 | +0.0133 | +0.0009 | +0.0124 | 14/24 |

Paired by quarter, **6m minus 24m = +0.0129 R/trade, SE 0.0143, t +0.90**.
That is not a difference. On a matched sample the two windows are the same
thing.

## The one quarter that carries the 24m number

2020Q4 (test Dec 2020..Feb 2021) is **+0.3063 net R on 282 trades** for the
24m book — five times any other quarter in the series. It exists in the 24m
report only because that is the first quarter the window can trade, and in a
by-year table it lands alone in a "2020" row that is one quarter long.

Drop it and look at **2021Q1 onward**, 23 quarters:

| window | trades | net R | random | entry | quarters > 0 |
|---|---|---|---|---|---|
| 6m | 6435 | **+0.0148** | -0.0087 | +0.0235 | 14/23 |
| 24m | 6339 | **+0.0002** | -0.0003 | +0.0006 | 13/23 |

**The live window has returned nothing since 2021.** Not a small edge — zero,
and its random-entry control is also zero, so neither the selection nor the
signal is contributing. Its entire pooled +0.0133 is one post-COVID quarter.

The 6m window's +0.0148 is positive but 0.85 SE (SE 0.0175). It is not
evidence of an edge either; it is only less bad.

## Full years, 1-slot P&L

Part-years excluded (the by-year tables now mark them `(k/4 q)`). 2026 is
three quarters and shown as an annualised rate, not a year's earnings.

| year | 6m | 24m |
|---|---|---|
| 2021 | Rs.+28,496 | Rs.+37,102 |
| 2022 | Rs.+3,185 | Rs.+32,470 |
| 2023 | Rs.-4,245 | Rs.-51,943 |
| 2024 | Rs.+11,978 | Rs.+9,894 |
| 2025 | Rs.+42,438 | Rs.-3,458 |
| **mean full year** | **Rs.+16,370** | **Rs.+4,813** |
| 2026 (3q, rate) | Rs.-8,681 | **Rs.-54,454** |

2026 to date is the sharpest number here: the live 24m window is running at
**-0.0581 R/trade**, six times worse than the 6m window's -0.0093 on the same
580 trades. The two books are nearly the same size and nearly as active; they
are holding different names.

## But window length is noise, so don't switch

Eight windows, all forced to start 2020-12, 21 quarters common to all
(2021Q1..2026Q3, 3m skips periods where fewer than 5 stocks clear the filter):

| window | trades | net R | entry | q > 0 | paired vs 24m |
|---|---|---|---|---|---|
| 3m | 4564 | +0.0220 | +0.0182 | 9/21 | +0.0098 (t +0.40) |
| 6m | 5838 | +0.0064 | +0.0160 | 12/21 | +0.0105 (t +0.68) |
| 9m | 5735 | -0.0108 | -0.0134 | 9/21 | -0.0018 (t -0.13) |
| 12m | 5791 | **+0.0347** | +0.0146 | 16/21 | +0.0496 (t +2.93) |
| 15m | 5812 | +0.0056 | -0.0032 | 11/21 | +0.0109 (t +0.85) |
| 18m | 5760 | +0.0120 | +0.0092 | 10/21 | +0.0186 (t +1.25) |
| 21m | 5800 | **+0.0241** | +0.0047 | 15/21 | +0.0336 (t +3.11) |
| 24m | 5752 | -0.0059 | -0.0041 | 11/21 | — |

Two windows clear t = 2.9 against the live one. Neither is real:

* 12m is +0.0347, and its neighbours are **9m at -0.0108** and 15m at +0.0056.
* 21m is +0.0241, and its neighbours are 18m at +0.0120 and 24m at -0.0059.

A parameter that genuinely matters does not swing 0.045 R on a three-month
change and then swing back. This is a sawtooth, not a curve with an optimum,
and the t-statistics are what picking the maximum of eight correlated series
looks like. Switching the live book to 12m would be fitting the noise.

## What this actually says

1. **The 24m window is not wrong; it is not right either.** Window length
   carries no signal, which also means there is no evidenced reason to change
   the live config. Leave it.
2. **The real finding is the one hiding behind the window question.** Every
   report in this folder shows a positive pooled number that leans on
   2019-2020. From 2021 onward the best of eight windows is +0.0347 (a noise
   peak) and the median is +0.0092 — against a 0.061 R cost already subtracted
   and roughly 0.06 R of unmodelled market impact still to come. Nothing here
   is distinguishable from zero over the last five years.
3. **2026Q1 is bad in every window** (-0.1437, -0.1570, -0.1561 for 6/12/24m).
   That is a regime quarter, not a selection failure, and is the one worth
   studying on its own.

## Reproduce

```sh
# the two committed reports, each at its natural start
python3 research/stock-analysis/detail_6m_quarterly.py --strategy-run 417 \
  --random-run 263 --window 6  --refresh 3 --topn 25 --min-trades 15 \
  --out research/stock-selection/detail-6m-quarterly.md
python3 research/stock-analysis/detail_6m_quarterly.py --strategy-run 417 \
  --random-run 263 --window 24 --refresh 3 --topn 25 --min-trades 15 \
  --out research/stock-selection/detail-24m-quarterly.md

# matched comparison — add --first-test 2020-12 to any window
```
