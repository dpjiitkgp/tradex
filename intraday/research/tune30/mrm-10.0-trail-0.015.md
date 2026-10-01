# tune30/mrm-10.0-trail-0.015 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 10.0x risk (7.500%), give-back trail 1.500% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 250 instruments, 2018-12-04..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_late_vwapslope_lb20 | 80882 | -1.704 | -2.272 | 44.3% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LICI | vwap_trend_short_late_vwapslope_lb20 | 196 | 0.168 | 0.224 | 65.8% |
| BAJAJHFL | vwap_trend_short_late_vwapslope_lb20 | 111 | 0.217 | 0.290 | 65.8% |
| LENSKART | vwap_trend_short_late_vwapslope_lb20 | 39 | 0.320 | 0.426 | 61.5% |
| ITCHOTELS | vwap_trend_short_late_vwapslope_lb20 | 68 | 0.226 | 0.301 | 60.3% |
| AWL | vwap_trend_short_late_vwapslope_lb20 | 222 | 0.179 | 0.239 | 58.1% |
| IREDA | vwap_trend_short_late_vwapslope_lb20 | 116 | 0.322 | 0.429 | 55.2% |
| NTPCGREEN | vwap_trend_short_late_vwapslope_lb20 | 95 | 0.108 | 0.144 | 54.7% |
| JIOFIN | vwap_trend_short_late_vwapslope_lb20 | 131 | 0.152 | 0.203 | 54.2% |
| RELIANCE | vwap_trend_short_late_vwapslope_lb20 | 296 | 0.124 | 0.165 | 53.7% |
| HAL | vwap_trend_short_late_vwapslope_lb20 | 404 | 0.171 | 0.228 | 53.0% |
| COCHINSHIP | vwap_trend_short_late_vwapslope_lb20 | 475 | 0.143 | 0.191 | 52.6% |
| JSWINFRA | vwap_trend_short_late_vwapslope_lb20 | 135 | 0.192 | 0.256 | 52.6% |
| ITC | vwap_trend_short_late_vwapslope_lb20 | 280 | 0.103 | 0.138 | 52.5% |
| INDUSINDBK | vwap_trend_short_late_vwapslope_lb20 | 292 | 0.265 | 0.354 | 52.1% |
| 3MINDIA | vwap_trend_short_late_vwapslope_lb20 | 527 | 0.096 | 0.128 | 52.0% |
| DMART | vwap_trend_short_late_vwapslope_lb20 | 386 | 0.183 | 0.244 | 51.8% |
| HONAUT | vwap_trend_short_late_vwapslope_lb20 | 454 | 0.099 | 0.132 | 51.8% |
| HDBFS | vwap_trend_short_late_vwapslope_lb20 | 62 | 0.014 | 0.019 | 51.6% |
| HDFCLIFE | vwap_trend_short_late_vwapslope_lb20 | 303 | 0.133 | 0.178 | 51.2% |
| EXIDEIND | vwap_trend_short_late_vwapslope_lb20 | 358 | 0.074 | 0.099 | 51.1% |
| HEROMOTOCO | vwap_trend_short_late_vwapslope_lb20 | 304 | 0.171 | 0.227 | 51.0% |
| IRCTC | vwap_trend_short_late_vwapslope_lb20 | 296 | 0.079 | 0.105 | 50.7% |
| LTM | vwap_trend_short_late_vwapslope_lb20 | 306 | 0.123 | 0.163 | 50.3% |
| CIPLA | vwap_trend_short_late_vwapslope_lb20 | 299 | 0.028 | 0.037 | 50.2% |
| MAZDOCK | vwap_trend_short_late_vwapslope_lb20 | 294 | 0.231 | 0.308 | 50.0% |
| TCS | vwap_trend_short_late_vwapslope_lb20 | 294 | 0.062 | 0.083 | 49.7% |
| TATAELXSI | vwap_trend_short_late_vwapslope_lb20 | 421 | 0.147 | 0.196 | 49.6% |
| NLCINDIA | vwap_trend_short_late_vwapslope_lb20 | 468 | 0.193 | 0.257 | 49.6% |
| LTTS | vwap_trend_short_late_vwapslope_lb20 | 349 | 0.130 | 0.174 | 49.6% |
| DIVISLAB | vwap_trend_short_late_vwapslope_lb20 | 313 | 0.040 | 0.053 | 49.5% |
| ATGL | vwap_trend_short_late_vwapslope_lb20 | 408 | 0.144 | 0.192 | 49.5% |
| UNITDSPR | vwap_trend_short_late_vwapslope_lb20 | 299 | 0.063 | 0.083 | 49.5% |
| MARICO | vwap_trend_short_late_vwapslope_lb20 | 295 | 0.052 | 0.070 | 49.5% |
| HUDCO | vwap_trend_short_late_vwapslope_lb20 | 489 | 0.070 | 0.093 | 49.5% |
| ABB | vwap_trend_short_late_vwapslope_lb20 | 356 | 0.098 | 0.131 | 49.4% |
| NESTLEIND | vwap_trend_short_late_vwapslope_lb20 | 318 | 0.057 | 0.076 | 49.4% |
| WAAREEENER | vwap_trend_short_late_vwapslope_lb20 | 79 | 0.259 | 0.345 | 49.4% |
| ACC | vwap_trend_short_late_vwapslope_lb20 | 329 | 0.080 | 0.107 | 49.2% |
| HDFCAMC | vwap_trend_short_late_vwapslope_lb20 | 368 | 0.080 | 0.106 | 49.2% |
| LLOYDSME | vwap_trend_short_late_vwapslope_lb20 | 165 | 0.116 | 0.154 | 49.1% |
| CRISIL | vwap_trend_short_late_vwapslope_lb20 | 438 | 0.070 | 0.093 | 49.1% |
| LINDEINDIA | vwap_trend_short_late_vwapslope_lb20 | 457 | 0.125 | 0.167 | 49.0% |
| NAUKRI | vwap_trend_short_late_vwapslope_lb20 | 355 | 0.196 | 0.262 | 49.0% |
| AUROPHARMA | vwap_trend_short_late_vwapslope_lb20 | 302 | 0.105 | 0.141 | 49.0% |
| BRITANNIA | vwap_trend_short_late_vwapslope_lb20 | 337 | 0.031 | 0.042 | 49.0% |
| ZYDUSLIFE | vwap_trend_short_late_vwapslope_lb20 | 321 | 0.115 | 0.153 | 48.9% |
| PNB | vwap_trend_short_late_vwapslope_lb20 | 403 | 0.172 | 0.229 | 48.9% |
| NYKAA | vwap_trend_short_late_vwapslope_lb20 | 221 | 0.141 | 0.188 | 48.9% |
| UPL | vwap_trend_short_late_vwapslope_lb20 | 307 | 0.059 | 0.079 | 48.9% |
| HAVELLS | vwap_trend_short_late_vwapslope_lb20 | 346 | 0.059 | 0.079 | 48.8% |
| LGEINDIA | vwap_trend_short_late_vwapslope_lb20 | 41 | 0.015 | 0.021 | 48.8% |
| PIDILITIND | vwap_trend_short_late_vwapslope_lb20 | 320 | 0.066 | 0.088 | 48.8% |
| LUPIN | vwap_trend_short_late_vwapslope_lb20 | 310 | 0.132 | 0.176 | 48.7% |
| SRF | vwap_trend_short_late_vwapslope_lb20 | 339 | 0.108 | 0.145 | 48.7% |
| BAJAJ-AUTO | vwap_trend_short_late_vwapslope_lb20 | 319 | 0.086 | 0.114 | 48.6% |
| DRREDDY | vwap_trend_short_late_vwapslope_lb20 | 303 | 0.055 | 0.073 | 48.5% |
| BANKINDIA | vwap_trend_short_late_vwapslope_lb20 | 425 | 0.130 | 0.174 | 48.5% |
| SUNPHARMA | vwap_trend_short_late_vwapslope_lb20 | 323 | 0.032 | 0.043 | 48.3% |
| TATAPOWER | vwap_trend_short_late_vwapslope_lb20 | 319 | 0.109 | 0.146 | 48.3% |
| DABUR | vwap_trend_short_late_vwapslope_lb20 | 284 | 0.090 | 0.120 | 48.2% |
| SBIN | vwap_trend_short_late_vwapslope_lb20 | 305 | 0.127 | 0.169 | 48.2% |
| APARINDS | vwap_trend_short_late_vwapslope_lb20 | 415 | 0.153 | 0.205 | 48.2% |
| GLAXO | vwap_trend_short_late_vwapslope_lb20 | 495 | 0.073 | 0.098 | 48.1% |
| DIXON | vwap_trend_short_late_vwapslope_lb20 | 363 | 0.199 | 0.265 | 47.9% |
| BDL | vwap_trend_short_late_vwapslope_lb20 | 467 | 0.089 | 0.118 | 47.8% |
| BIOCON | vwap_trend_short_late_vwapslope_lb20 | 333 | 0.066 | 0.088 | 47.7% |
| LT | vwap_trend_short_late_vwapslope_lb20 | 254 | 0.009 | 0.012 | 47.6% |
| VOLTAS | vwap_trend_short_late_vwapslope_lb20 | 294 | 0.104 | 0.139 | 47.6% |
| MANKIND | vwap_trend_short_late_vwapslope_lb20 | 160 | 0.149 | 0.198 | 47.5% |
| RVNL | vwap_trend_short_late_vwapslope_lb20 | 426 | 0.077 | 0.103 | 47.4% |
| TMPV | vwap_trend_short_late_vwapslope_lb20 | 344 | 0.166 | 0.222 | 47.4% |
| APOLLOTYRE | vwap_trend_short_late_vwapslope_lb20 | 351 | 0.113 | 0.150 | 47.3% |
| ANTHEM | vwap_trend_short_late_vwapslope_lb20 | 55 | 0.076 | 0.102 | 47.3% |
| GODREJIND | vwap_trend_short_late_vwapslope_lb20 | 440 | 0.023 | 0.030 | 47.0% |
| COALINDIA | vwap_trend_short_late_vwapslope_lb20 | 285 | 0.099 | 0.132 | 47.0% |
| APOLLOHOSP | vwap_trend_short_late_vwapslope_lb20 | 305 | 0.050 | 0.067 | 46.9% |
| AXISBANK | vwap_trend_short_late_vwapslope_lb20 | 286 | 0.004 | 0.006 | 46.9% |
| POWERGRID | vwap_trend_short_late_vwapslope_lb20 | 299 | 0.029 | 0.039 | 46.8% |
| ABCAPITAL | vwap_trend_short_late_vwapslope_lb20 | 393 | 0.057 | 0.075 | 46.8% |
| RECLTD | vwap_trend_short_late_vwapslope_lb20 | 361 | 0.156 | 0.208 | 46.8% |
| TATACONSUM | vwap_trend_short_late_vwapslope_lb20 | 355 | -0.010 | -0.014 | 46.8% |
| OFSS | vwap_trend_short_late_vwapslope_lb20 | 412 | 0.062 | 0.082 | 46.6% |
| KEI | vwap_trend_short_late_vwapslope_lb20 | 382 | 0.134 | 0.179 | 46.6% |
| INDHOTEL | vwap_trend_short_late_vwapslope_lb20 | 363 | 0.144 | 0.192 | 46.6% |
| M&M | vwap_trend_short_late_vwapslope_lb20 | 329 | 0.050 | 0.067 | 46.5% |
| MARUTI | vwap_trend_short_late_vwapslope_lb20 | 312 | 0.047 | 0.062 | 46.5% |
| IOC | vwap_trend_short_late_vwapslope_lb20 | 340 | 0.151 | 0.201 | 46.5% |
| KALYANKJIL | vwap_trend_short_late_vwapslope_lb20 | 308 | 0.030 | 0.040 | 46.4% |
| HCLTECH | vwap_trend_short_late_vwapslope_lb20 | 304 | 0.051 | 0.068 | 46.4% |
| GICRE | vwap_trend_short_late_vwapslope_lb20 | 483 | 0.085 | 0.113 | 46.4% |
| BERGEPAINT | vwap_trend_short_late_vwapslope_lb20 | 343 | 0.030 | 0.040 | 46.4% |
| PAGEIND | vwap_trend_short_late_vwapslope_lb20 | 311 | 0.058 | 0.078 | 46.3% |
| INFY | vwap_trend_short_late_vwapslope_lb20 | 283 | 0.016 | 0.022 | 46.3% |
| ASIANPAINT | vwap_trend_short_late_vwapslope_lb20 | 296 | 0.020 | 0.026 | 46.3% |
| WIPRO | vwap_trend_short_late_vwapslope_lb20 | 335 | 0.065 | 0.087 | 46.3% |
| HDFCBANK | vwap_trend_short_late_vwapslope_lb20 | 307 | 0.040 | 0.053 | 46.3% |
| THERMAX | vwap_trend_short_late_vwapslope_lb20 | 435 | 0.064 | 0.086 | 46.2% |
| TECHM | vwap_trend_short_late_vwapslope_lb20 | 280 | 0.067 | 0.089 | 46.1% |
| BAJAJHLDNG | vwap_trend_short_late_vwapslope_lb20 | 385 | 0.057 | 0.075 | 46.0% |
| EICHERMOT | vwap_trend_short_late_vwapslope_lb20 | 335 | 0.023 | 0.030 | 46.0% |
