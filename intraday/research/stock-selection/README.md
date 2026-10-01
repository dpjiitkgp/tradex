# Stock selection: a liquidity floor is the one validated improvement — 2026-09-29

> **CORRECTION (same day, later).** The claim below that the liquidity floor is
> "the one validated improvement" is **over-stated**. It was tested under one
> selection rule (zero-negative-years, top 10, no win-rate floor), where it gave
> 5/5 years above breakeven versus 3/5. Re-tested under the rule that actually
> selected the live book (top 15 by Avg R, win% >= 48) the benefit is **mixed**:
> it helps at a 4-year lookback (0.078 -> 0.099), hurts at 3-year
> (0.100 -> 0.087), and is neutral at 5-year.
>
> So the floor's value is **criteria-dependent**, not a robust win. What is
> robust is that it raises book capacity from ~Rs1-2L to ~Rs11L, which is an
> execution argument rather than an edge argument.
>
> See `window-length-findings.md` for the fuller picture, including a second
> over-claim (that "1 year is the sweet spot" for lookback) that also failed
> once tested at a different refresh frequency.

Five hypotheses were tested overnight (context filters, entry-start time,
entry-window narrowing, later cutoffs, time-of-day concentration). **Four
failed out-of-sample validation.** This is the one that survived, and it
survived the strictest test available: walk-forward, where each year's basket
is chosen using only data that existed before that year.

## The result

Re-select every year from the 250-stock universe, trade the chosen basket for
the following year. Top 10 by Avg R, requiring every training year non-negative.

| test year | no liquidity floor | **>= Rs4.5M/min turnover** |
|---|---|---|
| 2022 | 0.132 (2.16x BE) | 0.126 (2.07x) |
| 2023 | 0.117 (1.92x) | 0.090 (1.47x) |
| 2024 | 0.085 (1.39x) | 0.094 (1.55x) |
| 2025 | **-0.001 (fails)** | **0.116 (1.91x)** |
| 2026 | **0.012 (fails)** | **0.085 (1.39x)** |
| **pooled** | 0.077 (1.25x) | **0.105 (1.71x)** |

The filtered selection clears the 0.061 R breakeven in **all five years**. The
unfiltered selection — which is how the current live book was chosen —
degrades steadily and falls **below cost in 2025 and 2026**.

## It is not knife-edged

Pooled walk-forward Avg R, every cell fully out of sample:

| turnover floor | top8 | top10 | top15 | top20 |
|---|---|---|---|---|
| none | 0.061 (3/5 yrs) | 0.077 (3/5) | 0.079 (3/5) | 0.094 (3/5) |
| >= Rs2.0M/min | 0.085 (4/5) | 0.089 (3/5) | 0.096 (4/5) | 0.105 (4/5) |
| **>= Rs4.5M/min** | **0.100 (5/5)** | **0.105 (5/5)** | **0.104 (5/5)** | 0.088 (4/5) |
| >= Rs9.0M/min | 0.094 (3/5) | 0.070 (3/5) | 0.072 (3/5) | 0.072 (3/5) |

Best at every basket size from 8 to 15. A higher floor is worse — it starves
the candidate pool. So there is a real optimum near Rs4.5M/min rather than a
fitted one.

## Why it works

Thin stocks have noisier per-stock estimates, so when you rank 250 names by
Avg R and take the top, thin names win the ranking disproportionately *by
luck* and then regress. The floor removes the noisiest candidates before
selection rather than after. That also explains the unfiltered basket's
decay: as more thin recent listings entered the universe, more of them won
the ranking on noise.

## Training window

| window | pooled Avg R | years > BE | avg picks | trades/yr |
|---|---|---|---|---|
| expanding (2019->) | 0.104 | 5/5 | 11.2 | 460 |
| rolling 3y | 0.086 | 3/5 | 15.0 | 608 |
| rolling 4y | 0.075 | 3/5 | 15.0 | 595 |
| **rolling 5y** | **0.104** | **5/5** | 13.2 | **537** |

Rolling 5y matches expanding on edge and consistency while producing a larger
basket and more signals. Expanding over the full 8 years is too strict — the
zero-negative-year rule leaves only 5 names, too few to fill the single
position slot daily.

## The basket this selects today

Rolling 5y (2022-2026), >= Rs4.5M/min, top 15:

| symbol | Rs/min | Avg R 22-26 | capacity | vs live |
|---|---|---|---|---|
| APARINDS | 6,760,526 | 0.337 | 1,502,339 | keep |
| HAL | 10,810,897 | 0.327 | 2,402,422 | keep |
| TMPV | 9,359,231 | 0.293 | 2,079,829 | NEW |
| MAZDOCK | 5,164,906 | 0.276 | 1,147,757 | keep |
| VEDL | 12,156,283 | 0.230 | 2,701,396 | NEW |
| LICI | 9,059,053 | 0.224 | 2,013,123 | keep |
| COCHINSHIP | 4,993,158 | 0.180 | 1,109,591 | NEW |
| BHEL | 12,314,125 | 0.160 | 2,736,472 | NEW |
| BSE | 36,586,644 | 0.155 | 8,130,365 | NEW |
| RECLTD | 5,073,513 | 0.153 | 1,127,447 | NEW |
| UNIONBANK | 6,059,764 | 0.142 | 1,346,614 | NEW |
| LTM | 5,353,018 | 0.134 | 1,189,560 | keep |
| DMART | 5,994,582 | 0.131 | 1,332,129 | keep |
| LUPIN | 5,317,407 | 0.124 | 1,181,646 | NEW |
| APOLLOHOSP | 7,632,522 | 0.117 | 1,696,116 | NEW |

- ~641 trades/yr (2.6/day) — comfortably fills the one position slot
- **Book capacity ~Rs11L**, against ~Rs1-2L for the current book
- Overlap with the live 15: **6/15**
- Would drop: AWL, BAJAJHFL, DIXON, INDUSINDBK, IREDA, ITCHOTELS, JIOFIN,
  JSWINFRA, LLOYDSME

Note that the dropped set contains **all six of the recent listings**
(BAJAJHFL, IREDA, ITCHOTELS, JIOFIN, JSWINFRA, LLOYDSME) that were previously
flagged as having essentially no pre-2025 record. The rule excludes them on
principled grounds — they cannot be evaluated over a five-year window — rather
than on a judgement call.

## Before acting

1. **This is a 9-of-15 change to a live book.** It should be a deliberate
   decision, not an automatic one.
2. **The Avg R column is in-sample** for 2022-26. What is validated is the
   *rule* (~0.105 pooled, out of sample), not these per-stock numbers.
3. **The new names have no per-stock tuning.** Selection used the shared
   config (lookback 20, MRM 3.0, trail 0.75%). Per-stock lookback/MRM/trail
   would need running for the 9 new stocks — and note that lookback silently
   sets effective entry time (see ../entrywindow/README.md), which is a
   confound worth fixing in the same pass.
4. **Capacity is only as good as its thinnest name** — COCHINSHIP at ~Rs1.1M.
   Deploying Rs10L means every position is ~1x that stock's per-minute
   turnover, so expect fill slippage the backtest does not model.

## Reproduce

```bash
python3 research/stock-analysis/walkforward.py --topn 10 --min-turnover 4500000
```
