# tune30/mrm-6.0-trail-0.015 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 6.0x risk (4.500%), give-back trail 1.500% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 250 instruments, 2018-12-04..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_late_vwapslope_lb20 | 81026 | -1.701 | -2.268 | 44.3% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| BAJAJHFL | vwap_trend_short_late_vwapslope_lb20 | 111 | 0.217 | 0.290 | 65.8% |
| LICI | vwap_trend_short_late_vwapslope_lb20 | 199 | 0.187 | 0.250 | 65.3% |
| LENSKART | vwap_trend_short_late_vwapslope_lb20 | 39 | 0.320 | 0.426 | 61.5% |
| ITCHOTELS | vwap_trend_short_late_vwapslope_lb20 | 68 | 0.226 | 0.301 | 60.3% |
| AWL | vwap_trend_short_late_vwapslope_lb20 | 222 | 0.179 | 0.239 | 58.1% |
| NTPCGREEN | vwap_trend_short_late_vwapslope_lb20 | 95 | 0.108 | 0.144 | 54.7% |
| IREDA | vwap_trend_short_late_vwapslope_lb20 | 119 | 0.272 | 0.362 | 54.6% |
| JIOFIN | vwap_trend_short_late_vwapslope_lb20 | 133 | 0.174 | 0.232 | 54.1% |
| RELIANCE | vwap_trend_short_late_vwapslope_lb20 | 296 | 0.130 | 0.174 | 53.7% |
| HAL | vwap_trend_short_late_vwapslope_lb20 | 404 | 0.165 | 0.219 | 53.0% |
| ITC | vwap_trend_short_late_vwapslope_lb20 | 280 | 0.116 | 0.155 | 52.9% |
| JSWINFRA | vwap_trend_short_late_vwapslope_lb20 | 135 | 0.194 | 0.259 | 52.6% |
| COCHINSHIP | vwap_trend_short_late_vwapslope_lb20 | 477 | 0.117 | 0.156 | 52.4% |
| INDUSINDBK | vwap_trend_short_late_vwapslope_lb20 | 293 | 0.246 | 0.327 | 52.2% |
| 3MINDIA | vwap_trend_short_late_vwapslope_lb20 | 527 | 0.096 | 0.128 | 52.0% |
| DMART | vwap_trend_short_late_vwapslope_lb20 | 386 | 0.186 | 0.248 | 51.8% |
| HONAUT | vwap_trend_short_late_vwapslope_lb20 | 454 | 0.103 | 0.138 | 51.8% |
| HDBFS | vwap_trend_short_late_vwapslope_lb20 | 62 | 0.014 | 0.019 | 51.6% |
| HDFCLIFE | vwap_trend_short_late_vwapslope_lb20 | 303 | 0.133 | 0.178 | 51.2% |
| EXIDEIND | vwap_trend_short_late_vwapslope_lb20 | 358 | 0.074 | 0.099 | 51.1% |
| IRCTC | vwap_trend_short_late_vwapslope_lb20 | 296 | 0.079 | 0.105 | 50.7% |
| HEROMOTOCO | vwap_trend_short_late_vwapslope_lb20 | 307 | 0.138 | 0.184 | 50.5% |
| MAZDOCK | vwap_trend_short_late_vwapslope_lb20 | 295 | 0.216 | 0.289 | 50.2% |
| CIPLA | vwap_trend_short_late_vwapslope_lb20 | 299 | 0.028 | 0.037 | 50.2% |
| LTM | vwap_trend_short_late_vwapslope_lb20 | 308 | 0.110 | 0.147 | 50.0% |
| TATAELXSI | vwap_trend_short_late_vwapslope_lb20 | 420 | 0.143 | 0.191 | 49.8% |
| TCS | vwap_trend_short_late_vwapslope_lb20 | 294 | 0.062 | 0.083 | 49.7% |
| NLCINDIA | vwap_trend_short_late_vwapslope_lb20 | 468 | 0.183 | 0.245 | 49.6% |
| LTTS | vwap_trend_short_late_vwapslope_lb20 | 349 | 0.130 | 0.174 | 49.6% |
| NESTLEIND | vwap_trend_short_late_vwapslope_lb20 | 319 | 0.052 | 0.069 | 49.5% |
| DIVISLAB | vwap_trend_short_late_vwapslope_lb20 | 315 | 0.046 | 0.061 | 49.5% |
| ATGL | vwap_trend_short_late_vwapslope_lb20 | 408 | 0.138 | 0.185 | 49.5% |
| UNITDSPR | vwap_trend_short_late_vwapslope_lb20 | 299 | 0.063 | 0.083 | 49.5% |
| HUDCO | vwap_trend_short_late_vwapslope_lb20 | 489 | 0.071 | 0.094 | 49.5% |
| ABB | vwap_trend_short_late_vwapslope_lb20 | 356 | 0.082 | 0.109 | 49.4% |
| MARICO | vwap_trend_short_late_vwapslope_lb20 | 296 | 0.053 | 0.071 | 49.3% |
| HDFCAMC | vwap_trend_short_late_vwapslope_lb20 | 369 | 0.088 | 0.118 | 49.3% |
| ACC | vwap_trend_short_late_vwapslope_lb20 | 329 | 0.080 | 0.107 | 49.2% |
| LLOYDSME | vwap_trend_short_late_vwapslope_lb20 | 165 | 0.116 | 0.154 | 49.1% |
| CRISIL | vwap_trend_short_late_vwapslope_lb20 | 438 | 0.070 | 0.093 | 49.1% |
| UPL | vwap_trend_short_late_vwapslope_lb20 | 308 | 0.071 | 0.094 | 49.0% |
| LINDEINDIA | vwap_trend_short_late_vwapslope_lb20 | 457 | 0.118 | 0.158 | 49.0% |
| AUROPHARMA | vwap_trend_short_late_vwapslope_lb20 | 302 | 0.102 | 0.136 | 49.0% |
| BRITANNIA | vwap_trend_short_late_vwapslope_lb20 | 337 | 0.026 | 0.034 | 49.0% |
| PIDILITIND | vwap_trend_short_late_vwapslope_lb20 | 321 | 0.074 | 0.099 | 48.9% |
| PNB | vwap_trend_short_late_vwapslope_lb20 | 403 | 0.156 | 0.208 | 48.9% |
| NAUKRI | vwap_trend_short_late_vwapslope_lb20 | 356 | 0.196 | 0.262 | 48.9% |
| NYKAA | vwap_trend_short_late_vwapslope_lb20 | 221 | 0.120 | 0.160 | 48.9% |
| WAAREEENER | vwap_trend_short_late_vwapslope_lb20 | 82 | 0.252 | 0.335 | 48.8% |
| LGEINDIA | vwap_trend_short_late_vwapslope_lb20 | 41 | 0.015 | 0.021 | 48.8% |
| ZYDUSLIFE | vwap_trend_short_late_vwapslope_lb20 | 322 | 0.103 | 0.137 | 48.8% |
| LUPIN | vwap_trend_short_late_vwapslope_lb20 | 312 | 0.120 | 0.161 | 48.7% |
| HAVELLS | vwap_trend_short_late_vwapslope_lb20 | 349 | 0.069 | 0.092 | 48.7% |
| SRF | vwap_trend_short_late_vwapslope_lb20 | 339 | 0.101 | 0.135 | 48.7% |
| BAJAJ-AUTO | vwap_trend_short_late_vwapslope_lb20 | 319 | 0.086 | 0.114 | 48.6% |
| DRREDDY | vwap_trend_short_late_vwapslope_lb20 | 303 | 0.055 | 0.073 | 48.5% |
| BANKINDIA | vwap_trend_short_late_vwapslope_lb20 | 427 | 0.138 | 0.184 | 48.5% |
| SUNPHARMA | vwap_trend_short_late_vwapslope_lb20 | 324 | 0.045 | 0.059 | 48.5% |
| TATAPOWER | vwap_trend_short_late_vwapslope_lb20 | 319 | 0.102 | 0.136 | 48.3% |
| SBIN | vwap_trend_short_late_vwapslope_lb20 | 305 | 0.117 | 0.156 | 48.2% |
| APARINDS | vwap_trend_short_late_vwapslope_lb20 | 415 | 0.149 | 0.199 | 48.2% |
| GLAXO | vwap_trend_short_late_vwapslope_lb20 | 495 | 0.076 | 0.102 | 48.1% |
| DIXON | vwap_trend_short_late_vwapslope_lb20 | 364 | 0.206 | 0.274 | 48.1% |
| DABUR | vwap_trend_short_late_vwapslope_lb20 | 285 | 0.079 | 0.105 | 48.1% |
| BDL | vwap_trend_short_late_vwapslope_lb20 | 467 | 0.081 | 0.108 | 47.8% |
| BIOCON | vwap_trend_short_late_vwapslope_lb20 | 333 | 0.059 | 0.079 | 47.7% |
| LT | vwap_trend_short_late_vwapslope_lb20 | 256 | 0.028 | 0.037 | 47.7% |
| TMPV | vwap_trend_short_late_vwapslope_lb20 | 345 | 0.165 | 0.220 | 47.5% |
| MANKIND | vwap_trend_short_late_vwapslope_lb20 | 160 | 0.146 | 0.194 | 47.5% |
| VOLTAS | vwap_trend_short_late_vwapslope_lb20 | 295 | 0.093 | 0.123 | 47.5% |
| RVNL | vwap_trend_short_late_vwapslope_lb20 | 428 | 0.087 | 0.116 | 47.4% |
| APOLLOTYRE | vwap_trend_short_late_vwapslope_lb20 | 351 | 0.115 | 0.153 | 47.3% |
| ANTHEM | vwap_trend_short_late_vwapslope_lb20 | 55 | 0.076 | 0.102 | 47.3% |
| COALINDIA | vwap_trend_short_late_vwapslope_lb20 | 286 | 0.109 | 0.145 | 47.2% |
| GODREJIND | vwap_trend_short_late_vwapslope_lb20 | 440 | 0.022 | 0.030 | 47.0% |
| TATACONSUM | vwap_trend_short_late_vwapslope_lb20 | 356 | 0.003 | 0.004 | 46.9% |
| APOLLOHOSP | vwap_trend_short_late_vwapslope_lb20 | 305 | 0.051 | 0.069 | 46.9% |
| AXISBANK | vwap_trend_short_late_vwapslope_lb20 | 286 | 0.001 | 0.002 | 46.9% |
| POWERGRID | vwap_trend_short_late_vwapslope_lb20 | 299 | 0.029 | 0.039 | 46.8% |
| ABCAPITAL | vwap_trend_short_late_vwapslope_lb20 | 394 | 0.050 | 0.066 | 46.7% |
| M&M | vwap_trend_short_late_vwapslope_lb20 | 330 | 0.057 | 0.077 | 46.7% |
| OFSS | vwap_trend_short_late_vwapslope_lb20 | 414 | 0.057 | 0.076 | 46.6% |
| KEI | vwap_trend_short_late_vwapslope_lb20 | 384 | 0.141 | 0.188 | 46.6% |
| GICRE | vwap_trend_short_late_vwapslope_lb20 | 483 | 0.082 | 0.110 | 46.6% |
| RECLTD | vwap_trend_short_late_vwapslope_lb20 | 365 | 0.150 | 0.200 | 46.6% |
| IOC | vwap_trend_short_late_vwapslope_lb20 | 342 | 0.157 | 0.209 | 46.5% |
| MARUTI | vwap_trend_short_late_vwapslope_lb20 | 312 | 0.047 | 0.062 | 46.5% |
| INDHOTEL | vwap_trend_short_late_vwapslope_lb20 | 366 | 0.142 | 0.189 | 46.4% |
| KALYANKJIL | vwap_trend_short_late_vwapslope_lb20 | 308 | 0.030 | 0.040 | 46.4% |
| BERGEPAINT | vwap_trend_short_late_vwapslope_lb20 | 343 | 0.030 | 0.040 | 46.4% |
| ASIANPAINT | vwap_trend_short_late_vwapslope_lb20 | 298 | 0.026 | 0.035 | 46.3% |
| INFY | vwap_trend_short_late_vwapslope_lb20 | 283 | 0.016 | 0.022 | 46.3% |
| WIPRO | vwap_trend_short_late_vwapslope_lb20 | 335 | 0.065 | 0.087 | 46.3% |
| THERMAX | vwap_trend_short_late_vwapslope_lb20 | 435 | 0.058 | 0.077 | 46.2% |
| LICHSGFIN | vwap_trend_short_late_vwapslope_lb20 | 303 | 0.092 | 0.122 | 46.2% |
| PAGEIND | vwap_trend_short_late_vwapslope_lb20 | 312 | 0.045 | 0.060 | 46.2% |
| TECHM | vwap_trend_short_late_vwapslope_lb20 | 280 | 0.065 | 0.087 | 46.1% |
| EICHERMOT | vwap_trend_short_late_vwapslope_lb20 | 335 | 0.023 | 0.030 | 46.0% |
| HDFCBANK | vwap_trend_short_late_vwapslope_lb20 | 309 | 0.019 | 0.026 | 46.0% |
| OIL | vwap_trend_short_late_vwapslope_lb20 | 431 | 0.230 | 0.306 | 45.9% |
