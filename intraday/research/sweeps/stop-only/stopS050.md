# stopS050 — exit-rule research

Exit rule: risk_pct=0.500% (hard stop), armed at 4.5x risk (2.250%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 25 instruments, 2018-11-09..2026-09-30.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_late_vwapslope_lb20 | 8593 | 0.011 | 0.022 | 37.3% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LICI | vwap_trend_short_late_vwapslope_lb20 | 202 | 0.131 | 0.262 | 58.4% |
| LENSKART | vwap_trend_short_late_vwapslope_lb20 | 39 | 0.233 | 0.465 | 53.8% |
| ITC | vwap_trend_short_late_vwapslope_lb20 | 290 | 0.102 | 0.204 | 51.4% |
| DABUR | vwap_trend_short_late_vwapslope_lb20 | 287 | 0.087 | 0.174 | 46.7% |
| HAL | vwap_trend_short_late_vwapslope_lb20 | 407 | 0.093 | 0.186 | 46.4% |
| ASIANPAINT | vwap_trend_short_late_vwapslope_lb20 | 305 | -0.003 | -0.006 | 43.3% |
| NAUKRI | vwap_trend_short_late_vwapslope_lb20 | 357 | 0.143 | 0.285 | 42.6% |
| VOLTAS | vwap_trend_short_late_vwapslope_lb20 | 303 | 0.079 | 0.159 | 42.6% |
| DIXON | vwap_trend_short_late_vwapslope_lb20 | 369 | 0.142 | 0.284 | 41.7% |
| ATGL | vwap_trend_short_late_vwapslope_lb20 | 410 | 0.077 | 0.155 | 41.2% |
| WAAREEENER | vwap_trend_short_late_vwapslope_lb20 | 85 | 0.107 | 0.214 | 41.2% |
| PNB | vwap_trend_short_late_vwapslope_lb20 | 407 | 0.108 | 0.216 | 40.3% |
| TMPV | vwap_trend_short_late_vwapslope_lb20 | 352 | 0.098 | 0.197 | 40.1% |
| APARINDS | vwap_trend_short_late_vwapslope_lb20 | 424 | 0.081 | 0.163 | 39.6% |
| CANBK | vwap_trend_short_late_vwapslope_lb20 | 339 | 0.093 | 0.187 | 38.6% |
| LLOYDSME | vwap_trend_short_late_vwapslope_lb20 | 166 | 0.049 | 0.097 | 38.6% |
| TIINDIA | vwap_trend_short_late_vwapslope_lb20 | 416 | -0.005 | -0.010 | 35.3% |
| ICICIPRULI | vwap_trend_short_late_vwapslope_lb20 | 310 | 0.004 | 0.007 | 35.2% |
| TVSMOTOR | vwap_trend_short_late_vwapslope_lb20 | 323 | -0.049 | -0.098 | 34.4% |
| YESBANK | vwap_trend_short_late_vwapslope_lb20 | 565 | -0.085 | -0.171 | 33.8% |
| PRESTIGE | vwap_trend_short_late_vwapslope_lb20 | 407 | 0.044 | 0.087 | 33.7% |
| NMDC | vwap_trend_short_late_vwapslope_lb20 | 372 | -0.098 | -0.197 | 31.7% |
| GMRAIRPORT | vwap_trend_short_late_vwapslope_lb20 | 396 | -0.084 | -0.169 | 31.3% |
| LODHA | vwap_trend_short_late_vwapslope_lb20 | 298 | -0.029 | -0.058 | 30.5% |
| IDEA | vwap_trend_short_late_vwapslope_lb20 | 764 | -0.298 | -0.595 | 16.5% |
