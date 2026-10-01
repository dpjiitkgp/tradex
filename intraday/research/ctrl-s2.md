# ctrl-s2 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 3.0x risk (2.250%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 25 instruments, 2018-11-09..2026-09-30.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| random_short_s2 | 9039 | 0.043 | 0.057 | 45.1% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LICI | random_short_s2 | 246 | 0.149 | 0.199 | 56.9% |
| WAAREEENER | random_short_s2 | 95 | 0.132 | 0.177 | 53.7% |
| ITC | random_short_s2 | 393 | 0.080 | 0.107 | 52.7% |
| TMPV | random_short_s2 | 405 | 0.157 | 0.209 | 50.4% |
| ASIANPAINT | random_short_s2 | 391 | -0.020 | -0.026 | 47.6% |
| HAL | random_short_s2 | 422 | 0.037 | 0.049 | 47.2% |
| LLOYDSME | random_short_s2 | 168 | 0.138 | 0.184 | 47.0% |
| PRESTIGE | random_short_s2 | 381 | 0.153 | 0.204 | 47.0% |
| LENSKART | random_short_s2 | 47 | 0.074 | 0.099 | 46.8% |
| DABUR | random_short_s2 | 409 | -0.004 | -0.005 | 46.0% |
| ATGL | random_short_s2 | 415 | 0.090 | 0.120 | 45.8% |
| NAUKRI | random_short_s2 | 413 | 0.065 | 0.086 | 45.8% |
| PNB | random_short_s2 | 447 | 0.058 | 0.077 | 45.6% |
| LODHA | random_short_s2 | 265 | 0.075 | 0.100 | 45.3% |
| DIXON | random_short_s2 | 399 | 0.087 | 0.116 | 45.1% |
| ICICIPRULI | random_short_s2 | 437 | 0.052 | 0.069 | 45.1% |
| TVSMOTOR | random_short_s2 | 418 | 0.061 | 0.082 | 43.5% |
| NMDC | random_short_s2 | 410 | 0.033 | 0.044 | 43.4% |
| VOLTAS | random_short_s2 | 402 | -0.025 | -0.033 | 43.0% |
| TIINDIA | random_short_s2 | 418 | -0.010 | -0.013 | 42.8% |
| CANBK | random_short_s2 | 400 | 0.040 | 0.053 | 42.8% |
| YESBANK | random_short_s2 | 415 | 0.035 | 0.046 | 42.2% |
| APARINDS | random_short_s2 | 382 | -0.050 | -0.067 | 40.6% |
| GMRAIRPORT | random_short_s2 | 417 | -0.056 | -0.074 | 40.0% |
| IDEA | random_short_s2 | 444 | -0.058 | -0.077 | 35.6% |
