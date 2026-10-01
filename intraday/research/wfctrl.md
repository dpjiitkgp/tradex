# wfctrl — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 3.0x risk (2.250%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 249 instruments, 2018-11-09..2026-09-30.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| random_short_s1 | 92931 | 0.030 | 0.040 | 44.9% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LICI | random_short_s1 | 242 | 0.065 | 0.087 | 54.5% |
| AWL | random_short_s1 | 236 | 0.165 | 0.220 | 53.8% |
| ABBOTINDIA | random_short_s1 | 407 | 0.133 | 0.177 | 53.8% |
| NLCINDIA | random_short_s1 | 389 | 0.259 | 0.345 | 53.7% |
| JIOFIN | random_short_s1 | 149 | 0.140 | 0.187 | 53.7% |
| HDBFS | random_short_s1 | 74 | 0.085 | 0.113 | 52.7% |
| ITC | random_short_s1 | 441 | 0.035 | 0.047 | 52.2% |
| ITCHOTELS | random_short_s1 | 94 | 0.075 | 0.100 | 52.1% |
| HONAUT | random_short_s1 | 367 | 0.090 | 0.120 | 51.8% |
| BAJAJHFL | random_short_s1 | 116 | 0.006 | 0.008 | 51.7% |
| VMM | random_short_s1 | 99 | 0.129 | 0.172 | 51.5% |
| TMPV | random_short_s1 | 382 | 0.173 | 0.231 | 51.3% |
| TATAELXSI | random_short_s1 | 429 | 0.029 | 0.039 | 51.3% |
| HAL | random_short_s1 | 412 | 0.121 | 0.161 | 51.2% |
| ACC | random_short_s1 | 417 | 0.154 | 0.205 | 51.1% |
| JSWINFRA | random_short_s1 | 139 | 0.135 | 0.180 | 51.1% |
| WAAREEENER | random_short_s1 | 108 | 0.217 | 0.289 | 50.9% |
| HUDCO | random_short_s1 | 413 | 0.112 | 0.150 | 50.8% |
| NTPC | random_short_s1 | 412 | 0.099 | 0.132 | 50.2% |
| HINDUNILVR | random_short_s1 | 411 | 0.008 | 0.010 | 50.1% |
| BANKINDIA | random_short_s1 | 422 | 0.166 | 0.221 | 50.0% |
| APOLLOHOSP | random_short_s1 | 404 | 0.117 | 0.156 | 50.0% |
| DABUR | random_short_s1 | 435 | 0.039 | 0.052 | 49.7% |
| ZYDUSLIFE | random_short_s1 | 415 | 0.088 | 0.118 | 49.6% |
| HDFCLIFE | random_short_s1 | 413 | 0.096 | 0.128 | 49.6% |
| PAGEIND | random_short_s1 | 378 | 0.066 | 0.088 | 49.5% |
| COLPAL | random_short_s1 | 426 | 0.019 | 0.025 | 49.3% |
| BOSCHLTD | random_short_s1 | 420 | 0.031 | 0.041 | 49.3% |
| COCHINSHIP | random_short_s1 | 437 | 0.081 | 0.109 | 49.2% |
| ONGC | random_short_s1 | 414 | 0.046 | 0.061 | 49.0% |
| OIL | random_short_s1 | 427 | 0.165 | 0.220 | 48.9% |
| GICRE | random_short_s1 | 427 | 0.093 | 0.124 | 48.9% |
| BANKBARODA | random_short_s1 | 409 | 0.244 | 0.325 | 48.9% |
| UBL | random_short_s1 | 389 | 0.066 | 0.088 | 48.8% |
| IRCTC | random_short_s1 | 385 | -0.003 | -0.004 | 48.8% |
| DMART | random_short_s1 | 459 | 0.054 | 0.073 | 48.8% |
| NHPC | random_short_s1 | 433 | 0.105 | 0.140 | 48.7% |
| PAYTM | random_short_s1 | 271 | 0.136 | 0.181 | 48.7% |
| COALINDIA | random_short_s1 | 421 | 0.047 | 0.063 | 48.7% |
| BHARTIARTL | random_short_s1 | 419 | 0.052 | 0.069 | 48.7% |
| BRITANNIA | random_short_s1 | 432 | -0.009 | -0.012 | 48.6% |
| M&M | random_short_s1 | 430 | 0.110 | 0.147 | 48.6% |
| IRFC | random_short_s1 | 303 | 0.032 | 0.042 | 48.5% |
| HINDZINC | random_short_s1 | 429 | 0.117 | 0.157 | 48.5% |
| NTPCGREEN | random_short_s1 | 99 | 0.033 | 0.045 | 48.5% |
| KEI | random_short_s1 | 417 | 0.114 | 0.152 | 48.4% |
| ANTHEM | random_short_s1 | 62 | 0.016 | 0.021 | 48.4% |
| INDUSTOWER | random_short_s1 | 428 | 0.074 | 0.099 | 48.4% |
| BAJAJHLDNG | random_short_s1 | 408 | 0.106 | 0.142 | 48.3% |
| BAJAJ-AUTO | random_short_s1 | 406 | 0.043 | 0.057 | 48.3% |
| GLAXO | random_short_s1 | 407 | 0.025 | 0.033 | 48.2% |
| JKCEMENT | random_short_s1 | 405 | 0.098 | 0.130 | 48.1% |
| TORNTPHARM | random_short_s1 | 435 | 0.018 | 0.024 | 48.0% |
| BIOCON | random_short_s1 | 398 | 0.074 | 0.098 | 48.0% |
| LT | random_short_s1 | 398 | 0.039 | 0.052 | 48.0% |
| NIACL | random_short_s1 | 436 | 0.054 | 0.072 | 47.9% |
| BDL | random_short_s1 | 420 | 0.090 | 0.120 | 47.9% |
| TCS | random_short_s1 | 387 | 0.008 | 0.011 | 47.8% |
| ICICIAMC | random_short_s1 | 44 | 0.180 | 0.240 | 47.7% |
| SCHAEFFLER | random_short_s1 | 434 | 0.066 | 0.088 | 47.7% |
| MARUTI | random_short_s1 | 433 | 0.027 | 0.037 | 47.6% |
| UPL | random_short_s1 | 387 | 0.088 | 0.117 | 47.5% |
| JUBLFOOD | random_short_s1 | 406 | 0.073 | 0.097 | 47.5% |
| SJVN | random_short_s1 | 419 | 0.018 | 0.025 | 47.5% |
| TECHM | random_short_s1 | 394 | 0.040 | 0.053 | 47.5% |
| RVNL | random_short_s1 | 408 | 0.008 | 0.010 | 47.3% |
| NESTLEIND | random_short_s1 | 402 | -0.015 | -0.020 | 47.3% |
| DIVISLAB | random_short_s1 | 388 | 0.004 | 0.005 | 47.2% |
| BPCL | random_short_s1 | 416 | 0.040 | 0.054 | 47.1% |
| ADANIPORTS | random_short_s1 | 428 | 0.117 | 0.156 | 47.0% |
| LINDEINDIA | random_short_s1 | 411 | 0.098 | 0.131 | 47.0% |
| UNIONBANK | random_short_s1 | 426 | 0.127 | 0.169 | 46.9% |
| ICICIBANK | random_short_s1 | 427 | -0.001 | -0.001 | 46.8% |
| MANKIND | random_short_s1 | 173 | 0.075 | 0.100 | 46.8% |
| BAJFINANCE | random_short_s1 | 408 | 0.001 | 0.002 | 46.8% |
| APLAPOLLO | random_short_s1 | 405 | 0.036 | 0.048 | 46.7% |
| POWERGRID | random_short_s1 | 410 | 0.024 | 0.032 | 46.6% |
| PIDILITIND | random_short_s1 | 419 | 0.010 | 0.014 | 46.5% |
| RELIANCE | random_short_s1 | 430 | 0.029 | 0.038 | 46.5% |
| MAHABANK | random_short_s1 | 428 | 0.052 | 0.069 | 46.5% |
| LAURUSLABS | random_short_s1 | 413 | 0.123 | 0.164 | 46.5% |
| EXIDEIND | random_short_s1 | 424 | 0.078 | 0.104 | 46.5% |
| MAZDOCK | random_short_s1 | 308 | 0.014 | 0.018 | 46.4% |
| TVSMOTOR | random_short_s1 | 433 | 0.018 | 0.025 | 46.4% |
| ULTRACEMCO | random_short_s1 | 414 | 0.067 | 0.089 | 46.4% |
| GRASIM | random_short_s1 | 425 | 0.041 | 0.055 | 46.4% |
| HEROMOTOCO | random_short_s1 | 393 | 0.025 | 0.033 | 46.3% |
| ASIANPAINT | random_short_s1 | 378 | -0.043 | -0.057 | 46.3% |
| LICHSGFIN | random_short_s1 | 443 | 0.035 | 0.047 | 46.3% |
| 3MINDIA | random_short_s1 | 402 | 0.020 | 0.026 | 46.3% |
| INDHOTEL | random_short_s1 | 405 | 0.063 | 0.083 | 46.2% |
| SBICARD | random_short_s1 | 351 | -0.013 | -0.017 | 46.2% |
| SAIL | random_short_s1 | 441 | 0.134 | 0.179 | 46.0% |
| NYKAA | random_short_s1 | 263 | 0.028 | 0.037 | 46.0% |
| TIINDIA | random_short_s1 | 398 | 0.001 | 0.002 | 46.0% |
| EICHERMOT | random_short_s1 | 433 | 0.025 | 0.033 | 46.0% |
| PETRONET | random_short_s1 | 383 | 0.006 | 0.008 | 46.0% |
| UNITDSPR | random_short_s1 | 420 | 0.033 | 0.044 | 46.0% |
| ATGL | random_short_s1 | 412 | 0.128 | 0.170 | 45.9% |
| HCLTECH | random_short_s1 | 407 | -0.014 | -0.019 | 45.7% |
