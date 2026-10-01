# stopRn060 — exit-rule research

Exit rule: risk_pct=0.600% (hard stop), armed at 3.8x risk (2.250%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 25 instruments, 2018-11-09..2026-09-30.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| random_short_s1 | 9285 | 0.043 | 0.072 | 42.1% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LICI | random_short_s1 | 242 | 0.053 | 0.089 | 51.7% |
| ITC | random_short_s1 | 441 | 0.046 | 0.077 | 50.6% |
| HAL | random_short_s1 | 412 | 0.128 | 0.214 | 48.5% |
| DABUR | random_short_s1 | 436 | 0.050 | 0.083 | 48.2% |
| TMPV | random_short_s1 | 384 | 0.166 | 0.276 | 47.7% |
| WAAREEENER | random_short_s1 | 108 | 0.202 | 0.336 | 46.3% |
| ASIANPAINT | random_short_s1 | 379 | -0.041 | -0.068 | 43.3% |
| VOLTAS | random_short_s1 | 420 | 0.013 | 0.022 | 42.9% |
| TVSMOTOR | random_short_s1 | 435 | 0.032 | 0.053 | 42.5% |
| ICICIPRULI | random_short_s1 | 408 | 0.069 | 0.114 | 42.4% |
| ATGL | random_short_s1 | 413 | 0.098 | 0.164 | 42.4% |
| TIINDIA | random_short_s1 | 398 | 0.008 | 0.013 | 42.2% |
| NAUKRI | random_short_s1 | 391 | 0.052 | 0.087 | 42.2% |
| YESBANK | random_short_s1 | 443 | 0.018 | 0.029 | 41.5% |
| LLOYDSME | random_short_s1 | 181 | 0.074 | 0.124 | 40.9% |
| PRESTIGE | random_short_s1 | 438 | 0.086 | 0.144 | 40.6% |
| APARINDS | random_short_s1 | 419 | 0.046 | 0.077 | 40.3% |
| CANBK | random_short_s1 | 434 | 0.052 | 0.087 | 40.1% |
| LODHA | random_short_s1 | 286 | 0.086 | 0.144 | 39.9% |
| IDEA | random_short_s1 | 435 | 0.022 | 0.037 | 37.5% |
| NMDC | random_short_s1 | 449 | -0.034 | -0.057 | 37.4% |
| GMRAIRPORT | random_short_s1 | 453 | 0.010 | 0.017 | 37.3% |
| PNB | random_short_s1 | 426 | 0.008 | 0.013 | 37.1% |
| DIXON | random_short_s1 | 404 | -0.012 | -0.021 | 34.4% |
| LENSKART | random_short_s1 | 50 | -0.180 | -0.300 | 30.0% |
