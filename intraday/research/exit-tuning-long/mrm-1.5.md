# exit-tuning-long/mrm-1.5 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 1.5x risk (1.125%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 165 instruments, 2021-03-23..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| ma_alignment | 423151 | 0.021 | 0.028 | 46.1% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| HONAUT | ma_alignment | 2166 | 0.067 | 0.090 | 51.3% |
| AXISBANK | ma_alignment | 2112 | 0.036 | 0.048 | 50.3% |
| BAJAJHLDNG | ma_alignment | 2353 | 0.099 | 0.132 | 50.1% |
| LICI | ma_alignment | 1694 | 0.082 | 0.110 | 50.0% |
| APOLLOHOSP | ma_alignment | 2142 | 0.060 | 0.080 | 50.0% |
| ASIANPAINT | ma_alignment | 1969 | 0.032 | 0.042 | 49.6% |
| MANKIND | ma_alignment | 1533 | 0.073 | 0.097 | 49.6% |
| ONGC | ma_alignment | 2256 | 0.057 | 0.077 | 49.5% |
| BRITANNIA | ma_alignment | 2030 | 0.014 | 0.019 | 48.9% |
| HINDUNILVR | ma_alignment | 1923 | 0.023 | 0.031 | 48.7% |
| HCLTECH | ma_alignment | 2067 | 0.015 | 0.020 | 48.7% |
| BAJAJ-AUTO | ma_alignment | 2071 | 0.026 | 0.034 | 48.5% |
| BHARTIARTL | ma_alignment | 2067 | 0.024 | 0.032 | 48.5% |
| 3MINDIA | ma_alignment | 2424 | 0.038 | 0.050 | 48.5% |
| EICHERMOT | ma_alignment | 2196 | 0.018 | 0.024 | 48.5% |
| DMART | ma_alignment | 2217 | 0.066 | 0.088 | 48.4% |
| M&M | ma_alignment | 2315 | 0.029 | 0.039 | 48.3% |
| NTPC | ma_alignment | 2226 | 0.020 | 0.026 | 48.2% |
| INDHOTEL | ma_alignment | 2527 | 0.068 | 0.091 | 48.2% |
| MEDANTA | ma_alignment | 1967 | 0.043 | 0.057 | 48.2% |
| CHOLAFIN | ma_alignment | 2712 | 0.038 | 0.051 | 48.2% |
| NLCINDIA | ma_alignment | 3133 | 0.090 | 0.120 | 48.2% |
| OIL | ma_alignment | 2742 | 0.080 | 0.107 | 48.1% |
| LTM | ma_alignment | 2352 | 0.045 | 0.060 | 48.1% |
| ABBOTINDIA | ma_alignment | 2305 | 0.023 | 0.031 | 48.1% |
| DIXON | ma_alignment | 2756 | 0.068 | 0.091 | 48.0% |
| ABB | ma_alignment | 2624 | 0.042 | 0.056 | 47.9% |
| CRISIL | ma_alignment | 2662 | 0.054 | 0.072 | 47.9% |
| IRCTC | ma_alignment | 2354 | 0.053 | 0.071 | 47.9% |
| HAL | ma_alignment | 2535 | 0.044 | 0.059 | 47.9% |
| INFY | ma_alignment | 1952 | -0.004 | -0.005 | 47.8% |
| LT | ma_alignment | 2053 | 0.025 | 0.034 | 47.8% |
| ENDURANCE | ma_alignment | 2831 | 0.030 | 0.041 | 47.7% |
| ETERNAL | ma_alignment | 2752 | 0.105 | 0.140 | 47.7% |
| ACC | ma_alignment | 2350 | 0.016 | 0.021 | 47.6% |
| NESTLEIND | ma_alignment | 2033 | -0.009 | -0.012 | 47.6% |
| KEI | ma_alignment | 2821 | 0.076 | 0.101 | 47.6% |
| DLF | ma_alignment | 2570 | 0.080 | 0.107 | 47.5% |
| GODREJCP | ma_alignment | 2307 | -0.007 | -0.010 | 47.5% |
| HDFCLIFE | ma_alignment | 2141 | 0.030 | 0.039 | 47.5% |
| ITC | ma_alignment | 1948 | 0.023 | 0.031 | 47.5% |
| MPHASIS | ma_alignment | 2643 | 0.052 | 0.070 | 47.5% |
| MARICO | ma_alignment | 2138 | -0.016 | -0.021 | 47.5% |
| NYKAA | ma_alignment | 2453 | 0.068 | 0.091 | 47.5% |
| ASTRAL | ma_alignment | 2558 | 0.025 | 0.033 | 47.4% |
| HEROMOTOCO | ma_alignment | 2196 | 0.026 | 0.035 | 47.4% |
| LUPIN | ma_alignment | 2281 | 0.010 | 0.013 | 47.4% |
| BAJAJFINSV | ma_alignment | 2284 | 0.027 | 0.036 | 47.3% |
| DABUR | ma_alignment | 2001 | 0.014 | 0.019 | 47.3% |
| MAXHEALTH | ma_alignment | 2720 | 0.047 | 0.063 | 47.2% |
| GODREJPROP | ma_alignment | 2853 | 0.044 | 0.059 | 47.2% |
| JKCEMENT | ma_alignment | 2640 | 0.034 | 0.045 | 47.0% |
| COFORGE | ma_alignment | 2645 | 0.053 | 0.071 | 47.0% |
| JUBLFOOD | ma_alignment | 2616 | 0.029 | 0.039 | 46.9% |
| ADANIENT | ma_alignment | 2898 | 0.086 | 0.115 | 46.9% |
| INDUSINDBK | ma_alignment | 2436 | 0.025 | 0.033 | 46.9% |
| EXIDEIND | ma_alignment | 2393 | 0.035 | 0.046 | 46.8% |
| MARUTI | ma_alignment | 2079 | -0.011 | -0.015 | 46.8% |
| IDFCFIRSTB | ma_alignment | 2493 | 0.051 | 0.068 | 46.8% |
| APLAPOLLO | ma_alignment | 2780 | 0.031 | 0.042 | 46.8% |
| COALINDIA | ma_alignment | 2312 | 0.022 | 0.029 | 46.8% |
| ADANIPORTS | ma_alignment | 2625 | 0.053 | 0.070 | 46.7% |
| AUBANK | ma_alignment | 2594 | 0.029 | 0.038 | 46.7% |
| BANKBARODA | ma_alignment | 2753 | 0.050 | 0.067 | 46.7% |
| ICICIBANK | ma_alignment | 1993 | -0.008 | -0.011 | 46.7% |
| KPITTECH | ma_alignment | 2939 | 0.054 | 0.073 | 46.6% |
| GLENMARK | ma_alignment | 2622 | 0.016 | 0.022 | 46.6% |
| MOTHERSON | ma_alignment | 2661 | 0.039 | 0.052 | 46.6% |
| AJANTPHARM | ma_alignment | 2661 | 0.020 | 0.026 | 46.6% |
| IOC | ma_alignment | 2352 | 0.044 | 0.059 | 46.6% |
| MFSL | ma_alignment | 2651 | 0.007 | 0.009 | 46.6% |
| ABCAPITAL | ma_alignment | 2755 | 0.037 | 0.049 | 46.6% |
| AWL | ma_alignment | 2320 | 0.024 | 0.032 | 46.5% |
| PAGEIND | ma_alignment | 2276 | 0.002 | 0.003 | 46.4% |
| NAM-INDIA | ma_alignment | 2806 | 0.034 | 0.045 | 46.4% |
| DIVISLAB | ma_alignment | 2234 | 0.017 | 0.023 | 46.4% |
| MOTILALOFS | ma_alignment | 2898 | 0.071 | 0.095 | 46.4% |
| FORTIS | ma_alignment | 2681 | 0.039 | 0.052 | 46.4% |
| LINDEINDIA | ma_alignment | 2880 | 0.051 | 0.069 | 46.4% |
| KPRMILL | ma_alignment | 2970 | 0.071 | 0.095 | 46.4% |
| LICHSGFIN | ma_alignment | 2530 | 0.028 | 0.037 | 46.4% |
| MAZDOCK | ma_alignment | 3200 | 0.037 | 0.049 | 46.3% |
| HDFCBANK | ma_alignment | 1907 | -0.010 | -0.014 | 46.3% |
| BANKINDIA | ma_alignment | 2786 | 0.050 | 0.067 | 46.3% |
| CIPLA | ma_alignment | 2065 | -0.022 | -0.030 | 46.2% |
| OBEROIRLTY | ma_alignment | 2874 | 0.007 | 0.009 | 46.2% |
| BPCL | ma_alignment | 2380 | 0.017 | 0.023 | 46.2% |
| INDIGO | ma_alignment | 2415 | 0.020 | 0.027 | 46.2% |
| DRREDDY | ma_alignment | 2085 | -0.027 | -0.036 | 46.1% |
| COROMANDEL | ma_alignment | 2675 | 0.004 | 0.006 | 46.1% |
| NATIONALUM | ma_alignment | 2985 | 0.064 | 0.085 | 46.1% |
| AMBUJACEM | ma_alignment | 2558 | 0.013 | 0.017 | 46.1% |
| GRASIM | ma_alignment | 2302 | -0.015 | -0.021 | 46.1% |
| CONCOR | ma_alignment | 2519 | 0.031 | 0.041 | 46.1% |
| AUROPHARMA | ma_alignment | 2521 | 0.010 | 0.014 | 46.1% |
| JSWSTEEL | ma_alignment | 2413 | -0.001 | -0.001 | 46.0% |
| KALYANKJIL | ma_alignment | 3034 | 0.043 | 0.058 | 46.0% |
| HINDZINC | ma_alignment | 2478 | 0.012 | 0.016 | 46.0% |
| APOLLOTYRE | ma_alignment | 2576 | 0.022 | 0.030 | 46.0% |
| BLUESTARCO | ma_alignment | 2886 | 0.013 | 0.018 | 45.9% |
