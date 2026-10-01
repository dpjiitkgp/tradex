# tune30/mrm-6.0-trail-0.0025 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 6.0x risk (4.500%), give-back trail 0.250% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 250 instruments, 2018-12-04..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_late_vwapslope_lb20 | 81049 | -1.699 | -2.265 | 44.3% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| BAJAJHFL | vwap_trend_short_late_vwapslope_lb20 | 111 | 0.217 | 0.290 | 65.8% |
| LICI | vwap_trend_short_late_vwapslope_lb20 | 199 | 0.182 | 0.243 | 65.3% |
| LENSKART | vwap_trend_short_late_vwapslope_lb20 | 39 | 0.320 | 0.426 | 61.5% |
| ITCHOTELS | vwap_trend_short_late_vwapslope_lb20 | 68 | 0.226 | 0.301 | 60.3% |
| AWL | vwap_trend_short_late_vwapslope_lb20 | 222 | 0.179 | 0.239 | 58.1% |
| NTPCGREEN | vwap_trend_short_late_vwapslope_lb20 | 95 | 0.108 | 0.144 | 54.7% |
| IREDA | vwap_trend_short_late_vwapslope_lb20 | 120 | 0.278 | 0.370 | 54.2% |
| JIOFIN | vwap_trend_short_late_vwapslope_lb20 | 133 | 0.182 | 0.243 | 54.1% |
| RELIANCE | vwap_trend_short_late_vwapslope_lb20 | 296 | 0.132 | 0.176 | 53.7% |
| HAL | vwap_trend_short_late_vwapslope_lb20 | 404 | 0.167 | 0.223 | 53.0% |
| ITC | vwap_trend_short_late_vwapslope_lb20 | 280 | 0.118 | 0.157 | 52.9% |
| JSWINFRA | vwap_trend_short_late_vwapslope_lb20 | 135 | 0.200 | 0.266 | 52.6% |
| COCHINSHIP | vwap_trend_short_late_vwapslope_lb20 | 477 | 0.119 | 0.158 | 52.4% |
| INDUSINDBK | vwap_trend_short_late_vwapslope_lb20 | 293 | 0.247 | 0.329 | 52.2% |
| 3MINDIA | vwap_trend_short_late_vwapslope_lb20 | 527 | 0.096 | 0.129 | 52.0% |
| DMART | vwap_trend_short_late_vwapslope_lb20 | 386 | 0.190 | 0.253 | 51.8% |
| HONAUT | vwap_trend_short_late_vwapslope_lb20 | 454 | 0.101 | 0.134 | 51.8% |
| HDBFS | vwap_trend_short_late_vwapslope_lb20 | 62 | 0.014 | 0.019 | 51.6% |
| HDFCLIFE | vwap_trend_short_late_vwapslope_lb20 | 303 | 0.133 | 0.178 | 51.2% |
| EXIDEIND | vwap_trend_short_late_vwapslope_lb20 | 358 | 0.074 | 0.099 | 51.1% |
| IRCTC | vwap_trend_short_late_vwapslope_lb20 | 296 | 0.079 | 0.105 | 50.7% |
| HEROMOTOCO | vwap_trend_short_late_vwapslope_lb20 | 307 | 0.140 | 0.187 | 50.5% |
| CIPLA | vwap_trend_short_late_vwapslope_lb20 | 299 | 0.028 | 0.037 | 50.2% |
| MAZDOCK | vwap_trend_short_late_vwapslope_lb20 | 296 | 0.194 | 0.258 | 50.0% |
| LTM | vwap_trend_short_late_vwapslope_lb20 | 308 | 0.122 | 0.163 | 50.0% |
| TATAELXSI | vwap_trend_short_late_vwapslope_lb20 | 421 | 0.163 | 0.217 | 49.9% |
| TCS | vwap_trend_short_late_vwapslope_lb20 | 294 | 0.059 | 0.079 | 49.7% |
| NLCINDIA | vwap_trend_short_late_vwapslope_lb20 | 468 | 0.191 | 0.255 | 49.6% |
| LTTS | vwap_trend_short_late_vwapslope_lb20 | 349 | 0.134 | 0.179 | 49.6% |
| NESTLEIND | vwap_trend_short_late_vwapslope_lb20 | 319 | 0.057 | 0.077 | 49.5% |
| DIVISLAB | vwap_trend_short_late_vwapslope_lb20 | 315 | 0.051 | 0.068 | 49.5% |
| ATGL | vwap_trend_short_late_vwapslope_lb20 | 408 | 0.151 | 0.201 | 49.5% |
| UNITDSPR | vwap_trend_short_late_vwapslope_lb20 | 299 | 0.057 | 0.076 | 49.5% |
| HUDCO | vwap_trend_short_late_vwapslope_lb20 | 489 | 0.061 | 0.082 | 49.5% |
| MARICO | vwap_trend_short_late_vwapslope_lb20 | 296 | 0.051 | 0.068 | 49.3% |
| ABB | vwap_trend_short_late_vwapslope_lb20 | 357 | 0.080 | 0.107 | 49.3% |
| ACC | vwap_trend_short_late_vwapslope_lb20 | 329 | 0.083 | 0.110 | 49.2% |
| LLOYDSME | vwap_trend_short_late_vwapslope_lb20 | 165 | 0.116 | 0.154 | 49.1% |
| CRISIL | vwap_trend_short_late_vwapslope_lb20 | 438 | 0.070 | 0.094 | 49.1% |
| HDFCAMC | vwap_trend_short_late_vwapslope_lb20 | 371 | 0.079 | 0.106 | 49.1% |
| UPL | vwap_trend_short_late_vwapslope_lb20 | 308 | 0.074 | 0.099 | 49.0% |
| LINDEINDIA | vwap_trend_short_late_vwapslope_lb20 | 457 | 0.122 | 0.162 | 49.0% |
| AUROPHARMA | vwap_trend_short_late_vwapslope_lb20 | 302 | 0.106 | 0.141 | 49.0% |
| BRITANNIA | vwap_trend_short_late_vwapslope_lb20 | 337 | 0.029 | 0.039 | 49.0% |
| PIDILITIND | vwap_trend_short_late_vwapslope_lb20 | 321 | 0.077 | 0.102 | 48.9% |
| PNB | vwap_trend_short_late_vwapslope_lb20 | 403 | 0.160 | 0.214 | 48.9% |
| NAUKRI | vwap_trend_short_late_vwapslope_lb20 | 356 | 0.191 | 0.255 | 48.9% |
| NYKAA | vwap_trend_short_late_vwapslope_lb20 | 221 | 0.140 | 0.186 | 48.9% |
| WAAREEENER | vwap_trend_short_late_vwapslope_lb20 | 82 | 0.296 | 0.395 | 48.8% |
| LGEINDIA | vwap_trend_short_late_vwapslope_lb20 | 41 | 0.015 | 0.021 | 48.8% |
| ZYDUSLIFE | vwap_trend_short_late_vwapslope_lb20 | 322 | 0.098 | 0.130 | 48.8% |
| LUPIN | vwap_trend_short_late_vwapslope_lb20 | 312 | 0.131 | 0.175 | 48.7% |
| HAVELLS | vwap_trend_short_late_vwapslope_lb20 | 349 | 0.068 | 0.090 | 48.7% |
| SRF | vwap_trend_short_late_vwapslope_lb20 | 339 | 0.101 | 0.134 | 48.7% |
| BAJAJ-AUTO | vwap_trend_short_late_vwapslope_lb20 | 319 | 0.086 | 0.115 | 48.6% |
| DRREDDY | vwap_trend_short_late_vwapslope_lb20 | 303 | 0.040 | 0.053 | 48.5% |
| BANKINDIA | vwap_trend_short_late_vwapslope_lb20 | 427 | 0.150 | 0.200 | 48.5% |
| SUNPHARMA | vwap_trend_short_late_vwapslope_lb20 | 325 | 0.039 | 0.052 | 48.3% |
| TATAPOWER | vwap_trend_short_late_vwapslope_lb20 | 319 | 0.106 | 0.142 | 48.3% |
| SBIN | vwap_trend_short_late_vwapslope_lb20 | 305 | 0.115 | 0.153 | 48.2% |
| APARINDS | vwap_trend_short_late_vwapslope_lb20 | 415 | 0.155 | 0.207 | 48.2% |
| GLAXO | vwap_trend_short_late_vwapslope_lb20 | 495 | 0.082 | 0.109 | 48.1% |
| DIXON | vwap_trend_short_late_vwapslope_lb20 | 364 | 0.217 | 0.289 | 48.1% |
| DABUR | vwap_trend_short_late_vwapslope_lb20 | 286 | 0.078 | 0.104 | 47.9% |
| MANKIND | vwap_trend_short_late_vwapslope_lb20 | 159 | 0.140 | 0.186 | 47.8% |
| BDL | vwap_trend_short_late_vwapslope_lb20 | 467 | 0.091 | 0.121 | 47.8% |
| LT | vwap_trend_short_late_vwapslope_lb20 | 256 | 0.023 | 0.031 | 47.7% |
| VOLTAS | vwap_trend_short_late_vwapslope_lb20 | 296 | 0.109 | 0.146 | 47.6% |
| BIOCON | vwap_trend_short_late_vwapslope_lb20 | 334 | 0.058 | 0.078 | 47.6% |
| TMPV | vwap_trend_short_late_vwapslope_lb20 | 345 | 0.171 | 0.227 | 47.5% |
| RVNL | vwap_trend_short_late_vwapslope_lb20 | 428 | 0.083 | 0.110 | 47.4% |
| APOLLOTYRE | vwap_trend_short_late_vwapslope_lb20 | 351 | 0.111 | 0.148 | 47.3% |
| ANTHEM | vwap_trend_short_late_vwapslope_lb20 | 55 | 0.076 | 0.102 | 47.3% |
| COALINDIA | vwap_trend_short_late_vwapslope_lb20 | 286 | 0.118 | 0.158 | 47.2% |
| GODREJIND | vwap_trend_short_late_vwapslope_lb20 | 440 | 0.025 | 0.033 | 47.0% |
| APOLLOHOSP | vwap_trend_short_late_vwapslope_lb20 | 305 | 0.056 | 0.074 | 46.9% |
| AXISBANK | vwap_trend_short_late_vwapslope_lb20 | 286 | 0.005 | 0.006 | 46.9% |
| TATACONSUM | vwap_trend_short_late_vwapslope_lb20 | 357 | 0.000 | 0.001 | 46.8% |
| ABCAPITAL | vwap_trend_short_late_vwapslope_lb20 | 394 | 0.055 | 0.074 | 46.7% |
| POWERGRID | vwap_trend_short_late_vwapslope_lb20 | 300 | 0.021 | 0.027 | 46.7% |
| OFSS | vwap_trend_short_late_vwapslope_lb20 | 414 | 0.063 | 0.084 | 46.6% |
| KEI | vwap_trend_short_late_vwapslope_lb20 | 384 | 0.147 | 0.196 | 46.6% |
| GICRE | vwap_trend_short_late_vwapslope_lb20 | 483 | 0.089 | 0.119 | 46.6% |
| RECLTD | vwap_trend_short_late_vwapslope_lb20 | 365 | 0.144 | 0.191 | 46.6% |
| M&M | vwap_trend_short_late_vwapslope_lb20 | 331 | 0.059 | 0.078 | 46.5% |
| IOC | vwap_trend_short_late_vwapslope_lb20 | 342 | 0.163 | 0.217 | 46.5% |
| MARUTI | vwap_trend_short_late_vwapslope_lb20 | 312 | 0.040 | 0.054 | 46.5% |
| KALYANKJIL | vwap_trend_short_late_vwapslope_lb20 | 308 | 0.030 | 0.040 | 46.4% |
| BERGEPAINT | vwap_trend_short_late_vwapslope_lb20 | 343 | 0.030 | 0.040 | 46.4% |
| ASIANPAINT | vwap_trend_short_late_vwapslope_lb20 | 298 | 0.027 | 0.036 | 46.3% |
| INFY | vwap_trend_short_late_vwapslope_lb20 | 283 | 0.014 | 0.019 | 46.3% |
| WIPRO | vwap_trend_short_late_vwapslope_lb20 | 335 | 0.061 | 0.082 | 46.3% |
| THERMAX | vwap_trend_short_late_vwapslope_lb20 | 435 | 0.063 | 0.084 | 46.2% |
| LICHSGFIN | vwap_trend_short_late_vwapslope_lb20 | 303 | 0.094 | 0.125 | 46.2% |
| PAGEIND | vwap_trend_short_late_vwapslope_lb20 | 312 | 0.047 | 0.062 | 46.2% |
| TECHM | vwap_trend_short_late_vwapslope_lb20 | 280 | 0.067 | 0.090 | 46.1% |
| INDHOTEL | vwap_trend_short_late_vwapslope_lb20 | 369 | 0.151 | 0.201 | 46.1% |
| EICHERMOT | vwap_trend_short_late_vwapslope_lb20 | 335 | 0.022 | 0.029 | 46.0% |
| HDFCBANK | vwap_trend_short_late_vwapslope_lb20 | 309 | 0.022 | 0.030 | 46.0% |
| OIL | vwap_trend_short_late_vwapslope_lb20 | 431 | 0.221 | 0.295 | 45.9% |
