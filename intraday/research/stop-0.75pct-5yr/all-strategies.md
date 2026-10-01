# stop-0.75pct-5yr/all-strategies — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 3.0x risk (2.250%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 165 instruments, 2021-03-23..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| ma_alignment | 392537 | 0.029 | 0.038 | 43.2% |
| ma_alignment | 426081 | 0.022 | 0.029 | 42.2% |
| ma_alignment | 384316 | 0.029 | 0.039 | 43.4% |
| momentum_breakout | 329658 | -0.009 | -0.012 | 41.9% |
| orb | 82997 | -0.001 | -0.001 | 37.7% |
| strong_rally | 82141 | 0.005 | 0.007 | 37.5% |
| strong_rally | 223862 | 0.016 | 0.021 | 40.4% |
| trend_confluence | 69032 | 0.014 | 0.018 | 44.0% |
| trend_pullback | 395685 | -0.003 | -0.004 | 42.1% |
| vwap_reversion | 218765 | -0.024 | -0.032 | 40.2% |
| vwap_trend | 152499 | 0.009 | 0.012 | 41.6% |
| vwap_trend | 66653 | 0.033 | 0.044 | 45.3% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LICI | vwap_trend | 312 | 0.136 | 0.182 | 59.0% |
| AWL | vwap_trend | 342 | 0.151 | 0.201 | 57.9% |
| AWL | trend_confluence | 352 | 0.149 | 0.199 | 55.1% |
| ITC | vwap_trend | 346 | 0.099 | 0.131 | 53.5% |
| COCHINSHIP | vwap_trend | 482 | 0.116 | 0.155 | 53.3% |
| IRCTC | trend_confluence | 371 | 0.102 | 0.137 | 52.6% |
| EXIDEIND | vwap_trend | 395 | 0.084 | 0.111 | 51.9% |
| LICI | trend_confluence | 337 | 0.120 | 0.160 | 51.6% |
| BERGEPAINT | trend_confluence | 436 | 0.051 | 0.068 | 51.4% |
| LINDEINDIA | vwap_trend | 464 | 0.123 | 0.164 | 51.1% |
| ADANIENT | trend_confluence | 320 | 0.167 | 0.222 | 50.9% |
| 3MINDIA | vwap_trend | 528 | 0.054 | 0.073 | 50.8% |
| JUBLFOOD | trend_confluence | 366 | 0.084 | 0.112 | 50.5% |
| AXISBANK | trend_confluence | 328 | 0.036 | 0.048 | 50.3% |
| CIPLA | trend_confluence | 356 | 0.031 | 0.041 | 50.3% |
| HAVELLS | vwap_trend | 416 | 0.047 | 0.062 | 50.2% |
| INFY | vwap_trend | 313 | 0.019 | 0.026 | 50.2% |
| HINDUNILVR | ma_alignment | 1979 | 0.037 | 0.049 | 50.0% |
| COALINDIA | vwap_trend | 370 | 0.091 | 0.122 | 50.0% |
| INDUSINDBK | trend_confluence | 350 | 0.075 | 0.100 | 50.0% |
| BAJAJ-AUTO | trend_confluence | 352 | 0.060 | 0.080 | 50.0% |
| HONAUT | vwap_trend | 475 | 0.039 | 0.052 | 49.9% |
| IOC | vwap_trend | 408 | 0.093 | 0.124 | 49.8% |
| MARICO | trend_confluence | 392 | -0.010 | -0.014 | 49.7% |
| HONAUT | ma_alignment | 1998 | 0.084 | 0.112 | 49.7% |
| NLCINDIA | vwap_trend | 469 | 0.154 | 0.206 | 49.7% |
| HUDCO | vwap_trend | 465 | 0.092 | 0.123 | 49.7% |
| DRREDDY | trend_confluence | 363 | 0.003 | 0.003 | 49.6% |
| LUPIN | vwap_trend | 361 | 0.089 | 0.118 | 49.6% |
| HDFCBANK | orb | 416 | 0.081 | 0.108 | 49.5% |
| ASTRAL | trend_confluence | 410 | 0.048 | 0.064 | 49.5% |
| KPITTECH | vwap_trend | 406 | 0.152 | 0.203 | 49.5% |
| BAJAJ-AUTO | vwap_trend | 386 | 0.051 | 0.068 | 49.5% |
| DIVISLAB | vwap_trend | 370 | 0.042 | 0.056 | 49.5% |
| LICI | ma_alignment | 1606 | 0.094 | 0.125 | 49.4% |
| LTM | vwap_trend | 363 | 0.094 | 0.125 | 49.3% |
| HDFCLIFE | vwap_trend | 339 | 0.053 | 0.071 | 49.3% |
| HINDUNILVR | strong_rally | 128 | 0.145 | 0.193 | 49.2% |
| HINDUNILVR | ma_alignment | 1892 | 0.030 | 0.040 | 49.2% |
| MAZDOCK | trend_confluence | 409 | 0.126 | 0.168 | 49.1% |
| BOSCHLTD | trend_confluence | 452 | 0.056 | 0.075 | 49.1% |
| ABBOTINDIA | strong_rally | 224 | 0.081 | 0.107 | 49.1% |
| ATGL | vwap_trend | 442 | 0.106 | 0.141 | 49.1% |
| INDIGO | trend_confluence | 380 | 0.032 | 0.043 | 48.9% |
| GLAXO | vwap_trend | 528 | 0.078 | 0.104 | 48.9% |
| GODFRYPHLP | vwap_trend | 477 | 0.138 | 0.184 | 48.8% |
| KALYANKJIL | vwap_trend | 484 | 0.137 | 0.183 | 48.8% |
| MARICO | vwap_trend | 378 | -0.006 | -0.008 | 48.7% |
| AXISBANK | vwap_trend | 372 | 0.025 | 0.034 | 48.7% |
| ASIANPAINT | vwap_trend | 362 | 0.064 | 0.085 | 48.6% |
| ITC | vwap_trend | 811 | 0.007 | 0.009 | 48.6% |
| LUPIN | trend_confluence | 371 | 0.027 | 0.037 | 48.5% |
| LICI | ma_alignment | 1625 | 0.093 | 0.124 | 48.5% |
| APOLLOHOSP | ma_alignment | 2023 | 0.058 | 0.078 | 48.4% |
| HINDUNILVR | strong_rally | 698 | 0.048 | 0.063 | 48.4% |
| HDFCBANK | strong_rally | 93 | 0.130 | 0.174 | 48.4% |
| BAJAJ-AUTO | vwap_trend | 877 | 0.033 | 0.044 | 48.3% |
| HAL | vwap_trend | 393 | 0.110 | 0.146 | 48.3% |
| BERGEPAINT | vwap_trend | 422 | 0.025 | 0.033 | 48.3% |
| AXISBANK | ma_alignment | 1977 | 0.035 | 0.046 | 48.3% |
| APOLLOHOSP | ma_alignment | 2056 | 0.058 | 0.077 | 48.2% |
| ITC | trend_confluence | 367 | 0.032 | 0.043 | 48.2% |
| ASIANPAINT | ma_alignment | 1896 | 0.026 | 0.034 | 48.2% |
| MARUTI | momentum_breakout | 1581 | 0.035 | 0.047 | 48.2% |
| APARINDS | vwap_trend | 471 | 0.183 | 0.244 | 48.2% |
| ETERNAL | trend_confluence | 384 | 0.123 | 0.164 | 48.2% |
| BRITANNIA | ma_alignment | 1926 | 0.017 | 0.022 | 48.1% |
| MARUTI | vwap_trend | 393 | 0.047 | 0.063 | 48.1% |
| ONGC | ma_alignment | 2088 | 0.062 | 0.083 | 48.1% |
| LTM | trend_confluence | 337 | 0.005 | 0.006 | 48.1% |
| GAIL | trend_confluence | 381 | 0.040 | 0.054 | 48.0% |
| ITC | ma_alignment | 2028 | 0.030 | 0.040 | 48.0% |
| CHOLAFIN | trend_confluence | 346 | 0.042 | 0.055 | 48.0% |
| APOLLOTYRE | trend_confluence | 417 | 0.034 | 0.045 | 48.0% |
| ASIANPAINT | momentum_breakout | 1587 | -0.003 | -0.004 | 48.0% |
| DLF | trend_confluence | 317 | 0.027 | 0.036 | 47.9% |
| APOLLOHOSP | vwap_trend | 388 | 0.030 | 0.040 | 47.9% |
| GODREJCP | ma_alignment | 2139 | 0.026 | 0.034 | 47.9% |
| ASIANPAINT | ma_alignment | 1936 | 0.029 | 0.038 | 47.9% |
| LAURUSLABS | trend_confluence | 401 | 0.077 | 0.103 | 47.9% |
| APOLLOHOSP | trend_confluence | 353 | 0.010 | 0.013 | 47.9% |
| BDL | vwap_trend | 458 | 0.089 | 0.119 | 47.8% |
| BIOCON | vwap_trend | 385 | 0.061 | 0.082 | 47.8% |
| HCLTECH | momentum_breakout | 1564 | 0.001 | 0.001 | 47.8% |
| BAJAJ-AUTO | ma_alignment | 1971 | 0.027 | 0.036 | 47.7% |
| BAJAJHLDNG | ma_alignment | 2167 | 0.105 | 0.140 | 47.7% |
| HDFCBANK | vwap_trend | 350 | -0.010 | -0.013 | 47.7% |
| OBEROIRLTY | trend_confluence | 392 | 0.055 | 0.073 | 47.7% |
| MARUTI | trend_pullback | 2003 | 0.022 | 0.029 | 47.7% |
| HINDUNILVR | ma_alignment | 1873 | 0.017 | 0.023 | 47.7% |
| HEROMOTOCO | trend_confluence | 344 | 0.062 | 0.083 | 47.7% |
| HCLTECH | vwap_trend | 881 | 0.028 | 0.038 | 47.7% |
| KEI | trend_confluence | 470 | 0.123 | 0.164 | 47.7% |
| LICHSGFIN | vwap_trend | 340 | 0.054 | 0.072 | 47.6% |
| ACC | vwap_trend | 403 | 0.025 | 0.033 | 47.6% |
| HCLTECH | trend_pullback | 2047 | 0.008 | 0.010 | 47.6% |
| BRITANNIA | ma_alignment | 2098 | 0.012 | 0.016 | 47.6% |
| LT | trend_pullback | 2046 | 0.017 | 0.022 | 47.6% |
| BRITANNIA | trend_pullback | 2105 | 0.008 | 0.011 | 47.6% |
| KPITTECH | trend_confluence | 412 | 0.079 | 0.106 | 47.6% |
