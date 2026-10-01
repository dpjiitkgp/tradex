# ma-conf-stop075 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 3.0x risk (2.250%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 165 instruments, 2021-03-23..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| ma_alignment | 384316 | 0.029 | 0.039 | 43.4% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| HONAUT | ma_alignment | 1998 | 0.084 | 0.112 | 49.7% |
| LICI | ma_alignment | 1606 | 0.094 | 0.125 | 49.4% |
| APOLLOHOSP | ma_alignment | 2023 | 0.058 | 0.078 | 48.4% |
| AXISBANK | ma_alignment | 1977 | 0.035 | 0.046 | 48.3% |
| ASIANPAINT | ma_alignment | 1896 | 0.026 | 0.034 | 48.2% |
| BRITANNIA | ma_alignment | 1926 | 0.017 | 0.022 | 48.1% |
| ONGC | ma_alignment | 2088 | 0.062 | 0.083 | 48.1% |
| BAJAJ-AUTO | ma_alignment | 1971 | 0.027 | 0.036 | 47.7% |
| BAJAJHLDNG | ma_alignment | 2167 | 0.105 | 0.140 | 47.7% |
| HINDUNILVR | ma_alignment | 1873 | 0.017 | 0.023 | 47.7% |
| HCLTECH | ma_alignment | 1998 | 0.008 | 0.011 | 47.5% |
| INFY | ma_alignment | 1888 | 0.000 | 0.001 | 47.5% |
| EICHERMOT | ma_alignment | 2053 | 0.021 | 0.028 | 47.2% |
| BHARTIARTL | ma_alignment | 1964 | 0.023 | 0.031 | 47.2% |
| MANKIND | ma_alignment | 1362 | 0.091 | 0.121 | 47.1% |
| NESTLEIND | ma_alignment | 1969 | -0.006 | -0.008 | 47.0% |
| DABUR | ma_alignment | 1926 | 0.020 | 0.027 | 46.9% |
| NTPC | ma_alignment | 2089 | 0.020 | 0.027 | 46.9% |
| 3MINDIA | ma_alignment | 2247 | 0.058 | 0.077 | 46.7% |
| LT | ma_alignment | 1958 | 0.025 | 0.034 | 46.6% |
| ITC | ma_alignment | 1873 | 0.021 | 0.027 | 46.6% |
| IRCTC | ma_alignment | 2169 | 0.073 | 0.098 | 46.5% |
| MARICO | ma_alignment | 2038 | -0.013 | -0.018 | 46.4% |
| ACC | ma_alignment | 2178 | 0.033 | 0.043 | 46.4% |
| ABBOTINDIA | ma_alignment | 2170 | 0.021 | 0.027 | 46.2% |
| DMART | ma_alignment | 2075 | 0.058 | 0.078 | 46.2% |
| GODREJCP | ma_alignment | 2106 | 0.005 | 0.006 | 46.2% |
| HEROMOTOCO | ma_alignment | 2053 | 0.028 | 0.038 | 46.2% |
| HDFCBANK | ma_alignment | 1872 | -0.005 | -0.007 | 46.0% |
| LTM | ma_alignment | 2142 | 0.046 | 0.061 | 46.0% |
| INDHOTEL | ma_alignment | 2290 | 0.079 | 0.106 | 46.0% |
| HDFCLIFE | ma_alignment | 2029 | 0.029 | 0.039 | 45.9% |
| ICICIBANK | ma_alignment | 1935 | -0.009 | -0.011 | 45.8% |
| CIPLA | ma_alignment | 1997 | -0.024 | -0.032 | 45.8% |
| HAL | ma_alignment | 2311 | 0.051 | 0.068 | 45.5% |
| BAJAJFINSV | ma_alignment | 2127 | 0.030 | 0.040 | 45.5% |
| CRISIL | ma_alignment | 2410 | 0.062 | 0.082 | 45.4% |
| LUPIN | ma_alignment | 2102 | 0.015 | 0.020 | 45.4% |
| M&M | ma_alignment | 2154 | 0.028 | 0.037 | 45.3% |
| COLPAL | ma_alignment | 2042 | -0.012 | -0.016 | 45.2% |
| MARUTI | ma_alignment | 1987 | -0.013 | -0.017 | 45.2% |
| DIVISLAB | ma_alignment | 2086 | 0.023 | 0.031 | 45.1% |
| COALINDIA | ma_alignment | 2159 | 0.026 | 0.035 | 45.0% |
| ABB | ma_alignment | 2371 | 0.051 | 0.068 | 45.0% |
| MPHASIS | ma_alignment | 2373 | 0.060 | 0.080 | 45.0% |
| IOC | ma_alignment | 2155 | 0.052 | 0.069 | 44.8% |
| DLF | ma_alignment | 2322 | 0.084 | 0.112 | 44.8% |
| CHOLAFIN | ma_alignment | 2436 | 0.037 | 0.049 | 44.8% |
| EXIDEIND | ma_alignment | 2246 | 0.038 | 0.051 | 44.7% |
| ASTRAL | ma_alignment | 2321 | 0.026 | 0.035 | 44.7% |
| DRREDDY | ma_alignment | 2003 | -0.030 | -0.040 | 44.6% |
| OIL | ma_alignment | 2458 | 0.088 | 0.117 | 44.6% |
| KOTAKBANK | ma_alignment | 1969 | -0.015 | -0.020 | 44.6% |
| DIXON | ma_alignment | 2481 | 0.074 | 0.099 | 44.5% |
| BERGEPAINT | ma_alignment | 2225 | 0.008 | 0.011 | 44.4% |
| AUBANK | ma_alignment | 2331 | 0.046 | 0.062 | 44.4% |
| BPCL | ma_alignment | 2184 | 0.025 | 0.033 | 44.4% |
| AWL | ma_alignment | 2108 | 0.041 | 0.055 | 44.3% |
| GLENMARK | ma_alignment | 2361 | 0.023 | 0.031 | 44.3% |
| LICHSGFIN | ma_alignment | 2287 | 0.037 | 0.049 | 44.2% |
| IDFCFIRSTB | ma_alignment | 2249 | 0.055 | 0.074 | 44.2% |
| JUBLFOOD | ma_alignment | 2366 | 0.025 | 0.034 | 44.2% |
| GRASIM | ma_alignment | 2139 | -0.018 | -0.023 | 44.1% |
| INDUSINDBK | ma_alignment | 2239 | 0.027 | 0.036 | 44.1% |
| PAGEIND | ma_alignment | 2119 | -0.002 | -0.002 | 44.1% |
| INDIGO | ma_alignment | 2244 | 0.027 | 0.036 | 44.1% |
| NYKAA | ma_alignment | 2190 | 0.073 | 0.098 | 44.1% |
| HINDZINC | ma_alignment | 2298 | 0.027 | 0.036 | 44.0% |
| ADANIENT | ma_alignment | 2576 | 0.104 | 0.139 | 44.0% |
| BOSCHLTD | ma_alignment | 2170 | 0.008 | 0.011 | 44.0% |
| COROMANDEL | ma_alignment | 2432 | 0.002 | 0.003 | 44.0% |
| ABCAPITAL | ma_alignment | 2454 | 0.045 | 0.061 | 43.9% |
| MEDANTA | ma_alignment | 1770 | 0.039 | 0.052 | 43.9% |
| AMBUJACEM | ma_alignment | 2335 | 0.035 | 0.046 | 43.9% |
| BIOCON | ma_alignment | 2276 | 0.030 | 0.040 | 43.9% |
| MOTHERSON | ma_alignment | 2390 | 0.047 | 0.063 | 43.9% |
| APOLLOTYRE | ma_alignment | 2338 | 0.032 | 0.042 | 43.9% |
| HINDALCO | ma_alignment | 2313 | -0.006 | -0.008 | 43.9% |
| KPRMILL | ma_alignment | 2618 | 0.108 | 0.144 | 43.9% |
| KEI | ma_alignment | 2520 | 0.094 | 0.126 | 43.8% |
| NLCINDIA | ma_alignment | 2733 | 0.101 | 0.134 | 43.8% |
| GODREJPROP | ma_alignment | 2555 | 0.062 | 0.083 | 43.8% |
| ICICIGI | ma_alignment | 2192 | -0.035 | -0.047 | 43.8% |
| HAVELLS | ma_alignment | 2182 | -0.005 | -0.007 | 43.8% |
| JKCEMENT | ma_alignment | 2360 | 0.034 | 0.046 | 43.7% |
| CUMMINSIND | ma_alignment | 2329 | 0.010 | 0.013 | 43.7% |
| HDFCAMC | ma_alignment | 2224 | 0.010 | 0.014 | 43.6% |
| AUROPHARMA | ma_alignment | 2291 | 0.014 | 0.018 | 43.6% |
| ADANIPORTS | ma_alignment | 2395 | 0.055 | 0.074 | 43.6% |
| ALKEM | ma_alignment | 2173 | -0.020 | -0.026 | 43.6% |
| FORTIS | ma_alignment | 2422 | 0.039 | 0.052 | 43.6% |
| LINDEINDIA | ma_alignment | 2576 | 0.072 | 0.097 | 43.5% |
| MAXHEALTH | ma_alignment | 2409 | 0.056 | 0.075 | 43.5% |
| MFSL | ma_alignment | 2376 | 0.009 | 0.012 | 43.4% |
| AJANTPHARM | ma_alignment | 2406 | 0.013 | 0.017 | 43.4% |
| ESCORTS | ma_alignment | 2307 | 0.003 | 0.004 | 43.4% |
| GAIL | ma_alignment | 2283 | 0.016 | 0.022 | 43.4% |
| COFORGE | ma_alignment | 2356 | 0.056 | 0.074 | 43.3% |
| ENDURANCE | ma_alignment | 2525 | 0.010 | 0.014 | 43.3% |
| APLAPOLLO | ma_alignment | 2484 | 0.027 | 0.036 | 43.3% |
