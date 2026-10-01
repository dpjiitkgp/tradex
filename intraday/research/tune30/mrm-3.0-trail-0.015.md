# tune30/mrm-3.0-trail-0.015 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 3.0x risk (2.250%), give-back trail 1.500% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 250 instruments, 2018-12-04..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_late_vwapslope_lb20 | 81162 | -1.700 | -2.267 | 44.8% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| BAJAJHFL | vwap_trend_short_late_vwapslope_lb20 | 111 | 0.217 | 0.290 | 65.8% |
| LICI | vwap_trend_short_late_vwapslope_lb20 | 199 | 0.187 | 0.250 | 65.3% |
| LENSKART | vwap_trend_short_late_vwapslope_lb20 | 39 | 0.295 | 0.394 | 61.5% |
| ITCHOTELS | vwap_trend_short_late_vwapslope_lb20 | 68 | 0.226 | 0.301 | 60.3% |
| AWL | vwap_trend_short_late_vwapslope_lb20 | 222 | 0.173 | 0.231 | 58.6% |
| NTPCGREEN | vwap_trend_short_late_vwapslope_lb20 | 95 | 0.108 | 0.144 | 54.7% |
| IREDA | vwap_trend_short_late_vwapslope_lb20 | 121 | 0.215 | 0.286 | 54.5% |
| JIOFIN | vwap_trend_short_late_vwapslope_lb20 | 134 | 0.163 | 0.218 | 54.5% |
| RELIANCE | vwap_trend_short_late_vwapslope_lb20 | 296 | 0.130 | 0.174 | 53.7% |
| HAL | vwap_trend_short_late_vwapslope_lb20 | 404 | 0.158 | 0.210 | 53.5% |
| COCHINSHIP | vwap_trend_short_late_vwapslope_lb20 | 478 | 0.119 | 0.159 | 52.9% |
| ITC | vwap_trend_short_late_vwapslope_lb20 | 281 | 0.106 | 0.142 | 52.7% |
| JSWINFRA | vwap_trend_short_late_vwapslope_lb20 | 135 | 0.168 | 0.224 | 52.6% |
| INDUSINDBK | vwap_trend_short_late_vwapslope_lb20 | 295 | 0.250 | 0.333 | 52.5% |
| 3MINDIA | vwap_trend_short_late_vwapslope_lb20 | 527 | 0.091 | 0.121 | 52.0% |
| DMART | vwap_trend_short_late_vwapslope_lb20 | 389 | 0.193 | 0.257 | 51.9% |
| HONAUT | vwap_trend_short_late_vwapslope_lb20 | 454 | 0.101 | 0.134 | 51.8% |
| HDBFS | vwap_trend_short_late_vwapslope_lb20 | 62 | 0.014 | 0.019 | 51.6% |
| EXIDEIND | vwap_trend_short_late_vwapslope_lb20 | 358 | 0.075 | 0.100 | 51.4% |
| HDFCLIFE | vwap_trend_short_late_vwapslope_lb20 | 303 | 0.131 | 0.174 | 51.2% |
| HEROMOTOCO | vwap_trend_short_late_vwapslope_lb20 | 307 | 0.140 | 0.187 | 51.1% |
| TCS | vwap_trend_short_late_vwapslope_lb20 | 295 | 0.074 | 0.098 | 50.8% |
| IRCTC | vwap_trend_short_late_vwapslope_lb20 | 296 | 0.075 | 0.100 | 50.7% |
| LTM | vwap_trend_short_late_vwapslope_lb20 | 308 | 0.121 | 0.162 | 50.3% |
| TATAELXSI | vwap_trend_short_late_vwapslope_lb20 | 424 | 0.120 | 0.160 | 50.2% |
| NYKAA | vwap_trend_short_late_vwapslope_lb20 | 221 | 0.150 | 0.200 | 50.2% |
| MAZDOCK | vwap_trend_short_late_vwapslope_lb20 | 295 | 0.207 | 0.276 | 50.2% |
| CIPLA | vwap_trend_short_late_vwapslope_lb20 | 299 | 0.030 | 0.041 | 50.2% |
| WAAREEENER | vwap_trend_short_late_vwapslope_lb20 | 82 | 0.224 | 0.298 | 50.0% |
| NLCINDIA | vwap_trend_short_late_vwapslope_lb20 | 468 | 0.171 | 0.228 | 50.0% |
| ATGL | vwap_trend_short_late_vwapslope_lb20 | 408 | 0.131 | 0.175 | 50.0% |
| LTTS | vwap_trend_short_late_vwapslope_lb20 | 349 | 0.140 | 0.187 | 49.9% |
| DIVISLAB | vwap_trend_short_late_vwapslope_lb20 | 315 | 0.046 | 0.062 | 49.8% |
| HUDCO | vwap_trend_short_late_vwapslope_lb20 | 489 | 0.073 | 0.097 | 49.7% |
| NESTLEIND | vwap_trend_short_late_vwapslope_lb20 | 320 | 0.040 | 0.053 | 49.7% |
| ACC | vwap_trend_short_late_vwapslope_lb20 | 329 | 0.074 | 0.099 | 49.5% |
| CRISIL | vwap_trend_short_late_vwapslope_lb20 | 438 | 0.066 | 0.088 | 49.5% |
| UNITDSPR | vwap_trend_short_late_vwapslope_lb20 | 299 | 0.068 | 0.090 | 49.5% |
| MARICO | vwap_trend_short_late_vwapslope_lb20 | 297 | 0.045 | 0.060 | 49.5% |
| LINDEINDIA | vwap_trend_short_late_vwapslope_lb20 | 457 | 0.116 | 0.155 | 49.5% |
| ABB | vwap_trend_short_late_vwapslope_lb20 | 358 | 0.053 | 0.070 | 49.4% |
| APARINDS | vwap_trend_short_late_vwapslope_lb20 | 415 | 0.155 | 0.206 | 49.4% |
| PNB | vwap_trend_short_late_vwapslope_lb20 | 403 | 0.145 | 0.194 | 49.4% |
| ZYDUSLIFE | vwap_trend_short_late_vwapslope_lb20 | 322 | 0.106 | 0.141 | 49.4% |
| AUROPHARMA | vwap_trend_short_late_vwapslope_lb20 | 304 | 0.109 | 0.145 | 49.3% |
| LLOYDSME | vwap_trend_short_late_vwapslope_lb20 | 165 | 0.107 | 0.143 | 49.1% |
| UPL | vwap_trend_short_late_vwapslope_lb20 | 308 | 0.069 | 0.092 | 49.0% |
| HAVELLS | vwap_trend_short_late_vwapslope_lb20 | 349 | 0.056 | 0.075 | 49.0% |
| BRITANNIA | vwap_trend_short_late_vwapslope_lb20 | 337 | 0.018 | 0.025 | 49.0% |
| BANKINDIA | vwap_trend_short_late_vwapslope_lb20 | 429 | 0.118 | 0.157 | 49.0% |
| HDFCAMC | vwap_trend_short_late_vwapslope_lb20 | 372 | 0.062 | 0.083 | 48.9% |
| PIDILITIND | vwap_trend_short_late_vwapslope_lb20 | 321 | 0.061 | 0.081 | 48.9% |
| LUPIN | vwap_trend_short_late_vwapslope_lb20 | 313 | 0.108 | 0.144 | 48.9% |
| NAUKRI | vwap_trend_short_late_vwapslope_lb20 | 356 | 0.182 | 0.242 | 48.9% |
| DRREDDY | vwap_trend_short_late_vwapslope_lb20 | 303 | 0.058 | 0.078 | 48.8% |
| LGEINDIA | vwap_trend_short_late_vwapslope_lb20 | 41 | 0.015 | 0.021 | 48.8% |
| DIXON | vwap_trend_short_late_vwapslope_lb20 | 365 | 0.212 | 0.283 | 48.8% |
| SBIN | vwap_trend_short_late_vwapslope_lb20 | 306 | 0.076 | 0.102 | 48.7% |
| SRF | vwap_trend_short_late_vwapslope_lb20 | 341 | 0.098 | 0.131 | 48.7% |
| TATAPOWER | vwap_trend_short_late_vwapslope_lb20 | 319 | 0.105 | 0.140 | 48.6% |
| BAJAJ-AUTO | vwap_trend_short_late_vwapslope_lb20 | 319 | 0.086 | 0.114 | 48.6% |
| SUNPHARMA | vwap_trend_short_late_vwapslope_lb20 | 324 | 0.045 | 0.059 | 48.5% |
| VOLTAS | vwap_trend_short_late_vwapslope_lb20 | 298 | 0.108 | 0.144 | 48.3% |
| GLAXO | vwap_trend_short_late_vwapslope_lb20 | 495 | 0.077 | 0.103 | 48.3% |
| APOLLOTYRE | vwap_trend_short_late_vwapslope_lb20 | 351 | 0.101 | 0.135 | 48.1% |
| RVNL | vwap_trend_short_late_vwapslope_lb20 | 428 | 0.087 | 0.116 | 48.1% |
| KALYANKJIL | vwap_trend_short_late_vwapslope_lb20 | 310 | 0.061 | 0.082 | 48.1% |
| BDL | vwap_trend_short_late_vwapslope_lb20 | 467 | 0.055 | 0.074 | 48.0% |
| DABUR | vwap_trend_short_late_vwapslope_lb20 | 286 | 0.073 | 0.097 | 47.9% |
| LT | vwap_trend_short_late_vwapslope_lb20 | 257 | 0.022 | 0.029 | 47.9% |
| TMPV | vwap_trend_short_late_vwapslope_lb20 | 347 | 0.153 | 0.204 | 47.8% |
| MANKIND | vwap_trend_short_late_vwapslope_lb20 | 161 | 0.125 | 0.167 | 47.8% |
| BIOCON | vwap_trend_short_late_vwapslope_lb20 | 335 | 0.055 | 0.074 | 47.8% |
| AXISBANK | vwap_trend_short_late_vwapslope_lb20 | 286 | 0.008 | 0.010 | 47.6% |
| ABCAPITAL | vwap_trend_short_late_vwapslope_lb20 | 394 | 0.054 | 0.072 | 47.5% |
| ANTHEM | vwap_trend_short_late_vwapslope_lb20 | 55 | 0.076 | 0.102 | 47.3% |
| RECLTD | vwap_trend_short_late_vwapslope_lb20 | 366 | 0.160 | 0.214 | 47.3% |
| COALINDIA | vwap_trend_short_late_vwapslope_lb20 | 286 | 0.087 | 0.116 | 47.2% |
| ENRIN | vwap_trend_short_late_vwapslope_lb20 | 53 | 0.119 | 0.159 | 47.2% |
| M&M | vwap_trend_short_late_vwapslope_lb20 | 331 | 0.059 | 0.078 | 47.1% |
| BLUESTARCO | vwap_trend_short_late_vwapslope_lb20 | 412 | 0.047 | 0.063 | 47.1% |
| APOLLOHOSP | vwap_trend_short_late_vwapslope_lb20 | 306 | 0.051 | 0.067 | 47.1% |
| TATACONSUM | vwap_trend_short_late_vwapslope_lb20 | 357 | -0.006 | -0.008 | 47.1% |
| GODREJIND | vwap_trend_short_late_vwapslope_lb20 | 440 | 0.016 | 0.021 | 47.0% |
| POWERGRID | vwap_trend_short_late_vwapslope_lb20 | 300 | 0.031 | 0.041 | 47.0% |
| INDHOTEL | vwap_trend_short_late_vwapslope_lb20 | 366 | 0.150 | 0.200 | 47.0% |
| THERMAX | vwap_trend_short_late_vwapslope_lb20 | 435 | 0.067 | 0.089 | 46.9% |
| KEI | vwap_trend_short_late_vwapslope_lb20 | 384 | 0.102 | 0.136 | 46.9% |
| OFSS | vwap_trend_short_late_vwapslope_lb20 | 416 | 0.062 | 0.083 | 46.9% |
| ASIANPAINT | vwap_trend_short_late_vwapslope_lb20 | 301 | 0.036 | 0.048 | 46.8% |
| GICRE | vwap_trend_short_late_vwapslope_lb20 | 483 | 0.077 | 0.102 | 46.8% |
| POWERINDIA | vwap_trend_short_late_vwapslope_lb20 | 366 | 0.091 | 0.121 | 46.7% |
| IOC | vwap_trend_short_late_vwapslope_lb20 | 343 | 0.150 | 0.199 | 46.6% |
| INFY | vwap_trend_short_late_vwapslope_lb20 | 283 | 0.020 | 0.027 | 46.6% |
| LICHSGFIN | vwap_trend_short_late_vwapslope_lb20 | 303 | 0.082 | 0.110 | 46.5% |
| PAYTM | vwap_trend_short_late_vwapslope_lb20 | 215 | 0.164 | 0.218 | 46.5% |
| MARUTI | vwap_trend_short_late_vwapslope_lb20 | 312 | 0.047 | 0.063 | 46.5% |
| PAGEIND | vwap_trend_short_late_vwapslope_lb20 | 312 | 0.039 | 0.052 | 46.5% |
| TECHM | vwap_trend_short_late_vwapslope_lb20 | 280 | 0.075 | 0.100 | 46.4% |
| HCLTECH | vwap_trend_short_late_vwapslope_lb20 | 308 | 0.025 | 0.033 | 46.4% |
