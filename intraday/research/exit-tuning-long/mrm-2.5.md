# exit-tuning-long/mrm-2.5 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 2.5x risk (1.875%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 165 instruments, 2021-03-23..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| ma_alignment | 392330 | 0.027 | 0.036 | 43.8% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| HONAUT | ma_alignment | 2031 | 0.079 | 0.105 | 49.7% |
| LICI | ma_alignment | 1628 | 0.090 | 0.120 | 49.4% |
| APOLLOHOSP | ma_alignment | 2047 | 0.058 | 0.077 | 48.6% |
| AXISBANK | ma_alignment | 2003 | 0.034 | 0.045 | 48.4% |
| ASIANPAINT | ma_alignment | 1907 | 0.026 | 0.035 | 48.3% |
| BRITANNIA | ma_alignment | 1943 | 0.018 | 0.024 | 48.1% |
| BAJAJHLDNG | ma_alignment | 2213 | 0.102 | 0.136 | 48.0% |
| HINDUNILVR | ma_alignment | 1879 | 0.023 | 0.030 | 48.0% |
| BAJAJ-AUTO | ma_alignment | 2000 | 0.029 | 0.039 | 47.9% |
| ONGC | ma_alignment | 2115 | 0.059 | 0.079 | 47.8% |
| HCLTECH | ma_alignment | 2003 | 0.013 | 0.018 | 47.8% |
| INFY | ma_alignment | 1890 | -0.000 | -0.000 | 47.5% |
| EICHERMOT | ma_alignment | 2080 | 0.022 | 0.030 | 47.4% |
| BHARTIARTL | ma_alignment | 1980 | 0.023 | 0.031 | 47.3% |
| MANKIND | ma_alignment | 1404 | 0.078 | 0.104 | 47.2% |
| 3MINDIA | ma_alignment | 2285 | 0.051 | 0.069 | 46.9% |
| NESTLEIND | ma_alignment | 1977 | -0.009 | -0.011 | 46.9% |
| DABUR | ma_alignment | 1941 | 0.017 | 0.023 | 46.8% |
| NTPC | ma_alignment | 2114 | 0.021 | 0.028 | 46.7% |
| LT | ma_alignment | 1970 | 0.025 | 0.034 | 46.7% |
| MARICO | ma_alignment | 2045 | -0.009 | -0.012 | 46.7% |
| IRCTC | ma_alignment | 2211 | 0.073 | 0.097 | 46.6% |
| DMART | ma_alignment | 2096 | 0.063 | 0.084 | 46.6% |
| ABBOTINDIA | ma_alignment | 2192 | 0.024 | 0.032 | 46.6% |
| ITC | ma_alignment | 1879 | 0.022 | 0.029 | 46.6% |
| ACC | ma_alignment | 2209 | 0.033 | 0.044 | 46.5% |
| GODREJCP | ma_alignment | 2151 | 0.006 | 0.008 | 46.4% |
| HEROMOTOCO | ma_alignment | 2075 | 0.029 | 0.039 | 46.4% |
| INDHOTEL | ma_alignment | 2330 | 0.076 | 0.101 | 46.2% |
| HDFCBANK | ma_alignment | 1875 | -0.006 | -0.008 | 46.1% |
| LTM | ma_alignment | 2176 | 0.044 | 0.058 | 46.0% |
| HAL | ma_alignment | 2352 | 0.045 | 0.061 | 46.0% |
| ICICIBANK | ma_alignment | 1945 | -0.006 | -0.008 | 46.0% |
| HDFCLIFE | ma_alignment | 2046 | 0.030 | 0.040 | 46.0% |
| CIPLA | ma_alignment | 2006 | -0.024 | -0.032 | 46.0% |
| BAJAJFINSV | ma_alignment | 2149 | 0.032 | 0.043 | 45.8% |
| LUPIN | ma_alignment | 2137 | 0.012 | 0.016 | 45.7% |
| M&M | ma_alignment | 2192 | 0.021 | 0.028 | 45.5% |
| CRISIL | ma_alignment | 2457 | 0.055 | 0.073 | 45.5% |
| COLPAL | ma_alignment | 2055 | -0.011 | -0.015 | 45.5% |
| CHOLAFIN | ma_alignment | 2487 | 0.040 | 0.053 | 45.4% |
| DLF | ma_alignment | 2373 | 0.086 | 0.114 | 45.3% |
| COALINDIA | ma_alignment | 2192 | 0.028 | 0.037 | 45.3% |
| MARUTI | ma_alignment | 2002 | -0.013 | -0.017 | 45.3% |
| ABB | ma_alignment | 2402 | 0.048 | 0.063 | 45.2% |
| IOC | ma_alignment | 2201 | 0.054 | 0.072 | 45.1% |
| MPHASIS | ma_alignment | 2427 | 0.060 | 0.080 | 45.1% |
| DIVISLAB | ma_alignment | 2110 | 0.015 | 0.019 | 45.0% |
| ASTRAL | ma_alignment | 2362 | 0.029 | 0.039 | 45.0% |
| OIL | ma_alignment | 2529 | 0.083 | 0.110 | 45.0% |
| BPCL | ma_alignment | 2218 | 0.028 | 0.037 | 44.9% |
| EXIDEIND | ma_alignment | 2286 | 0.031 | 0.042 | 44.8% |
| KOTAKBANK | ma_alignment | 1983 | -0.014 | -0.019 | 44.8% |
| AUBANK | ma_alignment | 2366 | 0.049 | 0.065 | 44.8% |
| DRREDDY | ma_alignment | 2012 | -0.029 | -0.039 | 44.7% |
| GLENMARK | ma_alignment | 2411 | 0.019 | 0.025 | 44.7% |
| AWL | ma_alignment | 2163 | 0.041 | 0.054 | 44.7% |
| NYKAA | ma_alignment | 2232 | 0.076 | 0.101 | 44.6% |
| NLCINDIA | ma_alignment | 2840 | 0.097 | 0.129 | 44.6% |
| PAGEIND | ma_alignment | 2143 | 0.003 | 0.004 | 44.6% |
| JUBLFOOD | ma_alignment | 2421 | 0.027 | 0.035 | 44.6% |
| GODREJPROP | ma_alignment | 2633 | 0.060 | 0.080 | 44.5% |
| BERGEPAINT | ma_alignment | 2247 | 0.006 | 0.008 | 44.5% |
| IDFCFIRSTB | ma_alignment | 2288 | 0.055 | 0.073 | 44.4% |
| KEI | ma_alignment | 2577 | 0.091 | 0.121 | 44.4% |
| DIXON | ma_alignment | 2544 | 0.065 | 0.086 | 44.3% |
| INDUSINDBK | ma_alignment | 2277 | 0.022 | 0.029 | 44.3% |
| ABCAPITAL | ma_alignment | 2515 | 0.042 | 0.055 | 44.3% |
| GRASIM | ma_alignment | 2159 | -0.017 | -0.022 | 44.3% |
| HINDZINC | ma_alignment | 2341 | 0.019 | 0.026 | 44.3% |
| LICHSGFIN | ma_alignment | 2323 | 0.029 | 0.039 | 44.3% |
| MOTHERSON | ma_alignment | 2432 | 0.050 | 0.067 | 44.2% |
| COROMANDEL | ma_alignment | 2477 | 0.002 | 0.003 | 44.2% |
| HINDALCO | ma_alignment | 2337 | -0.001 | -0.002 | 44.2% |
| MFSL | ma_alignment | 2423 | 0.012 | 0.016 | 44.2% |
| BOSCHLTD | ma_alignment | 2206 | 0.014 | 0.019 | 44.2% |
| MAXHEALTH | ma_alignment | 2474 | 0.054 | 0.073 | 44.2% |
| COFORGE | ma_alignment | 2431 | 0.049 | 0.066 | 44.2% |
| ADANIENT | ma_alignment | 2647 | 0.093 | 0.124 | 44.2% |
| LINDEINDIA | ma_alignment | 2629 | 0.070 | 0.093 | 44.2% |
| FORTIS | ma_alignment | 2458 | 0.045 | 0.060 | 44.1% |
| ENDURANCE | ma_alignment | 2609 | 0.011 | 0.015 | 44.1% |
| MEDANTA | ma_alignment | 1795 | 0.038 | 0.050 | 44.1% |
| APLAPOLLO | ma_alignment | 2553 | 0.035 | 0.047 | 44.1% |
| AMBUJACEM | ma_alignment | 2383 | 0.024 | 0.032 | 44.1% |
| APOLLOTYRE | ma_alignment | 2381 | 0.027 | 0.036 | 44.1% |
| BIOCON | ma_alignment | 2313 | 0.027 | 0.036 | 44.1% |
| INDIGO | ma_alignment | 2279 | 0.021 | 0.028 | 44.1% |
| ICICIGI | ma_alignment | 2226 | -0.036 | -0.048 | 44.0% |
| ETERNAL | ma_alignment | 2507 | 0.122 | 0.163 | 44.0% |
| KPRMILL | ma_alignment | 2700 | 0.091 | 0.122 | 44.0% |
| CUMMINSIND | ma_alignment | 2381 | 0.005 | 0.007 | 43.8% |
| JKCEMENT | ma_alignment | 2411 | 0.031 | 0.041 | 43.8% |
| HAVELLS | ma_alignment | 2206 | -0.005 | -0.007 | 43.8% |
| AJANTPHARM | ma_alignment | 2446 | 0.014 | 0.018 | 43.8% |
| BANKBARODA | ma_alignment | 2508 | 0.054 | 0.072 | 43.8% |
| AUROPHARMA | ma_alignment | 2339 | 0.011 | 0.015 | 43.7% |
| ESCORTS | ma_alignment | 2349 | 0.004 | 0.006 | 43.7% |
| ADANIPORTS | ma_alignment | 2458 | 0.043 | 0.058 | 43.7% |
| CONCOR | ma_alignment | 2338 | 0.030 | 0.040 | 43.6% |
