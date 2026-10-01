# band-ceil010 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 3.0x risk (2.250%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 25 instruments, 2018-11-09..2026-09-30.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwapslope_lb20_ceil010 | 7664 | 0.054 | 0.072 | 45.8% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LICI | vwapslope_lb20_ceil010 | 198 | 0.117 | 0.156 | 61.1% |
| LENSKART | vwapslope_lb20_ceil010 | 37 | 0.233 | 0.310 | 59.5% |
| ITC | vwapslope_lb20_ceil010 | 279 | 0.108 | 0.143 | 56.3% |
| HAL | vwapslope_lb20_ceil010 | 381 | 0.167 | 0.222 | 55.1% |
| NAUKRI | vwapslope_lb20_ceil010 | 322 | 0.189 | 0.252 | 50.3% |
| APARINDS | vwapslope_lb20_ceil010 | 366 | 0.147 | 0.195 | 50.3% |
| TMPV | vwapslope_lb20_ceil010 | 332 | 0.147 | 0.196 | 50.0% |
| VOLTAS | vwapslope_lb20_ceil010 | 284 | 0.082 | 0.109 | 49.6% |
| PNB | vwapslope_lb20_ceil010 | 380 | 0.147 | 0.196 | 49.2% |
| ATGL | vwapslope_lb20_ceil010 | 382 | 0.102 | 0.136 | 49.0% |
| DABUR | vwapslope_lb20_ceil010 | 278 | 0.071 | 0.094 | 48.6% |
| DIXON | vwapslope_lb20_ceil010 | 334 | 0.152 | 0.202 | 47.9% |
| CANBK | vwapslope_lb20_ceil010 | 301 | 0.122 | 0.163 | 47.2% |
| WAAREEENER | vwapslope_lb20_ceil010 | 79 | 0.051 | 0.068 | 46.8% |
| ASIANPAINT | vwapslope_lb20_ceil010 | 297 | -0.018 | -0.024 | 46.5% |
| LLOYDSME | vwapslope_lb20_ceil010 | 142 | 0.069 | 0.092 | 45.8% |
| TIINDIA | vwapslope_lb20_ceil010 | 360 | 0.037 | 0.049 | 43.9% |
| PRESTIGE | vwapslope_lb20_ceil010 | 354 | 0.089 | 0.118 | 43.2% |
| TVSMOTOR | vwapslope_lb20_ceil010 | 293 | -0.033 | -0.045 | 42.3% |
| ICICIPRULI | vwapslope_lb20_ceil010 | 280 | -0.001 | -0.001 | 42.1% |
| YESBANK | vwapslope_lb20_ceil010 | 524 | -0.010 | -0.014 | 41.8% |
| LODHA | vwapslope_lb20_ceil010 | 246 | 0.012 | 0.015 | 40.7% |
| GMRAIRPORT | vwapslope_lb20_ceil010 | 342 | -0.020 | -0.027 | 40.1% |
| NMDC | vwapslope_lb20_ceil010 | 332 | -0.084 | -0.112 | 38.3% |
| IDEA | vwapslope_lb20_ceil010 | 541 | -0.195 | -0.260 | 29.2% |
