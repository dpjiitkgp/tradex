# stopS065 — exit-rule research

Exit rule: risk_pct=0.650% (hard stop), armed at 3.5x risk (2.250%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 25 instruments, 2018-11-09..2026-09-30.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_late_vwapslope_lb20 | 8442 | 0.033 | 0.051 | 42.6% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LICI | vwap_trend_short_late_vwapslope_lb20 | 202 | 0.135 | 0.208 | 60.9% |
| LENSKART | vwap_trend_short_late_vwapslope_lb20 | 39 | 0.286 | 0.439 | 59.0% |
| ITC | vwap_trend_short_late_vwapslope_lb20 | 289 | 0.088 | 0.135 | 52.9% |
| HAL | vwap_trend_short_late_vwapslope_lb20 | 404 | 0.134 | 0.207 | 52.0% |
| DABUR | vwap_trend_short_late_vwapslope_lb20 | 287 | 0.076 | 0.117 | 48.4% |
| VOLTAS | vwap_trend_short_late_vwapslope_lb20 | 302 | 0.092 | 0.142 | 47.4% |
| NAUKRI | vwap_trend_short_late_vwapslope_lb20 | 358 | 0.156 | 0.240 | 47.2% |
| PNB | vwap_trend_short_late_vwapslope_lb20 | 406 | 0.137 | 0.210 | 46.8% |
| TMPV | vwap_trend_short_late_vwapslope_lb20 | 351 | 0.117 | 0.180 | 46.4% |
| APARINDS | vwap_trend_short_late_vwapslope_lb20 | 417 | 0.126 | 0.194 | 46.3% |
| ASIANPAINT | vwap_trend_short_late_vwapslope_lb20 | 305 | -0.007 | -0.011 | 46.2% |
| DIXON | vwap_trend_short_late_vwapslope_lb20 | 368 | 0.142 | 0.219 | 46.2% |
| ATGL | vwap_trend_short_late_vwapslope_lb20 | 407 | 0.091 | 0.141 | 45.9% |
| WAAREEENER | vwap_trend_short_late_vwapslope_lb20 | 83 | 0.152 | 0.234 | 45.8% |
| LLOYDSME | vwap_trend_short_late_vwapslope_lb20 | 165 | 0.049 | 0.075 | 43.0% |
| CANBK | vwap_trend_short_late_vwapslope_lb20 | 338 | 0.074 | 0.115 | 42.6% |
| ICICIPRULI | vwap_trend_short_late_vwapslope_lb20 | 308 | -0.001 | -0.002 | 40.6% |
| TVSMOTOR | vwap_trend_short_late_vwapslope_lb20 | 320 | -0.030 | -0.046 | 40.0% |
| PRESTIGE | vwap_trend_short_late_vwapslope_lb20 | 403 | 0.063 | 0.096 | 39.5% |
| TIINDIA | vwap_trend_short_late_vwapslope_lb20 | 414 | -0.014 | -0.022 | 39.4% |
| YESBANK | vwap_trend_short_late_vwapslope_lb20 | 560 | -0.040 | -0.062 | 38.4% |
| GMRAIRPORT | vwap_trend_short_late_vwapslope_lb20 | 392 | -0.028 | -0.043 | 38.0% |
| LODHA | vwap_trend_short_late_vwapslope_lb20 | 293 | -0.018 | -0.027 | 36.2% |
| NMDC | vwap_trend_short_late_vwapslope_lb20 | 367 | -0.087 | -0.134 | 35.7% |
| IDEA | vwap_trend_short_late_vwapslope_lb20 | 664 | -0.245 | -0.376 | 24.7% |
