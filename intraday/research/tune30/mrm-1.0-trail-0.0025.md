# tune30/mrm-1.0-trail-0.0025 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 1.0x risk (0.750%), give-back trail 0.250% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 250 instruments, 2018-12-04..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_late_vwapslope_lb20 | 81509 | -1.726 | -2.302 | 51.6% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LENSKART | vwap_trend_short_late_vwapslope_lb20 | 39 | 0.369 | 0.492 | 69.2% |
| BAJAJHFL | vwap_trend_short_late_vwapslope_lb20 | 111 | 0.193 | 0.257 | 68.5% |
| JSWINFRA | vwap_trend_short_late_vwapslope_lb20 | 135 | 0.206 | 0.275 | 66.7% |
| LICI | vwap_trend_short_late_vwapslope_lb20 | 203 | 0.133 | 0.177 | 66.5% |
| ITCHOTELS | vwap_trend_short_late_vwapslope_lb20 | 68 | 0.210 | 0.280 | 64.7% |
| AWL | vwap_trend_short_late_vwapslope_lb20 | 222 | 0.072 | 0.096 | 61.3% |
| ENRIN | vwap_trend_short_late_vwapslope_lb20 | 54 | 0.180 | 0.240 | 59.3% |
| NLCINDIA | vwap_trend_short_late_vwapslope_lb20 | 468 | 0.116 | 0.155 | 59.0% |
| JIOFIN | vwap_trend_short_late_vwapslope_lb20 | 134 | 0.139 | 0.185 | 59.0% |
| EXIDEIND | vwap_trend_short_late_vwapslope_lb20 | 360 | 0.090 | 0.121 | 58.9% |
| HAL | vwap_trend_short_late_vwapslope_lb20 | 404 | 0.111 | 0.148 | 58.7% |
| MAZDOCK | vwap_trend_short_late_vwapslope_lb20 | 297 | 0.059 | 0.079 | 58.6% |
| SWIGGY | vwap_trend_short_late_vwapslope_lb20 | 70 | 0.069 | 0.092 | 58.6% |
| IREDA | vwap_trend_short_late_vwapslope_lb20 | 123 | 0.111 | 0.148 | 58.5% |
| COCHINSHIP | vwap_trend_short_late_vwapslope_lb20 | 479 | 0.085 | 0.113 | 58.5% |
| NYKAA | vwap_trend_short_late_vwapslope_lb20 | 221 | 0.100 | 0.133 | 58.4% |
| INDUSINDBK | vwap_trend_short_late_vwapslope_lb20 | 297 | 0.127 | 0.169 | 58.2% |
| DIXON | vwap_trend_short_late_vwapslope_lb20 | 367 | 0.107 | 0.143 | 58.0% |
| HUDCO | vwap_trend_short_late_vwapslope_lb20 | 489 | 0.040 | 0.054 | 57.9% |
| CANBK | vwap_trend_short_late_vwapslope_lb20 | 339 | 0.064 | 0.086 | 57.8% |
| DMART | vwap_trend_short_late_vwapslope_lb20 | 391 | 0.116 | 0.154 | 57.8% |
| 3MINDIA | vwap_trend_short_late_vwapslope_lb20 | 527 | 0.030 | 0.040 | 57.3% |
| APARINDS | vwap_trend_short_late_vwapslope_lb20 | 416 | 0.096 | 0.128 | 57.2% |
| PREMIERENE | vwap_trend_short_late_vwapslope_lb20 | 98 | 0.144 | 0.192 | 57.1% |
| AUROPHARMA | vwap_trend_short_late_vwapslope_lb20 | 306 | 0.090 | 0.120 | 56.9% |
| ATGL | vwap_trend_short_late_vwapslope_lb20 | 408 | 0.045 | 0.060 | 56.9% |
| GROWW | vwap_trend_short_late_vwapslope_lb20 | 37 | 0.154 | 0.205 | 56.8% |
| WAAREEENER | vwap_trend_short_late_vwapslope_lb20 | 83 | 0.106 | 0.141 | 56.6% |
| ITC | vwap_trend_short_late_vwapslope_lb20 | 285 | 0.105 | 0.140 | 56.5% |
| ABB | vwap_trend_short_late_vwapslope_lb20 | 360 | 0.059 | 0.079 | 56.4% |
| HONAUT | vwap_trend_short_late_vwapslope_lb20 | 454 | 0.080 | 0.107 | 56.4% |
| ADANIGREEN | vwap_trend_short_late_vwapslope_lb20 | 401 | 0.045 | 0.060 | 56.4% |
| NAUKRI | vwap_trend_short_late_vwapslope_lb20 | 356 | 0.114 | 0.152 | 56.2% |
| RELIANCE | vwap_trend_short_late_vwapslope_lb20 | 296 | 0.073 | 0.098 | 56.1% |
| LLOYDSME | vwap_trend_short_late_vwapslope_lb20 | 166 | 0.034 | 0.045 | 56.0% |
| ETERNAL | vwap_trend_short_late_vwapslope_lb20 | 218 | 0.069 | 0.091 | 56.0% |
| PNB | vwap_trend_short_late_vwapslope_lb20 | 404 | 0.042 | 0.056 | 55.9% |
| BHEL | vwap_trend_short_late_vwapslope_lb20 | 349 | 0.039 | 0.052 | 55.9% |
| UNITDSPR | vwap_trend_short_late_vwapslope_lb20 | 299 | 0.063 | 0.084 | 55.9% |
| BDL | vwap_trend_short_late_vwapslope_lb20 | 468 | 0.014 | 0.019 | 55.8% |
| TATAELXSI | vwap_trend_short_late_vwapslope_lb20 | 425 | 0.069 | 0.092 | 55.8% |
| JSWENERGY | vwap_trend_short_late_vwapslope_lb20 | 391 | 0.049 | 0.065 | 55.8% |
| CIPLA | vwap_trend_short_late_vwapslope_lb20 | 298 | 0.067 | 0.090 | 55.7% |
| BANKINDIA | vwap_trend_short_late_vwapslope_lb20 | 431 | 0.066 | 0.088 | 55.7% |
| TMPV | vwap_trend_short_late_vwapslope_lb20 | 349 | 0.076 | 0.101 | 55.6% |
| OIL | vwap_trend_short_late_vwapslope_lb20 | 434 | 0.112 | 0.149 | 55.5% |
| HEROMOTOCO | vwap_trend_short_late_vwapslope_lb20 | 310 | 0.101 | 0.134 | 55.5% |
| INDIANB | vwap_trend_short_late_vwapslope_lb20 | 420 | 0.040 | 0.053 | 55.5% |
| HDFCLIFE | vwap_trend_short_late_vwapslope_lb20 | 303 | 0.077 | 0.103 | 55.4% |
| ZYDUSLIFE | vwap_trend_short_late_vwapslope_lb20 | 323 | 0.064 | 0.085 | 55.4% |
| LTM | vwap_trend_short_late_vwapslope_lb20 | 309 | 0.071 | 0.095 | 55.3% |
| CRISIL | vwap_trend_short_late_vwapslope_lb20 | 438 | 0.041 | 0.054 | 55.3% |
| TATACOMM | vwap_trend_short_late_vwapslope_lb20 | 384 | 0.054 | 0.072 | 55.2% |
| UPL | vwap_trend_short_late_vwapslope_lb20 | 308 | 0.059 | 0.078 | 55.2% |
| VOLTAS | vwap_trend_short_late_vwapslope_lb20 | 301 | 0.083 | 0.111 | 55.1% |
| LTTS | vwap_trend_short_late_vwapslope_lb20 | 349 | 0.057 | 0.076 | 55.0% |
| TMCV | vwap_trend_short_late_vwapslope_lb20 | 40 | 0.096 | 0.128 | 55.0% |
| GODREJIND | vwap_trend_short_late_vwapslope_lb20 | 440 | 0.023 | 0.031 | 55.0% |
| AJANTPHARM | vwap_trend_short_late_vwapslope_lb20 | 391 | 0.072 | 0.097 | 55.0% |
| TATAPOWER | vwap_trend_short_late_vwapslope_lb20 | 322 | 0.060 | 0.080 | 55.0% |
| SCHAEFFLER | vwap_trend_short_late_vwapslope_lb20 | 375 | 0.035 | 0.047 | 54.9% |
| LINDEINDIA | vwap_trend_short_late_vwapslope_lb20 | 459 | 0.023 | 0.031 | 54.9% |
| MANKIND | vwap_trend_short_late_vwapslope_lb20 | 164 | 0.035 | 0.046 | 54.9% |
| POLICYBZR | vwap_trend_short_late_vwapslope_lb20 | 226 | 0.092 | 0.122 | 54.9% |
| KALYANKJIL | vwap_trend_short_late_vwapslope_lb20 | 310 | 0.071 | 0.094 | 54.8% |
| SHRIRAMFIN | vwap_trend_short_late_vwapslope_lb20 | 292 | 0.103 | 0.137 | 54.8% |
| LUPIN | vwap_trend_short_late_vwapslope_lb20 | 316 | 0.078 | 0.104 | 54.7% |
| NTPCGREEN | vwap_trend_short_late_vwapslope_lb20 | 95 | 0.079 | 0.105 | 54.7% |
| BLUESTARCO | vwap_trend_short_late_vwapslope_lb20 | 413 | 0.041 | 0.055 | 54.7% |
| APOLLOTYRE | vwap_trend_short_late_vwapslope_lb20 | 351 | 0.090 | 0.120 | 54.7% |
| KEI | vwap_trend_short_late_vwapslope_lb20 | 384 | 0.074 | 0.099 | 54.7% |
| ABCAPITAL | vwap_trend_short_late_vwapslope_lb20 | 394 | 0.014 | 0.019 | 54.6% |
| COALINDIA | vwap_trend_short_late_vwapslope_lb20 | 290 | 0.061 | 0.082 | 54.5% |
| THERMAX | vwap_trend_short_late_vwapslope_lb20 | 435 | 0.059 | 0.079 | 54.5% |
| BIOCON | vwap_trend_short_late_vwapslope_lb20 | 336 | 0.054 | 0.072 | 54.5% |
| HAVELLS | vwap_trend_short_late_vwapslope_lb20 | 351 | 0.040 | 0.053 | 54.4% |
| GODREJPROP | vwap_trend_short_late_vwapslope_lb20 | 331 | 0.063 | 0.084 | 54.4% |
| DLF | vwap_trend_short_late_vwapslope_lb20 | 333 | 0.060 | 0.080 | 54.4% |
| OFSS | vwap_trend_short_late_vwapslope_lb20 | 418 | 0.038 | 0.051 | 54.3% |
| VEDL | vwap_trend_short_late_vwapslope_lb20 | 341 | 0.032 | 0.042 | 54.3% |
| GICRE | vwap_trend_short_late_vwapslope_lb20 | 483 | 0.013 | 0.018 | 54.2% |
| DIVISLAB | vwap_trend_short_late_vwapslope_lb20 | 316 | 0.039 | 0.052 | 54.1% |
| LICHSGFIN | vwap_trend_short_late_vwapslope_lb20 | 304 | 0.061 | 0.082 | 53.9% |
| PAGEIND | vwap_trend_short_late_vwapslope_lb20 | 312 | 0.021 | 0.028 | 53.8% |
| M&M | vwap_trend_short_late_vwapslope_lb20 | 333 | 0.053 | 0.070 | 53.8% |
| HDFCAMC | vwap_trend_short_late_vwapslope_lb20 | 374 | 0.052 | 0.069 | 53.7% |
| UNOMINDA | vwap_trend_short_late_vwapslope_lb20 | 376 | 0.046 | 0.061 | 53.7% |
| RVNL | vwap_trend_short_late_vwapslope_lb20 | 430 | -0.008 | -0.010 | 53.7% |
| TCS | vwap_trend_short_late_vwapslope_lb20 | 296 | 0.054 | 0.073 | 53.7% |
| SOLARINDS | vwap_trend_short_late_vwapslope_lb20 | 345 | 0.058 | 0.077 | 53.6% |
| SRF | vwap_trend_short_late_vwapslope_lb20 | 345 | 0.049 | 0.065 | 53.6% |
| ICICIAMC | vwap_trend_short_late_vwapslope_lb20 | 28 | 0.161 | 0.214 | 53.6% |
| BAJAJ-AUTO | vwap_trend_short_late_vwapslope_lb20 | 323 | 0.081 | 0.107 | 53.6% |
| LAURUSLABS | vwap_trend_short_late_vwapslope_lb20 | 323 | 0.022 | 0.030 | 53.6% |
| VBL | vwap_trend_short_late_vwapslope_lb20 | 382 | -0.205 | -0.274 | 53.4% |
| IRCTC | vwap_trend_short_late_vwapslope_lb20 | 296 | 0.047 | 0.062 | 53.4% |
| ACC | vwap_trend_short_late_vwapslope_lb20 | 330 | 0.050 | 0.067 | 53.3% |
| GLAXO | vwap_trend_short_late_vwapslope_lb20 | 495 | 0.038 | 0.051 | 53.3% |
| RADICO | vwap_trend_short_late_vwapslope_lb20 | 392 | 0.034 | 0.046 | 53.3% |
| PIDILITIND | vwap_trend_short_late_vwapslope_lb20 | 323 | 0.058 | 0.078 | 53.3% |
