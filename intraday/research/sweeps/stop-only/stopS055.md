# stopS055 — exit-rule research

Exit rule: risk_pct=0.550% (hard stop), armed at 4.1x risk (2.250%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 25 instruments, 2018-11-09..2026-09-30.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_late_vwapslope_lb20 | 8533 | 0.017 | 0.032 | 39.2% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LICI | vwap_trend_short_late_vwapslope_lb20 | 202 | 0.122 | 0.223 | 58.4% |
| LENSKART | vwap_trend_short_late_vwapslope_lb20 | 39 | 0.232 | 0.421 | 56.4% |
| ITC | vwap_trend_short_late_vwapslope_lb20 | 289 | 0.095 | 0.172 | 51.9% |
| HAL | vwap_trend_short_late_vwapslope_lb20 | 406 | 0.100 | 0.181 | 48.3% |
| DABUR | vwap_trend_short_late_vwapslope_lb20 | 287 | 0.089 | 0.162 | 48.1% |
| VOLTAS | vwap_trend_short_late_vwapslope_lb20 | 302 | 0.098 | 0.178 | 45.7% |
| ASIANPAINT | vwap_trend_short_late_vwapslope_lb20 | 305 | -0.009 | -0.016 | 44.9% |
| NAUKRI | vwap_trend_short_late_vwapslope_lb20 | 357 | 0.144 | 0.261 | 44.3% |
| WAAREEENER | vwap_trend_short_late_vwapslope_lb20 | 85 | 0.163 | 0.297 | 43.5% |
| PNB | vwap_trend_short_late_vwapslope_lb20 | 406 | 0.133 | 0.242 | 43.3% |
| ATGL | vwap_trend_short_late_vwapslope_lb20 | 408 | 0.082 | 0.150 | 42.9% |
| DIXON | vwap_trend_short_late_vwapslope_lb20 | 367 | 0.136 | 0.248 | 42.8% |
| TMPV | vwap_trend_short_late_vwapslope_lb20 | 352 | 0.110 | 0.201 | 42.3% |
| APARINDS | vwap_trend_short_late_vwapslope_lb20 | 421 | 0.094 | 0.171 | 42.0% |
| CANBK | vwap_trend_short_late_vwapslope_lb20 | 339 | 0.091 | 0.165 | 40.4% |
| LLOYDSME | vwap_trend_short_late_vwapslope_lb20 | 165 | 0.037 | 0.068 | 39.4% |
| ICICIPRULI | vwap_trend_short_late_vwapslope_lb20 | 310 | 0.008 | 0.015 | 37.7% |
| TIINDIA | vwap_trend_short_late_vwapslope_lb20 | 416 | -0.010 | -0.017 | 37.5% |
| TVSMOTOR | vwap_trend_short_late_vwapslope_lb20 | 320 | -0.028 | -0.051 | 36.9% |
| PRESTIGE | vwap_trend_short_late_vwapslope_lb20 | 405 | 0.059 | 0.106 | 36.3% |
| YESBANK | vwap_trend_short_late_vwapslope_lb20 | 564 | -0.084 | -0.152 | 34.8% |
| GMRAIRPORT | vwap_trend_short_late_vwapslope_lb20 | 394 | -0.065 | -0.119 | 33.2% |
| NMDC | vwap_trend_short_late_vwapslope_lb20 | 367 | -0.106 | -0.194 | 33.2% |
| LODHA | vwap_trend_short_late_vwapslope_lb20 | 296 | -0.041 | -0.074 | 31.4% |
| IDEA | vwap_trend_short_late_vwapslope_lb20 | 731 | -0.291 | -0.529 | 18.9% |
