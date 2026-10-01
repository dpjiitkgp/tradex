# stopRn065 — exit-rule research

Exit rule: risk_pct=0.650% (hard stop), armed at 3.5x risk (2.250%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 25 instruments, 2018-11-09..2026-09-30.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| random_short_s1 | 9276 | 0.042 | 0.065 | 43.3% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LICI | random_short_s1 | 242 | 0.054 | 0.084 | 52.5% |
| ITC | random_short_s1 | 441 | 0.044 | 0.068 | 51.5% |
| HAL | random_short_s1 | 412 | 0.121 | 0.186 | 50.0% |
| TMPV | random_short_s1 | 383 | 0.176 | 0.270 | 49.1% |
| DABUR | random_short_s1 | 436 | 0.040 | 0.062 | 48.6% |
| WAAREEENER | random_short_s1 | 108 | 0.230 | 0.354 | 48.1% |
| TIINDIA | random_short_s1 | 398 | 0.025 | 0.038 | 44.5% |
| ASIANPAINT | random_short_s1 | 379 | -0.048 | -0.074 | 44.3% |
| VOLTAS | random_short_s1 | 420 | 0.008 | 0.012 | 44.0% |
| TVSMOTOR | random_short_s1 | 435 | 0.025 | 0.038 | 43.9% |
| ICICIPRULI | random_short_s1 | 407 | 0.062 | 0.096 | 43.7% |
| ATGL | random_short_s1 | 413 | 0.108 | 0.166 | 43.6% |
| NAUKRI | random_short_s1 | 390 | 0.039 | 0.059 | 42.8% |
| YESBANK | random_short_s1 | 441 | 0.021 | 0.032 | 42.4% |
| APARINDS | random_short_s1 | 419 | 0.062 | 0.095 | 42.2% |
| LLOYDSME | random_short_s1 | 181 | 0.068 | 0.104 | 42.0% |
| PRESTIGE | random_short_s1 | 438 | 0.072 | 0.111 | 41.8% |
| CANBK | random_short_s1 | 434 | 0.056 | 0.086 | 41.2% |
| LODHA | random_short_s1 | 286 | 0.075 | 0.116 | 40.2% |
| GMRAIRPORT | random_short_s1 | 452 | 0.028 | 0.044 | 39.8% |
| IDEA | random_short_s1 | 434 | 0.020 | 0.031 | 38.9% |
| PNB | random_short_s1 | 425 | 0.005 | 0.008 | 38.1% |
| NMDC | random_short_s1 | 448 | -0.037 | -0.058 | 37.9% |
| DIXON | random_short_s1 | 404 | -0.033 | -0.050 | 35.1% |
| LENSKART | random_short_s1 | 50 | -0.212 | -0.326 | 30.0% |
