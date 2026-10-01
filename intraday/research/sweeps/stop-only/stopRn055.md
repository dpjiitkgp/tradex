# stopRn055 — exit-rule research

Exit rule: risk_pct=0.550% (hard stop), armed at 4.1x risk (2.250%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 25 instruments, 2018-11-09..2026-09-30.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| random_short_s1 | 9304 | 0.044 | 0.080 | 40.6% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LICI | random_short_s1 | 242 | 0.055 | 0.101 | 50.8% |
| ITC | random_short_s1 | 441 | 0.039 | 0.072 | 49.9% |
| DABUR | random_short_s1 | 437 | 0.049 | 0.090 | 47.6% |
| HAL | random_short_s1 | 413 | 0.122 | 0.222 | 47.2% |
| TMPV | random_short_s1 | 385 | 0.171 | 0.311 | 46.8% |
| WAAREEENER | random_short_s1 | 108 | 0.186 | 0.338 | 43.5% |
| TVSMOTOR | random_short_s1 | 435 | 0.043 | 0.079 | 42.3% |
| ICICIPRULI | random_short_s1 | 410 | 0.091 | 0.165 | 42.2% |
| ASIANPAINT | random_short_s1 | 379 | -0.041 | -0.075 | 42.0% |
| ATGL | random_short_s1 | 413 | 0.102 | 0.185 | 41.2% |
| TIINDIA | random_short_s1 | 399 | 0.029 | 0.052 | 41.1% |
| VOLTAS | random_short_s1 | 420 | 0.014 | 0.025 | 41.0% |
| NAUKRI | random_short_s1 | 393 | 0.057 | 0.104 | 40.5% |
| YESBANK | random_short_s1 | 443 | 0.016 | 0.030 | 40.4% |
| LLOYDSME | random_short_s1 | 183 | 0.088 | 0.159 | 39.9% |
| PRESTIGE | random_short_s1 | 441 | 0.078 | 0.142 | 38.5% |
| LODHA | random_short_s1 | 286 | 0.084 | 0.152 | 37.8% |
| APARINDS | random_short_s1 | 420 | 0.041 | 0.075 | 37.6% |
| CANBK | random_short_s1 | 434 | 0.042 | 0.076 | 37.6% |
| NMDC | random_short_s1 | 449 | -0.028 | -0.052 | 36.1% |
| IDEA | random_short_s1 | 439 | 0.027 | 0.049 | 35.8% |
| GMRAIRPORT | random_short_s1 | 453 | 0.003 | 0.005 | 35.1% |
| PNB | random_short_s1 | 427 | -0.014 | -0.026 | 34.7% |
| DIXON | random_short_s1 | 404 | -0.006 | -0.011 | 33.2% |
| LENSKART | random_short_s1 | 50 | -0.166 | -0.303 | 30.0% |
