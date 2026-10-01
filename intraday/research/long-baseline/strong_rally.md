# long-baseline/strong_rally — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 3.0x risk (2.250%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 250 instruments, 2018-12-04..2026-09-28.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| strong_rally | 186798 | -0.063 | -0.084 | 36.5% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| NTPCGREEN | strong_rally | 134 | 0.242 | 0.322 | 45.5% |
| HDFCBANK | strong_rally | 222 | 0.144 | 0.192 | 45.5% |
| ASIANPAINT | strong_rally | 318 | 0.118 | 0.157 | 45.3% |
| WIPRO | strong_rally | 392 | 0.112 | 0.149 | 45.2% |
| JIOFIN | strong_rally | 213 | 0.191 | 0.254 | 45.1% |
| ABBOTINDIA | strong_rally | 377 | 0.070 | 0.093 | 44.6% |
| TATACAP | strong_rally | 45 | -0.121 | -0.161 | 44.4% |
| MARUTI | strong_rally | 410 | 0.130 | 0.173 | 44.1% |
| POLICYBZR | strong_rally | 758 | 0.202 | 0.269 | 43.9% |
| BAJAJFINSV | strong_rally | 583 | 0.148 | 0.197 | 43.4% |
| LT | strong_rally | 333 | 0.149 | 0.199 | 43.2% |
| HINDUNILVR | strong_rally | 250 | 0.107 | 0.142 | 43.2% |
| DABUR | strong_rally | 301 | 0.024 | 0.032 | 43.2% |
| LICI | strong_rally | 224 | 0.025 | 0.034 | 42.9% |
| POWERGRID | strong_rally | 456 | 0.088 | 0.117 | 42.8% |
| ULTRACEMCO | strong_rally | 393 | 0.093 | 0.123 | 42.7% |
| INDHOTEL | strong_rally | 817 | 0.149 | 0.199 | 42.2% |
| HEROMOTOCO | strong_rally | 590 | 0.099 | 0.132 | 42.2% |
| ICICIBANK | strong_rally | 386 | 0.051 | 0.068 | 42.0% |
| APOLLOHOSP | strong_rally | 641 | 0.052 | 0.070 | 42.0% |
| PAYTM | strong_rally | 766 | 0.185 | 0.246 | 41.9% |
| BAJAJHFL | strong_rally | 137 | 0.077 | 0.103 | 41.6% |
| ONGC | strong_rally | 663 | 0.075 | 0.100 | 41.3% |
| MPHASIS | strong_rally | 875 | 0.061 | 0.082 | 41.3% |
| AUBANK | strong_rally | 793 | 0.070 | 0.094 | 41.2% |
| TMPV | strong_rally | 850 | 0.144 | 0.193 | 41.2% |
| GROWW | strong_rally | 146 | 0.176 | 0.235 | 41.1% |
| HINDALCO | strong_rally | 768 | 0.025 | 0.034 | 41.0% |
| INDUSINDBK | strong_rally | 850 | 0.108 | 0.144 | 40.8% |
| TITAN | strong_rally | 461 | 0.055 | 0.073 | 40.8% |
| HDFCAMC | strong_rally | 574 | 0.037 | 0.049 | 40.8% |
| PAGEIND | strong_rally | 535 | 0.025 | 0.033 | 40.7% |
| MEDANTA | strong_rally | 378 | 0.027 | 0.035 | 40.7% |
| M&M | strong_rally | 619 | 0.095 | 0.127 | 40.7% |
| BOSCHLTD | strong_rally | 504 | 0.054 | 0.072 | 40.7% |
| PETRONET | strong_rally | 512 | 0.054 | 0.071 | 40.6% |
| BPCL | strong_rally | 791 | 0.103 | 0.138 | 40.5% |
| ANTHEM | strong_rally | 94 | 0.081 | 0.108 | 40.4% |
| SBICARD | strong_rally | 416 | 0.045 | 0.059 | 40.4% |
| INDIGO | strong_rally | 738 | 0.123 | 0.164 | 40.4% |
| SIEMENS | strong_rally | 679 | 0.069 | 0.091 | 40.4% |
| ENRIN | strong_rally | 119 | 0.131 | 0.175 | 40.3% |
| ITCHOTELS | strong_rally | 119 | 0.034 | 0.045 | 40.3% |
| VOLTAS | strong_rally | 672 | -0.019 | -0.026 | 40.3% |
| ADANIPORTS | strong_rally | 827 | 0.082 | 0.109 | 40.3% |
| BANKBARODA | strong_rally | 1232 | 0.136 | 0.181 | 40.3% |
| MANKIND | strong_rally | 279 | 0.035 | 0.047 | 40.1% |
| ESCORTS | strong_rally | 865 | 0.060 | 0.080 | 40.0% |
| ACC | strong_rally | 589 | 0.020 | 0.027 | 39.9% |
| SUNPHARMA | strong_rally | 464 | -0.038 | -0.050 | 39.9% |
| DLF | strong_rally | 1046 | 0.028 | 0.038 | 39.9% |
| HCLTECH | strong_rally | 412 | -0.037 | -0.049 | 39.8% |
| ITC | strong_rally | 287 | 0.026 | 0.035 | 39.7% |
| HYUNDAI | strong_rally | 136 | -0.061 | -0.081 | 39.7% |
| GODREJCP | strong_rally | 555 | -0.047 | -0.063 | 39.6% |
| NESTLEIND | strong_rally | 265 | -0.056 | -0.075 | 39.6% |
| ETERNAL | strong_rally | 720 | 0.125 | 0.167 | 39.6% |
| HEXT | strong_rally | 180 | 0.139 | 0.185 | 39.4% |
| DMART | strong_rally | 548 | -0.034 | -0.045 | 39.4% |
| BERGEPAINT | strong_rally | 492 | 0.008 | 0.010 | 39.2% |
| HDBFS | strong_rally | 51 | -0.079 | -0.105 | 39.2% |
| FEDERALBNK | strong_rally | 778 | 0.058 | 0.078 | 39.2% |
| IOC | strong_rally | 804 | 0.072 | 0.096 | 39.2% |
| BAJFINANCE | strong_rally | 682 | 0.110 | 0.146 | 39.1% |
| RELIANCE | strong_rally | 317 | 0.029 | 0.039 | 39.1% |
| TMCV | strong_rally | 105 | -0.083 | -0.111 | 39.0% |
| AJANTPHARM | strong_rally | 794 | -0.032 | -0.043 | 39.0% |
| BAJAJHLDNG | strong_rally | 641 | 0.013 | 0.017 | 39.0% |
| SBIN | strong_rally | 500 | 0.035 | 0.046 | 39.0% |
| JINDALSTEL | strong_rally | 1273 | 0.134 | 0.179 | 39.0% |
| COROMANDEL | strong_rally | 745 | 0.018 | 0.023 | 38.9% |
| LENSKART | strong_rally | 90 | -0.081 | -0.108 | 38.9% |
| IPCALAB | strong_rally | 744 | 0.008 | 0.011 | 38.8% |
| TCS | strong_rally | 263 | -0.039 | -0.052 | 38.8% |
| UNOMINDA | strong_rally | 1069 | 0.021 | 0.028 | 38.7% |
| PNB | strong_rally | 956 | 0.065 | 0.087 | 38.7% |
| 3MINDIA | strong_rally | 584 | -0.031 | -0.041 | 38.7% |
| HAL | strong_rally | 758 | 0.028 | 0.037 | 38.7% |
| TORNTPHARM | strong_rally | 520 | 0.014 | 0.019 | 38.7% |
| AXISBANK | strong_rally | 489 | -0.024 | -0.032 | 38.7% |
| RECLTD | strong_rally | 909 | 0.092 | 0.122 | 38.6% |
| JUBLFOOD | strong_rally | 821 | 0.059 | 0.078 | 38.6% |
| SWIGGY | strong_rally | 298 | 0.080 | 0.107 | 38.6% |
| GICRE | strong_rally | 1049 | 0.087 | 0.116 | 38.5% |
| COALINDIA | strong_rally | 598 | 0.037 | 0.050 | 38.5% |
| PFC | strong_rally | 1003 | 0.072 | 0.096 | 38.4% |
| UNIONBANK | strong_rally | 1282 | 0.085 | 0.113 | 38.4% |
| NTPC | strong_rally | 545 | 0.028 | 0.038 | 38.3% |
| SBILIFE | strong_rally | 473 | -0.029 | -0.038 | 38.3% |
| HDFCLIFE | strong_rally | 468 | 0.023 | 0.030 | 38.2% |
| ADANIENT | strong_rally | 1300 | 0.128 | 0.171 | 38.2% |
| ABCAPITAL | strong_rally | 987 | 0.043 | 0.058 | 38.2% |
| SRF | strong_rally | 664 | 0.010 | 0.013 | 38.1% |
| LTF | strong_rally | 1105 | 0.046 | 0.061 | 38.1% |
| NATIONALUM | strong_rally | 1204 | 0.076 | 0.102 | 38.0% |
| TATAPOWER | strong_rally | 834 | 0.035 | 0.046 | 38.0% |
| DIVISLAB | strong_rally | 542 | -0.054 | -0.072 | 38.0% |
| ZYDUSLIFE | strong_rally | 642 | -0.039 | -0.052 | 38.0% |
| ASHOKLEY | strong_rally | 1100 | 0.096 | 0.129 | 37.9% |
| GLENMARK | strong_rally | 895 | 0.007 | 0.009 | 37.9% |
