# exit-tuning-long/mrm-4.0 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 4.0x risk (3.000%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 165 instruments, 2021-03-23..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| ma_alignment | 374893 | 0.031 | 0.042 | 43.1% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| HONAUT | ma_alignment | 1973 | 0.080 | 0.107 | 49.1% |
| LICI | ma_alignment | 1582 | 0.096 | 0.128 | 48.9% |
| ASIANPAINT | ma_alignment | 1886 | 0.028 | 0.038 | 48.3% |
| APOLLOHOSP | ma_alignment | 1996 | 0.057 | 0.076 | 48.2% |
| ONGC | ma_alignment | 2041 | 0.069 | 0.092 | 48.2% |
| AXISBANK | ma_alignment | 1958 | 0.033 | 0.044 | 48.1% |
| BRITANNIA | ma_alignment | 1916 | 0.016 | 0.021 | 48.0% |
| HINDUNILVR | ma_alignment | 1863 | 0.019 | 0.025 | 47.7% |
| INFY | ma_alignment | 1876 | 0.000 | 0.000 | 47.6% |
| BAJAJ-AUTO | ma_alignment | 1965 | 0.025 | 0.034 | 47.6% |
| BAJAJHLDNG | ma_alignment | 2111 | 0.113 | 0.151 | 47.6% |
| HCLTECH | ma_alignment | 1982 | 0.002 | 0.003 | 47.4% |
| BHARTIARTL | ma_alignment | 1949 | 0.026 | 0.035 | 47.3% |
| NESTLEIND | ma_alignment | 1954 | -0.007 | -0.009 | 47.0% |
| EICHERMOT | ma_alignment | 2033 | 0.022 | 0.030 | 47.0% |
| DABUR | ma_alignment | 1921 | 0.021 | 0.027 | 46.9% |
| MARICO | ma_alignment | 2026 | -0.013 | -0.018 | 46.6% |
| 3MINDIA | ma_alignment | 2208 | 0.062 | 0.083 | 46.6% |
| NTPC | ma_alignment | 2076 | 0.021 | 0.028 | 46.5% |
| ITC | ma_alignment | 1868 | 0.022 | 0.030 | 46.5% |
| LT | ma_alignment | 1944 | 0.021 | 0.028 | 46.3% |
| ABBOTINDIA | ma_alignment | 2148 | 0.025 | 0.033 | 46.3% |
| GODREJCP | ma_alignment | 2087 | 0.006 | 0.008 | 46.2% |
| IRCTC | ma_alignment | 2116 | 0.080 | 0.106 | 46.2% |
| MANKIND | ma_alignment | 1337 | 0.079 | 0.106 | 46.1% |
| HEROMOTOCO | ma_alignment | 2028 | 0.031 | 0.042 | 46.1% |
| ACC | ma_alignment | 2135 | 0.032 | 0.043 | 46.0% |
| LTM | ma_alignment | 2107 | 0.051 | 0.068 | 46.0% |
| HDFCBANK | ma_alignment | 1867 | -0.009 | -0.012 | 45.8% |
| DMART | ma_alignment | 2043 | 0.061 | 0.081 | 45.8% |
| HDFCLIFE | ma_alignment | 2009 | 0.024 | 0.032 | 45.7% |
| ICICIBANK | ma_alignment | 1924 | -0.011 | -0.015 | 45.7% |
| LUPIN | ma_alignment | 2073 | 0.019 | 0.026 | 45.7% |
| CIPLA | ma_alignment | 1991 | -0.028 | -0.037 | 45.7% |
| MARUTI | ma_alignment | 1969 | -0.006 | -0.008 | 45.4% |
| INDHOTEL | ma_alignment | 2225 | 0.080 | 0.106 | 45.4% |
| HAL | ma_alignment | 2252 | 0.048 | 0.064 | 45.4% |
| COLPAL | ma_alignment | 2031 | -0.012 | -0.015 | 45.3% |
| BAJAJFINSV | ma_alignment | 2102 | 0.029 | 0.039 | 45.2% |
| CRISIL | ma_alignment | 2354 | 0.067 | 0.090 | 45.2% |
| DIVISLAB | ma_alignment | 2065 | 0.026 | 0.034 | 45.0% |
| M&M | ma_alignment | 2122 | 0.026 | 0.034 | 44.9% |
| COALINDIA | ma_alignment | 2135 | 0.027 | 0.036 | 44.8% |
| DRREDDY | ma_alignment | 1984 | -0.027 | -0.037 | 44.8% |
| ABB | ma_alignment | 2305 | 0.057 | 0.076 | 44.6% |
| DIXON | ma_alignment | 2397 | 0.087 | 0.117 | 44.6% |
| IOC | ma_alignment | 2116 | 0.051 | 0.068 | 44.6% |
| BERGEPAINT | ma_alignment | 2212 | 0.009 | 0.012 | 44.5% |
| DLF | ma_alignment | 2262 | 0.081 | 0.108 | 44.5% |
| ASTRAL | ma_alignment | 2260 | 0.026 | 0.035 | 44.5% |
| MPHASIS | ma_alignment | 2310 | 0.045 | 0.060 | 44.5% |
| LICHSGFIN | ma_alignment | 2239 | 0.049 | 0.066 | 44.4% |
| KOTAKBANK | ma_alignment | 1954 | -0.019 | -0.025 | 44.4% |
| BPCL | ma_alignment | 2154 | 0.034 | 0.045 | 44.3% |
| AUBANK | ma_alignment | 2276 | 0.050 | 0.067 | 44.2% |
| EXIDEIND | ma_alignment | 2211 | 0.030 | 0.040 | 44.2% |
| GRASIM | ma_alignment | 2120 | -0.013 | -0.018 | 44.1% |
| IDFCFIRSTB | ma_alignment | 2192 | 0.053 | 0.071 | 44.1% |
| AWL | ma_alignment | 2047 | 0.043 | 0.058 | 44.1% |
| PAGEIND | ma_alignment | 2090 | -0.002 | -0.003 | 44.0% |
| INDUSINDBK | ma_alignment | 2213 | 0.029 | 0.038 | 44.0% |
| HINDZINC | ma_alignment | 2250 | 0.032 | 0.043 | 44.0% |
| BIOCON | ma_alignment | 2218 | 0.035 | 0.047 | 43.9% |
| BOSCHLTD | ma_alignment | 2144 | 0.013 | 0.017 | 43.9% |
| ABCAPITAL | ma_alignment | 2384 | 0.046 | 0.062 | 43.9% |
| KEI | ma_alignment | 2425 | 0.106 | 0.142 | 43.8% |
| INDIGO | ma_alignment | 2204 | 0.026 | 0.034 | 43.8% |
| HAVELLS | ma_alignment | 2146 | -0.000 | -0.000 | 43.8% |
| GLENMARK | ma_alignment | 2320 | 0.024 | 0.033 | 43.8% |
| NYKAA | ma_alignment | 2108 | 0.083 | 0.111 | 43.8% |
| COROMANDEL | ma_alignment | 2374 | 0.003 | 0.004 | 43.8% |
| OIL | ma_alignment | 2395 | 0.087 | 0.116 | 43.8% |
| HINDALCO | ma_alignment | 2268 | -0.007 | -0.009 | 43.7% |
| CHOLAFIN | ma_alignment | 2368 | 0.021 | 0.029 | 43.7% |
| JUBLFOOD | ma_alignment | 2320 | 0.017 | 0.022 | 43.7% |
| ADANIENT | ma_alignment | 2490 | 0.105 | 0.141 | 43.7% |
| ICICIGI | ma_alignment | 2167 | -0.035 | -0.046 | 43.6% |
| ADANIPORTS | ma_alignment | 2326 | 0.066 | 0.088 | 43.6% |
| HDFCAMC | ma_alignment | 2196 | 0.005 | 0.006 | 43.5% |
| APOLLOTYRE | ma_alignment | 2300 | 0.026 | 0.035 | 43.5% |
| MEDANTA | ma_alignment | 1734 | 0.039 | 0.052 | 43.5% |
| ALKEM | ma_alignment | 2148 | -0.021 | -0.028 | 43.4% |
| AUROPHARMA | ma_alignment | 2253 | 0.013 | 0.017 | 43.4% |
| NLCINDIA | ma_alignment | 2623 | 0.118 | 0.157 | 43.4% |
| KPRMILL | ma_alignment | 2530 | 0.096 | 0.129 | 43.4% |
| APLAPOLLO | ma_alignment | 2404 | 0.043 | 0.057 | 43.3% |
| LINDEINDIA | ma_alignment | 2493 | 0.081 | 0.107 | 43.3% |
| FORTIS | ma_alignment | 2362 | 0.039 | 0.052 | 43.3% |
| AMBUJACEM | ma_alignment | 2270 | 0.039 | 0.052 | 43.3% |
| GAIL | ma_alignment | 2243 | 0.015 | 0.020 | 43.2% |
| BEL | ma_alignment | 2230 | 0.034 | 0.046 | 43.2% |
| JSWSTEEL | ma_alignment | 2205 | -0.009 | -0.012 | 43.2% |
| COFORGE | ma_alignment | 2296 | 0.058 | 0.077 | 43.2% |
| ICICIPRULI | ma_alignment | 2208 | 0.010 | 0.014 | 43.2% |
| MFSL | ma_alignment | 2317 | 0.008 | 0.011 | 43.2% |
| CUMMINSIND | ma_alignment | 2274 | -0.003 | -0.003 | 43.1% |
| GODREJPROP | ma_alignment | 2468 | 0.070 | 0.094 | 43.1% |
| MOTHERSON | ma_alignment | 2342 | 0.043 | 0.057 | 43.0% |
| AJANTPHARM | ma_alignment | 2338 | 0.019 | 0.025 | 43.0% |
| ENDURANCE | ma_alignment | 2449 | 0.010 | 0.013 | 43.0% |
