# ctrl-s3 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 3.0x risk (2.250%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 25 instruments, 2018-11-09..2026-09-30.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| random_short_s3 | 8931 | 0.039 | 0.052 | 44.7% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LICI | random_short_s3 | 230 | 0.181 | 0.242 | 60.9% |
| WAAREEENER | random_short_s3 | 104 | 0.071 | 0.095 | 51.9% |
| TVSMOTOR | random_short_s3 | 366 | 0.111 | 0.148 | 51.4% |
| ASIANPAINT | random_short_s3 | 398 | 0.046 | 0.061 | 50.0% |
| ITC | random_short_s3 | 407 | 0.026 | 0.035 | 49.6% |
| HAL | random_short_s3 | 406 | 0.094 | 0.125 | 49.3% |
| PNB | random_short_s3 | 427 | 0.177 | 0.237 | 48.2% |
| TMPV | random_short_s3 | 430 | 0.104 | 0.139 | 48.1% |
| ATGL | random_short_s3 | 403 | 0.100 | 0.133 | 46.4% |
| LENSKART | random_short_s3 | 48 | 0.165 | 0.220 | 45.8% |
| DABUR | random_short_s3 | 399 | -0.008 | -0.010 | 45.4% |
| TIINDIA | random_short_s3 | 376 | 0.007 | 0.010 | 44.9% |
| APARINDS | random_short_s3 | 369 | 0.021 | 0.028 | 43.9% |
| CANBK | random_short_s3 | 436 | 0.089 | 0.119 | 43.8% |
| DIXON | random_short_s3 | 386 | 0.023 | 0.030 | 43.8% |
| GMRAIRPORT | random_short_s3 | 407 | 0.032 | 0.043 | 43.5% |
| YESBANK | random_short_s3 | 405 | 0.065 | 0.086 | 43.2% |
| LLOYDSME | random_short_s3 | 172 | 0.104 | 0.139 | 42.4% |
| NMDC | random_short_s3 | 410 | 0.053 | 0.070 | 42.2% |
| PRESTIGE | random_short_s3 | 388 | 0.055 | 0.073 | 42.0% |
| ICICIPRULI | random_short_s3 | 406 | -0.015 | -0.020 | 40.4% |
| VOLTAS | random_short_s3 | 464 | -0.080 | -0.107 | 40.3% |
| LODHA | random_short_s3 | 267 | -0.057 | -0.077 | 38.6% |
| NAUKRI | random_short_s3 | 412 | -0.068 | -0.091 | 37.9% |
| IDEA | random_short_s3 | 415 | -0.102 | -0.135 | 34.5% |
