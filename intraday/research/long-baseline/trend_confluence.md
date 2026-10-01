# long-baseline/trend_confluence — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 3.0x risk (2.250%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 250 instruments, 2018-12-04..2026-09-28.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| trend_confluence | 142359 | -1.707 | -2.276 | 42.4% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| GROWW | trend_confluence | 60 | 0.381 | 0.508 | 61.7% |
| ICICIAMC | trend_confluence | 64 | 0.332 | 0.443 | 57.8% |
| AWL | trend_confluence | 352 | 0.149 | 0.199 | 55.1% |
| HEXT | trend_confluence | 114 | 0.166 | 0.221 | 54.4% |
| IRCTC | trend_confluence | 489 | 0.158 | 0.211 | 54.0% |
| IREDA | trend_confluence | 185 | 0.206 | 0.275 | 53.5% |
| TATACAP | trend_confluence | 85 | 0.076 | 0.101 | 51.8% |
| LICI | trend_confluence | 337 | 0.120 | 0.160 | 51.6% |
| BERGEPAINT | trend_confluence | 595 | 0.068 | 0.091 | 51.4% |
| RELIANCE | trend_confluence | 461 | 0.126 | 0.168 | 51.4% |
| JUBLFOOD | trend_confluence | 505 | 0.096 | 0.128 | 50.9% |
| TCS | trend_confluence | 450 | 0.048 | 0.064 | 50.9% |
| SBIN | trend_confluence | 497 | 0.096 | 0.129 | 50.3% |
| BAJAJ-AUTO | trend_confluence | 499 | 0.080 | 0.107 | 50.3% |
| TMPV | trend_confluence | 474 | 0.140 | 0.186 | 50.0% |
| PIDILITIND | trend_confluence | 591 | 0.027 | 0.036 | 49.7% |
| BOSCHLTD | trend_confluence | 641 | 0.073 | 0.098 | 49.5% |
| TATASTEEL | trend_confluence | 483 | 0.132 | 0.176 | 49.3% |
| ITCHOTELS | trend_confluence | 138 | -0.004 | -0.006 | 49.3% |
| HYUNDAI | trend_confluence | 126 | 0.076 | 0.101 | 49.2% |
| LUPIN | trend_confluence | 512 | 0.025 | 0.033 | 49.0% |
| MARICO | trend_confluence | 562 | 0.005 | 0.006 | 48.9% |
| BAJAJHFL | trend_confluence | 178 | 0.106 | 0.142 | 48.9% |
| DRREDDY | trend_confluence | 515 | 0.003 | 0.004 | 48.7% |
| WIPRO | trend_confluence | 569 | 0.063 | 0.084 | 48.7% |
| EICHERMOT | trend_confluence | 528 | 0.062 | 0.082 | 48.7% |
| BALKRISIND | trend_confluence | 623 | 0.087 | 0.116 | 48.6% |
| MAZDOCK | trend_confluence | 459 | 0.108 | 0.145 | 48.6% |
| AXISBANK | trend_confluence | 468 | 0.047 | 0.062 | 48.5% |
| INDUSINDBK | trend_confluence | 501 | 0.064 | 0.086 | 48.5% |
| CIPLA | trend_confluence | 512 | 0.036 | 0.049 | 48.4% |
| POWERINDIA | trend_confluence | 631 | 0.093 | 0.124 | 48.3% |
| SBICARD | trend_confluence | 476 | 0.016 | 0.021 | 48.3% |
| HDBFS | trend_confluence | 114 | 0.014 | 0.019 | 48.2% |
| ETERNAL | trend_confluence | 384 | 0.123 | 0.164 | 48.2% |
| TATACONSUM | trend_confluence | 558 | 0.036 | 0.049 | 48.0% |
| TVSMOTOR | trend_confluence | 534 | 0.053 | 0.070 | 47.9% |
| ITC | trend_confluence | 523 | 0.023 | 0.030 | 47.8% |
| TATAELXSI | trend_confluence | 597 | 0.064 | 0.086 | 47.7% |
| JIOFIN | trend_confluence | 212 | 0.102 | 0.136 | 47.6% |
| BAJAJFINSV | trend_confluence | 464 | 0.067 | 0.089 | 47.6% |
| GRASIM | trend_confluence | 505 | 0.058 | 0.077 | 47.5% |
| HEROMOTOCO | trend_confluence | 484 | 0.047 | 0.063 | 47.5% |
| APOLLOHOSP | trend_confluence | 527 | 0.020 | 0.026 | 47.4% |
| INDIGO | trend_confluence | 546 | 0.055 | 0.073 | 47.3% |
| ACC | trend_confluence | 591 | 0.046 | 0.062 | 47.2% |
| ADANIENT | trend_confluence | 498 | 0.139 | 0.186 | 47.2% |
| COLPAL | trend_confluence | 551 | -0.016 | -0.021 | 47.2% |
| PAYTM | trend_confluence | 318 | 0.208 | 0.277 | 47.2% |
| NYKAA | trend_confluence | 344 | 0.069 | 0.093 | 47.1% |
| LTM | trend_confluence | 544 | 0.021 | 0.029 | 47.1% |
| TATAPOWER | trend_confluence | 583 | 0.078 | 0.104 | 47.0% |
| NTPCGREEN | trend_confluence | 162 | 0.080 | 0.107 | 46.9% |
| TITAN | trend_confluence | 480 | 0.004 | 0.006 | 46.9% |
| APOLLOTYRE | trend_confluence | 591 | 0.050 | 0.067 | 46.9% |
| GAIL | trend_confluence | 566 | 0.070 | 0.093 | 46.8% |
| HDFCAMC | trend_confluence | 643 | 0.056 | 0.074 | 46.8% |
| MANKIND | trend_confluence | 276 | 0.060 | 0.080 | 46.7% |
| ANTHEM | trend_confluence | 122 | 0.060 | 0.081 | 46.7% |
| LT | trend_confluence | 457 | -0.011 | -0.015 | 46.6% |
| DIVISLAB | trend_confluence | 498 | 0.022 | 0.029 | 46.6% |
| ATGL | trend_confluence | 674 | 0.065 | 0.086 | 46.4% |
| AUROPHARMA | trend_confluence | 491 | 0.015 | 0.021 | 46.4% |
| SUNPHARMA | trend_confluence | 515 | 0.009 | 0.012 | 46.4% |
| JSWSTEEL | trend_confluence | 567 | 0.029 | 0.039 | 46.4% |
| DLF | trend_confluence | 478 | 0.024 | 0.032 | 46.2% |
| BAJAJHLDNG | trend_confluence | 799 | 0.081 | 0.108 | 46.2% |
| CHOLAFIN | trend_confluence | 520 | 0.041 | 0.054 | 46.2% |
| TORNTPHARM | trend_confluence | 585 | -0.003 | -0.004 | 46.2% |
| SHREECEM | trend_confluence | 631 | 0.007 | 0.009 | 46.1% |
| SIEMENS | trend_confluence | 517 | 0.044 | 0.058 | 46.0% |
| GODREJCP | trend_confluence | 554 | 0.005 | 0.006 | 46.0% |
| MARUTI | trend_confluence | 522 | 0.036 | 0.048 | 46.0% |
| PAGEIND | trend_confluence | 596 | 0.035 | 0.047 | 46.0% |
| OBEROIRLTY | trend_confluence | 631 | 0.061 | 0.082 | 46.0% |
| DABUR | trend_confluence | 599 | 0.013 | 0.018 | 45.9% |
| ICICIPRULI | trend_confluence | 565 | 0.023 | 0.031 | 45.8% |
| HINDZINC | trend_confluence | 707 | 0.046 | 0.061 | 45.8% |
| PHOENIXLTD | trend_confluence | 653 | 0.057 | 0.076 | 45.8% |
| ASTRAL | trend_confluence | 625 | 0.029 | 0.038 | 45.8% |
| KALYANKJIL | trend_confluence | 455 | 0.051 | 0.067 | 45.7% |
| 3MINDIA | trend_confluence | 910 | 0.016 | 0.021 | 45.7% |
| TORNTPOWER | trend_confluence | 633 | 0.030 | 0.041 | 45.7% |
| BRITANNIA | trend_confluence | 561 | 0.006 | 0.008 | 45.6% |
| ASIANPAINT | trend_confluence | 469 | 0.003 | 0.004 | 45.6% |
| ABB | trend_confluence | 601 | 0.062 | 0.083 | 45.6% |
| BHARATFORG | trend_confluence | 549 | 0.052 | 0.070 | 45.5% |
| MEDANTA | trend_confluence | 323 | 0.049 | 0.065 | 45.5% |
| KOTAKBANK | trend_confluence | 517 | -0.007 | -0.009 | 45.5% |
| UPL | trend_confluence | 537 | 0.008 | 0.011 | 45.4% |
| HINDUNILVR | trend_confluence | 467 | -0.018 | -0.023 | 45.4% |
| HCLTECH | trend_confluence | 525 | -0.011 | -0.014 | 45.1% |
| MUTHOOTFIN | trend_confluence | 543 | -0.052 | -0.069 | 45.1% |
| HONAUT | trend_confluence | 869 | 0.030 | 0.040 | 45.1% |
| HAVELLS | trend_confluence | 541 | -0.004 | -0.006 | 45.1% |
| MFSL | trend_confluence | 570 | 0.023 | 0.030 | 45.1% |
| M&M | trend_confluence | 488 | 0.027 | 0.036 | 45.1% |
| KEI | trend_confluence | 720 | 0.073 | 0.098 | 45.0% |
| SHRIRAMFIN | trend_confluence | 479 | 0.060 | 0.081 | 44.9% |
| MAXHEALTH | trend_confluence | 488 | 0.104 | 0.138 | 44.9% |
