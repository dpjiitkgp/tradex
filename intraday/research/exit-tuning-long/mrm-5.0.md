# exit-tuning-long/mrm-5.0 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 5.0x risk (3.750%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 165 instruments, 2021-03-23..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| ma_alignment | 370441 | 0.033 | 0.044 | 43.0% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| HONAUT | ma_alignment | 1962 | 0.080 | 0.106 | 49.1% |
| LICI | ma_alignment | 1578 | 0.099 | 0.132 | 48.7% |
| ASIANPAINT | ma_alignment | 1880 | 0.027 | 0.036 | 48.3% |
| APOLLOHOSP | ma_alignment | 1976 | 0.055 | 0.073 | 48.2% |
| ONGC | ma_alignment | 2024 | 0.069 | 0.092 | 48.2% |
| AXISBANK | ma_alignment | 1956 | 0.033 | 0.044 | 48.2% |
| BRITANNIA | ma_alignment | 1913 | 0.015 | 0.020 | 47.9% |
| HINDUNILVR | ma_alignment | 1860 | 0.019 | 0.026 | 47.7% |
| INFY | ma_alignment | 1867 | 0.000 | 0.000 | 47.6% |
| BAJAJ-AUTO | ma_alignment | 1961 | 0.026 | 0.034 | 47.6% |
| BAJAJHLDNG | ma_alignment | 2084 | 0.111 | 0.148 | 47.4% |
| BHARTIARTL | ma_alignment | 1941 | 0.025 | 0.034 | 47.3% |
| HCLTECH | ma_alignment | 1978 | 0.001 | 0.001 | 47.3% |
| NESTLEIND | ma_alignment | 1944 | -0.005 | -0.007 | 47.1% |
| EICHERMOT | ma_alignment | 2030 | 0.025 | 0.033 | 46.9% |
| DABUR | ma_alignment | 1921 | 0.021 | 0.027 | 46.9% |
| ITC | ma_alignment | 1859 | 0.026 | 0.035 | 46.7% |
| NTPC | ma_alignment | 2064 | 0.022 | 0.030 | 46.6% |
| MARICO | ma_alignment | 2026 | -0.014 | -0.019 | 46.6% |
| 3MINDIA | ma_alignment | 2196 | 0.066 | 0.089 | 46.6% |
| LT | ma_alignment | 1940 | 0.021 | 0.028 | 46.3% |
| GODREJCP | ma_alignment | 2079 | 0.006 | 0.008 | 46.3% |
| MANKIND | ma_alignment | 1311 | 0.095 | 0.127 | 46.2% |
| ABBOTINDIA | ma_alignment | 2139 | 0.026 | 0.034 | 46.2% |
| IRCTC | ma_alignment | 2096 | 0.079 | 0.105 | 46.1% |
| HEROMOTOCO | ma_alignment | 2021 | 0.029 | 0.038 | 46.0% |
| LTM | ma_alignment | 2085 | 0.052 | 0.069 | 45.9% |
| HDFCLIFE | ma_alignment | 2003 | 0.027 | 0.036 | 45.8% |
| ACC | ma_alignment | 2121 | 0.032 | 0.043 | 45.8% |
| HDFCBANK | ma_alignment | 1868 | -0.008 | -0.011 | 45.8% |
| ICICIBANK | ma_alignment | 1915 | -0.010 | -0.014 | 45.8% |
| INDHOTEL | ma_alignment | 2189 | 0.083 | 0.111 | 45.7% |
| CIPLA | ma_alignment | 1980 | -0.027 | -0.036 | 45.7% |
| DMART | ma_alignment | 2031 | 0.059 | 0.078 | 45.6% |
| LUPIN | ma_alignment | 2064 | 0.011 | 0.015 | 45.5% |
| MARUTI | ma_alignment | 1963 | -0.006 | -0.008 | 45.4% |
| HAL | ma_alignment | 2228 | 0.048 | 0.064 | 45.4% |
| BAJAJFINSV | ma_alignment | 2096 | 0.032 | 0.043 | 45.2% |
| COLPAL | ma_alignment | 2019 | -0.010 | -0.014 | 45.2% |
| DIVISLAB | ma_alignment | 2063 | 0.026 | 0.035 | 45.0% |
| M&M | ma_alignment | 2108 | 0.027 | 0.035 | 45.0% |
| COALINDIA | ma_alignment | 2108 | 0.031 | 0.041 | 44.9% |
| CRISIL | ma_alignment | 2319 | 0.062 | 0.083 | 44.8% |
| DRREDDY | ma_alignment | 1974 | -0.026 | -0.034 | 44.7% |
| IOC | ma_alignment | 2093 | 0.058 | 0.077 | 44.6% |
| BERGEPAINT | ma_alignment | 2201 | 0.012 | 0.016 | 44.6% |
| ASTRAL | ma_alignment | 2237 | 0.028 | 0.037 | 44.4% |
| ABB | ma_alignment | 2275 | 0.051 | 0.069 | 44.4% |
| DLF | ma_alignment | 2237 | 0.081 | 0.108 | 44.4% |
| DIXON | ma_alignment | 2355 | 0.082 | 0.109 | 44.3% |
| KOTAKBANK | ma_alignment | 1958 | -0.019 | -0.026 | 44.3% |
| MPHASIS | ma_alignment | 2295 | 0.044 | 0.058 | 44.3% |
| EXIDEIND | ma_alignment | 2203 | 0.030 | 0.040 | 44.3% |
| LICHSGFIN | ma_alignment | 2215 | 0.045 | 0.060 | 44.2% |
| BPCL | ma_alignment | 2144 | 0.030 | 0.040 | 44.2% |
| IDFCFIRSTB | ma_alignment | 2161 | 0.057 | 0.075 | 44.1% |
| HINDZINC | ma_alignment | 2220 | 0.042 | 0.056 | 44.1% |
| GRASIM | ma_alignment | 2111 | -0.015 | -0.020 | 44.0% |
| PAGEIND | ma_alignment | 2083 | 0.002 | 0.002 | 44.0% |
| AUBANK | ma_alignment | 2247 | 0.043 | 0.057 | 43.9% |
| BIOCON | ma_alignment | 2195 | 0.033 | 0.044 | 43.8% |
| INDIGO | ma_alignment | 2190 | 0.031 | 0.041 | 43.8% |
| GLENMARK | ma_alignment | 2295 | 0.025 | 0.034 | 43.7% |
| AWL | ma_alignment | 2005 | 0.040 | 0.054 | 43.7% |
| KEI | ma_alignment | 2374 | 0.110 | 0.146 | 43.7% |
| ABCAPITAL | ma_alignment | 2356 | 0.056 | 0.075 | 43.7% |
| HDFCAMC | ma_alignment | 2180 | 0.009 | 0.012 | 43.7% |
| JUBLFOOD | ma_alignment | 2300 | 0.021 | 0.028 | 43.7% |
| BOSCHLTD | ma_alignment | 2140 | 0.016 | 0.021 | 43.7% |
| INDUSINDBK | ma_alignment | 2198 | 0.027 | 0.036 | 43.7% |
| HAVELLS | ma_alignment | 2137 | -0.000 | -0.001 | 43.7% |
| COROMANDEL | ma_alignment | 2363 | -0.001 | -0.001 | 43.6% |
| HINDALCO | ma_alignment | 2256 | -0.006 | -0.008 | 43.6% |
| APOLLOTYRE | ma_alignment | 2285 | 0.024 | 0.032 | 43.6% |
| OIL | ma_alignment | 2361 | 0.083 | 0.111 | 43.5% |
| ALKEM | ma_alignment | 2132 | -0.018 | -0.024 | 43.5% |
| CHOLAFIN | ma_alignment | 2334 | 0.023 | 0.031 | 43.5% |
| NYKAA | ma_alignment | 2063 | 0.083 | 0.111 | 43.4% |
| AMBUJACEM | ma_alignment | 2239 | 0.051 | 0.067 | 43.4% |
| ADANIENT | ma_alignment | 2445 | 0.106 | 0.142 | 43.4% |
| APLAPOLLO | ma_alignment | 2365 | 0.041 | 0.054 | 43.3% |
| FORTIS | ma_alignment | 2333 | 0.033 | 0.045 | 43.3% |
| MFSL | ma_alignment | 2288 | 0.011 | 0.014 | 43.3% |
| JSWSTEEL | ma_alignment | 2196 | -0.011 | -0.014 | 43.3% |
| MEDANTA | ma_alignment | 1712 | 0.040 | 0.053 | 43.2% |
| GAIL | ma_alignment | 2235 | 0.011 | 0.015 | 43.2% |
| AUROPHARMA | ma_alignment | 2236 | 0.010 | 0.014 | 43.2% |
| ICICIGI | ma_alignment | 2170 | -0.045 | -0.060 | 43.2% |
| KPRMILL | ma_alignment | 2485 | 0.097 | 0.129 | 43.2% |
| CUMMINSIND | ma_alignment | 2252 | 0.001 | 0.001 | 43.1% |
| BEL | ma_alignment | 2211 | 0.033 | 0.044 | 43.1% |
| MOTHERSON | ma_alignment | 2304 | 0.044 | 0.058 | 43.1% |
| ADANIPORTS | ma_alignment | 2293 | 0.059 | 0.079 | 43.1% |
| NLCINDIA | ma_alignment | 2584 | 0.125 | 0.166 | 43.1% |
| LINDEINDIA | ma_alignment | 2459 | 0.077 | 0.103 | 43.1% |
| COFORGE | ma_alignment | 2275 | 0.057 | 0.076 | 43.0% |
| JKCEMENT | ma_alignment | 2264 | 0.034 | 0.045 | 43.0% |
| AJANTPHARM | ma_alignment | 2315 | 0.026 | 0.035 | 43.0% |
| ICICIPRULI | ma_alignment | 2198 | 0.005 | 0.007 | 42.9% |
| CONCOR | ma_alignment | 2232 | 0.027 | 0.036 | 42.9% |
