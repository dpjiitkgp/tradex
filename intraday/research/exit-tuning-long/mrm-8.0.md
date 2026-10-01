# exit-tuning-long/mrm-8.0 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 8.0x risk (6.000%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 165 instruments, 2021-03-23..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| ma_alignment | 365699 | 0.033 | 0.045 | 43.0% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| HONAUT | ma_alignment | 1957 | 0.072 | 0.096 | 49.0% |
| LICI | ma_alignment | 1542 | 0.107 | 0.142 | 48.7% |
| ASIANPAINT | ma_alignment | 1869 | 0.037 | 0.050 | 48.6% |
| APOLLOHOSP | ma_alignment | 1959 | 0.057 | 0.076 | 48.4% |
| AXISBANK | ma_alignment | 1947 | 0.032 | 0.043 | 48.3% |
| ONGC | ma_alignment | 2013 | 0.065 | 0.086 | 48.1% |
| BRITANNIA | ma_alignment | 1910 | 0.015 | 0.020 | 48.0% |
| HINDUNILVR | ma_alignment | 1857 | 0.019 | 0.025 | 47.7% |
| INFY | ma_alignment | 1861 | -0.001 | -0.001 | 47.6% |
| BAJAJ-AUTO | ma_alignment | 1955 | 0.020 | 0.027 | 47.5% |
| BAJAJHLDNG | ma_alignment | 2056 | 0.107 | 0.143 | 47.5% |
| HCLTECH | ma_alignment | 1969 | 0.004 | 0.006 | 47.4% |
| BHARTIARTL | ma_alignment | 1933 | 0.028 | 0.037 | 47.3% |
| NESTLEIND | ma_alignment | 1933 | -0.004 | -0.005 | 47.2% |
| DABUR | ma_alignment | 1905 | 0.026 | 0.035 | 47.1% |
| EICHERMOT | ma_alignment | 2028 | 0.024 | 0.032 | 46.9% |
| ITC | ma_alignment | 1852 | 0.025 | 0.034 | 46.7% |
| NTPC | ma_alignment | 2057 | 0.026 | 0.035 | 46.6% |
| MARICO | ma_alignment | 2020 | -0.015 | -0.020 | 46.6% |
| GODREJCP | ma_alignment | 2063 | 0.010 | 0.013 | 46.6% |
| MANKIND | ma_alignment | 1296 | 0.093 | 0.124 | 46.5% |
| 3MINDIA | ma_alignment | 2183 | 0.058 | 0.077 | 46.4% |
| LT | ma_alignment | 1937 | 0.021 | 0.028 | 46.3% |
| ABBOTINDIA | ma_alignment | 2132 | 0.026 | 0.035 | 46.2% |
| IRCTC | ma_alignment | 2066 | 0.080 | 0.106 | 46.1% |
| HEROMOTOCO | ma_alignment | 2012 | 0.028 | 0.037 | 46.0% |
| INDHOTEL | ma_alignment | 2157 | 0.083 | 0.111 | 45.9% |
| LTM | ma_alignment | 2067 | 0.045 | 0.060 | 45.9% |
| HDFCBANK | ma_alignment | 1860 | -0.011 | -0.015 | 45.9% |
| ICICIBANK | ma_alignment | 1914 | -0.008 | -0.011 | 45.8% |
| ACC | ma_alignment | 2100 | 0.033 | 0.044 | 45.8% |
| HDFCLIFE | ma_alignment | 1995 | 0.030 | 0.040 | 45.8% |
| CIPLA | ma_alignment | 1971 | -0.024 | -0.032 | 45.7% |
| LUPIN | ma_alignment | 2052 | 0.018 | 0.024 | 45.7% |
| DMART | ma_alignment | 2026 | 0.061 | 0.081 | 45.5% |
| COLPAL | ma_alignment | 2013 | -0.008 | -0.011 | 45.4% |
| MARUTI | ma_alignment | 1959 | -0.008 | -0.011 | 45.3% |
| BAJAJFINSV | ma_alignment | 2086 | 0.031 | 0.041 | 45.3% |
| HAL | ma_alignment | 2213 | 0.042 | 0.056 | 45.1% |
| M&M | ma_alignment | 2102 | 0.023 | 0.030 | 44.9% |
| COALINDIA | ma_alignment | 2093 | 0.027 | 0.036 | 44.9% |
| DIVISLAB | ma_alignment | 2052 | 0.017 | 0.023 | 44.8% |
| DRREDDY | ma_alignment | 1967 | -0.028 | -0.037 | 44.7% |
| CRISIL | ma_alignment | 2303 | 0.063 | 0.085 | 44.7% |
| ASTRAL | ma_alignment | 2212 | 0.030 | 0.040 | 44.5% |
| IOC | ma_alignment | 2084 | 0.054 | 0.072 | 44.5% |
| BERGEPAINT | ma_alignment | 2193 | 0.013 | 0.017 | 44.5% |
| ABB | ma_alignment | 2249 | 0.052 | 0.069 | 44.5% |
| KOTAKBANK | ma_alignment | 1946 | -0.021 | -0.028 | 44.4% |
| LICHSGFIN | ma_alignment | 2196 | 0.048 | 0.064 | 44.3% |
| DLF | ma_alignment | 2226 | 0.077 | 0.102 | 44.2% |
| DIXON | ma_alignment | 2327 | 0.076 | 0.102 | 44.2% |
| EXIDEIND | ma_alignment | 2183 | 0.030 | 0.040 | 44.2% |
| MPHASIS | ma_alignment | 2277 | 0.040 | 0.053 | 44.2% |
| IDFCFIRSTB | ma_alignment | 2150 | 0.053 | 0.071 | 44.1% |
| PAGEIND | ma_alignment | 2071 | 0.003 | 0.004 | 44.1% |
| BPCL | ma_alignment | 2132 | 0.025 | 0.033 | 44.0% |
| GRASIM | ma_alignment | 2108 | -0.015 | -0.020 | 44.0% |
| AWL | ma_alignment | 1968 | 0.053 | 0.071 | 43.9% |
| AUBANK | ma_alignment | 2231 | 0.048 | 0.064 | 43.9% |
| BIOCON | ma_alignment | 2172 | 0.036 | 0.048 | 43.9% |
| GLENMARK | ma_alignment | 2274 | 0.026 | 0.035 | 43.8% |
| HINDZINC | ma_alignment | 2192 | 0.034 | 0.045 | 43.8% |
| HINDALCO | ma_alignment | 2240 | -0.004 | -0.006 | 43.8% |
| BOSCHLTD | ma_alignment | 2117 | 0.014 | 0.019 | 43.7% |
| KEI | ma_alignment | 2310 | 0.114 | 0.153 | 43.7% |
| INDUSINDBK | ma_alignment | 2185 | 0.024 | 0.032 | 43.7% |
| OIL | ma_alignment | 2318 | 0.089 | 0.119 | 43.7% |
| INDIGO | ma_alignment | 2175 | 0.030 | 0.041 | 43.7% |
| HDFCAMC | ma_alignment | 2168 | 0.004 | 0.006 | 43.6% |
| HAVELLS | ma_alignment | 2127 | 0.001 | 0.002 | 43.6% |
| ADANIPORTS | ma_alignment | 2238 | 0.083 | 0.111 | 43.6% |
| JUBLFOOD | ma_alignment | 2284 | 0.023 | 0.031 | 43.6% |
| COROMANDEL | ma_alignment | 2342 | -0.004 | -0.006 | 43.6% |
| CHOLAFIN | ma_alignment | 2310 | 0.026 | 0.034 | 43.6% |
| ABCAPITAL | ma_alignment | 2329 | 0.047 | 0.063 | 43.5% |
| APLAPOLLO | ma_alignment | 2325 | 0.051 | 0.068 | 43.5% |
| FORTIS | ma_alignment | 2293 | 0.036 | 0.048 | 43.5% |
| APOLLOTYRE | ma_alignment | 2275 | 0.023 | 0.031 | 43.5% |
| ALKEM | ma_alignment | 2123 | -0.025 | -0.033 | 43.4% |
| ICICIGI | ma_alignment | 2157 | -0.042 | -0.056 | 43.3% |
| AMBUJACEM | ma_alignment | 2199 | 0.048 | 0.064 | 43.3% |
| ADANIENT | ma_alignment | 2366 | 0.129 | 0.172 | 43.3% |
| AUROPHARMA | ma_alignment | 2215 | 0.007 | 0.009 | 43.3% |
| JSWSTEEL | ma_alignment | 2185 | -0.011 | -0.015 | 43.2% |
| CUMMINSIND | ma_alignment | 2241 | 0.008 | 0.011 | 43.2% |
| MEDANTA | ma_alignment | 1688 | 0.035 | 0.047 | 43.2% |
| BEL | ma_alignment | 2199 | 0.033 | 0.044 | 43.2% |
| LINDEINDIA | ma_alignment | 2395 | 0.092 | 0.123 | 43.1% |
| GAIL | ma_alignment | 2220 | 0.007 | 0.009 | 43.1% |
| GODREJPROP | ma_alignment | 2389 | 0.070 | 0.093 | 43.0% |
| AJANTPHARM | ma_alignment | 2287 | 0.023 | 0.030 | 43.0% |
| JKCEMENT | ma_alignment | 2257 | 0.033 | 0.043 | 43.0% |
| MFSL | ma_alignment | 2286 | 0.003 | 0.004 | 43.0% |
| CONCOR | ma_alignment | 2202 | 0.030 | 0.040 | 42.9% |
| NLCINDIA | ma_alignment | 2522 | 0.121 | 0.161 | 42.9% |
| NYKAA | ma_alignment | 2031 | 0.071 | 0.095 | 42.9% |
| KPRMILL | ma_alignment | 2426 | 0.100 | 0.134 | 42.9% |
| MOTHERSON | ma_alignment | 2280 | 0.032 | 0.043 | 42.9% |
| ICICIPRULI | ma_alignment | 2191 | 0.001 | 0.001 | 42.8% |
