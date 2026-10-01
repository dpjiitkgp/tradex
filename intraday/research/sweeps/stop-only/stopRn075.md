# stopRn075 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 3.0x risk (2.250%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 25 instruments, 2018-11-09..2026-09-30.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| random_short_s1 | 9251 | 0.038 | 0.051 | 45.1% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LICI | random_short_s1 | 242 | 0.065 | 0.087 | 54.5% |
| ITC | random_short_s1 | 441 | 0.035 | 0.047 | 52.2% |
| TMPV | random_short_s1 | 382 | 0.173 | 0.231 | 51.3% |
| HAL | random_short_s1 | 412 | 0.121 | 0.161 | 51.2% |
| WAAREEENER | random_short_s1 | 108 | 0.217 | 0.289 | 50.9% |
| DABUR | random_short_s1 | 435 | 0.039 | 0.052 | 49.7% |
| TVSMOTOR | random_short_s1 | 433 | 0.018 | 0.025 | 46.4% |
| ASIANPAINT | random_short_s1 | 378 | -0.043 | -0.057 | 46.3% |
| TIINDIA | random_short_s1 | 398 | 0.001 | 0.002 | 46.0% |
| ATGL | random_short_s1 | 412 | 0.128 | 0.170 | 45.9% |
| VOLTAS | random_short_s1 | 420 | 0.004 | 0.005 | 45.5% |
| ICICIPRULI | random_short_s1 | 403 | 0.059 | 0.079 | 45.4% |
| NAUKRI | random_short_s1 | 389 | 0.046 | 0.062 | 45.2% |
| YESBANK | random_short_s1 | 440 | 0.026 | 0.035 | 44.3% |
| CANBK | random_short_s1 | 430 | 0.065 | 0.087 | 43.7% |
| APARINDS | random_short_s1 | 419 | 0.042 | 0.056 | 43.7% |
| LLOYDSME | random_short_s1 | 181 | 0.039 | 0.052 | 43.1% |
| PRESTIGE | random_short_s1 | 434 | 0.055 | 0.074 | 42.9% |
| LODHA | random_short_s1 | 284 | 0.051 | 0.068 | 41.9% |
| IDEA | random_short_s1 | 434 | 0.016 | 0.021 | 41.5% |
| GMRAIRPORT | random_short_s1 | 451 | 0.014 | 0.018 | 41.2% |
| PNB | random_short_s1 | 424 | 0.024 | 0.032 | 40.8% |
| NMDC | random_short_s1 | 448 | -0.049 | -0.065 | 39.7% |
| DIXON | random_short_s1 | 403 | -0.031 | -0.041 | 37.5% |
| LENSKART | random_short_s1 | 50 | -0.277 | -0.370 | 30.0% |
