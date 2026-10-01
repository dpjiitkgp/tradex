# stopRn050 — exit-rule research

Exit rule: risk_pct=0.500% (hard stop), armed at 4.5x risk (2.250%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 25 instruments, 2018-11-09..2026-09-30.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| random_short_s1 | 9322 | 0.045 | 0.090 | 39.2% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LICI | random_short_s1 | 242 | 0.063 | 0.125 | 49.6% |
| ITC | random_short_s1 | 441 | 0.038 | 0.075 | 48.1% |
| HAL | random_short_s1 | 413 | 0.136 | 0.272 | 46.7% |
| DABUR | random_short_s1 | 437 | 0.039 | 0.078 | 45.5% |
| TMPV | random_short_s1 | 385 | 0.173 | 0.346 | 45.2% |
| WAAREEENER | random_short_s1 | 108 | 0.201 | 0.401 | 43.5% |
| ICICIPRULI | random_short_s1 | 411 | 0.090 | 0.180 | 40.6% |
| ASIANPAINT | random_short_s1 | 381 | -0.049 | -0.099 | 40.4% |
| TIINDIA | random_short_s1 | 399 | 0.050 | 0.100 | 40.4% |
| VOLTAS | random_short_s1 | 422 | 0.023 | 0.046 | 40.3% |
| YESBANK | random_short_s1 | 444 | 0.025 | 0.050 | 39.9% |
| TVSMOTOR | random_short_s1 | 437 | 0.039 | 0.078 | 39.4% |
| ATGL | random_short_s1 | 415 | 0.095 | 0.190 | 39.3% |
| NAUKRI | random_short_s1 | 393 | 0.056 | 0.111 | 38.7% |
| LLOYDSME | random_short_s1 | 184 | 0.091 | 0.183 | 38.0% |
| PRESTIGE | random_short_s1 | 442 | 0.070 | 0.139 | 36.2% |
| CANBK | random_short_s1 | 435 | 0.034 | 0.069 | 36.1% |
| LODHA | random_short_s1 | 286 | 0.075 | 0.149 | 36.0% |
| APARINDS | random_short_s1 | 420 | 0.035 | 0.070 | 36.0% |
| GMRAIRPORT | random_short_s1 | 456 | 0.011 | 0.022 | 34.6% |
| NMDC | random_short_s1 | 449 | -0.032 | -0.064 | 34.5% |
| PNB | random_short_s1 | 427 | -0.001 | -0.003 | 34.0% |
| IDEA | random_short_s1 | 441 | 0.010 | 0.021 | 33.8% |
| DIXON | random_short_s1 | 404 | 0.009 | 0.018 | 31.9% |
| LENSKART | random_short_s1 | 50 | -0.130 | -0.259 | 30.0% |
