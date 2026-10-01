# tune30/mrm-1.0-trail-0.015 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 1.0x risk (0.750%), give-back trail 1.500% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 250 instruments, 2018-12-04..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_late_vwapslope_lb20 | 81240 | -1.702 | -2.269 | 44.4% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LICI | vwap_trend_short_late_vwapslope_lb20 | 199 | 0.190 | 0.254 | 65.3% |
| BAJAJHFL | vwap_trend_short_late_vwapslope_lb20 | 111 | 0.195 | 0.259 | 64.0% |
| LENSKART | vwap_trend_short_late_vwapslope_lb20 | 39 | 0.298 | 0.397 | 61.5% |
| ITCHOTELS | vwap_trend_short_late_vwapslope_lb20 | 68 | 0.241 | 0.322 | 60.3% |
| AWL | vwap_trend_short_late_vwapslope_lb20 | 222 | 0.161 | 0.215 | 57.7% |
| IREDA | vwap_trend_short_late_vwapslope_lb20 | 121 | 0.201 | 0.268 | 54.5% |
| JIOFIN | vwap_trend_short_late_vwapslope_lb20 | 134 | 0.171 | 0.228 | 53.7% |
| WAAREEENER | vwap_trend_short_late_vwapslope_lb20 | 82 | 0.234 | 0.312 | 53.7% |
| ITC | vwap_trend_short_late_vwapslope_lb20 | 281 | 0.119 | 0.159 | 53.4% |
| RELIANCE | vwap_trend_short_late_vwapslope_lb20 | 296 | 0.126 | 0.167 | 53.4% |
| JSWINFRA | vwap_trend_short_late_vwapslope_lb20 | 135 | 0.201 | 0.268 | 53.3% |
| COCHINSHIP | vwap_trend_short_late_vwapslope_lb20 | 478 | 0.108 | 0.143 | 53.1% |
| NTPCGREEN | vwap_trend_short_late_vwapslope_lb20 | 95 | 0.090 | 0.120 | 52.6% |
| HDFCLIFE | vwap_trend_short_late_vwapslope_lb20 | 303 | 0.138 | 0.184 | 51.8% |
| HAL | vwap_trend_short_late_vwapslope_lb20 | 404 | 0.140 | 0.187 | 51.7% |
| DMART | vwap_trend_short_late_vwapslope_lb20 | 389 | 0.203 | 0.271 | 51.7% |
| HONAUT | vwap_trend_short_late_vwapslope_lb20 | 454 | 0.105 | 0.140 | 51.5% |
| HEROMOTOCO | vwap_trend_short_late_vwapslope_lb20 | 307 | 0.142 | 0.189 | 51.5% |
| 3MINDIA | vwap_trend_short_late_vwapslope_lb20 | 527 | 0.087 | 0.117 | 51.2% |
| EXIDEIND | vwap_trend_short_late_vwapslope_lb20 | 358 | 0.086 | 0.115 | 51.1% |
| MAZDOCK | vwap_trend_short_late_vwapslope_lb20 | 295 | 0.193 | 0.257 | 50.8% |
| TCS | vwap_trend_short_late_vwapslope_lb20 | 295 | 0.069 | 0.092 | 50.8% |
| IRCTC | vwap_trend_short_late_vwapslope_lb20 | 296 | 0.075 | 0.100 | 50.7% |
| LTM | vwap_trend_short_late_vwapslope_lb20 | 308 | 0.113 | 0.151 | 50.6% |
| UNITDSPR | vwap_trend_short_late_vwapslope_lb20 | 299 | 0.066 | 0.088 | 50.5% |
| CIPLA | vwap_trend_short_late_vwapslope_lb20 | 299 | 0.039 | 0.053 | 50.5% |
| INDUSINDBK | vwap_trend_short_late_vwapslope_lb20 | 295 | 0.226 | 0.301 | 50.2% |
| TATAELXSI | vwap_trend_short_late_vwapslope_lb20 | 424 | 0.110 | 0.147 | 50.0% |
| NESTLEIND | vwap_trend_short_late_vwapslope_lb20 | 320 | 0.040 | 0.054 | 50.0% |
| HDBFS | vwap_trend_short_late_vwapslope_lb20 | 62 | 0.006 | 0.008 | 50.0% |
| ACC | vwap_trend_short_late_vwapslope_lb20 | 329 | 0.075 | 0.099 | 49.8% |
| DIVISLAB | vwap_trend_short_late_vwapslope_lb20 | 315 | 0.051 | 0.067 | 49.8% |
| NLCINDIA | vwap_trend_short_late_vwapslope_lb20 | 468 | 0.174 | 0.232 | 49.8% |
| LTTS | vwap_trend_short_late_vwapslope_lb20 | 349 | 0.123 | 0.164 | 49.6% |
| MARICO | vwap_trend_short_late_vwapslope_lb20 | 298 | 0.044 | 0.058 | 49.3% |
| HUDCO | vwap_trend_short_late_vwapslope_lb20 | 489 | 0.066 | 0.088 | 49.3% |
| ATGL | vwap_trend_short_late_vwapslope_lb20 | 408 | 0.119 | 0.158 | 49.3% |
| PNB | vwap_trend_short_late_vwapslope_lb20 | 404 | 0.140 | 0.187 | 49.3% |
| UPL | vwap_trend_short_late_vwapslope_lb20 | 309 | 0.056 | 0.074 | 49.2% |
| DIXON | vwap_trend_short_late_vwapslope_lb20 | 365 | 0.213 | 0.284 | 49.0% |
| BANKINDIA | vwap_trend_short_late_vwapslope_lb20 | 429 | 0.119 | 0.159 | 49.0% |
| PIDILITIND | vwap_trend_short_late_vwapslope_lb20 | 321 | 0.073 | 0.098 | 48.9% |
| AUROPHARMA | vwap_trend_short_late_vwapslope_lb20 | 305 | 0.114 | 0.152 | 48.9% |
| LGEINDIA | vwap_trend_short_late_vwapslope_lb20 | 41 | -0.006 | -0.008 | 48.8% |
| BAJAJ-AUTO | vwap_trend_short_late_vwapslope_lb20 | 322 | 0.090 | 0.121 | 48.8% |
| TATAPOWER | vwap_trend_short_late_vwapslope_lb20 | 320 | 0.113 | 0.150 | 48.8% |
| HAVELLS | vwap_trend_short_late_vwapslope_lb20 | 349 | 0.060 | 0.080 | 48.7% |
| DRREDDY | vwap_trend_short_late_vwapslope_lb20 | 304 | 0.063 | 0.084 | 48.7% |
| APARINDS | vwap_trend_short_late_vwapslope_lb20 | 415 | 0.143 | 0.191 | 48.7% |
| HDFCAMC | vwap_trend_short_late_vwapslope_lb20 | 372 | 0.049 | 0.065 | 48.7% |
| GROWW | vwap_trend_short_late_vwapslope_lb20 | 37 | -0.006 | -0.007 | 48.6% |
| CRISIL | vwap_trend_short_late_vwapslope_lb20 | 438 | 0.052 | 0.069 | 48.6% |
| NAUKRI | vwap_trend_short_late_vwapslope_lb20 | 356 | 0.182 | 0.242 | 48.6% |
| LINDEINDIA | vwap_trend_short_late_vwapslope_lb20 | 457 | 0.101 | 0.135 | 48.6% |
| LUPIN | vwap_trend_short_late_vwapslope_lb20 | 313 | 0.112 | 0.149 | 48.6% |
| GLAXO | vwap_trend_short_late_vwapslope_lb20 | 495 | 0.077 | 0.103 | 48.5% |
| NYKAA | vwap_trend_short_late_vwapslope_lb20 | 221 | 0.138 | 0.184 | 48.4% |
| VOLTAS | vwap_trend_short_late_vwapslope_lb20 | 300 | 0.097 | 0.129 | 48.3% |
| BRITANNIA | vwap_trend_short_late_vwapslope_lb20 | 339 | -0.004 | -0.005 | 48.1% |
| SBIN | vwap_trend_short_late_vwapslope_lb20 | 306 | 0.074 | 0.099 | 48.0% |
| SRF | vwap_trend_short_late_vwapslope_lb20 | 342 | 0.088 | 0.117 | 48.0% |
| DABUR | vwap_trend_short_late_vwapslope_lb20 | 286 | 0.074 | 0.099 | 47.9% |
| SUNPHARMA | vwap_trend_short_late_vwapslope_lb20 | 324 | 0.050 | 0.067 | 47.8% |
| TMPV | vwap_trend_short_late_vwapslope_lb20 | 347 | 0.149 | 0.199 | 47.8% |
| ABB | vwap_trend_short_late_vwapslope_lb20 | 358 | 0.049 | 0.066 | 47.8% |
| RVNL | vwap_trend_short_late_vwapslope_lb20 | 428 | 0.079 | 0.105 | 47.7% |
| AXISBANK | vwap_trend_short_late_vwapslope_lb20 | 286 | -0.006 | -0.008 | 47.6% |
| MANKIND | vwap_trend_short_late_vwapslope_lb20 | 162 | 0.117 | 0.156 | 47.5% |
| ZYDUSLIFE | vwap_trend_short_late_vwapslope_lb20 | 322 | 0.078 | 0.104 | 47.5% |
| LT | vwap_trend_short_late_vwapslope_lb20 | 259 | 0.010 | 0.013 | 47.5% |
| M&M | vwap_trend_short_late_vwapslope_lb20 | 331 | 0.061 | 0.082 | 47.4% |
| APOLLOHOSP | vwap_trend_short_late_vwapslope_lb20 | 306 | 0.057 | 0.075 | 47.4% |
| INDHOTEL | vwap_trend_short_late_vwapslope_lb20 | 366 | 0.151 | 0.201 | 47.3% |
| UNOMINDA | vwap_trend_short_late_vwapslope_lb20 | 373 | 0.095 | 0.127 | 47.2% |
| ENRIN | vwap_trend_short_late_vwapslope_lb20 | 53 | 0.131 | 0.174 | 47.2% |
| TATACONSUM | vwap_trend_short_late_vwapslope_lb20 | 357 | -0.012 | -0.016 | 47.1% |
| GODREJIND | vwap_trend_short_late_vwapslope_lb20 | 440 | 0.010 | 0.013 | 47.0% |
| HCLTECH | vwap_trend_short_late_vwapslope_lb20 | 311 | 0.026 | 0.034 | 46.9% |
| KEI | vwap_trend_short_late_vwapslope_lb20 | 384 | 0.102 | 0.135 | 46.9% |
| BIOCON | vwap_trend_short_late_vwapslope_lb20 | 335 | 0.048 | 0.064 | 46.9% |
| MARUTI | vwap_trend_short_late_vwapslope_lb20 | 314 | 0.045 | 0.061 | 46.8% |
| IOC | vwap_trend_short_late_vwapslope_lb20 | 344 | 0.128 | 0.171 | 46.8% |
| ETERNAL | vwap_trend_short_late_vwapslope_lb20 | 218 | 0.096 | 0.128 | 46.8% |
| KALYANKJIL | vwap_trend_short_late_vwapslope_lb20 | 310 | 0.056 | 0.075 | 46.8% |
| OFSS | vwap_trend_short_late_vwapslope_lb20 | 417 | 0.056 | 0.075 | 46.8% |
| POWERINDIA | vwap_trend_short_late_vwapslope_lb20 | 366 | 0.090 | 0.120 | 46.7% |
| COALINDIA | vwap_trend_short_late_vwapslope_lb20 | 287 | 0.078 | 0.104 | 46.7% |
| BDL | vwap_trend_short_late_vwapslope_lb20 | 467 | 0.029 | 0.039 | 46.7% |
| LLOYDSME | vwap_trend_short_late_vwapslope_lb20 | 165 | 0.094 | 0.125 | 46.7% |
| BERGEPAINT | vwap_trend_short_late_vwapslope_lb20 | 343 | 0.024 | 0.032 | 46.6% |
| RECLTD | vwap_trend_short_late_vwapslope_lb20 | 367 | 0.159 | 0.212 | 46.6% |
| POLICYBZR | vwap_trend_short_late_vwapslope_lb20 | 226 | 0.157 | 0.210 | 46.5% |
| BHARTIARTL | vwap_trend_short_late_vwapslope_lb20 | 323 | 0.024 | 0.032 | 46.4% |
| OIL | vwap_trend_short_late_vwapslope_lb20 | 433 | 0.199 | 0.266 | 46.4% |
| KPRMILL | vwap_trend_short_late_vwapslope_lb20 | 414 | 0.027 | 0.036 | 46.4% |
| POWERGRID | vwap_trend_short_late_vwapslope_lb20 | 300 | 0.023 | 0.031 | 46.3% |
| THERMAX | vwap_trend_short_late_vwapslope_lb20 | 435 | 0.068 | 0.091 | 46.2% |
| LICHSGFIN | vwap_trend_short_late_vwapslope_lb20 | 303 | 0.091 | 0.122 | 46.2% |
| ABCAPITAL | vwap_trend_short_late_vwapslope_lb20 | 394 | 0.043 | 0.057 | 46.2% |
| APOLLOTYRE | vwap_trend_short_late_vwapslope_lb20 | 351 | 0.094 | 0.126 | 46.2% |
