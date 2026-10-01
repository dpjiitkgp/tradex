# confluence-v1 — exit-rule research

Exit rule: risk_pct=0.500% (hard stop), armed at 3.0x risk (1.500%), give-back trail 0.500% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 165 instruments, 2021-03-23..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| trend_confluence | 70955 | 0.001 | 0.003 | 40.3% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| IRCTC | trend_confluence | 382 | 0.078 | 0.155 | 49.2% |
| AWL | trend_confluence | 358 | 0.106 | 0.212 | 48.9% |
| BERGEPAINT | trend_confluence | 442 | 0.048 | 0.096 | 48.2% |
| LICI | trend_confluence | 347 | 0.088 | 0.176 | 48.1% |
| CIPLA | trend_confluence | 363 | 0.030 | 0.060 | 47.7% |
| AXISBANK | trend_confluence | 333 | 0.017 | 0.034 | 47.1% |
| BAJAJ-AUTO | trend_confluence | 361 | 0.050 | 0.099 | 47.1% |
| ADANIENT | trend_confluence | 330 | 0.140 | 0.281 | 47.0% |
| DRREDDY | trend_confluence | 367 | -0.007 | -0.014 | 46.6% |
| MARICO | trend_confluence | 393 | -0.020 | -0.039 | 46.6% |
| INDIGO | trend_confluence | 385 | 0.043 | 0.087 | 46.5% |
| MAZDOCK | trend_confluence | 424 | 0.132 | 0.264 | 45.8% |
| ITC | trend_confluence | 370 | 0.025 | 0.049 | 45.7% |
| HINDUNILVR | trend_confluence | 316 | -0.004 | -0.009 | 45.6% |
| APOLLOTYRE | trend_confluence | 423 | 0.050 | 0.100 | 45.4% |
| HEROMOTOCO | trend_confluence | 353 | 0.050 | 0.101 | 45.3% |
| EICHERMOT | trend_confluence | 373 | 0.017 | 0.034 | 45.3% |
| JUBLFOOD | trend_confluence | 382 | 0.054 | 0.109 | 45.3% |
| MANKIND | trend_confluence | 294 | 0.062 | 0.124 | 45.2% |
| ASTRAL | trend_confluence | 423 | 0.075 | 0.150 | 45.2% |
| CHOLAFIN | trend_confluence | 354 | 0.054 | 0.107 | 44.9% |
| CONCOR | trend_confluence | 391 | 0.018 | 0.035 | 44.8% |
| LTM | trend_confluence | 342 | 0.013 | 0.025 | 44.7% |
| BRITANNIA | trend_confluence | 392 | 0.016 | 0.032 | 44.6% |
| LUPIN | trend_confluence | 381 | 0.006 | 0.012 | 44.6% |
| HDFCAMC | trend_confluence | 430 | 0.053 | 0.105 | 44.4% |
| GODREJCP | trend_confluence | 370 | -0.010 | -0.020 | 44.3% |
| LICHSGFIN | trend_confluence | 404 | 0.030 | 0.059 | 44.3% |
| APOLLOHOSP | trend_confluence | 359 | -0.006 | -0.011 | 44.3% |
| INFY | trend_confluence | 359 | -0.023 | -0.046 | 44.3% |
| M&M | trend_confluence | 357 | 0.021 | 0.041 | 44.3% |
| NYKAA | trend_confluence | 348 | 0.092 | 0.183 | 44.3% |
| COFORGE | trend_confluence | 355 | 0.103 | 0.206 | 44.2% |
| HONAUT | trend_confluence | 606 | 0.035 | 0.070 | 44.2% |
| ICICIBANK | trend_confluence | 376 | -0.015 | -0.031 | 44.1% |
| BALKRISIND | trend_confluence | 451 | 0.052 | 0.104 | 44.1% |
| DLF | trend_confluence | 322 | 0.013 | 0.026 | 44.1% |
| DABUR | trend_confluence | 443 | 0.003 | 0.006 | 44.0% |
| AUBANK | trend_confluence | 360 | 0.062 | 0.124 | 43.9% |
| BOSCHLTD | trend_confluence | 465 | 0.026 | 0.053 | 43.9% |
| INDUSINDBK | trend_confluence | 359 | 0.035 | 0.071 | 43.7% |
| LAURUSLABS | trend_confluence | 414 | 0.065 | 0.130 | 43.7% |
| KEI | trend_confluence | 483 | 0.091 | 0.181 | 43.7% |
| KPITTECH | trend_confluence | 424 | 0.066 | 0.132 | 43.6% |
| MUTHOOTFIN | trend_confluence | 374 | -0.021 | -0.043 | 43.6% |
| HAL | trend_confluence | 409 | 0.041 | 0.082 | 43.5% |
| MFSL | trend_confluence | 403 | 0.038 | 0.076 | 43.4% |
| BAJAJHLDNG | trend_confluence | 554 | 0.072 | 0.144 | 43.3% |
| HAVELLS | trend_confluence | 395 | 0.008 | 0.016 | 43.3% |
| DIVISLAB | trend_confluence | 372 | 0.009 | 0.018 | 43.3% |
| ASIANPAINT | trend_confluence | 361 | -0.010 | -0.019 | 43.2% |
| LT | trend_confluence | 336 | -0.005 | -0.010 | 42.9% |
| MARUTI | trend_confluence | 365 | -0.006 | -0.013 | 42.7% |
| HCLTECH | trend_confluence | 365 | -0.013 | -0.026 | 42.7% |
| HINDZINC | trend_confluence | 509 | 0.040 | 0.080 | 42.6% |
| EXIDEIND | trend_confluence | 397 | 0.018 | 0.036 | 42.6% |
| ETERNAL | trend_confluence | 395 | 0.086 | 0.171 | 42.5% |
| MEDANTA | trend_confluence | 334 | 0.067 | 0.134 | 42.5% |
| 3MINDIA | trend_confluence | 654 | 0.017 | 0.033 | 42.5% |
| GLAXO | trend_confluence | 586 | 0.000 | 0.000 | 42.5% |
| BEL | trend_confluence | 401 | 0.078 | 0.156 | 42.4% |
| OBEROIRLTY | trend_confluence | 399 | 0.017 | 0.034 | 42.4% |
| BDL | trend_confluence | 451 | 0.079 | 0.158 | 42.4% |
| BAJAJFINSV | trend_confluence | 345 | 0.031 | 0.062 | 42.3% |
| JSWSTEEL | trend_confluence | 403 | 0.003 | 0.006 | 42.2% |
| ABBOTINDIA | trend_confluence | 572 | 0.027 | 0.054 | 42.1% |
| LINDEINDIA | trend_confluence | 547 | 0.039 | 0.079 | 42.0% |
| COALINDIA | trend_confluence | 376 | 0.007 | 0.014 | 42.0% |
| HINDALCO | trend_confluence | 338 | -0.006 | -0.011 | 42.0% |
| OIL | trend_confluence | 462 | 0.067 | 0.134 | 42.0% |
| PAGEIND | trend_confluence | 440 | -0.001 | -0.002 | 41.8% |
| BHARATFORG | trend_confluence | 408 | 0.040 | 0.080 | 41.7% |
| GAIL | trend_confluence | 399 | 0.005 | 0.009 | 41.6% |
| ONGC | trend_confluence | 399 | 0.001 | 0.002 | 41.6% |
| KPRMILL | trend_confluence | 570 | 0.042 | 0.085 | 41.6% |
| DMART | trend_confluence | 421 | 0.061 | 0.122 | 41.6% |
| COCHINSHIP | trend_confluence | 504 | 0.052 | 0.105 | 41.5% |
| NATIONALUM | trend_confluence | 379 | 0.019 | 0.038 | 41.4% |
| MAXHEALTH | trend_confluence | 430 | 0.074 | 0.148 | 41.4% |
| ENDURANCE | trend_confluence | 546 | 0.030 | 0.059 | 41.4% |
| BLUESTARCO | trend_confluence | 539 | 0.040 | 0.079 | 41.2% |
| AUROPHARMA | trend_confluence | 374 | -0.016 | -0.032 | 41.2% |
| OFSS | trend_confluence | 443 | 0.014 | 0.028 | 41.1% |
| FORTIS | trend_confluence | 464 | 0.009 | 0.018 | 40.9% |
| DIXON | trend_confluence | 364 | 0.012 | 0.024 | 40.9% |
| ATGL | trend_confluence | 453 | 0.024 | 0.048 | 40.8% |
| AMBUJACEM | trend_confluence | 409 | 0.002 | 0.003 | 40.8% |
| NESTLEIND | trend_confluence | 446 | -0.006 | -0.013 | 40.8% |
| HDFCBANK | trend_confluence | 373 | -0.019 | -0.039 | 40.8% |
| IPCALAB | trend_confluence | 459 | -0.023 | -0.047 | 40.7% |
| GLENMARK | trend_confluence | 405 | -0.024 | -0.049 | 40.7% |
| BANKINDIA | trend_confluence | 464 | 0.064 | 0.128 | 40.7% |
| KOTAKBANK | trend_confluence | 371 | -0.049 | -0.098 | 40.7% |
| KALYANKJIL | trend_confluence | 467 | 0.031 | 0.062 | 40.7% |
| ADANIPORTS | trend_confluence | 322 | 0.049 | 0.098 | 40.7% |
| CRISIL | trend_confluence | 587 | 0.045 | 0.089 | 40.5% |
| MPHASIS | trend_confluence | 370 | 0.013 | 0.025 | 40.5% |
| BIOCON | trend_confluence | 414 | -0.013 | -0.026 | 40.3% |
| ABB | trend_confluence | 362 | 0.028 | 0.056 | 40.3% |
| HDFCLIFE | trend_confluence | 355 | -0.037 | -0.075 | 40.3% |
