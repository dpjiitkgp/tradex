# tune30/mrm-6.0-trail-0.0075 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 6.0x risk (4.500%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 250 instruments, 2018-12-04..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_late_vwapslope_lb20 | 81046 | -1.699 | -2.266 | 44.3% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| BAJAJHFL | vwap_trend_short_late_vwapslope_lb20 | 111 | 0.217 | 0.290 | 65.8% |
| LICI | vwap_trend_short_late_vwapslope_lb20 | 199 | 0.180 | 0.239 | 65.3% |
| LENSKART | vwap_trend_short_late_vwapslope_lb20 | 39 | 0.320 | 0.426 | 61.5% |
| ITCHOTELS | vwap_trend_short_late_vwapslope_lb20 | 68 | 0.226 | 0.301 | 60.3% |
| AWL | vwap_trend_short_late_vwapslope_lb20 | 222 | 0.179 | 0.239 | 58.1% |
| NTPCGREEN | vwap_trend_short_late_vwapslope_lb20 | 95 | 0.108 | 0.144 | 54.7% |
| IREDA | vwap_trend_short_late_vwapslope_lb20 | 119 | 0.284 | 0.379 | 54.6% |
| JIOFIN | vwap_trend_short_late_vwapslope_lb20 | 133 | 0.180 | 0.239 | 54.1% |
| RELIANCE | vwap_trend_short_late_vwapslope_lb20 | 296 | 0.132 | 0.176 | 53.7% |
| HAL | vwap_trend_short_late_vwapslope_lb20 | 404 | 0.167 | 0.223 | 53.0% |
| ITC | vwap_trend_short_late_vwapslope_lb20 | 280 | 0.120 | 0.160 | 52.9% |
| JSWINFRA | vwap_trend_short_late_vwapslope_lb20 | 135 | 0.195 | 0.260 | 52.6% |
| COCHINSHIP | vwap_trend_short_late_vwapslope_lb20 | 477 | 0.118 | 0.157 | 52.4% |
| INDUSINDBK | vwap_trend_short_late_vwapslope_lb20 | 293 | 0.242 | 0.323 | 52.2% |
| 3MINDIA | vwap_trend_short_late_vwapslope_lb20 | 527 | 0.094 | 0.125 | 52.0% |
| DMART | vwap_trend_short_late_vwapslope_lb20 | 386 | 0.188 | 0.250 | 51.8% |
| HONAUT | vwap_trend_short_late_vwapslope_lb20 | 454 | 0.102 | 0.136 | 51.8% |
| HDBFS | vwap_trend_short_late_vwapslope_lb20 | 62 | 0.014 | 0.019 | 51.6% |
| HDFCLIFE | vwap_trend_short_late_vwapslope_lb20 | 303 | 0.133 | 0.178 | 51.2% |
| EXIDEIND | vwap_trend_short_late_vwapslope_lb20 | 358 | 0.074 | 0.099 | 51.1% |
| IRCTC | vwap_trend_short_late_vwapslope_lb20 | 296 | 0.079 | 0.105 | 50.7% |
| HEROMOTOCO | vwap_trend_short_late_vwapslope_lb20 | 307 | 0.137 | 0.183 | 50.5% |
| MAZDOCK | vwap_trend_short_late_vwapslope_lb20 | 295 | 0.204 | 0.272 | 50.2% |
| CIPLA | vwap_trend_short_late_vwapslope_lb20 | 299 | 0.028 | 0.037 | 50.2% |
| LTM | vwap_trend_short_late_vwapslope_lb20 | 308 | 0.118 | 0.158 | 50.0% |
| TATAELXSI | vwap_trend_short_late_vwapslope_lb20 | 421 | 0.159 | 0.211 | 49.9% |
| TCS | vwap_trend_short_late_vwapslope_lb20 | 294 | 0.058 | 0.077 | 49.7% |
| NLCINDIA | vwap_trend_short_late_vwapslope_lb20 | 468 | 0.190 | 0.253 | 49.6% |
| LTTS | vwap_trend_short_late_vwapslope_lb20 | 349 | 0.130 | 0.174 | 49.6% |
| NESTLEIND | vwap_trend_short_late_vwapslope_lb20 | 319 | 0.057 | 0.076 | 49.5% |
| DIVISLAB | vwap_trend_short_late_vwapslope_lb20 | 315 | 0.048 | 0.064 | 49.5% |
| ATGL | vwap_trend_short_late_vwapslope_lb20 | 408 | 0.143 | 0.191 | 49.5% |
| UNITDSPR | vwap_trend_short_late_vwapslope_lb20 | 299 | 0.056 | 0.075 | 49.5% |
| HUDCO | vwap_trend_short_late_vwapslope_lb20 | 489 | 0.073 | 0.097 | 49.5% |
| ABB | vwap_trend_short_late_vwapslope_lb20 | 356 | 0.084 | 0.112 | 49.4% |
| MARICO | vwap_trend_short_late_vwapslope_lb20 | 296 | 0.050 | 0.066 | 49.3% |
| ACC | vwap_trend_short_late_vwapslope_lb20 | 329 | 0.082 | 0.109 | 49.2% |
| LLOYDSME | vwap_trend_short_late_vwapslope_lb20 | 165 | 0.116 | 0.154 | 49.1% |
| CRISIL | vwap_trend_short_late_vwapslope_lb20 | 438 | 0.070 | 0.093 | 49.1% |
| HDFCAMC | vwap_trend_short_late_vwapslope_lb20 | 371 | 0.078 | 0.105 | 49.1% |
| UPL | vwap_trend_short_late_vwapslope_lb20 | 308 | 0.073 | 0.097 | 49.0% |
| LINDEINDIA | vwap_trend_short_late_vwapslope_lb20 | 457 | 0.123 | 0.165 | 49.0% |
| AUROPHARMA | vwap_trend_short_late_vwapslope_lb20 | 302 | 0.103 | 0.137 | 49.0% |
| BRITANNIA | vwap_trend_short_late_vwapslope_lb20 | 337 | 0.027 | 0.037 | 49.0% |
| PIDILITIND | vwap_trend_short_late_vwapslope_lb20 | 321 | 0.076 | 0.101 | 48.9% |
| PNB | vwap_trend_short_late_vwapslope_lb20 | 403 | 0.161 | 0.215 | 48.9% |
| NAUKRI | vwap_trend_short_late_vwapslope_lb20 | 356 | 0.196 | 0.261 | 48.9% |
| NYKAA | vwap_trend_short_late_vwapslope_lb20 | 221 | 0.126 | 0.168 | 48.9% |
| WAAREEENER | vwap_trend_short_late_vwapslope_lb20 | 82 | 0.271 | 0.361 | 48.8% |
| LGEINDIA | vwap_trend_short_late_vwapslope_lb20 | 41 | 0.015 | 0.021 | 48.8% |
| ZYDUSLIFE | vwap_trend_short_late_vwapslope_lb20 | 322 | 0.104 | 0.139 | 48.8% |
| LUPIN | vwap_trend_short_late_vwapslope_lb20 | 312 | 0.127 | 0.169 | 48.7% |
| HAVELLS | vwap_trend_short_late_vwapslope_lb20 | 349 | 0.075 | 0.100 | 48.7% |
| SRF | vwap_trend_short_late_vwapslope_lb20 | 339 | 0.102 | 0.136 | 48.7% |
| BAJAJ-AUTO | vwap_trend_short_late_vwapslope_lb20 | 319 | 0.085 | 0.113 | 48.6% |
| DRREDDY | vwap_trend_short_late_vwapslope_lb20 | 303 | 0.036 | 0.048 | 48.5% |
| BANKINDIA | vwap_trend_short_late_vwapslope_lb20 | 427 | 0.146 | 0.194 | 48.5% |
| SUNPHARMA | vwap_trend_short_late_vwapslope_lb20 | 324 | 0.046 | 0.061 | 48.5% |
| TATAPOWER | vwap_trend_short_late_vwapslope_lb20 | 319 | 0.108 | 0.144 | 48.3% |
| SBIN | vwap_trend_short_late_vwapslope_lb20 | 305 | 0.114 | 0.152 | 48.2% |
| APARINDS | vwap_trend_short_late_vwapslope_lb20 | 415 | 0.153 | 0.204 | 48.2% |
| GLAXO | vwap_trend_short_late_vwapslope_lb20 | 495 | 0.079 | 0.106 | 48.1% |
| DIXON | vwap_trend_short_late_vwapslope_lb20 | 364 | 0.213 | 0.283 | 48.1% |
| DABUR | vwap_trend_short_late_vwapslope_lb20 | 285 | 0.081 | 0.108 | 48.1% |
| BDL | vwap_trend_short_late_vwapslope_lb20 | 467 | 0.085 | 0.114 | 47.8% |
| BIOCON | vwap_trend_short_late_vwapslope_lb20 | 333 | 0.061 | 0.082 | 47.7% |
| LT | vwap_trend_short_late_vwapslope_lb20 | 256 | 0.023 | 0.030 | 47.7% |
| VOLTAS | vwap_trend_short_late_vwapslope_lb20 | 296 | 0.106 | 0.141 | 47.6% |
| TMPV | vwap_trend_short_late_vwapslope_lb20 | 345 | 0.171 | 0.228 | 47.5% |
| MANKIND | vwap_trend_short_late_vwapslope_lb20 | 160 | 0.132 | 0.176 | 47.5% |
| RVNL | vwap_trend_short_late_vwapslope_lb20 | 428 | 0.083 | 0.111 | 47.4% |
| APOLLOTYRE | vwap_trend_short_late_vwapslope_lb20 | 351 | 0.121 | 0.161 | 47.3% |
| ANTHEM | vwap_trend_short_late_vwapslope_lb20 | 55 | 0.076 | 0.102 | 47.3% |
| COALINDIA | vwap_trend_short_late_vwapslope_lb20 | 286 | 0.116 | 0.155 | 47.2% |
| GODREJIND | vwap_trend_short_late_vwapslope_lb20 | 440 | 0.023 | 0.031 | 47.0% |
| APOLLOHOSP | vwap_trend_short_late_vwapslope_lb20 | 305 | 0.054 | 0.072 | 46.9% |
| AXISBANK | vwap_trend_short_late_vwapslope_lb20 | 286 | 0.004 | 0.005 | 46.9% |
| TATACONSUM | vwap_trend_short_late_vwapslope_lb20 | 357 | -0.001 | -0.002 | 46.8% |
| ABCAPITAL | vwap_trend_short_late_vwapslope_lb20 | 394 | 0.054 | 0.072 | 46.7% |
| POWERGRID | vwap_trend_short_late_vwapslope_lb20 | 300 | 0.020 | 0.026 | 46.7% |
| OFSS | vwap_trend_short_late_vwapslope_lb20 | 414 | 0.061 | 0.081 | 46.6% |
| KEI | vwap_trend_short_late_vwapslope_lb20 | 384 | 0.143 | 0.191 | 46.6% |
| GICRE | vwap_trend_short_late_vwapslope_lb20 | 483 | 0.088 | 0.117 | 46.6% |
| RECLTD | vwap_trend_short_late_vwapslope_lb20 | 365 | 0.154 | 0.205 | 46.6% |
| M&M | vwap_trend_short_late_vwapslope_lb20 | 331 | 0.057 | 0.076 | 46.5% |
| IOC | vwap_trend_short_late_vwapslope_lb20 | 342 | 0.161 | 0.214 | 46.5% |
| MARUTI | vwap_trend_short_late_vwapslope_lb20 | 312 | 0.043 | 0.057 | 46.5% |
| KALYANKJIL | vwap_trend_short_late_vwapslope_lb20 | 308 | 0.030 | 0.040 | 46.4% |
| BERGEPAINT | vwap_trend_short_late_vwapslope_lb20 | 343 | 0.030 | 0.040 | 46.4% |
| ASIANPAINT | vwap_trend_short_late_vwapslope_lb20 | 298 | 0.030 | 0.040 | 46.3% |
| INFY | vwap_trend_short_late_vwapslope_lb20 | 283 | 0.014 | 0.019 | 46.3% |
| WIPRO | vwap_trend_short_late_vwapslope_lb20 | 335 | 0.069 | 0.092 | 46.3% |
| THERMAX | vwap_trend_short_late_vwapslope_lb20 | 435 | 0.062 | 0.082 | 46.2% |
| LICHSGFIN | vwap_trend_short_late_vwapslope_lb20 | 303 | 0.093 | 0.123 | 46.2% |
| PAGEIND | vwap_trend_short_late_vwapslope_lb20 | 312 | 0.045 | 0.061 | 46.2% |
| TECHM | vwap_trend_short_late_vwapslope_lb20 | 280 | 0.066 | 0.088 | 46.1% |
| INDHOTEL | vwap_trend_short_late_vwapslope_lb20 | 369 | 0.143 | 0.191 | 46.1% |
| EICHERMOT | vwap_trend_short_late_vwapslope_lb20 | 335 | 0.020 | 0.027 | 46.0% |
| HDFCBANK | vwap_trend_short_late_vwapslope_lb20 | 309 | 0.022 | 0.029 | 46.0% |
| OIL | vwap_trend_short_late_vwapslope_lb20 | 431 | 0.225 | 0.300 | 45.9% |
