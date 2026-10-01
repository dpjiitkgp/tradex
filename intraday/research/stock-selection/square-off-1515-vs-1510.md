# What the 15:15 -> 15:10 square-off cost the live 24-month book

Zerodha refuses MIS orders from **15:12**, so a 15:15 square-off was never
tradable. Every number produced before 2026-09-30 assumed one. This measures
exactly what moving to the achievable 15:10 did to the live book's selection
(24-month window, quarterly refresh, top 25, `n >= 15`, win% >= 48).

Run **110** = 15:15, run **417** = 15:10. Both verified from the data, not the
label: the EOD exit spike sits at 09:45 UTC for run 110 and 09:40 UTC for run
417 (and for the random control, run 263).

## Headline

| 24m book, 24 matched quarters 2020Q4..2026Q3 | 15:15 | 15:10 |
|---|---|---|
| net R / trade | **+0.0559** | **+0.0133** |
| 1-slot P&L at 250 trades/yr | Rs.+52,401 | Rs.+12,446 |
| quarters positive | 18/24 | 14/24 |

**-0.0426 R per trade — about Rs.40,000/year of the 1-slot P&L, and 76% of the
measured edge.** Mean quarterly delta -0.0475, SE 0.0122, **t -3.91**, 19 of 24
quarters worse. This is not a sampling artefact.

## It is the same strategy, not a different one

Joining the two runs on `(instrument_token, entry_ts)`:

| | trades |
|---|---|
| matched in both runs | 80,781 |
| only in run 110 (15:15) | 441 |
| only in run 417 (15:10) | 377 |

99.5% of trades are the same trade in both runs. So this is purely an exit
change, and it splits cleanly:

| matched trades | n | 15:15 Avg R | 15:10 Avg R | delta |
|---|---|---|---|---|
| exited before 15:10 anyway (stop/trail/target) | 35,514 | -0.7410 | -0.7410 | 0.0000 |
| exit moved | 45,267 | +0.6633 | +0.6307 | **-0.0325** |
| of which 15:15 -> 15:10 | 43,999 | +0.6762 | +0.6367 | -0.0394 |

**56% of trades were still open at 15:10**, and those are the profitable ones
(+0.66 Avg R against -0.74 for the ones already stopped out). The five minutes
removed cost each of them 0.0325 R.

## The selected book is hurt 2.3x more than the average stock

Averaged over the whole universe the exit change is worth **-0.0182 R/trade**.
On the 24-month selected book it is **-0.0426**. Holding the basket fixed and
changing only the exit isolates why:

| | net R |
|---|---|
| 15:15, selecting on 15:15 data (what the old report showed) | +0.0559 |
| 15:15, on the basket the 15:10 run selected | +0.0388 |
| 15:10, on that same basket (the live book) | +0.0133 |

* **-0.0255 is the five minutes themselves** on a fixed basket (t -3.65, 20/24
  quarters worse, Rs.-23,900/yr).
* **-0.0171 is the selection getting worse.** Re-ranking on 15:10 data picks a
  basket that is inferior *measured at either square-off*. Cutting the five
  minutes flattens the trailing Avg R spread the ranking depends on, so the
  top 25 is less separated from the rest.

The 2.3x gap is the uncomfortable part: ranking stocks by trailing Avg R
preferentially selects stocks that make their money in the last five minutes
of the session — money the broker will not let us collect.

## By year

| year | 15:15 R | 15:10 R | delta | 15:15 1-slot | 15:10 1-slot | lost/yr |
|---|---|---|---|---|---|---|
| 2020 (1q) | +0.2705 | +0.3063 | +0.0358 | Rs.+253,594 | Rs.+287,156 | Rs.+33,562 |
| 2021 | +0.0337 | +0.0396 | +0.0059 | Rs.+31,594 | Rs.+37,125 | Rs.+5,531 |
| 2022 | +0.1597 | +0.0346 | **-0.1251** | Rs.+149,719 | Rs.+32,438 | **Rs.-117,281** |
| 2023 | -0.0281 | -0.0554 | -0.0273 | Rs.-26,344 | Rs.-51,938 | Rs.-25,594 |
| 2024 | +0.0634 | +0.0106 | -0.0528 | Rs.+59,438 | Rs.+9,938 | Rs.-49,500 |
| 2025 | +0.0236 | -0.0037 | -0.0273 | Rs.+22,125 | Rs.-3,469 | Rs.-25,594 |
| 2026 (3q) | -0.0107 | -0.0581 | -0.0474 | Rs.-10,031 | Rs.-54,469 | Rs.-44,438 |
| **mean full year** | **+0.0505** | **+0.0051** | **-0.0453** | **Rs.+47,306** | **Rs.+4,819** | **Rs.-42,488** |

Full years profitable: **4/5 at 15:15, 3/5 at 15:10**.

2022 carries most of the damage. It was the book's best year at 15:15
(+0.1597, all four quarters between +0.13 and +0.17) and is ordinary at 15:10
(+0.0346). A trending-down year is exactly when a short still open at 15:10 is
most likely to keep falling into the close, so the five minutes were worth the
most there — and are now unreachable.

## Why this matters beyond the number

The old 24-month report showed a book earning roughly Rs.47,000 in a mean full
year. The same book at a square-off we can actually execute earns **Rs.4,819**,
before ~0.06 R of unmodelled market impact and before the fill failures already
seen live (2 for 2 on 2026-09-30/10-01). The 15:15 assumption was not a rounding
error; it was most of the edge.

Related: [[window-length-matched.md]] shows the residual +0.0133 is itself one
post-COVID quarter, and that the live window has returned ~0 since 2021.

## Reproduce

```sh
# 15:15 book
python3 research/stock-analysis/detail_6m_quarterly.py --strategy-run 110 \
  --random-run 263 --window 24 --refresh 3 --topn 25 --min-trades 15 \
  --out /tmp/w24-run110-1515.md
# 15:10 book is the committed research/stock-selection/detail-24m-quarterly.md
```

Note the `random` / `entry` columns of the run-110 report are **not** usable:
its control (run 263) is a 15:10 run, which is the confound that invalidated
the original "it beats random entry" claim. Only the strategy's own net R is
comparable across these two runs.
