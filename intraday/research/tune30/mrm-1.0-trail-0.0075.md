# tune30/mrm-1.0-trail-0.0075 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 1.0x risk (0.750%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 250 instruments, 2018-12-04..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_late_vwapslope_lb20 | 81413 | -1.715 | -2.287 | 48.5% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LENSKART | vwap_trend_short_late_vwapslope_lb20 | 39 | 0.327 | 0.436 | 69.2% |
| LICI | vwap_trend_short_late_vwapslope_lb20 | 200 | 0.176 | 0.235 | 66.5% |
| BAJAJHFL | vwap_trend_short_late_vwapslope_lb20 | 111 | 0.209 | 0.279 | 64.9% |
| ITCHOTELS | vwap_trend_short_late_vwapslope_lb20 | 68 | 0.170 | 0.227 | 61.8% |
| JSWINFRA | vwap_trend_short_late_vwapslope_lb20 | 135 | 0.251 | 0.334 | 61.5% |
| AWL | vwap_trend_short_late_vwapslope_lb20 | 222 | 0.106 | 0.141 | 59.0% |
| JIOFIN | vwap_trend_short_late_vwapslope_lb20 | 134 | 0.197 | 0.263 | 58.2% |
| IREDA | vwap_trend_short_late_vwapslope_lb20 | 122 | 0.135 | 0.179 | 58.2% |
| INDUSINDBK | vwap_trend_short_late_vwapslope_lb20 | 297 | 0.176 | 0.235 | 56.6% |
| HAL | vwap_trend_short_late_vwapslope_lb20 | 404 | 0.131 | 0.174 | 56.4% |
| MAZDOCK | vwap_trend_short_late_vwapslope_lb20 | 296 | 0.097 | 0.129 | 56.1% |
| AUROPHARMA | vwap_trend_short_late_vwapslope_lb20 | 306 | 0.135 | 0.180 | 55.9% |
| NLCINDIA | vwap_trend_short_late_vwapslope_lb20 | 468 | 0.158 | 0.210 | 55.8% |
| SWIGGY | vwap_trend_short_late_vwapslope_lb20 | 70 | 0.050 | 0.067 | 55.7% |
| CIPLA | vwap_trend_short_late_vwapslope_lb20 | 298 | 0.049 | 0.066 | 55.7% |
| DIXON | vwap_trend_short_late_vwapslope_lb20 | 367 | 0.187 | 0.249 | 55.6% |
| ITC | vwap_trend_short_late_vwapslope_lb20 | 283 | 0.120 | 0.159 | 55.5% |
| 3MINDIA | vwap_trend_short_late_vwapslope_lb20 | 527 | 0.062 | 0.083 | 55.4% |
| RELIANCE | vwap_trend_short_late_vwapslope_lb20 | 296 | 0.120 | 0.160 | 55.4% |
| DMART | vwap_trend_short_late_vwapslope_lb20 | 390 | 0.144 | 0.192 | 55.1% |
| WAAREEENER | vwap_trend_short_late_vwapslope_lb20 | 82 | 0.230 | 0.307 | 54.9% |
| EXIDEIND | vwap_trend_short_late_vwapslope_lb20 | 359 | 0.076 | 0.102 | 54.9% |
| UNITDSPR | vwap_trend_short_late_vwapslope_lb20 | 299 | 0.072 | 0.096 | 54.5% |
| HONAUT | vwap_trend_short_late_vwapslope_lb20 | 455 | 0.088 | 0.117 | 54.3% |
| HEROMOTOCO | vwap_trend_short_late_vwapslope_lb20 | 310 | 0.114 | 0.151 | 54.2% |
| COCHINSHIP | vwap_trend_short_late_vwapslope_lb20 | 478 | 0.086 | 0.115 | 54.2% |
| TMPV | vwap_trend_short_late_vwapslope_lb20 | 349 | 0.124 | 0.166 | 53.9% |
| LTTS | vwap_trend_short_late_vwapslope_lb20 | 349 | 0.069 | 0.092 | 53.9% |
| NYKAA | vwap_trend_short_late_vwapslope_lb20 | 221 | 0.130 | 0.173 | 53.8% |
| NTPCGREEN | vwap_trend_short_late_vwapslope_lb20 | 95 | 0.070 | 0.094 | 53.7% |
| VOLTAS | vwap_trend_short_late_vwapslope_lb20 | 302 | 0.112 | 0.150 | 53.6% |
| ICICIAMC | vwap_trend_short_late_vwapslope_lb20 | 28 | 0.130 | 0.173 | 53.6% |
| HDFCLIFE | vwap_trend_short_late_vwapslope_lb20 | 303 | 0.081 | 0.108 | 53.5% |
| LTM | vwap_trend_short_late_vwapslope_lb20 | 309 | 0.099 | 0.131 | 53.4% |
| ABB | vwap_trend_short_late_vwapslope_lb20 | 360 | 0.033 | 0.045 | 53.3% |
| APOLLOTYRE | vwap_trend_short_late_vwapslope_lb20 | 351 | 0.127 | 0.170 | 53.3% |
| HDBFS | vwap_trend_short_late_vwapslope_lb20 | 62 | 0.016 | 0.021 | 53.2% |
| TCS | vwap_trend_short_late_vwapslope_lb20 | 295 | 0.073 | 0.098 | 53.2% |
| ETERNAL | vwap_trend_short_late_vwapslope_lb20 | 218 | 0.084 | 0.112 | 53.2% |
| GODREJPROP | vwap_trend_short_late_vwapslope_lb20 | 331 | 0.091 | 0.121 | 53.2% |
| HAVELLS | vwap_trend_short_late_vwapslope_lb20 | 352 | 0.048 | 0.064 | 53.1% |
| COALINDIA | vwap_trend_short_late_vwapslope_lb20 | 290 | 0.087 | 0.116 | 53.1% |
| PREMIERENE | vwap_trend_short_late_vwapslope_lb20 | 98 | 0.080 | 0.106 | 53.1% |
| LUPIN | vwap_trend_short_late_vwapslope_lb20 | 315 | 0.101 | 0.135 | 53.0% |
| GODREJIND | vwap_trend_short_late_vwapslope_lb20 | 440 | 0.008 | 0.011 | 53.0% |
| TATAELXSI | vwap_trend_short_late_vwapslope_lb20 | 425 | 0.092 | 0.122 | 52.9% |
| ATGL | vwap_trend_short_late_vwapslope_lb20 | 408 | 0.043 | 0.057 | 52.9% |
| ENRIN | vwap_trend_short_late_vwapslope_lb20 | 53 | 0.200 | 0.266 | 52.8% |
| ZYDUSLIFE | vwap_trend_short_late_vwapslope_lb20 | 322 | 0.078 | 0.105 | 52.8% |
| APARINDS | vwap_trend_short_late_vwapslope_lb20 | 415 | 0.125 | 0.166 | 52.8% |
| MANKIND | vwap_trend_short_late_vwapslope_lb20 | 163 | 0.096 | 0.128 | 52.8% |
| HUDCO | vwap_trend_short_late_vwapslope_lb20 | 489 | 0.067 | 0.089 | 52.8% |
| BIOCON | vwap_trend_short_late_vwapslope_lb20 | 336 | 0.035 | 0.046 | 52.7% |
| BLUESTARCO | vwap_trend_short_late_vwapslope_lb20 | 413 | 0.025 | 0.034 | 52.3% |
| ACC | vwap_trend_short_late_vwapslope_lb20 | 329 | 0.072 | 0.096 | 52.3% |
| PNB | vwap_trend_short_late_vwapslope_lb20 | 404 | 0.132 | 0.176 | 52.2% |
| KEI | vwap_trend_short_late_vwapslope_lb20 | 385 | 0.106 | 0.141 | 52.2% |
| ADANIGREEN | vwap_trend_short_late_vwapslope_lb20 | 403 | 0.101 | 0.135 | 52.1% |
| DLF | vwap_trend_short_late_vwapslope_lb20 | 332 | 0.077 | 0.103 | 52.1% |
| SHRIRAMFIN | vwap_trend_short_late_vwapslope_lb20 | 292 | 0.085 | 0.113 | 52.1% |
| IRCTC | vwap_trend_short_late_vwapslope_lb20 | 296 | 0.061 | 0.081 | 52.0% |
| BANKINDIA | vwap_trend_short_late_vwapslope_lb20 | 431 | 0.076 | 0.101 | 52.0% |
| UPL | vwap_trend_short_late_vwapslope_lb20 | 308 | 0.048 | 0.065 | 51.9% |
| SRF | vwap_trend_short_late_vwapslope_lb20 | 343 | 0.086 | 0.114 | 51.9% |
| HDFCAMC | vwap_trend_short_late_vwapslope_lb20 | 372 | 0.036 | 0.048 | 51.9% |
| LINDEINDIA | vwap_trend_short_late_vwapslope_lb20 | 457 | 0.075 | 0.100 | 51.9% |
| BAJAJ-AUTO | vwap_trend_short_late_vwapslope_lb20 | 324 | 0.071 | 0.095 | 51.9% |
| OIL | vwap_trend_short_late_vwapslope_lb20 | 434 | 0.175 | 0.233 | 51.8% |
| POLICYBZR | vwap_trend_short_late_vwapslope_lb20 | 226 | 0.140 | 0.187 | 51.8% |
| SCHAEFFLER | vwap_trend_short_late_vwapslope_lb20 | 375 | 0.006 | 0.008 | 51.7% |
| TATAPOWER | vwap_trend_short_late_vwapslope_lb20 | 321 | 0.090 | 0.119 | 51.7% |
| DRREDDY | vwap_trend_short_late_vwapslope_lb20 | 304 | 0.018 | 0.024 | 51.6% |
| PAGEIND | vwap_trend_short_late_vwapslope_lb20 | 312 | 0.028 | 0.037 | 51.6% |
| LT | vwap_trend_short_late_vwapslope_lb20 | 260 | 0.029 | 0.038 | 51.5% |
| LLOYDSME | vwap_trend_short_late_vwapslope_lb20 | 165 | 0.048 | 0.064 | 51.5% |
| DIVISLAB | vwap_trend_short_late_vwapslope_lb20 | 315 | 0.016 | 0.022 | 51.4% |
| GROWW | vwap_trend_short_late_vwapslope_lb20 | 37 | 0.054 | 0.073 | 51.4% |
| GLAXO | vwap_trend_short_late_vwapslope_lb20 | 495 | 0.064 | 0.085 | 51.3% |
| PIDILITIND | vwap_trend_short_late_vwapslope_lb20 | 322 | 0.064 | 0.086 | 51.2% |
| NESTLEIND | vwap_trend_short_late_vwapslope_lb20 | 322 | 0.037 | 0.049 | 51.2% |
| VEDL | vwap_trend_short_late_vwapslope_lb20 | 338 | 0.105 | 0.140 | 51.2% |
| MARICO | vwap_trend_short_late_vwapslope_lb20 | 299 | 0.050 | 0.067 | 51.2% |
| SBIN | vwap_trend_short_late_vwapslope_lb20 | 307 | 0.080 | 0.107 | 51.1% |
| GODREJCP | vwap_trend_short_late_vwapslope_lb20 | 315 | 0.004 | 0.006 | 51.1% |
| TECHM | vwap_trend_short_late_vwapslope_lb20 | 280 | 0.051 | 0.068 | 51.1% |
| M&M | vwap_trend_short_late_vwapslope_lb20 | 333 | 0.057 | 0.075 | 51.1% |
| BHEL | vwap_trend_short_late_vwapslope_lb20 | 347 | 0.067 | 0.089 | 51.0% |
| LICHSGFIN | vwap_trend_short_late_vwapslope_lb20 | 304 | 0.034 | 0.045 | 51.0% |
| APOLLOHOSP | vwap_trend_short_late_vwapslope_lb20 | 308 | 0.051 | 0.069 | 51.0% |
| PAYTM | vwap_trend_short_late_vwapslope_lb20 | 216 | 0.155 | 0.206 | 50.9% |
| NAUKRI | vwap_trend_short_late_vwapslope_lb20 | 356 | 0.120 | 0.160 | 50.8% |
| INDIANB | vwap_trend_short_late_vwapslope_lb20 | 421 | 0.069 | 0.093 | 50.8% |
| THERMAX | vwap_trend_short_late_vwapslope_lb20 | 435 | 0.060 | 0.080 | 50.8% |
| SUNPHARMA | vwap_trend_short_late_vwapslope_lb20 | 325 | 0.034 | 0.045 | 50.8% |
| BDL | vwap_trend_short_late_vwapslope_lb20 | 467 | 0.033 | 0.045 | 50.7% |
| EICHERMOT | vwap_trend_short_late_vwapslope_lb20 | 335 | 0.016 | 0.022 | 50.7% |
| BANKBARODA | vwap_trend_short_late_vwapslope_lb20 | 353 | 0.019 | 0.025 | 50.7% |
| JKCEMENT | vwap_trend_short_late_vwapslope_lb20 | 365 | 0.061 | 0.081 | 50.7% |
| TATACOMM | vwap_trend_short_late_vwapslope_lb20 | 383 | 0.051 | 0.068 | 50.7% |
| OFSS | vwap_trend_short_late_vwapslope_lb20 | 417 | 0.055 | 0.074 | 50.6% |
