# exit-tuning-long/mrm-2.0 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 2.0x risk (1.500%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 165 instruments, 2021-03-23..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| ma_alignment | 404424 | 0.025 | 0.034 | 44.5% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| HONAUT | ma_alignment | 2072 | 0.075 | 0.100 | 50.1% |
| LICI | ma_alignment | 1660 | 0.089 | 0.119 | 49.7% |
| APOLLOHOSP | ma_alignment | 2079 | 0.059 | 0.079 | 48.9% |
| AXISBANK | ma_alignment | 2049 | 0.032 | 0.043 | 48.7% |
| ONGC | ma_alignment | 2157 | 0.062 | 0.082 | 48.7% |
| BAJAJHLDNG | ma_alignment | 2278 | 0.100 | 0.133 | 48.6% |
| ASIANPAINT | ma_alignment | 1929 | 0.028 | 0.037 | 48.6% |
| HINDUNILVR | ma_alignment | 1893 | 0.024 | 0.031 | 48.3% |
| BAJAJ-AUTO | ma_alignment | 2028 | 0.027 | 0.036 | 48.2% |
| BRITANNIA | ma_alignment | 1981 | 0.012 | 0.016 | 48.1% |
| HCLTECH | ma_alignment | 2028 | 0.011 | 0.015 | 47.9% |
| EICHERMOT | ma_alignment | 2129 | 0.017 | 0.023 | 47.7% |
| BHARTIARTL | ma_alignment | 2012 | 0.025 | 0.033 | 47.6% |
| MANKIND | ma_alignment | 1453 | 0.071 | 0.095 | 47.6% |
| INFY | ma_alignment | 1908 | -0.002 | -0.003 | 47.5% |
| INDHOTEL | ma_alignment | 2410 | 0.074 | 0.099 | 47.3% |
| NTPC | ma_alignment | 2149 | 0.025 | 0.033 | 47.2% |
| LTM | ma_alignment | 2249 | 0.053 | 0.071 | 47.2% |
| 3MINDIA | ma_alignment | 2341 | 0.040 | 0.053 | 47.1% |
| DABUR | ma_alignment | 1959 | 0.016 | 0.022 | 47.0% |
| MARICO | ma_alignment | 2085 | -0.011 | -0.015 | 47.0% |
| DMART | ma_alignment | 2146 | 0.061 | 0.082 | 47.0% |
| LT | ma_alignment | 2008 | 0.025 | 0.033 | 47.0% |
| ITC | ma_alignment | 1917 | 0.023 | 0.031 | 46.9% |
| GODREJCP | ma_alignment | 2207 | 0.005 | 0.007 | 46.9% |
| NESTLEIND | ma_alignment | 2001 | -0.009 | -0.012 | 46.9% |
| ABBOTINDIA | ma_alignment | 2223 | 0.027 | 0.036 | 46.8% |
| IRCTC | ma_alignment | 2262 | 0.065 | 0.087 | 46.7% |
| HAL | ma_alignment | 2419 | 0.049 | 0.065 | 46.5% |
| CHOLAFIN | ma_alignment | 2585 | 0.046 | 0.061 | 46.5% |
| ACC | ma_alignment | 2272 | 0.024 | 0.032 | 46.5% |
| HDFCLIFE | ma_alignment | 2080 | 0.031 | 0.041 | 46.4% |
| M&M | ma_alignment | 2226 | 0.025 | 0.034 | 46.4% |
| HEROMOTOCO | ma_alignment | 2122 | 0.025 | 0.033 | 46.4% |
| CRISIL | ma_alignment | 2534 | 0.050 | 0.066 | 46.4% |
| ABB | ma_alignment | 2486 | 0.050 | 0.067 | 46.3% |
| CIPLA | ma_alignment | 2023 | -0.018 | -0.024 | 46.2% |
| LUPIN | ma_alignment | 2197 | 0.009 | 0.013 | 46.2% |
| OIL | ma_alignment | 2625 | 0.085 | 0.113 | 46.1% |
| BAJAJFINSV | ma_alignment | 2207 | 0.032 | 0.042 | 46.1% |
| ICICIBANK | ma_alignment | 1961 | -0.008 | -0.011 | 46.0% |
| HDFCBANK | ma_alignment | 1888 | -0.008 | -0.010 | 46.0% |
| DIXON | ma_alignment | 2622 | 0.077 | 0.103 | 45.9% |
| COALINDIA | ma_alignment | 2243 | 0.026 | 0.035 | 45.9% |
| ETERNAL | ma_alignment | 2617 | 0.116 | 0.155 | 45.8% |
| MPHASIS | ma_alignment | 2515 | 0.058 | 0.077 | 45.8% |
| DLF | ma_alignment | 2460 | 0.082 | 0.109 | 45.7% |
| KEI | ma_alignment | 2676 | 0.086 | 0.115 | 45.7% |
| MEDANTA | ma_alignment | 1857 | 0.046 | 0.061 | 45.7% |
| NLCINDIA | ma_alignment | 2960 | 0.089 | 0.119 | 45.6% |
| COLPAL | ma_alignment | 2078 | -0.012 | -0.016 | 45.6% |
| ASTRAL | ma_alignment | 2432 | 0.022 | 0.029 | 45.6% |
| MARUTI | ma_alignment | 2033 | -0.012 | -0.017 | 45.6% |
| NYKAA | ma_alignment | 2325 | 0.073 | 0.097 | 45.6% |
| DIVISLAB | ma_alignment | 2157 | 0.017 | 0.023 | 45.6% |
| IOC | ma_alignment | 2254 | 0.054 | 0.073 | 45.6% |
| JUBLFOOD | ma_alignment | 2497 | 0.028 | 0.038 | 45.5% |
| INDUSINDBK | ma_alignment | 2340 | 0.027 | 0.036 | 45.5% |
| GODREJPROP | ma_alignment | 2714 | 0.053 | 0.071 | 45.4% |
| AUBANK | ma_alignment | 2464 | 0.035 | 0.047 | 45.3% |
| AWL | ma_alignment | 2237 | 0.038 | 0.050 | 45.2% |
| BPCL | ma_alignment | 2283 | 0.022 | 0.029 | 45.2% |
| GLENMARK | ma_alignment | 2489 | 0.021 | 0.028 | 45.2% |
| MFSL | ma_alignment | 2514 | 0.014 | 0.018 | 45.1% |
| APLAPOLLO | ma_alignment | 2630 | 0.039 | 0.053 | 45.1% |
| ENDURANCE | ma_alignment | 2701 | 0.016 | 0.021 | 45.1% |
| MAXHEALTH | ma_alignment | 2569 | 0.052 | 0.069 | 45.1% |
| FORTIS | ma_alignment | 2551 | 0.048 | 0.064 | 45.1% |
| KOTAKBANK | ma_alignment | 2013 | -0.012 | -0.015 | 45.1% |
| ADANIENT | ma_alignment | 2750 | 0.089 | 0.119 | 45.1% |
| INDIGO | ma_alignment | 2340 | 0.025 | 0.033 | 45.0% |
| DRREDDY | ma_alignment | 2032 | -0.026 | -0.035 | 45.0% |
| GRASIM | ma_alignment | 2208 | -0.009 | -0.011 | 45.0% |
| LINDEINDIA | ma_alignment | 2739 | 0.065 | 0.087 | 45.0% |
| ABCAPITAL | ma_alignment | 2597 | 0.048 | 0.064 | 45.0% |
| ADANIPORTS | ma_alignment | 2520 | 0.053 | 0.070 | 45.0% |
| AJANTPHARM | ma_alignment | 2520 | 0.019 | 0.026 | 45.0% |
| EXIDEIND | ma_alignment | 2329 | 0.028 | 0.037 | 45.0% |
| BANKBARODA | ma_alignment | 2614 | 0.056 | 0.075 | 45.0% |
| COFORGE | ma_alignment | 2528 | 0.053 | 0.070 | 44.9% |
| LICHSGFIN | ma_alignment | 2414 | 0.025 | 0.034 | 44.9% |
| AMBUJACEM | ma_alignment | 2450 | 0.024 | 0.033 | 44.9% |
| BERGEPAINT | ma_alignment | 2279 | 0.005 | 0.007 | 44.9% |
| JKCEMENT | ma_alignment | 2507 | 0.026 | 0.035 | 44.9% |
| CONCOR | ma_alignment | 2409 | 0.035 | 0.047 | 44.9% |
| PAGEIND | ma_alignment | 2198 | -0.001 | -0.002 | 44.9% |
| HINDZINC | ma_alignment | 2401 | 0.017 | 0.023 | 44.8% |
| IDFCFIRSTB | ma_alignment | 2365 | 0.052 | 0.069 | 44.8% |
| KPRMILL | ma_alignment | 2818 | 0.080 | 0.106 | 44.7% |
| MOTHERSON | ma_alignment | 2529 | 0.045 | 0.059 | 44.6% |
| APOLLOTYRE | ma_alignment | 2447 | 0.027 | 0.036 | 44.6% |
| BIOCON | ma_alignment | 2393 | 0.024 | 0.033 | 44.6% |
| ICICIGI | ma_alignment | 2277 | -0.038 | -0.050 | 44.5% |
| BANKINDIA | ma_alignment | 2668 | 0.055 | 0.073 | 44.5% |
| CUMMINSIND | ma_alignment | 2463 | 0.003 | 0.005 | 44.5% |
| JSWSTEEL | ma_alignment | 2320 | -0.006 | -0.008 | 44.4% |
| BDL | ma_alignment | 2875 | 0.073 | 0.097 | 44.4% |
| HAVELLS | ma_alignment | 2257 | -0.003 | -0.005 | 44.4% |
| HINDALCO | ma_alignment | 2398 | -0.008 | -0.011 | 44.3% |
| BOSCHLTD | ma_alignment | 2269 | 0.012 | 0.015 | 44.3% |
