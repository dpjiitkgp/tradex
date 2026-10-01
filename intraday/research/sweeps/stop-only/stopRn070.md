# stopRn070 — exit-rule research

Exit rule: risk_pct=0.700% (hard stop), armed at 3.2x risk (2.250%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 25 instruments, 2018-11-09..2026-09-30.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| random_short_s1 | 9263 | 0.039 | 0.056 | 44.2% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LICI | random_short_s1 | 242 | 0.053 | 0.076 | 53.7% |
| ITC | random_short_s1 | 441 | 0.041 | 0.058 | 51.9% |
| WAAREEENER | random_short_s1 | 108 | 0.244 | 0.348 | 50.9% |
| HAL | random_short_s1 | 412 | 0.120 | 0.171 | 50.7% |
| TMPV | random_short_s1 | 382 | 0.158 | 0.225 | 49.7% |
| DABUR | random_short_s1 | 435 | 0.031 | 0.045 | 48.7% |
| ASIANPAINT | random_short_s1 | 378 | -0.043 | -0.062 | 45.8% |
| TVSMOTOR | random_short_s1 | 434 | 0.024 | 0.035 | 45.4% |
| VOLTAS | random_short_s1 | 420 | 0.010 | 0.014 | 45.2% |
| ICICIPRULI | random_short_s1 | 406 | 0.068 | 0.097 | 45.1% |
| TIINDIA | random_short_s1 | 398 | 0.003 | 0.004 | 45.0% |
| ATGL | random_short_s1 | 413 | 0.116 | 0.166 | 44.6% |
| NAUKRI | random_short_s1 | 389 | 0.042 | 0.060 | 44.2% |
| APARINDS | random_short_s1 | 419 | 0.064 | 0.092 | 43.0% |
| YESBANK | random_short_s1 | 440 | 0.014 | 0.020 | 43.0% |
| LLOYDSME | random_short_s1 | 181 | 0.060 | 0.086 | 42.5% |
| PRESTIGE | random_short_s1 | 435 | 0.056 | 0.080 | 42.3% |
| CANBK | random_short_s1 | 432 | 0.049 | 0.070 | 42.1% |
| LODHA | random_short_s1 | 286 | 0.061 | 0.086 | 40.9% |
| GMRAIRPORT | random_short_s1 | 451 | 0.022 | 0.032 | 40.8% |
| IDEA | random_short_s1 | 434 | 0.030 | 0.043 | 40.3% |
| PNB | random_short_s1 | 425 | 0.015 | 0.022 | 39.3% |
| NMDC | random_short_s1 | 448 | -0.045 | -0.065 | 39.1% |
| DIXON | random_short_s1 | 404 | -0.030 | -0.043 | 36.6% |
| LENSKART | random_short_s1 | 50 | -0.250 | -0.357 | 30.0% |
