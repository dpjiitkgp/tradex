# tune30/mrm-3.0-trail-0.0025 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 3.0x risk (2.250%), give-back trail 0.250% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 250 instruments, 2018-12-04..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_late_vwapslope_lb20 | 81240 | -1.697 | -2.263 | 44.7% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| BAJAJHFL | vwap_trend_short_late_vwapslope_lb20 | 111 | 0.204 | 0.272 | 65.8% |
| LICI | vwap_trend_short_late_vwapslope_lb20 | 199 | 0.167 | 0.223 | 65.3% |
| LENSKART | vwap_trend_short_late_vwapslope_lb20 | 39 | 0.331 | 0.441 | 61.5% |
| ITCHOTELS | vwap_trend_short_late_vwapslope_lb20 | 68 | 0.226 | 0.301 | 60.3% |
| AWL | vwap_trend_short_late_vwapslope_lb20 | 222 | 0.153 | 0.203 | 58.6% |
| IREDA | vwap_trend_short_late_vwapslope_lb20 | 120 | 0.251 | 0.335 | 55.0% |
| NTPCGREEN | vwap_trend_short_late_vwapslope_lb20 | 95 | 0.114 | 0.152 | 54.7% |
| JIOFIN | vwap_trend_short_late_vwapslope_lb20 | 134 | 0.177 | 0.236 | 54.5% |
| RELIANCE | vwap_trend_short_late_vwapslope_lb20 | 296 | 0.125 | 0.167 | 53.7% |
| HAL | vwap_trend_short_late_vwapslope_lb20 | 404 | 0.172 | 0.229 | 53.5% |
| COCHINSHIP | vwap_trend_short_late_vwapslope_lb20 | 478 | 0.121 | 0.161 | 52.9% |
| ITC | vwap_trend_short_late_vwapslope_lb20 | 282 | 0.096 | 0.129 | 52.8% |
| JSWINFRA | vwap_trend_short_late_vwapslope_lb20 | 135 | 0.153 | 0.204 | 52.6% |
| INDUSINDBK | vwap_trend_short_late_vwapslope_lb20 | 296 | 0.229 | 0.305 | 52.4% |
| 3MINDIA | vwap_trend_short_late_vwapslope_lb20 | 527 | 0.081 | 0.108 | 52.0% |
| DMART | vwap_trend_short_late_vwapslope_lb20 | 390 | 0.184 | 0.245 | 51.8% |
| HONAUT | vwap_trend_short_late_vwapslope_lb20 | 454 | 0.089 | 0.119 | 51.8% |
| HDBFS | vwap_trend_short_late_vwapslope_lb20 | 62 | 0.014 | 0.019 | 51.6% |
| EXIDEIND | vwap_trend_short_late_vwapslope_lb20 | 358 | 0.082 | 0.110 | 51.4% |
| HDFCLIFE | vwap_trend_short_late_vwapslope_lb20 | 303 | 0.127 | 0.170 | 51.2% |
| TCS | vwap_trend_short_late_vwapslope_lb20 | 295 | 0.090 | 0.120 | 50.8% |
| IRCTC | vwap_trend_short_late_vwapslope_lb20 | 296 | 0.074 | 0.099 | 50.7% |
| HEROMOTOCO | vwap_trend_short_late_vwapslope_lb20 | 310 | 0.128 | 0.171 | 50.6% |
| LTM | vwap_trend_short_late_vwapslope_lb20 | 308 | 0.146 | 0.195 | 50.3% |
| TATAELXSI | vwap_trend_short_late_vwapslope_lb20 | 424 | 0.132 | 0.176 | 50.2% |
| NYKAA | vwap_trend_short_late_vwapslope_lb20 | 221 | 0.189 | 0.252 | 50.2% |
| CIPLA | vwap_trend_short_late_vwapslope_lb20 | 299 | 0.030 | 0.040 | 50.2% |
| WAAREEENER | vwap_trend_short_late_vwapslope_lb20 | 82 | 0.220 | 0.294 | 50.0% |
| NLCINDIA | vwap_trend_short_late_vwapslope_lb20 | 468 | 0.181 | 0.242 | 50.0% |
| MAZDOCK | vwap_trend_short_late_vwapslope_lb20 | 296 | 0.144 | 0.192 | 50.0% |
| ATGL | vwap_trend_short_late_vwapslope_lb20 | 408 | 0.116 | 0.155 | 50.0% |
| LTTS | vwap_trend_short_late_vwapslope_lb20 | 349 | 0.114 | 0.152 | 49.9% |
| NESTLEIND | vwap_trend_short_late_vwapslope_lb20 | 321 | 0.052 | 0.070 | 49.8% |
| DIVISLAB | vwap_trend_short_late_vwapslope_lb20 | 315 | 0.037 | 0.050 | 49.8% |
| HUDCO | vwap_trend_short_late_vwapslope_lb20 | 489 | 0.059 | 0.079 | 49.7% |
| ACC | vwap_trend_short_late_vwapslope_lb20 | 329 | 0.075 | 0.100 | 49.5% |
| CRISIL | vwap_trend_short_late_vwapslope_lb20 | 438 | 0.079 | 0.106 | 49.5% |
| UNITDSPR | vwap_trend_short_late_vwapslope_lb20 | 299 | 0.058 | 0.077 | 49.5% |
| MARICO | vwap_trend_short_late_vwapslope_lb20 | 297 | 0.046 | 0.062 | 49.5% |
| LINDEINDIA | vwap_trend_short_late_vwapslope_lb20 | 457 | 0.117 | 0.156 | 49.5% |
| APARINDS | vwap_trend_short_late_vwapslope_lb20 | 415 | 0.184 | 0.245 | 49.4% |
| PNB | vwap_trend_short_late_vwapslope_lb20 | 403 | 0.145 | 0.193 | 49.4% |
| ZYDUSLIFE | vwap_trend_short_late_vwapslope_lb20 | 322 | 0.089 | 0.118 | 49.4% |
| AUROPHARMA | vwap_trend_short_late_vwapslope_lb20 | 304 | 0.112 | 0.149 | 49.3% |
| ABB | vwap_trend_short_late_vwapslope_lb20 | 359 | 0.057 | 0.076 | 49.3% |
| LLOYDSME | vwap_trend_short_late_vwapslope_lb20 | 165 | 0.128 | 0.171 | 49.1% |
| UPL | vwap_trend_short_late_vwapslope_lb20 | 308 | 0.063 | 0.084 | 49.0% |
| HDFCAMC | vwap_trend_short_late_vwapslope_lb20 | 372 | 0.058 | 0.077 | 48.9% |
| TATAPOWER | vwap_trend_short_late_vwapslope_lb20 | 319 | 0.112 | 0.149 | 48.9% |
| NAUKRI | vwap_trend_short_late_vwapslope_lb20 | 356 | 0.178 | 0.238 | 48.9% |
| HAVELLS | vwap_trend_short_late_vwapslope_lb20 | 350 | 0.042 | 0.057 | 48.9% |
| DRREDDY | vwap_trend_short_late_vwapslope_lb20 | 303 | 0.029 | 0.039 | 48.8% |
| BANKINDIA | vwap_trend_short_late_vwapslope_lb20 | 430 | 0.133 | 0.178 | 48.8% |
| BRITANNIA | vwap_trend_short_late_vwapslope_lb20 | 338 | 0.012 | 0.016 | 48.8% |
| LGEINDIA | vwap_trend_short_late_vwapslope_lb20 | 41 | 0.015 | 0.021 | 48.8% |
| DIXON | vwap_trend_short_late_vwapslope_lb20 | 365 | 0.179 | 0.238 | 48.8% |
| PIDILITIND | vwap_trend_short_late_vwapslope_lb20 | 322 | 0.055 | 0.074 | 48.8% |
| LUPIN | vwap_trend_short_late_vwapslope_lb20 | 314 | 0.108 | 0.144 | 48.7% |
| SBIN | vwap_trend_short_late_vwapslope_lb20 | 306 | 0.106 | 0.142 | 48.7% |
| BAJAJ-AUTO | vwap_trend_short_late_vwapslope_lb20 | 319 | 0.076 | 0.102 | 48.6% |
| SRF | vwap_trend_short_late_vwapslope_lb20 | 342 | 0.102 | 0.136 | 48.5% |
| VOLTAS | vwap_trend_short_late_vwapslope_lb20 | 300 | 0.131 | 0.175 | 48.3% |
| SUNPHARMA | vwap_trend_short_late_vwapslope_lb20 | 325 | 0.017 | 0.023 | 48.3% |
| GLAXO | vwap_trend_short_late_vwapslope_lb20 | 495 | 0.076 | 0.101 | 48.3% |
| MANKIND | vwap_trend_short_late_vwapslope_lb20 | 162 | 0.143 | 0.190 | 48.1% |
| APOLLOTYRE | vwap_trend_short_late_vwapslope_lb20 | 351 | 0.134 | 0.179 | 48.1% |
| KALYANKJIL | vwap_trend_short_late_vwapslope_lb20 | 310 | 0.092 | 0.123 | 48.1% |
| RVNL | vwap_trend_short_late_vwapslope_lb20 | 429 | 0.089 | 0.119 | 48.0% |
| BDL | vwap_trend_short_late_vwapslope_lb20 | 467 | 0.073 | 0.098 | 48.0% |
| DABUR | vwap_trend_short_late_vwapslope_lb20 | 286 | 0.072 | 0.095 | 47.9% |
| LT | vwap_trend_short_late_vwapslope_lb20 | 257 | 0.025 | 0.034 | 47.9% |
| TMPV | vwap_trend_short_late_vwapslope_lb20 | 347 | 0.141 | 0.188 | 47.8% |
| BIOCON | vwap_trend_short_late_vwapslope_lb20 | 335 | 0.051 | 0.068 | 47.8% |
| ABCAPITAL | vwap_trend_short_late_vwapslope_lb20 | 394 | 0.074 | 0.099 | 47.5% |
| AXISBANK | vwap_trend_short_late_vwapslope_lb20 | 287 | 0.023 | 0.031 | 47.4% |
| ANTHEM | vwap_trend_short_late_vwapslope_lb20 | 55 | 0.075 | 0.100 | 47.3% |
| RECLTD | vwap_trend_short_late_vwapslope_lb20 | 366 | 0.130 | 0.174 | 47.3% |
| COALINDIA | vwap_trend_short_late_vwapslope_lb20 | 286 | 0.064 | 0.086 | 47.2% |
| ENRIN | vwap_trend_short_late_vwapslope_lb20 | 53 | 0.155 | 0.206 | 47.2% |
| BLUESTARCO | vwap_trend_short_late_vwapslope_lb20 | 412 | 0.073 | 0.098 | 47.1% |
| APOLLOHOSP | vwap_trend_short_late_vwapslope_lb20 | 306 | 0.066 | 0.088 | 47.1% |
| TATACONSUM | vwap_trend_short_late_vwapslope_lb20 | 357 | -0.006 | -0.008 | 47.1% |
| GODREJIND | vwap_trend_short_late_vwapslope_lb20 | 440 | 0.013 | 0.017 | 47.0% |
| KEI | vwap_trend_short_late_vwapslope_lb20 | 385 | 0.127 | 0.170 | 47.0% |
| GICRE | vwap_trend_short_late_vwapslope_lb20 | 483 | 0.067 | 0.089 | 47.0% |
| THERMAX | vwap_trend_short_late_vwapslope_lb20 | 435 | 0.092 | 0.122 | 46.9% |
| OFSS | vwap_trend_short_late_vwapslope_lb20 | 416 | 0.072 | 0.097 | 46.9% |
| M&M | vwap_trend_short_late_vwapslope_lb20 | 333 | 0.058 | 0.077 | 46.8% |
| INFY | vwap_trend_short_late_vwapslope_lb20 | 284 | 0.016 | 0.022 | 46.8% |
| POWERINDIA | vwap_trend_short_late_vwapslope_lb20 | 366 | 0.102 | 0.136 | 46.7% |
| ASIANPAINT | vwap_trend_short_late_vwapslope_lb20 | 302 | 0.039 | 0.052 | 46.7% |
| POWERGRID | vwap_trend_short_late_vwapslope_lb20 | 302 | 0.014 | 0.019 | 46.7% |
| IOC | vwap_trend_short_late_vwapslope_lb20 | 343 | 0.130 | 0.173 | 46.6% |
| INDHOTEL | vwap_trend_short_late_vwapslope_lb20 | 370 | 0.125 | 0.167 | 46.5% |
| PAGEIND | vwap_trend_short_late_vwapslope_lb20 | 312 | 0.051 | 0.068 | 46.5% |
| MARUTI | vwap_trend_short_late_vwapslope_lb20 | 312 | 0.043 | 0.057 | 46.5% |
| TECHM | vwap_trend_short_late_vwapslope_lb20 | 280 | 0.077 | 0.103 | 46.4% |
| HCLTECH | vwap_trend_short_late_vwapslope_lb20 | 308 | 0.030 | 0.040 | 46.4% |
| LICHSGFIN | vwap_trend_short_late_vwapslope_lb20 | 304 | 0.077 | 0.103 | 46.4% |
| BERGEPAINT | vwap_trend_short_late_vwapslope_lb20 | 343 | 0.045 | 0.060 | 46.4% |
