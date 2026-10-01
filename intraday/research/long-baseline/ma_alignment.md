# long-baseline/ma_alignment — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 3.0x risk (2.250%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 250 instruments, 2018-12-04..2026-09-28.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| ma_alignment | 810338 | -0.312 | -0.416 | 41.9% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| TCS | ma_alignment | 2792 | 0.056 | 0.074 | 50.0% |
| BAJAJHFL | ma_alignment | 759 | 0.054 | 0.072 | 48.9% |
| HDBFS | ma_alignment | 462 | 0.004 | 0.005 | 48.7% |
| ASIANPAINT | ma_alignment | 2821 | 0.057 | 0.076 | 48.6% |
| LICI | ma_alignment | 1626 | 0.092 | 0.123 | 48.5% |
| HINDUNILVR | ma_alignment | 2796 | 0.025 | 0.033 | 48.2% |
| RELIANCE | ma_alignment | 2883 | 0.083 | 0.111 | 48.0% |
| SBICARD | ma_alignment | 2616 | 0.063 | 0.084 | 47.5% |
| LT | ma_alignment | 2934 | 0.048 | 0.064 | 47.3% |
| NESTLEIND | ma_alignment | 2876 | 0.019 | 0.026 | 47.1% |
| ULTRACEMCO | ma_alignment | 3012 | 0.046 | 0.061 | 47.1% |
| ANTHEM | ma_alignment | 493 | 0.069 | 0.093 | 47.1% |
| NTPCGREEN | ma_alignment | 726 | 0.094 | 0.125 | 47.0% |
| APOLLOHOSP | ma_alignment | 3143 | 0.067 | 0.090 | 46.9% |
| MANKIND | ma_alignment | 1390 | 0.083 | 0.110 | 46.6% |
| MARUTI | ma_alignment | 3010 | 0.046 | 0.062 | 46.5% |
| INFY | ma_alignment | 2822 | 0.007 | 0.009 | 46.5% |
| ITC | ma_alignment | 2834 | 0.038 | 0.051 | 46.3% |
| BRITANNIA | ma_alignment | 2969 | 0.004 | 0.005 | 46.2% |
| AXISBANK | ma_alignment | 3163 | 0.069 | 0.092 | 46.2% |
| GODREJCP | ma_alignment | 3171 | 0.019 | 0.025 | 46.0% |
| DMART | ma_alignment | 3174 | 0.075 | 0.100 | 46.0% |
| TORNTPHARM | ma_alignment | 3188 | 0.016 | 0.022 | 45.9% |
| BAJAJHLDNG | ma_alignment | 3299 | 0.082 | 0.109 | 45.8% |
| HYUNDAI | ma_alignment | 762 | 0.062 | 0.082 | 45.8% |
| LGEINDIA | ma_alignment | 397 | -0.044 | -0.059 | 45.6% |
| BAJAJ-AUTO | ma_alignment | 2995 | 0.022 | 0.030 | 45.6% |
| HONAUT | ma_alignment | 3153 | 0.042 | 0.056 | 45.6% |
| HEROMOTOCO | ma_alignment | 3130 | 0.055 | 0.073 | 45.6% |
| ACC | ma_alignment | 3253 | 0.046 | 0.062 | 45.5% |
| DABUR | ma_alignment | 2954 | 0.008 | 0.011 | 45.5% |
| JIOFIN | ma_alignment | 1241 | 0.067 | 0.090 | 45.4% |
| EICHERMOT | ma_alignment | 3158 | 0.043 | 0.058 | 45.4% |
| POWERGRID | ma_alignment | 3113 | 0.033 | 0.044 | 45.4% |
| WIPRO | ma_alignment | 2935 | 0.030 | 0.039 | 45.3% |
| CIPLA | ma_alignment | 3052 | 0.004 | 0.005 | 45.3% |
| LTM | ma_alignment | 3211 | 0.054 | 0.073 | 45.3% |
| 3MINDIA | ma_alignment | 3373 | 0.023 | 0.030 | 45.3% |
| HDFCLIFE | ma_alignment | 3140 | 0.039 | 0.052 | 45.3% |
| TATACAP | ma_alignment | 371 | 0.014 | 0.018 | 45.3% |
| ITCHOTELS | ma_alignment | 685 | 0.067 | 0.089 | 45.3% |
| HEXT | ma_alignment | 745 | 0.136 | 0.182 | 45.2% |
| NTPC | ma_alignment | 3199 | 0.025 | 0.034 | 45.1% |
| HDFCBANK | ma_alignment | 2841 | 0.015 | 0.019 | 45.1% |
| SHREECEM | ma_alignment | 3232 | 0.032 | 0.043 | 45.0% |
| CRISIL | ma_alignment | 3579 | 0.038 | 0.051 | 45.0% |
| IRCTC | ma_alignment | 2886 | 0.066 | 0.088 | 45.0% |
| GLENMARK | ma_alignment | 3484 | 0.046 | 0.062 | 45.0% |
| TMPV | ma_alignment | 3457 | 0.114 | 0.152 | 45.0% |
| DRREDDY | ma_alignment | 2997 | 0.006 | 0.008 | 44.9% |
| AWL | ma_alignment | 2143 | 0.046 | 0.062 | 44.9% |
| HCLTECH | ma_alignment | 3038 | 0.000 | 0.001 | 44.9% |
| BAJAJFINSV | ma_alignment | 3195 | 0.070 | 0.093 | 44.9% |
| SBIN | ma_alignment | 3184 | 0.066 | 0.088 | 44.9% |
| COALINDIA | ma_alignment | 3198 | 0.048 | 0.064 | 44.8% |
| ONGC | ma_alignment | 3261 | 0.059 | 0.079 | 44.8% |
| TITAN | ma_alignment | 3078 | 0.009 | 0.012 | 44.8% |
| HAL | ma_alignment | 3477 | 0.064 | 0.085 | 44.8% |
| BHARTIARTL | ma_alignment | 3198 | 0.020 | 0.027 | 44.7% |
| BOSCHLTD | ma_alignment | 3314 | 0.034 | 0.046 | 44.6% |
| VOLTAS | ma_alignment | 3424 | 0.027 | 0.036 | 44.5% |
| SUNPHARMA | ma_alignment | 3177 | 0.003 | 0.004 | 44.5% |
| PAGEIND | ma_alignment | 3283 | 0.041 | 0.055 | 44.5% |
| INDIGO | ma_alignment | 3455 | 0.051 | 0.069 | 44.4% |
| MARICO | ma_alignment | 3071 | -0.016 | -0.022 | 44.4% |
| IREDA | ma_alignment | 1289 | 0.083 | 0.111 | 44.3% |
| AUROPHARMA | ma_alignment | 3486 | 0.059 | 0.078 | 44.3% |
| TVSMOTOR | ma_alignment | 3477 | 0.026 | 0.035 | 44.3% |
| DIXON | ma_alignment | 3738 | 0.115 | 0.153 | 44.2% |
| M&M | ma_alignment | 3298 | 0.031 | 0.042 | 44.1% |
| DIVISLAB | ma_alignment | 3176 | 0.020 | 0.026 | 44.1% |
| ICICIBANK | ma_alignment | 3099 | 0.009 | 0.012 | 44.1% |
| COLPAL | ma_alignment | 3087 | -0.023 | -0.031 | 44.1% |
| SBILIFE | ma_alignment | 3158 | 0.005 | 0.006 | 44.1% |
| JUBLFOOD | ma_alignment | 3561 | 0.038 | 0.051 | 44.0% |
| INDHOTEL | ma_alignment | 3540 | 0.082 | 0.109 | 44.0% |
| ABB | ma_alignment | 3523 | 0.037 | 0.049 | 44.0% |
| TATAELXSI | ma_alignment | 3352 | 0.043 | 0.058 | 43.9% |
| BERGEPAINT | ma_alignment | 3316 | 0.007 | 0.009 | 43.9% |
| TECHM | ma_alignment | 3201 | 0.021 | 0.028 | 43.9% |
| SRF | ma_alignment | 3379 | 0.029 | 0.039 | 43.8% |
| LUPIN | ma_alignment | 3221 | 0.014 | 0.018 | 43.7% |
| AJANTPHARM | ma_alignment | 3634 | 0.033 | 0.044 | 43.7% |
| ADANIPORTS | ma_alignment | 3516 | 0.060 | 0.080 | 43.7% |
| VMM | ma_alignment | 759 | 0.067 | 0.089 | 43.6% |
| HINDZINC | ma_alignment | 3458 | 0.046 | 0.061 | 43.6% |
| MPHASIS | ma_alignment | 3594 | 0.062 | 0.083 | 43.6% |
| NYKAA | ma_alignment | 2249 | 0.068 | 0.090 | 43.6% |
| OIL | ma_alignment | 3613 | 0.071 | 0.094 | 43.6% |
| UPL | ma_alignment | 3386 | 0.024 | 0.033 | 43.6% |
| UNITDSPR | ma_alignment | 3333 | 0.010 | 0.014 | 43.5% |
| ABCAPITAL | ma_alignment | 3774 | 0.073 | 0.098 | 43.5% |
| KEI | ma_alignment | 3680 | 0.092 | 0.123 | 43.5% |
| GRASIM | ma_alignment | 3364 | 0.001 | 0.001 | 43.5% |
| INDUSINDBK | ma_alignment | 3535 | 0.083 | 0.111 | 43.5% |
| LICHSGFIN | ma_alignment | 3602 | 0.056 | 0.074 | 43.4% |
| PAYTM | ma_alignment | 2396 | 0.118 | 0.158 | 43.4% |
| ABBOTINDIA | ma_alignment | 3260 | 0.010 | 0.013 | 43.4% |
| TATACONSUM | ma_alignment | 3308 | -0.004 | -0.005 | 43.4% |
| AMBUJACEM | ma_alignment | 3517 | 0.036 | 0.048 | 43.4% |
