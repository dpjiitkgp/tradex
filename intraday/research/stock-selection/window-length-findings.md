# Selection window length and refresh frequency — findings

**Date:** 2026-09-29
**Data:** run 110 (`tune30/mrm-3.0-trail-0.0075-vwap_trend_short_late_vwapslope_lb20`),
250-stock universe, PIIND and indices excluded.
**Question:** how far back should stock selection look, and how often should it rebalance?

---

## Conclusion

**Neither lookback length nor refresh frequency measurably matters.** Across
14 tested variants the differences are within noise (best-vs-worst t = 1.50).

What does hold up:

- **The selection method works.** Every one of the 14 variants is net positive.
- **Magnitude is ~+0.05 R net per trade**, with an honest error bar of about
  ±0.02 R — roughly **Rs37,000/year on Rs1L**, but credibly anywhere between
  Rs15,000 and Rs60,000.
- **Pick whichever cadence is operationally simplest.** Monthly rebalancing
  means 12 config changes and engine restarts a year for no demonstrable gain,
  so annual or 6-monthly is preferable.

All figures are **gross**; breakeven is **0.061 R** (~Rs206 per round trip
against a 0.75% risk unit). `net` subtracts it. Slippage is NOT included.

---

## The data

### Refresh every 6 months

| lookback | tests | trades | pooled Avg R | xBE | net | beat cost |
|---|---|---|---|---|---|---|
| 6 months | 15 | 5,027 | 0.0825 | 1.35x | +0.0215 | 9/15 |
| 1 year | 15 | 5,181 | 0.1117 | 1.83x | +0.0507 | 12/15 |
| 18 months | 14 | 4,720 | 0.0902 | 1.48x | +0.0292 | 7/14 |
| 2 years | 13 | 4,412 | 0.1124 | 1.84x | +0.0514 | 7/13 |
| 3 years | 11 | 3,680 | 0.1044 | 1.71x | +0.0434 | 7/11 |
| 4 years | 9 | 2,856 | 0.0864 | 1.42x | +0.0254 | 6/9 |
| 5 years | 7 | 2,185 | 0.0997 | 1.63x | +0.0387 | 4/7 |

### Refresh every month

| lookback | tests | trades | pooled Avg R | xBE | net | beat cost | median n/stock |
|---|---|---|---|---|---|---|---|
| 3 months | 91 | 5,196 | 0.1260 | 2.06x | +0.0650 | 51/91 | 17 |
| 6 months | 88 | 5,173 | 0.0987 | 1.62x | +0.0377 | 42/88 | 23 |
| 9 months | 85 | 4,962 | 0.1061 | 1.74x | +0.0451 | 54/85 | 33 |
| 12 months | 82 | 4,865 | 0.1035 | 1.70x | +0.0425 | 53/82 | 43 |
| 18 months | 76 | 4,460 | 0.1253 | 2.05x | +0.0643 | 44/76 | 65 |
| 24 months | 70 | 4,062 | 0.1428 | 2.34x | +0.0818 | 45/70 | 86 |
| 36 months | 58 | 3,330 | 0.1222 | 2.00x | +0.0612 | 37/58 | 129 |

### Significance

| lookback | Avg R | 95% CI |
|---|---|---|
| 3mo | 0.1260 | [+0.0879, +0.1641] |
| 6mo | 0.0987 | [+0.0605, +0.1369] |
| 9mo | 0.1061 | [+0.0671, +0.1451] |
| 12mo | 0.1035 | [+0.0642, +0.1428] |
| 18mo | 0.1253 | [+0.0842, +0.1664] |
| 24mo | 0.1428 | [+0.0997, +0.1859] |
| 36mo | 0.1222 | [+0.0746, +0.1698] |

Best minus worst = +0.0441, SE = 0.0293, **t = 1.50 — not significant**.
Every interval overlaps every other one.

---

## A wrong conclusion I drew, and why

After the 6-monthly sweep I reported that **"1 year is the sweet spot"**, with a
mechanism: below a year there are too few trades per stock to rank on (~22,
SE +/-0.30 against a 0.24 R spread), and beyond two years the information goes
stale.

The monthly sweep refutes it:

- Under monthly refresh **24 months wins**, not 12.
- **3 months came second** (+0.0650) on ~17 trades per stock — the case my
  mechanism said could not work at all.
- The "optimum" moves with refresh frequency, which a real effect would not.

The numbers were right; the mechanism was invented to fit them. A plausible
causal story made noise far more convincing than a bare table would have been.
**Where a ranking is not statistically distinguishable, do not explain its
order.**

---

## Selection rule comparison (3-year window, annual)

| rule | picks | pooled Avg R | net | Rs/yr on Rs1L |
|---|---|---|---|---|
| top 15 by Avg R | 15 | 0.1003 | +0.0393 | ~29,000 |
| threshold win>48 & R>0.2 | 26-37 | 0.0745 | +0.0135 | ~10,100 |
| strict: win>50 & R>0.2 **every year** | 1-3 | 0.0615 | +0.0005 | ~400 |

**Ranking beats thresholding, and tightening thresholds makes it worse.** The
strict rule collapses to 1-3 stocks — too few to fill the position slot, and
selected for three consecutive lucky years, which mean-revert hardest.

Unlike window length, this gap *is* large enough to act on: top-15 versus the
strict rule is +0.0393 against +0.0005.

---

## Verification

- Spot-checked against raw `trades`: HAL 2026 n=29 / 0.4690, DIXON 37 / 0.6011,
  NYKAA 37 / -0.3074 — all exact.
- The 3-year window report was rebuilt end-to-end from the `trades` table with
  no intermediate file: 6,511 trades, Avg R 0.0745, net +0.0135 — exact match.
- Aggregation arithmetic re-derived from the published tables for every window.

## Reproduce

```bash
python3 research/stock-analysis/window_length_sweep.py --verbose
python3 research/stock-analysis/halfyear_sweep.py \
    --data research/long-baseline/analysis/short_monthly_110.tsv \
    --lookbacks 3,6,9,12,18,24,36 --unit month --min-trades 15
python3 research/stock-analysis/detail_report.py --lookback 2   # per-period detail
```

Per-period detail reports: `detail_lookback_2hy.md` (1-year lookback),
`detail_lookback_4hy.md` (2-year), `walkforward_3y_detail.md`,
`walkforward_3y_strict.md`.
