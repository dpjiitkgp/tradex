# stopS075 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 3.0x risk (2.250%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 25 instruments, 2018-11-09..2026-09-30.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_late_vwapslope_lb20 | 8401 | 0.045 | 0.060 | 45.0% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LENSKART | vwap_trend_short_late_vwapslope_lb20 | 39 | 0.312 | 0.416 | 61.5% |
| LICI | vwap_trend_short_late_vwapslope_lb20 | 202 | 0.125 | 0.167 | 61.4% |
| ITC | vwap_trend_short_late_vwapslope_lb20 | 289 | 0.091 | 0.122 | 55.0% |
| HAL | vwap_trend_short_late_vwapslope_lb20 | 404 | 0.148 | 0.197 | 54.5% |
| NAUKRI | vwap_trend_short_late_vwapslope_lb20 | 357 | 0.184 | 0.245 | 49.9% |
| VOLTAS | vwap_trend_short_late_vwapslope_lb20 | 301 | 0.094 | 0.125 | 49.8% |
| WAAREEENER | vwap_trend_short_late_vwapslope_lb20 | 83 | 0.172 | 0.229 | 49.4% |
| DABUR | vwap_trend_short_late_vwapslope_lb20 | 287 | 0.072 | 0.096 | 49.1% |
| TMPV | vwap_trend_short_late_vwapslope_lb20 | 350 | 0.126 | 0.168 | 48.9% |
| APARINDS | vwap_trend_short_late_vwapslope_lb20 | 416 | 0.128 | 0.170 | 48.8% |
| DIXON | vwap_trend_short_late_vwapslope_lb20 | 367 | 0.165 | 0.221 | 48.8% |
| ATGL | vwap_trend_short_late_vwapslope_lb20 | 408 | 0.103 | 0.137 | 48.5% |
| PNB | vwap_trend_short_late_vwapslope_lb20 | 404 | 0.128 | 0.170 | 48.5% |
| ASIANPAINT | vwap_trend_short_late_vwapslope_lb20 | 305 | 0.004 | 0.005 | 47.5% |
| LLOYDSME | vwap_trend_short_late_vwapslope_lb20 | 165 | 0.068 | 0.090 | 46.1% |
| CANBK | vwap_trend_short_late_vwapslope_lb20 | 337 | 0.100 | 0.134 | 46.0% |
| PRESTIGE | vwap_trend_short_late_vwapslope_lb20 | 403 | 0.075 | 0.099 | 42.4% |
| TIINDIA | vwap_trend_short_late_vwapslope_lb20 | 414 | 0.000 | 0.001 | 42.3% |
| ICICIPRULI | vwap_trend_short_late_vwapslope_lb20 | 306 | -0.004 | -0.005 | 42.2% |
| TVSMOTOR | vwap_trend_short_late_vwapslope_lb20 | 318 | -0.037 | -0.049 | 41.5% |
| YESBANK | vwap_trend_short_late_vwapslope_lb20 | 557 | -0.024 | -0.032 | 40.9% |
| GMRAIRPORT | vwap_trend_short_late_vwapslope_lb20 | 389 | -0.037 | -0.049 | 39.1% |
| LODHA | vwap_trend_short_late_vwapslope_lb20 | 293 | -0.032 | -0.043 | 37.9% |
| NMDC | vwap_trend_short_late_vwapslope_lb20 | 367 | -0.101 | -0.135 | 37.1% |
| IDEA | vwap_trend_short_late_vwapslope_lb20 | 640 | -0.176 | -0.234 | 28.9% |
