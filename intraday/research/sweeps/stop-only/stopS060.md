# stopS060 — exit-rule research

Exit rule: risk_pct=0.600% (hard stop), armed at 3.8x risk (2.250%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 25 instruments, 2018-11-09..2026-09-30.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_late_vwapslope_lb20 | 8472 | 0.029 | 0.048 | 41.2% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LICI | vwap_trend_short_late_vwapslope_lb20 | 202 | 0.134 | 0.224 | 59.9% |
| LENSKART | vwap_trend_short_late_vwapslope_lb20 | 39 | 0.299 | 0.498 | 59.0% |
| ITC | vwap_trend_short_late_vwapslope_lb20 | 289 | 0.091 | 0.152 | 52.6% |
| HAL | vwap_trend_short_late_vwapslope_lb20 | 406 | 0.119 | 0.198 | 50.5% |
| DABUR | vwap_trend_short_late_vwapslope_lb20 | 287 | 0.076 | 0.127 | 48.1% |
| VOLTAS | vwap_trend_short_late_vwapslope_lb20 | 302 | 0.103 | 0.172 | 46.7% |
| NAUKRI | vwap_trend_short_late_vwapslope_lb20 | 358 | 0.160 | 0.267 | 46.4% |
| PNB | vwap_trend_short_late_vwapslope_lb20 | 406 | 0.140 | 0.233 | 46.1% |
| WAAREEENER | vwap_trend_short_late_vwapslope_lb20 | 83 | 0.167 | 0.279 | 45.8% |
| ASIANPAINT | vwap_trend_short_late_vwapslope_lb20 | 305 | -0.005 | -0.008 | 45.6% |
| DIXON | vwap_trend_short_late_vwapslope_lb20 | 367 | 0.145 | 0.241 | 45.0% |
| ATGL | vwap_trend_short_late_vwapslope_lb20 | 407 | 0.093 | 0.154 | 44.7% |
| APARINDS | vwap_trend_short_late_vwapslope_lb20 | 419 | 0.123 | 0.205 | 44.6% |
| TMPV | vwap_trend_short_late_vwapslope_lb20 | 351 | 0.116 | 0.194 | 44.4% |
| LLOYDSME | vwap_trend_short_late_vwapslope_lb20 | 165 | 0.041 | 0.069 | 41.8% |
| CANBK | vwap_trend_short_late_vwapslope_lb20 | 339 | 0.083 | 0.139 | 41.6% |
| ICICIPRULI | vwap_trend_short_late_vwapslope_lb20 | 309 | 0.005 | 0.009 | 39.8% |
| TIINDIA | vwap_trend_short_late_vwapslope_lb20 | 417 | -0.011 | -0.018 | 38.4% |
| PRESTIGE | vwap_trend_short_late_vwapslope_lb20 | 404 | 0.065 | 0.109 | 38.1% |
| TVSMOTOR | vwap_trend_short_late_vwapslope_lb20 | 320 | -0.035 | -0.059 | 37.8% |
| YESBANK | vwap_trend_short_late_vwapslope_lb20 | 561 | -0.063 | -0.105 | 36.7% |
| GMRAIRPORT | vwap_trend_short_late_vwapslope_lb20 | 393 | -0.059 | -0.098 | 34.6% |
| NMDC | vwap_trend_short_late_vwapslope_lb20 | 367 | -0.110 | -0.183 | 33.8% |
| LODHA | vwap_trend_short_late_vwapslope_lb20 | 296 | -0.039 | -0.064 | 33.8% |
| IDEA | vwap_trend_short_late_vwapslope_lb20 | 680 | -0.252 | -0.420 | 22.9% |
