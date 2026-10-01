# long-baseline/ma_alignment_confirmed — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 3.0x risk (2.250%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 250 instruments, 2018-12-04..2026-09-28.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| ma_alignment | 789384 | -0.099 | -0.133 | 42.3% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LICI | ma_alignment | 1607 | 0.093 | 0.125 | 49.3% |
| TCS | ma_alignment | 2745 | 0.049 | 0.065 | 49.1% |
| BAJAJHFL | ma_alignment | 739 | 0.065 | 0.086 | 49.0% |
| ASIANPAINT | ma_alignment | 2785 | 0.053 | 0.070 | 48.4% |
| RELIANCE | ma_alignment | 2824 | 0.077 | 0.103 | 48.1% |
| HONAUT | ma_alignment | 3015 | 0.068 | 0.091 | 47.4% |
| NTPCGREEN | ma_alignment | 718 | 0.090 | 0.120 | 47.4% |
| AXISBANK | ma_alignment | 3074 | 0.074 | 0.099 | 47.3% |
| SBICARD | ma_alignment | 2591 | 0.066 | 0.087 | 47.3% |
| NESTLEIND | ma_alignment | 2860 | 0.014 | 0.018 | 47.1% |
| MANKIND | ma_alignment | 1362 | 0.091 | 0.121 | 47.1% |
| APOLLOHOSP | ma_alignment | 3095 | 0.068 | 0.090 | 47.0% |
| BRITANNIA | ma_alignment | 2876 | 0.018 | 0.024 | 46.9% |
| TATACAP | ma_alignment | 364 | 0.017 | 0.022 | 46.7% |
| LT | ma_alignment | 2876 | 0.044 | 0.058 | 46.7% |
| EICHERMOT | ma_alignment | 3086 | 0.061 | 0.081 | 46.6% |
| BAJAJHLDNG | ma_alignment | 3199 | 0.086 | 0.115 | 46.5% |
| HINDUNILVR | ma_alignment | 2766 | 0.013 | 0.017 | 46.5% |
| INFY | ma_alignment | 2789 | 0.010 | 0.013 | 46.5% |
| HDBFS | ma_alignment | 470 | -0.019 | -0.026 | 46.4% |
| ITC | ma_alignment | 2772 | 0.039 | 0.051 | 46.2% |
| HEROMOTOCO | ma_alignment | 3054 | 0.057 | 0.076 | 46.2% |
| 3MINDIA | ma_alignment | 3270 | 0.038 | 0.050 | 46.1% |
| HCLTECH | ma_alignment | 2969 | 0.010 | 0.013 | 46.1% |
| ULTRACEMCO | ma_alignment | 2953 | 0.044 | 0.059 | 46.1% |
| ANTHEM | ma_alignment | 490 | 0.050 | 0.066 | 46.1% |
| DMART | ma_alignment | 3101 | 0.074 | 0.098 | 46.1% |
| HDFCBANK | ma_alignment | 2801 | 0.019 | 0.026 | 46.0% |
| BAJAJ-AUTO | ma_alignment | 2957 | 0.020 | 0.027 | 46.0% |
| SHREECEM | ma_alignment | 3168 | 0.038 | 0.051 | 45.9% |
| DABUR | ma_alignment | 2896 | 0.005 | 0.006 | 45.9% |
| NTPC | ma_alignment | 3132 | 0.026 | 0.034 | 45.8% |
| SBIN | ma_alignment | 3122 | 0.075 | 0.100 | 45.8% |
| MARUTI | ma_alignment | 2957 | 0.032 | 0.043 | 45.8% |
| CIPLA | ma_alignment | 2989 | -0.006 | -0.009 | 45.8% |
| ACC | ma_alignment | 3209 | 0.039 | 0.052 | 45.6% |
| IRCTC | ma_alignment | 2835 | 0.072 | 0.096 | 45.6% |
| TECHM | ma_alignment | 3101 | 0.043 | 0.057 | 45.6% |
| HEXT | ma_alignment | 729 | 0.139 | 0.186 | 45.5% |
| HDFCLIFE | ma_alignment | 3047 | 0.044 | 0.059 | 45.5% |
| LTM | ma_alignment | 3158 | 0.054 | 0.072 | 45.4% |
| WIPRO | ma_alignment | 2873 | 0.034 | 0.046 | 45.4% |
| SUNPHARMA | ma_alignment | 3097 | 0.018 | 0.024 | 45.2% |
| HYUNDAI | ma_alignment | 750 | 0.056 | 0.075 | 45.1% |
| TORNTPHARM | ma_alignment | 3123 | 0.012 | 0.016 | 45.0% |
| POWERGRID | ma_alignment | 3046 | 0.032 | 0.043 | 45.0% |
| ABB | ma_alignment | 3435 | 0.049 | 0.066 | 45.0% |
| HAL | ma_alignment | 3391 | 0.062 | 0.083 | 45.0% |
| JIOFIN | ma_alignment | 1214 | 0.070 | 0.094 | 45.0% |
| MARICO | ma_alignment | 3001 | -0.012 | -0.017 | 45.0% |
| ONGC | ma_alignment | 3197 | 0.056 | 0.074 | 44.9% |
| BAJAJFINSV | ma_alignment | 3167 | 0.069 | 0.092 | 44.9% |
| CRISIL | ma_alignment | 3484 | 0.044 | 0.058 | 44.8% |
| ZYDUSLIFE | ma_alignment | 3253 | 0.019 | 0.026 | 44.8% |
| TMPV | ma_alignment | 3439 | 0.094 | 0.125 | 44.8% |
| TATAELXSI | ma_alignment | 3294 | 0.045 | 0.061 | 44.7% |
| TITAN | ma_alignment | 3008 | 0.012 | 0.016 | 44.7% |
| UNITDSPR | ma_alignment | 3233 | 0.019 | 0.025 | 44.7% |
| BHARTIARTL | ma_alignment | 3116 | 0.030 | 0.041 | 44.7% |
| COALINDIA | ma_alignment | 3172 | 0.040 | 0.053 | 44.7% |
| ITCHOTELS | ma_alignment | 663 | 0.087 | 0.116 | 44.6% |
| ICICIBANK | ma_alignment | 3028 | 0.018 | 0.024 | 44.6% |
| INDHOTEL | ma_alignment | 3423 | 0.090 | 0.120 | 44.6% |
| TVSMOTOR | ma_alignment | 3397 | 0.030 | 0.039 | 44.5% |
| M&M | ma_alignment | 3232 | 0.040 | 0.053 | 44.5% |
| PAGEIND | ma_alignment | 3229 | 0.039 | 0.052 | 44.5% |
| LUPIN | ma_alignment | 3142 | 0.022 | 0.030 | 44.5% |
| GODREJCP | ma_alignment | 3129 | 0.000 | 0.000 | 44.5% |
| COLPAL | ma_alignment | 3004 | -0.020 | -0.026 | 44.5% |
| SRF | ma_alignment | 3286 | 0.032 | 0.043 | 44.5% |
| DIVISLAB | ma_alignment | 3089 | 0.024 | 0.033 | 44.4% |
| VOLTAS | ma_alignment | 3372 | 0.030 | 0.040 | 44.4% |
| UPL | ma_alignment | 3302 | 0.038 | 0.050 | 44.4% |
| JUBLFOOD | ma_alignment | 3471 | 0.045 | 0.060 | 44.3% |
| AWL | ma_alignment | 2109 | 0.041 | 0.055 | 44.3% |
| DRREDDY | ma_alignment | 2966 | -0.002 | -0.003 | 44.3% |
| SBILIFE | ma_alignment | 3098 | 0.008 | 0.011 | 44.3% |
| TMCV | ma_alignment | 416 | 0.063 | 0.083 | 44.2% |
| ABBOTINDIA | ma_alignment | 3157 | 0.017 | 0.023 | 44.2% |
| TORNTPOWER | ma_alignment | 3575 | 0.049 | 0.066 | 44.1% |
| MPHASIS | ma_alignment | 3489 | 0.075 | 0.100 | 44.1% |
| HDFCAMC | ma_alignment | 3186 | 0.028 | 0.038 | 44.1% |
| NYKAA | ma_alignment | 2190 | 0.073 | 0.098 | 44.1% |
| PAYTM | ma_alignment | 2344 | 0.117 | 0.156 | 44.0% |
| BOSCHLTD | ma_alignment | 3227 | 0.036 | 0.048 | 44.0% |
| HINDZINC | ma_alignment | 3352 | 0.047 | 0.063 | 44.0% |
| MEDANTA | ma_alignment | 1770 | 0.039 | 0.052 | 43.9% |
| IREDA | ma_alignment | 1269 | 0.081 | 0.108 | 43.9% |
| BERGEPAINT | ma_alignment | 3235 | 0.008 | 0.011 | 43.8% |
| GLENMARK | ma_alignment | 3497 | 0.017 | 0.022 | 43.8% |
| TATACONSUM | ma_alignment | 3224 | -0.013 | -0.017 | 43.7% |
| AJANTPHARM | ma_alignment | 3518 | 0.032 | 0.043 | 43.7% |
| INDIGO | ma_alignment | 3391 | 0.042 | 0.056 | 43.7% |
| AUROPHARMA | ma_alignment | 3454 | 0.041 | 0.054 | 43.7% |
| VMM | ma_alignment | 740 | 0.059 | 0.078 | 43.6% |
| BIOCON | ma_alignment | 3322 | 0.034 | 0.045 | 43.6% |
| ABCAPITAL | ma_alignment | 3687 | 0.080 | 0.106 | 43.6% |
| INDUSINDBK | ma_alignment | 3481 | 0.071 | 0.094 | 43.6% |
| POLICYBZR | ma_alignment | 2399 | 0.152 | 0.203 | 43.6% |
| KOTAKBANK | ma_alignment | 3021 | -0.009 | -0.012 | 43.6% |
