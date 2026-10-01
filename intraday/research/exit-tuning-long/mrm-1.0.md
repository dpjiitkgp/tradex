# exit-tuning-long/mrm-1.0 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 1.0x risk (0.750%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 165 instruments, 2021-03-23..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| ma_alignment | 452138 | 0.018 | 0.024 | 47.5% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| HONAUT | ma_alignment | 2270 | 0.068 | 0.090 | 52.0% |
| AXISBANK | ma_alignment | 2205 | 0.035 | 0.047 | 52.0% |
| LICI | ma_alignment | 1788 | 0.075 | 0.100 | 51.5% |
| APOLLOHOSP | ma_alignment | 2251 | 0.057 | 0.076 | 51.4% |
| MANKIND | ma_alignment | 1640 | 0.066 | 0.088 | 50.9% |
| BAJAJHLDNG | ma_alignment | 2466 | 0.090 | 0.120 | 50.8% |
| DIXON | ma_alignment | 2961 | 0.072 | 0.096 | 50.6% |
| INDHOTEL | ma_alignment | 2732 | 0.062 | 0.082 | 50.4% |
| ABB | ma_alignment | 2799 | 0.039 | 0.051 | 50.3% |
| HINDUNILVR | ma_alignment | 1995 | 0.025 | 0.033 | 50.3% |
| M&M | ma_alignment | 2443 | 0.033 | 0.044 | 50.3% |
| HCLTECH | ma_alignment | 2176 | 0.016 | 0.021 | 50.2% |
| IRCTC | ma_alignment | 2483 | 0.061 | 0.081 | 50.2% |
| LTM | ma_alignment | 2519 | 0.040 | 0.053 | 50.1% |
| DLF | ma_alignment | 2751 | 0.072 | 0.096 | 50.1% |
| ASIANPAINT | ma_alignment | 2062 | 0.028 | 0.037 | 50.0% |
| EICHERMOT | ma_alignment | 2338 | 0.013 | 0.018 | 50.0% |
| GODREJPROP | ma_alignment | 3074 | 0.048 | 0.064 | 49.9% |
| CHOLAFIN | ma_alignment | 2911 | 0.026 | 0.035 | 49.8% |
| ETERNAL | ma_alignment | 2986 | 0.097 | 0.129 | 49.8% |
| OIL | ma_alignment | 2910 | 0.080 | 0.106 | 49.8% |
| ONGC | ma_alignment | 2373 | 0.053 | 0.070 | 49.8% |
| HAL | ma_alignment | 2747 | 0.039 | 0.052 | 49.7% |
| GODREJCP | ma_alignment | 2482 | -0.014 | -0.018 | 49.6% |
| KEI | ma_alignment | 3039 | 0.073 | 0.097 | 49.6% |
| BAJAJ-AUTO | ma_alignment | 2168 | 0.027 | 0.036 | 49.6% |
| NTPC | ma_alignment | 2363 | 0.013 | 0.017 | 49.6% |
| ASTRAL | ma_alignment | 2730 | 0.026 | 0.035 | 49.5% |
| HEROMOTOCO | ma_alignment | 2314 | 0.027 | 0.036 | 49.4% |
| BRITANNIA | ma_alignment | 2111 | 0.014 | 0.018 | 49.4% |
| ABBOTINDIA | ma_alignment | 2416 | 0.025 | 0.034 | 49.3% |
| DMART | ma_alignment | 2323 | 0.069 | 0.092 | 49.3% |
| INFY | ma_alignment | 2038 | -0.003 | -0.004 | 49.3% |
| NLCINDIA | ma_alignment | 3340 | 0.087 | 0.117 | 49.3% |
| ACC | ma_alignment | 2455 | 0.017 | 0.023 | 49.2% |
| MEDANTA | ma_alignment | 2086 | 0.044 | 0.059 | 49.2% |
| INDUSINDBK | ma_alignment | 2578 | 0.029 | 0.039 | 49.2% |
| BAJAJFINSV | ma_alignment | 2420 | 0.029 | 0.038 | 49.2% |
| ADANIENT | ma_alignment | 3141 | 0.070 | 0.093 | 49.2% |
| BHARTIARTL | ma_alignment | 2176 | 0.018 | 0.024 | 49.2% |
| AUBANK | ma_alignment | 2813 | 0.027 | 0.036 | 49.2% |
| EXIDEIND | ma_alignment | 2565 | 0.034 | 0.045 | 49.2% |
| NYKAA | ma_alignment | 2637 | 0.061 | 0.081 | 49.1% |
| CRISIL | ma_alignment | 2837 | 0.054 | 0.071 | 49.1% |
| DABUR | ma_alignment | 2090 | 0.019 | 0.025 | 49.0% |
| AJANTPHARM | ma_alignment | 2831 | 0.025 | 0.033 | 49.0% |
| JUBLFOOD | ma_alignment | 2806 | 0.023 | 0.031 | 49.0% |
| MPHASIS | ma_alignment | 2843 | 0.040 | 0.053 | 49.0% |
| LT | ma_alignment | 2141 | 0.022 | 0.029 | 49.0% |
| HDFCLIFE | ma_alignment | 2257 | 0.025 | 0.033 | 49.0% |
| 3MINDIA | ma_alignment | 2557 | 0.032 | 0.042 | 48.9% |
| ADANIPORTS | ma_alignment | 2808 | 0.042 | 0.056 | 48.9% |
| CONCOR | ma_alignment | 2665 | 0.035 | 0.047 | 48.9% |
| JKCEMENT | ma_alignment | 2815 | 0.030 | 0.040 | 48.8% |
| NESTLEIND | ma_alignment | 2121 | -0.014 | -0.019 | 48.8% |
| ICICIBANK | ma_alignment | 2079 | -0.004 | -0.006 | 48.8% |
| GLENMARK | ma_alignment | 2826 | 0.011 | 0.015 | 48.8% |
| NAM-INDIA | ma_alignment | 3040 | 0.021 | 0.028 | 48.6% |
| MARICO | ma_alignment | 2250 | -0.021 | -0.027 | 48.5% |
| AMBUJACEM | ma_alignment | 2715 | 0.015 | 0.020 | 48.4% |
| MFSL | ma_alignment | 2880 | 0.001 | 0.002 | 48.4% |
| ABCAPITAL | ma_alignment | 2972 | 0.037 | 0.050 | 48.4% |
| ENDURANCE | ma_alignment | 3037 | 0.024 | 0.032 | 48.4% |
| PAGEIND | ma_alignment | 2448 | -0.002 | -0.003 | 48.4% |
| OBEROIRLTY | ma_alignment | 3113 | 0.003 | 0.004 | 48.3% |
| HDFCBANK | ma_alignment | 1969 | -0.004 | -0.006 | 48.3% |
| LUPIN | ma_alignment | 2438 | 0.001 | 0.002 | 48.3% |
| MAXHEALTH | ma_alignment | 2943 | 0.033 | 0.043 | 48.2% |
| COROMANDEL | ma_alignment | 2859 | 0.002 | 0.003 | 48.2% |
| COFORGE | ma_alignment | 2851 | 0.033 | 0.044 | 48.2% |
| BIOCON | ma_alignment | 2644 | 0.027 | 0.036 | 48.2% |
| DIVISLAB | ma_alignment | 2375 | 0.014 | 0.019 | 48.1% |
| LICHSGFIN | ma_alignment | 2700 | 0.024 | 0.032 | 48.1% |
| LINDEINDIA | ma_alignment | 3061 | 0.045 | 0.060 | 48.1% |
| MAZDOCK | ma_alignment | 3425 | 0.035 | 0.047 | 48.1% |
| CUMMINSIND | ma_alignment | 2793 | 0.006 | 0.008 | 48.1% |
| LAURUSLABS | ma_alignment | 2947 | 0.010 | 0.013 | 48.1% |
| APOLLOTYRE | ma_alignment | 2742 | 0.024 | 0.032 | 48.1% |
| MOTILALOFS | ma_alignment | 3139 | 0.059 | 0.079 | 48.0% |
| GRASIM | ma_alignment | 2455 | -0.020 | -0.027 | 48.0% |
| ITC | ma_alignment | 2024 | 0.016 | 0.022 | 48.0% |
| MARUTI | ma_alignment | 2175 | -0.009 | -0.013 | 48.0% |
| APLAPOLLO | ma_alignment | 2968 | 0.033 | 0.044 | 47.9% |
| CIPLA | ma_alignment | 2180 | -0.019 | -0.026 | 47.9% |
| AUROPHARMA | ma_alignment | 2675 | 0.016 | 0.022 | 47.9% |
| BDL | ma_alignment | 3277 | 0.045 | 0.060 | 47.9% |
| KPRMILL | ma_alignment | 3181 | 0.053 | 0.071 | 47.9% |
| DRREDDY | ma_alignment | 2175 | -0.018 | -0.025 | 47.9% |
| AWL | ma_alignment | 2439 | 0.016 | 0.021 | 47.9% |
| ALKEM | ma_alignment | 2502 | -0.005 | -0.007 | 47.9% |
| KPITTECH | ma_alignment | 3157 | 0.041 | 0.054 | 47.9% |
| COALINDIA | ma_alignment | 2461 | 0.015 | 0.020 | 47.8% |
| IDFCFIRSTB | ma_alignment | 2663 | 0.044 | 0.059 | 47.8% |
| FORTIS | ma_alignment | 2894 | 0.030 | 0.040 | 47.8% |
| INDIGO | ma_alignment | 2583 | 0.019 | 0.025 | 47.7% |
| JSL | ma_alignment | 3476 | 0.034 | 0.045 | 47.7% |
| MOTHERSON | ma_alignment | 2875 | 0.026 | 0.035 | 47.7% |
| JSWSTEEL | ma_alignment | 2555 | 0.002 | 0.003 | 47.7% |
| BANKBARODA | ma_alignment | 2971 | 0.043 | 0.058 | 47.6% |
| NATIONALUM | ma_alignment | 3259 | 0.049 | 0.065 | 47.6% |
