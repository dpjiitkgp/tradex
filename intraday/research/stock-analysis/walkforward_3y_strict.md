# 3-year window walk-forward — detailed

Selection rule: **win rate > 50.0% AND Avg R > 0.2 in EVERY ONE of the 3 training years** (min 15 trades per year).

This is the strict form — a stock must have shown the edge in each year separately, so one exceptional year cannot carry two weak ones.

Each basket is then measured on the **following year only** — data the selection never saw.

Avg R is gross. Breakeven is **0.061 R** (~Rs206 per round trip against a 0.75% risk unit); `net` subtracts it.

All aggregates are trade-weighted, never a simple mean of averages.


## Train 2019-2021  →  test 2022

**2 stocks qualified.**

| stock | train trades | train Avg R | train win% | 2022 trades | 2022 Avg R | 2022 win% | held up? |
|---|---|---|---|---|---|---|---|
| RELIANCE | 118 | 0.342 | 59.3 | 46 | **-0.060** | 45.7 | NO |
| CIPLA | 129 | 0.226 | 55.8 | 45 | **0.034** | 48.9 | NO |

**Basket in 2022: 91 trades, Avg R -0.0140 (-0.23x breakeven), net -0.0750**  
Stocks that beat cost in 2022: **0/2**


## Train 2020-2022  →  test 2023

**1 stocks qualified.**

| stock | train trades | train Avg R | train win% | 2023 trades | 2023 Avg R | 2023 win% | held up? |
|---|---|---|---|---|---|---|---|
| HONAUT | 178 | 0.415 | 59.5 | 70 | **-0.036** | 50.0 | NO |

**Basket in 2023: 70 trades, Avg R -0.0357 (-0.59x breakeven), net -0.0967**  
Stocks that beat cost in 2023: **0/1**


## Train 2021-2023  →  test 2024

**1 stocks qualified.**

| stock | train trades | train Avg R | train win% | 2024 trades | 2024 Avg R | 2024 win% | held up? |
|---|---|---|---|---|---|---|---|
| OIL | 185 | 0.343 | 53.0 | 36 | **0.698** | 50.0 | yes |

**Basket in 2024: 36 trades, Avg R 0.6981 (11.44x breakeven), net +0.6371**  
Stocks that beat cost in 2024: **1/1**


## Train 2022-2024  →  test 2025

**2 stocks qualified.**

| stock | train trades | train Avg R | train win% | 2025 trades | 2025 Avg R | 2025 win% | held up? |
|---|---|---|---|---|---|---|---|
| DLF | 127 | 0.372 | 55.1 | 39 | **0.098** | 48.7 | yes |
| TMPV | 129 | 0.329 | 56.6 | 43 | **0.039** | 51.2 | NO |

**Basket in 2025: 82 trades, Avg R 0.0674 (1.11x breakeven), net +0.0064**  
Stocks that beat cost in 2025: **1/2**


## Train 2023-2025  →  test 2026

**3 stocks qualified.**

| stock | train trades | train Avg R | train win% | 2026 trades | 2026 Avg R | 2026 win% | held up? |
|---|---|---|---|---|---|---|---|
| NYKAA | 116 | 0.380 | 58.6 | 37 | **-0.307** | 29.7 | NO |
| HAL | 135 | 0.379 | 59.3 | 29 | **0.469** | 51.7 | yes |
| AWL | 142 | 0.340 | 64.1 | 39 | **-0.141** | 48.7 | NO |

**Basket in 2026: 105 trades, Avg R -0.0311 (-0.51x breakeven), net -0.0921**  
Stocks that beat cost in 2026: **1/3**


## Summary

| train | test | picks | test trades | test Avg R | xBE | net | stocks beating cost |
|---|---|---|---|---|---|---|---|
| 2019-2021 | 2022 | 2 | 91 | -0.0140 | -0.23x | -0.0750 | 0% |
| 2020-2022 | 2023 | 1 | 70 | -0.0357 | -0.59x | -0.0967 | 0% |
| 2021-2023 | 2024 | 1 | 36 | 0.6981 | 11.44x | +0.6371 | 100% |
| 2022-2024 | 2025 | 2 | 82 | 0.0674 | 1.11x | +0.0064 | 50% |
| 2023-2025 | 2026 | 3 | 105 | -0.0311 | -0.51x | -0.0921 | 33% |

**Pooled across every test year: 384 trades, Avg R 0.0615 (1.01x breakeven), net +0.0005 per trade.**

At ~200 live trades/year and 1 R = Rs3,740 (today's actual sizing: 238 MAZDOCK @ Rs2,096 = Rs4.99L notional), that net is **Rs384/year on Rs1L**.


Slippage is NOT included — the backtest fills at candle prices.

