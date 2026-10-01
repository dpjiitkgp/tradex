# stopS070 — exit-rule research

Exit rule: risk_pct=0.700% (hard stop), armed at 3.2x risk (2.250%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 25 instruments, 2018-11-09..2026-09-30.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_late_vwapslope_lb20 | 8418 | 0.040 | 0.057 | 43.9% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LENSKART | vwap_trend_short_late_vwapslope_lb20 | 39 | 0.329 | 0.470 | 61.5% |
| LICI | vwap_trend_short_late_vwapslope_lb20 | 202 | 0.127 | 0.182 | 60.9% |
| ITC | vwap_trend_short_late_vwapslope_lb20 | 289 | 0.097 | 0.138 | 54.3% |
| HAL | vwap_trend_short_late_vwapslope_lb20 | 404 | 0.136 | 0.195 | 53.0% |
| NAUKRI | vwap_trend_short_late_vwapslope_lb20 | 357 | 0.188 | 0.269 | 49.3% |
| DABUR | vwap_trend_short_late_vwapslope_lb20 | 287 | 0.070 | 0.099 | 48.4% |
| VOLTAS | vwap_trend_short_late_vwapslope_lb20 | 301 | 0.086 | 0.122 | 48.2% |
| DIXON | vwap_trend_short_late_vwapslope_lb20 | 368 | 0.163 | 0.232 | 48.1% |
| TMPV | vwap_trend_short_late_vwapslope_lb20 | 350 | 0.123 | 0.176 | 48.0% |
| PNB | vwap_trend_short_late_vwapslope_lb20 | 405 | 0.135 | 0.193 | 47.9% |
| APARINDS | vwap_trend_short_late_vwapslope_lb20 | 415 | 0.128 | 0.183 | 47.7% |
| ASIANPAINT | vwap_trend_short_late_vwapslope_lb20 | 306 | -0.003 | -0.004 | 47.1% |
| ATGL | vwap_trend_short_late_vwapslope_lb20 | 409 | 0.093 | 0.132 | 46.9% |
| WAAREEENER | vwap_trend_short_late_vwapslope_lb20 | 83 | 0.139 | 0.199 | 45.8% |
| CANBK | vwap_trend_short_late_vwapslope_lb20 | 338 | 0.095 | 0.136 | 45.0% |
| LLOYDSME | vwap_trend_short_late_vwapslope_lb20 | 165 | 0.065 | 0.093 | 44.8% |
| ICICIPRULI | vwap_trend_short_late_vwapslope_lb20 | 306 | -0.006 | -0.008 | 41.5% |
| PRESTIGE | vwap_trend_short_late_vwapslope_lb20 | 403 | 0.077 | 0.111 | 41.4% |
| TVSMOTOR | vwap_trend_short_late_vwapslope_lb20 | 318 | -0.026 | -0.036 | 41.2% |
| TIINDIA | vwap_trend_short_late_vwapslope_lb20 | 414 | -0.013 | -0.018 | 40.6% |
| YESBANK | vwap_trend_short_late_vwapslope_lb20 | 557 | -0.036 | -0.051 | 39.3% |
| GMRAIRPORT | vwap_trend_short_late_vwapslope_lb20 | 389 | -0.032 | -0.046 | 38.8% |
| LODHA | vwap_trend_short_late_vwapslope_lb20 | 293 | -0.017 | -0.025 | 37.5% |
| NMDC | vwap_trend_short_late_vwapslope_lb20 | 367 | -0.091 | -0.129 | 36.5% |
| IDEA | vwap_trend_short_late_vwapslope_lb20 | 653 | -0.221 | -0.316 | 26.3% |
