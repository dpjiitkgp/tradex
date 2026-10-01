# tune30/mrm-1.5-trail-0.0075 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 1.5x risk (1.125%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 251 instruments, 2018-11-09..2026-09-29.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_late_vwapslope_lb20 | 80996 | 0.024 | 0.032 | 47.4% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LENSKART | vwap_trend_short_late_vwapslope_lb20 | 39 | 0.345 | 0.461 | 66.7% |
| LICI | vwap_trend_short_late_vwapslope_lb20 | 200 | 0.176 | 0.235 | 66.0% |
| BAJAJHFL | vwap_trend_short_late_vwapslope_lb20 | 112 | 0.207 | 0.276 | 65.2% |
| ITCHOTELS | vwap_trend_short_late_vwapslope_lb20 | 68 | 0.158 | 0.211 | 60.3% |
| AWL | vwap_trend_short_late_vwapslope_lb20 | 222 | 0.126 | 0.168 | 59.9% |
| JSWINFRA | vwap_trend_short_late_vwapslope_lb20 | 135 | 0.234 | 0.312 | 58.5% |
| IREDA | vwap_trend_short_late_vwapslope_lb20 | 122 | 0.181 | 0.241 | 57.4% |
| JIOFIN | vwap_trend_short_late_vwapslope_lb20 | 134 | 0.189 | 0.252 | 56.7% |
| WAAREEENER | vwap_trend_short_late_vwapslope_lb20 | 82 | 0.290 | 0.386 | 56.1% |
| MAZDOCK | vwap_trend_short_late_vwapslope_lb20 | 296 | 0.154 | 0.206 | 55.1% |
| RELIANCE | vwap_trend_short_late_vwapslope_lb20 | 296 | 0.129 | 0.172 | 55.1% |
| HAL | vwap_trend_short_late_vwapslope_lb20 | 404 | 0.154 | 0.205 | 55.0% |
| NTPCGREEN | vwap_trend_short_late_vwapslope_lb20 | 95 | 0.099 | 0.131 | 54.7% |
| ITC | vwap_trend_short_late_vwapslope_lb20 | 284 | 0.114 | 0.152 | 54.6% |
| 3MINDIA | vwap_trend_short_late_vwapslope_lb20 | 527 | 0.073 | 0.097 | 54.5% |
| COCHINSHIP | vwap_trend_short_late_vwapslope_lb20 | 478 | 0.094 | 0.125 | 54.2% |
| NLCINDIA | vwap_trend_short_late_vwapslope_lb20 | 468 | 0.167 | 0.222 | 54.1% |
| EXIDEIND | vwap_trend_short_late_vwapslope_lb20 | 359 | 0.106 | 0.141 | 54.0% |
| HONAUT | vwap_trend_short_late_vwapslope_lb20 | 454 | 0.087 | 0.116 | 54.0% |
| ATGL | vwap_trend_short_late_vwapslope_lb20 | 408 | 0.090 | 0.120 | 53.7% |
| INDUSINDBK | vwap_trend_short_late_vwapslope_lb20 | 297 | 0.191 | 0.255 | 53.5% |
| PREMIERENE | vwap_trend_short_late_vwapslope_lb20 | 99 | 0.083 | 0.111 | 53.5% |
| DMART | vwap_trend_short_late_vwapslope_lb20 | 390 | 0.154 | 0.206 | 53.3% |
| HUDCO | vwap_trend_short_late_vwapslope_lb20 | 490 | 0.080 | 0.107 | 53.3% |
| DIXON | vwap_trend_short_late_vwapslope_lb20 | 367 | 0.187 | 0.249 | 53.1% |
| NYKAA | vwap_trend_short_late_vwapslope_lb20 | 221 | 0.134 | 0.179 | 52.9% |
| APARINDS | vwap_trend_short_late_vwapslope_lb20 | 415 | 0.151 | 0.201 | 52.8% |
| GROWW | vwap_trend_short_late_vwapslope_lb20 | 38 | 0.206 | 0.275 | 52.6% |
| HEROMOTOCO | vwap_trend_short_late_vwapslope_lb20 | 310 | 0.118 | 0.158 | 52.6% |
| IRCTC | vwap_trend_short_late_vwapslope_lb20 | 297 | 0.093 | 0.124 | 52.5% |
| HDFCLIFE | vwap_trend_short_late_vwapslope_lb20 | 303 | 0.103 | 0.138 | 52.5% |
| NAUKRI | vwap_trend_short_late_vwapslope_lb20 | 357 | 0.161 | 0.214 | 52.4% |
| AUROPHARMA | vwap_trend_short_late_vwapslope_lb20 | 306 | 0.124 | 0.165 | 52.3% |
| LTTS | vwap_trend_short_late_vwapslope_lb20 | 349 | 0.092 | 0.122 | 52.1% |
| LINDEINDIA | vwap_trend_short_late_vwapslope_lb20 | 457 | 0.095 | 0.126 | 52.1% |
| PNB | vwap_trend_short_late_vwapslope_lb20 | 404 | 0.137 | 0.182 | 52.0% |
| LTM | vwap_trend_short_late_vwapslope_lb20 | 308 | 0.106 | 0.141 | 51.9% |
| VOLTAS | vwap_trend_short_late_vwapslope_lb20 | 302 | 0.124 | 0.166 | 51.7% |
| TATAELXSI | vwap_trend_short_late_vwapslope_lb20 | 424 | 0.100 | 0.134 | 51.7% |
| HDBFS | vwap_trend_short_late_vwapslope_lb20 | 62 | 0.011 | 0.015 | 51.6% |
| ETERNAL | vwap_trend_short_late_vwapslope_lb20 | 219 | 0.095 | 0.127 | 51.6% |
| GODREJIND | vwap_trend_short_late_vwapslope_lb20 | 440 | 0.011 | 0.015 | 51.4% |
| TATAPOWER | vwap_trend_short_late_vwapslope_lb20 | 320 | 0.091 | 0.121 | 51.2% |
| NESTLEIND | vwap_trend_short_late_vwapslope_lb20 | 322 | 0.030 | 0.039 | 51.2% |
| TCS | vwap_trend_short_late_vwapslope_lb20 | 295 | 0.070 | 0.094 | 51.2% |
| CIPLA | vwap_trend_short_late_vwapslope_lb20 | 299 | 0.038 | 0.050 | 51.2% |
| BANKINDIA | vwap_trend_short_late_vwapslope_lb20 | 430 | 0.083 | 0.110 | 51.2% |
| TMPV | vwap_trend_short_late_vwapslope_lb20 | 349 | 0.164 | 0.218 | 51.0% |
| ABB | vwap_trend_short_late_vwapslope_lb20 | 359 | 0.026 | 0.034 | 51.0% |
| UNITDSPR | vwap_trend_short_late_vwapslope_lb20 | 299 | 0.060 | 0.080 | 50.8% |
| HDFCAMC | vwap_trend_short_late_vwapslope_lb20 | 372 | 0.043 | 0.057 | 50.8% |
| DIVISLAB | vwap_trend_short_late_vwapslope_lb20 | 317 | 0.010 | 0.014 | 50.8% |
| CRISIL | vwap_trend_short_late_vwapslope_lb20 | 438 | 0.059 | 0.079 | 50.7% |
| MARICO | vwap_trend_short_late_vwapslope_lb20 | 300 | 0.069 | 0.092 | 50.7% |
| SRF | vwap_trend_short_late_vwapslope_lb20 | 344 | 0.111 | 0.148 | 50.6% |
| THERMAX | vwap_trend_short_late_vwapslope_lb20 | 435 | 0.086 | 0.115 | 50.6% |
| APOLLOTYRE | vwap_trend_short_late_vwapslope_lb20 | 352 | 0.137 | 0.182 | 50.6% |
| PIDILITIND | vwap_trend_short_late_vwapslope_lb20 | 323 | 0.091 | 0.122 | 50.5% |
| ACC | vwap_trend_short_late_vwapslope_lb20 | 329 | 0.068 | 0.090 | 50.5% |
| OIL | vwap_trend_short_late_vwapslope_lb20 | 433 | 0.180 | 0.240 | 50.3% |
| LUPIN | vwap_trend_short_late_vwapslope_lb20 | 314 | 0.100 | 0.133 | 50.3% |
| LLOYDSME | vwap_trend_short_late_vwapslope_lb20 | 165 | 0.078 | 0.104 | 50.3% |
| GLAXO | vwap_trend_short_late_vwapslope_lb20 | 495 | 0.078 | 0.103 | 50.3% |
| CANBK | vwap_trend_short_late_vwapslope_lb20 | 336 | 0.102 | 0.136 | 50.3% |
| HAVELLS | vwap_trend_short_late_vwapslope_lb20 | 350 | 0.041 | 0.055 | 50.3% |
| SCHAEFFLER | vwap_trend_short_late_vwapslope_lb20 | 374 | 0.025 | 0.034 | 50.3% |
| PAYTM | vwap_trend_short_late_vwapslope_lb20 | 217 | 0.183 | 0.244 | 50.2% |
| BDL | vwap_trend_short_late_vwapslope_lb20 | 468 | 0.045 | 0.061 | 50.2% |
| KALYANKJIL | vwap_trend_short_late_vwapslope_lb20 | 311 | 0.067 | 0.089 | 50.2% |
| BAJAJ-AUTO | vwap_trend_short_late_vwapslope_lb20 | 325 | 0.091 | 0.121 | 50.2% |
| KEI | vwap_trend_short_late_vwapslope_lb20 | 385 | 0.129 | 0.172 | 50.1% |
| ADANIGREEN | vwap_trend_short_late_vwapslope_lb20 | 401 | 0.108 | 0.144 | 50.1% |
| ENRIN | vwap_trend_short_late_vwapslope_lb20 | 54 | 0.143 | 0.191 | 50.0% |
| DLF | vwap_trend_short_late_vwapslope_lb20 | 332 | 0.093 | 0.124 | 50.0% |
| ZYDUSLIFE | vwap_trend_short_late_vwapslope_lb20 | 322 | 0.093 | 0.124 | 50.0% |
| TMCV | vwap_trend_short_late_vwapslope_lb20 | 40 | 0.064 | 0.085 | 50.0% |
| LICHSGFIN | vwap_trend_short_late_vwapslope_lb20 | 304 | 0.042 | 0.056 | 50.0% |
| POWERINDIA | vwap_trend_short_late_vwapslope_lb20 | 367 | 0.069 | 0.092 | 49.9% |
| SHRIRAMFIN | vwap_trend_short_late_vwapslope_lb20 | 293 | 0.097 | 0.130 | 49.8% |
| DRREDDY | vwap_trend_short_late_vwapslope_lb20 | 304 | 0.017 | 0.022 | 49.7% |
| RVNL | vwap_trend_short_late_vwapslope_lb20 | 429 | 0.053 | 0.071 | 49.7% |
| UPL | vwap_trend_short_late_vwapslope_lb20 | 309 | 0.046 | 0.061 | 49.5% |
| ABCAPITAL | vwap_trend_short_late_vwapslope_lb20 | 396 | 0.059 | 0.078 | 49.5% |
| BRITANNIA | vwap_trend_short_late_vwapslope_lb20 | 338 | 0.013 | 0.018 | 49.4% |
| SBIN | vwap_trend_short_late_vwapslope_lb20 | 308 | 0.085 | 0.114 | 49.4% |
| SWIGGY | vwap_trend_short_late_vwapslope_lb20 | 71 | 0.000 | 0.000 | 49.3% |
| BIOCON | vwap_trend_short_late_vwapslope_lb20 | 335 | 0.043 | 0.057 | 49.3% |
| SUNPHARMA | vwap_trend_short_late_vwapslope_lb20 | 325 | 0.024 | 0.032 | 49.2% |
| OFSS | vwap_trend_short_late_vwapslope_lb20 | 417 | 0.057 | 0.077 | 49.2% |
| POLICYBZR | vwap_trend_short_late_vwapslope_lb20 | 226 | 0.145 | 0.194 | 49.1% |
| ANTHEM | vwap_trend_short_late_vwapslope_lb20 | 55 | 0.114 | 0.153 | 49.1% |
| MANKIND | vwap_trend_short_late_vwapslope_lb20 | 163 | 0.076 | 0.102 | 49.1% |
| UNOMINDA | vwap_trend_short_late_vwapslope_lb20 | 376 | 0.066 | 0.087 | 48.9% |
| LT | vwap_trend_short_late_vwapslope_lb20 | 260 | 0.012 | 0.015 | 48.8% |
| VEDL | vwap_trend_short_late_vwapslope_lb20 | 338 | 0.129 | 0.173 | 48.8% |
| INDIGO | vwap_trend_short_late_vwapslope_lb20 | 338 | 0.025 | 0.034 | 48.8% |
| COALINDIA | vwap_trend_short_late_vwapslope_lb20 | 289 | 0.060 | 0.080 | 48.8% |
| LGEINDIA | vwap_trend_short_late_vwapslope_lb20 | 41 | -0.014 | -0.019 | 48.8% |
| BHEL | vwap_trend_short_late_vwapslope_lb20 | 347 | 0.105 | 0.141 | 48.7% |
| TATACOMM | vwap_trend_short_late_vwapslope_lb20 | 384 | 0.052 | 0.069 | 48.7% |
