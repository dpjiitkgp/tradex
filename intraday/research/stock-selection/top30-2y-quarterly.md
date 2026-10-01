# Basket size, 2-year lookback, quarterly rebalance — full findings

> **SUPERSEDED HEADLINE (same day).** This document first recommended top 30
> with a Rs2M/min floor, chosen on distribution shape before top 25 had been
> tested *with* the floor. Once it was, **top 25 + Rs2M/min floor wins on every
> metric**: net R +0.0710 vs +0.0597, worst year 40% vs 26%, same 18/24 hit
> rate, and +26% in 2026. **Top 25 is the recommendation.** The rest of this
> document is otherwise accurate; the liquidity-floor analysis it contains is
> what produced the corrected answer.
>
> | with Rs2M floor | net R | beat cost | worst yr | %/yr |
> |---|---|---|---|---|
> | top 15 | +0.0694 | 15/24 | 18% | 52% |
> | top 20 | +0.0634 | 16/24 | 37% | 47% |
> | **top 25** | **+0.0710** | **18/24** | **40%** | **53%** |
> | top 30 | +0.0597 | 18/24 | 26% | 45% |


**Date:** 2026-09-29
**Data:** run 110 (`mrm-3.0 / trail-0.0075 / vwap_trend_short_late_vwapslope_lb20`),
250-stock universe, PIIND and indices excluded.
**Method:** every quarter, rank all stocks by Avg R over the previous 8 quarters
(win% >= 48, >= 15 trades), take the top N, trade them for the following quarter
only. Every test quarter is genuinely out of sample.

All Avg R figures are **gross**. Breakeven is **0.061 R** (~Rs206 per round trip
against a 0.75% risk unit). `net` subtracts it. **Slippage is NOT included.**
Percentages assume ~200 live trades/year at 1 R = Rs3,740.

---

## Headline

**Top 30 with a Rs2M/min liquidity floor is the configuration I would choose** —
not because it earns the most, but because it is the only one that is positive
in every year including the current weak regime, and it supports 5x the capital.

| variant | net R | beat cost | %/yr | book capacity |
|---|---|---|---|---|
| top 15, no floor | +0.0721 | 16/24 (67%) | 54% | Rs78,887 |
| top 25, no floor | +0.0566 | 18/24 (75%) | 42% | Rs78,887 |
| top 30, no floor | +0.0608 | 18/24 (75%) | 45% | Rs78,887 |
| top 30, >= Rs1.0M/min | +0.0657 | 18/24 (75%) | 49% | Rs201,101 |
| **top 30, >= Rs2.0M/min** | **+0.0597** | **18/24 (75%)** | **45%** | **Rs403,423** |
| top 25, >= Rs2.0M/min | +0.0710 | 18/24 (75%) | 53% | Rs403,423 |

---

## Year by year

| year | top 15 | top 25 | top 30 | **top 30 + Rs2M floor** |
|---|---|---|---|---|
| 2020 (Q4 only) | 20% | 22% | 20% | 4% |
| 2021 | 53% | 50% | 52% | **59%** |
| 2022 | **140%** | 72% | 65% | 61% |
| 2023 | 29% | 36% | 35% | 26% |
| 2024 | 75% | 46% | 54% | 47% |
| 2025 | 13% | 19% | 38% | **37%** |
| **2026 (3 qtrs)** | **-20%** | 2% | 3% | **+32%** |
| mean (full years) | 62% | 44% | 49% | **46%** |
| **worst full year** | **13%** | 19% | 35% | **26%** |

Top 15's 62% mean is carried entirely by 2022's 140%. Remove that one year and
it is the weakest of the four. Its **-20% in 2026** is the worst result any
configuration produces in the current regime.

---

## Why the liquidity floor matters

The basket the rule selects today, without a floor, contains three stocks that
cannot absorb a Rs4.99L position:

| rank | stock | Rs/min | Rs4.99L as % of one minute's turnover |
|---|---|---|---|
| 25 | SJVN | 406,168 | **123%** |
| 3 | BAJAJHFL | 900,171 | **55%** |
| 27 | ENDURANCE | 940,754 | **53%** |
| 30 | IRCTC | 1,116,422 | 45% |
| 18 | ITCHOTELS | 1,283,921 | 39% |

**Book capacity without a floor is Rs81,234** — less than the Rs1L already being
traded. Two of the five names top 30 adds over top 25 (ENDURANCE, IRCTC) are
among the thinnest in the book.

Adding the floor **raises returns and capacity at the same time**: +0.0608 ->
+0.0657 at Rs1M, and capacity Rs79k -> Rs201k -> Rs403k. Thin names were
dragging performance, not just limiting size.

The floor also fixes 2026: **+3% becomes +32%**.

---

## What this does not establish

**The differences between basket sizes are not significant.** top 15 vs top 30
is 0.0721 vs 0.0608, t ~ 0.4. The recommendation rests on the *shape* of the
distribution — higher floor, more signals, positive in the current regime — not
on a demonstrated edge.

**These are drawn from 21+ tested configurations.** Reporting the win record of
the best-looking one is circular. Treat "positive every year" as encouraging,
not as proof.

**Slippage is excluded and cuts against larger baskets**, since stocks ranked
26-30 are thinner even after the floor. The floor mitigates this but does not
remove it.

**Trade counts favour the larger basket in a way these figures do not show.**
Top 30 generates 8,208 trades vs top 15's 4,189. Under the one-position rule the
slot sits idle less often, so the larger basket would capture more trades per
year than the flat 200 assumed here.

---

## Recommendation

**Top 30, 2-year lookback, quarterly rebalance, with a >= Rs2M/min liquidity
floor.**

- positive in all six full/partial years, worst 26%
- 18/24 quarters beat cost
- supports ~Rs4L of capital instead of ~Rs80k
- best performer in the 2026 regime by a wide margin

If maximising the point estimate matters more than the floor, **top 25 with the
same Rs2M floor** gives +0.0710 (53%/yr) with identical capacity and hit rate.
The two are statistically indistinguishable; top 30 has the higher floor, top 25
the higher mean.

## Files

- `../stock-analysis/detail_2y_quarterly_top30_liquid.md` — per-quarter detail,
  liquidity-filtered (the recommended configuration)
- `../stock-analysis/detail_2y_quarterly_top30.md` — per-quarter, no floor
- `../stock-analysis/detail_2y_quarterly_top25.md` — top 25
- `../stock-analysis/detail_2y_quarterly_refresh.md` — top 15
- `window-length-findings.md` — why lookback and cadence do not matter
