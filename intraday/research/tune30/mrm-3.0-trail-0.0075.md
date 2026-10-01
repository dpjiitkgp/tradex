# tune30/mrm-3.0-trail-0.0075 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 3.0x risk (2.250%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 250 instruments, 2018-12-04..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_late_vwapslope_lb20 | 81222 | -1.699 | -2.265 | 44.8% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| BAJAJHFL | vwap_trend_short_late_vwapslope_lb20 | 111 | 0.206 | 0.274 | 65.8% |
| LICI | vwap_trend_short_late_vwapslope_lb20 | 199 | 0.168 | 0.224 | 65.3% |
| LENSKART | vwap_trend_short_late_vwapslope_lb20 | 39 | 0.326 | 0.435 | 61.5% |
| ITCHOTELS | vwap_trend_short_late_vwapslope_lb20 | 68 | 0.226 | 0.301 | 60.3% |
| AWL | vwap_trend_short_late_vwapslope_lb20 | 222 | 0.162 | 0.216 | 58.6% |
| IREDA | vwap_trend_short_late_vwapslope_lb20 | 120 | 0.222 | 0.296 | 55.0% |
| NTPCGREEN | vwap_trend_short_late_vwapslope_lb20 | 95 | 0.111 | 0.148 | 54.7% |
| JIOFIN | vwap_trend_short_late_vwapslope_lb20 | 134 | 0.168 | 0.224 | 54.5% |
| RELIANCE | vwap_trend_short_late_vwapslope_lb20 | 296 | 0.124 | 0.166 | 53.7% |
| HAL | vwap_trend_short_late_vwapslope_lb20 | 404 | 0.165 | 0.220 | 53.5% |
| COCHINSHIP | vwap_trend_short_late_vwapslope_lb20 | 478 | 0.125 | 0.167 | 52.9% |
| ITC | vwap_trend_short_late_vwapslope_lb20 | 282 | 0.101 | 0.135 | 52.8% |
| JSWINFRA | vwap_trend_short_late_vwapslope_lb20 | 135 | 0.162 | 0.216 | 52.6% |
| INDUSINDBK | vwap_trend_short_late_vwapslope_lb20 | 296 | 0.232 | 0.309 | 52.4% |
| 3MINDIA | vwap_trend_short_late_vwapslope_lb20 | 527 | 0.082 | 0.109 | 52.0% |
| DMART | vwap_trend_short_late_vwapslope_lb20 | 389 | 0.178 | 0.237 | 51.9% |
| HONAUT | vwap_trend_short_late_vwapslope_lb20 | 454 | 0.099 | 0.131 | 51.8% |
| HDBFS | vwap_trend_short_late_vwapslope_lb20 | 62 | 0.014 | 0.019 | 51.6% |
| EXIDEIND | vwap_trend_short_late_vwapslope_lb20 | 358 | 0.077 | 0.103 | 51.4% |
| HDFCLIFE | vwap_trend_short_late_vwapslope_lb20 | 303 | 0.111 | 0.148 | 51.2% |
| TCS | vwap_trend_short_late_vwapslope_lb20 | 295 | 0.087 | 0.116 | 50.8% |
| HEROMOTOCO | vwap_trend_short_late_vwapslope_lb20 | 309 | 0.127 | 0.170 | 50.8% |
| IRCTC | vwap_trend_short_late_vwapslope_lb20 | 296 | 0.074 | 0.099 | 50.7% |
| LTM | vwap_trend_short_late_vwapslope_lb20 | 308 | 0.135 | 0.179 | 50.3% |
| TATAELXSI | vwap_trend_short_late_vwapslope_lb20 | 424 | 0.130 | 0.173 | 50.2% |
| NYKAA | vwap_trend_short_late_vwapslope_lb20 | 221 | 0.159 | 0.212 | 50.2% |
| MAZDOCK | vwap_trend_short_late_vwapslope_lb20 | 295 | 0.160 | 0.214 | 50.2% |
| CIPLA | vwap_trend_short_late_vwapslope_lb20 | 299 | 0.032 | 0.043 | 50.2% |
| WAAREEENER | vwap_trend_short_late_vwapslope_lb20 | 82 | 0.247 | 0.329 | 50.0% |
| NLCINDIA | vwap_trend_short_late_vwapslope_lb20 | 468 | 0.177 | 0.236 | 50.0% |
| ATGL | vwap_trend_short_late_vwapslope_lb20 | 408 | 0.125 | 0.167 | 50.0% |
| LTTS | vwap_trend_short_late_vwapslope_lb20 | 349 | 0.108 | 0.145 | 49.9% |
| NESTLEIND | vwap_trend_short_late_vwapslope_lb20 | 321 | 0.044 | 0.059 | 49.8% |
| DIVISLAB | vwap_trend_short_late_vwapslope_lb20 | 315 | 0.046 | 0.061 | 49.8% |
| HUDCO | vwap_trend_short_late_vwapslope_lb20 | 489 | 0.081 | 0.109 | 49.7% |
| ACC | vwap_trend_short_late_vwapslope_lb20 | 329 | 0.085 | 0.114 | 49.5% |
| CRISIL | vwap_trend_short_late_vwapslope_lb20 | 438 | 0.073 | 0.097 | 49.5% |
| UNITDSPR | vwap_trend_short_late_vwapslope_lb20 | 299 | 0.064 | 0.085 | 49.5% |
| MARICO | vwap_trend_short_late_vwapslope_lb20 | 297 | 0.054 | 0.071 | 49.5% |
| LINDEINDIA | vwap_trend_short_late_vwapslope_lb20 | 457 | 0.123 | 0.165 | 49.5% |
| APARINDS | vwap_trend_short_late_vwapslope_lb20 | 415 | 0.180 | 0.240 | 49.4% |
| PNB | vwap_trend_short_late_vwapslope_lb20 | 403 | 0.158 | 0.210 | 49.4% |
| ZYDUSLIFE | vwap_trend_short_late_vwapslope_lb20 | 322 | 0.115 | 0.154 | 49.4% |
| AUROPHARMA | vwap_trend_short_late_vwapslope_lb20 | 304 | 0.113 | 0.151 | 49.3% |
| ABB | vwap_trend_short_late_vwapslope_lb20 | 359 | 0.052 | 0.070 | 49.3% |
| LLOYDSME | vwap_trend_short_late_vwapslope_lb20 | 165 | 0.122 | 0.162 | 49.1% |
| UPL | vwap_trend_short_late_vwapslope_lb20 | 308 | 0.071 | 0.095 | 49.0% |
| BRITANNIA | vwap_trend_short_late_vwapslope_lb20 | 337 | 0.020 | 0.026 | 49.0% |
| HDFCAMC | vwap_trend_short_late_vwapslope_lb20 | 372 | 0.051 | 0.068 | 48.9% |
| TATAPOWER | vwap_trend_short_late_vwapslope_lb20 | 319 | 0.105 | 0.140 | 48.9% |
| LUPIN | vwap_trend_short_late_vwapslope_lb20 | 313 | 0.114 | 0.152 | 48.9% |
| NAUKRI | vwap_trend_short_late_vwapslope_lb20 | 356 | 0.193 | 0.258 | 48.9% |
| HAVELLS | vwap_trend_short_late_vwapslope_lb20 | 350 | 0.044 | 0.059 | 48.9% |
| DRREDDY | vwap_trend_short_late_vwapslope_lb20 | 303 | 0.029 | 0.039 | 48.8% |
| BANKINDIA | vwap_trend_short_late_vwapslope_lb20 | 430 | 0.128 | 0.171 | 48.8% |
| LGEINDIA | vwap_trend_short_late_vwapslope_lb20 | 41 | 0.015 | 0.021 | 48.8% |
| DIXON | vwap_trend_short_late_vwapslope_lb20 | 365 | 0.205 | 0.273 | 48.8% |
| PIDILITIND | vwap_trend_short_late_vwapslope_lb20 | 322 | 0.061 | 0.082 | 48.8% |
| SBIN | vwap_trend_short_late_vwapslope_lb20 | 306 | 0.099 | 0.132 | 48.7% |
| BAJAJ-AUTO | vwap_trend_short_late_vwapslope_lb20 | 319 | 0.087 | 0.116 | 48.6% |
| SRF | vwap_trend_short_late_vwapslope_lb20 | 342 | 0.101 | 0.135 | 48.5% |
| VOLTAS | vwap_trend_short_late_vwapslope_lb20 | 299 | 0.131 | 0.174 | 48.5% |
| SUNPHARMA | vwap_trend_short_late_vwapslope_lb20 | 325 | 0.031 | 0.041 | 48.3% |
| GLAXO | vwap_trend_short_late_vwapslope_lb20 | 495 | 0.081 | 0.107 | 48.3% |
| APOLLOTYRE | vwap_trend_short_late_vwapslope_lb20 | 351 | 0.130 | 0.173 | 48.1% |
| RVNL | vwap_trend_short_late_vwapslope_lb20 | 428 | 0.085 | 0.114 | 48.1% |
| KALYANKJIL | vwap_trend_short_late_vwapslope_lb20 | 310 | 0.085 | 0.114 | 48.1% |
| BDL | vwap_trend_short_late_vwapslope_lb20 | 467 | 0.073 | 0.098 | 48.0% |
| DABUR | vwap_trend_short_late_vwapslope_lb20 | 286 | 0.071 | 0.095 | 47.9% |
| LT | vwap_trend_short_late_vwapslope_lb20 | 257 | 0.018 | 0.024 | 47.9% |
| TMPV | vwap_trend_short_late_vwapslope_lb20 | 347 | 0.152 | 0.202 | 47.8% |
| MANKIND | vwap_trend_short_late_vwapslope_lb20 | 161 | 0.116 | 0.155 | 47.8% |
| BIOCON | vwap_trend_short_late_vwapslope_lb20 | 335 | 0.058 | 0.077 | 47.8% |
| AXISBANK | vwap_trend_short_late_vwapslope_lb20 | 287 | 0.012 | 0.016 | 47.4% |
| ABCAPITAL | vwap_trend_short_late_vwapslope_lb20 | 395 | 0.059 | 0.079 | 47.3% |
| ANTHEM | vwap_trend_short_late_vwapslope_lb20 | 55 | 0.076 | 0.102 | 47.3% |
| RECLTD | vwap_trend_short_late_vwapslope_lb20 | 366 | 0.147 | 0.197 | 47.3% |
| COALINDIA | vwap_trend_short_late_vwapslope_lb20 | 286 | 0.087 | 0.116 | 47.2% |
| ENRIN | vwap_trend_short_late_vwapslope_lb20 | 53 | 0.135 | 0.180 | 47.2% |
| BLUESTARCO | vwap_trend_short_late_vwapslope_lb20 | 412 | 0.052 | 0.069 | 47.1% |
| APOLLOHOSP | vwap_trend_short_late_vwapslope_lb20 | 306 | 0.058 | 0.077 | 47.1% |
| TATACONSUM | vwap_trend_short_late_vwapslope_lb20 | 357 | -0.004 | -0.006 | 47.1% |
| GODREJIND | vwap_trend_short_late_vwapslope_lb20 | 440 | 0.015 | 0.020 | 47.0% |
| KEI | vwap_trend_short_late_vwapslope_lb20 | 385 | 0.117 | 0.156 | 47.0% |
| GICRE | vwap_trend_short_late_vwapslope_lb20 | 483 | 0.065 | 0.087 | 47.0% |
| THERMAX | vwap_trend_short_late_vwapslope_lb20 | 435 | 0.089 | 0.118 | 46.9% |
| OFSS | vwap_trend_short_late_vwapslope_lb20 | 416 | 0.066 | 0.088 | 46.9% |
| M&M | vwap_trend_short_late_vwapslope_lb20 | 333 | 0.050 | 0.067 | 46.8% |
| ASIANPAINT | vwap_trend_short_late_vwapslope_lb20 | 301 | 0.040 | 0.054 | 46.8% |
| INFY | vwap_trend_short_late_vwapslope_lb20 | 284 | 0.014 | 0.019 | 46.8% |
| POWERINDIA | vwap_trend_short_late_vwapslope_lb20 | 366 | 0.091 | 0.121 | 46.7% |
| POWERGRID | vwap_trend_short_late_vwapslope_lb20 | 302 | 0.019 | 0.026 | 46.7% |
| IOC | vwap_trend_short_late_vwapslope_lb20 | 343 | 0.148 | 0.198 | 46.6% |
| INDHOTEL | vwap_trend_short_late_vwapslope_lb20 | 369 | 0.138 | 0.184 | 46.6% |
| PAGEIND | vwap_trend_short_late_vwapslope_lb20 | 312 | 0.043 | 0.057 | 46.5% |
| MARUTI | vwap_trend_short_late_vwapslope_lb20 | 312 | 0.038 | 0.051 | 46.5% |
| WIPRO | vwap_trend_short_late_vwapslope_lb20 | 338 | 0.071 | 0.095 | 46.4% |
| TECHM | vwap_trend_short_late_vwapslope_lb20 | 280 | 0.067 | 0.089 | 46.4% |
| HCLTECH | vwap_trend_short_late_vwapslope_lb20 | 308 | 0.033 | 0.044 | 46.4% |
| LICHSGFIN | vwap_trend_short_late_vwapslope_lb20 | 304 | 0.073 | 0.097 | 46.4% |
