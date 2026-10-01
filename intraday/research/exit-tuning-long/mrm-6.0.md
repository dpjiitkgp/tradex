# exit-tuning-long/mrm-6.0 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 6.0x risk (4.500%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 165 instruments, 2021-03-23..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| ma_alignment | 368019 | 0.033 | 0.044 | 43.0% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| HONAUT | ma_alignment | 1957 | 0.071 | 0.094 | 49.0% |
| LICI | ma_alignment | 1565 | 0.109 | 0.145 | 48.8% |
| ASIANPAINT | ma_alignment | 1879 | 0.031 | 0.041 | 48.3% |
| APOLLOHOSP | ma_alignment | 1975 | 0.055 | 0.073 | 48.2% |
| AXISBANK | ma_alignment | 1951 | 0.031 | 0.041 | 48.2% |
| ONGC | ma_alignment | 2019 | 0.064 | 0.085 | 48.1% |
| BRITANNIA | ma_alignment | 1912 | 0.016 | 0.021 | 48.0% |
| HINDUNILVR | ma_alignment | 1859 | 0.020 | 0.026 | 47.7% |
| INFY | ma_alignment | 1863 | -0.000 | -0.001 | 47.6% |
| BAJAJ-AUTO | ma_alignment | 1959 | 0.022 | 0.029 | 47.5% |
| BHARTIARTL | ma_alignment | 1933 | 0.030 | 0.039 | 47.4% |
| BAJAJHLDNG | ma_alignment | 2077 | 0.105 | 0.140 | 47.3% |
| HCLTECH | ma_alignment | 1980 | 0.004 | 0.006 | 47.3% |
| NESTLEIND | ma_alignment | 1944 | -0.005 | -0.007 | 47.0% |
| DABUR | ma_alignment | 1917 | 0.022 | 0.030 | 46.9% |
| EICHERMOT | ma_alignment | 2030 | 0.025 | 0.033 | 46.9% |
| ITC | ma_alignment | 1857 | 0.024 | 0.032 | 46.6% |
| NTPC | ma_alignment | 2062 | 0.024 | 0.032 | 46.6% |
| MARICO | ma_alignment | 2024 | -0.017 | -0.022 | 46.5% |
| GODREJCP | ma_alignment | 2071 | 0.008 | 0.011 | 46.5% |
| 3MINDIA | ma_alignment | 2192 | 0.062 | 0.082 | 46.4% |
| MANKIND | ma_alignment | 1301 | 0.092 | 0.123 | 46.3% |
| LT | ma_alignment | 1939 | 0.021 | 0.029 | 46.3% |
| ABBOTINDIA | ma_alignment | 2134 | 0.025 | 0.033 | 46.2% |
| IRCTC | ma_alignment | 2087 | 0.080 | 0.107 | 46.1% |
| HEROMOTOCO | ma_alignment | 2016 | 0.026 | 0.035 | 46.0% |
| INDHOTEL | ma_alignment | 2166 | 0.088 | 0.118 | 46.0% |
| HDFCBANK | ma_alignment | 1860 | -0.004 | -0.006 | 46.0% |
| HDFCLIFE | ma_alignment | 2002 | 0.029 | 0.039 | 45.9% |
| ACC | ma_alignment | 2109 | 0.034 | 0.045 | 45.8% |
| ICICIBANK | ma_alignment | 1917 | -0.011 | -0.014 | 45.7% |
| CIPLA | ma_alignment | 1976 | -0.027 | -0.035 | 45.7% |
| LTM | ma_alignment | 2085 | 0.040 | 0.053 | 45.6% |
| LUPIN | ma_alignment | 2060 | 0.012 | 0.017 | 45.5% |
| DMART | ma_alignment | 2026 | 0.056 | 0.075 | 45.5% |
| BAJAJFINSV | ma_alignment | 2090 | 0.037 | 0.049 | 45.4% |
| MARUTI | ma_alignment | 1964 | -0.008 | -0.010 | 45.3% |
| COLPAL | ma_alignment | 2019 | -0.010 | -0.014 | 45.2% |
| HAL | ma_alignment | 2225 | 0.044 | 0.059 | 45.2% |
| DIVISLAB | ma_alignment | 2055 | 0.023 | 0.031 | 44.9% |
| M&M | ma_alignment | 2105 | 0.023 | 0.031 | 44.9% |
| COALINDIA | ma_alignment | 2099 | 0.034 | 0.045 | 44.9% |
| CRISIL | ma_alignment | 2312 | 0.065 | 0.086 | 44.7% |
| DRREDDY | ma_alignment | 1972 | -0.029 | -0.039 | 44.7% |
| IOC | ma_alignment | 2091 | 0.054 | 0.073 | 44.5% |
| ABB | ma_alignment | 2269 | 0.055 | 0.074 | 44.5% |
| BERGEPAINT | ma_alignment | 2196 | 0.013 | 0.017 | 44.5% |
| ASTRAL | ma_alignment | 2221 | 0.028 | 0.037 | 44.5% |
| KOTAKBANK | ma_alignment | 1946 | -0.021 | -0.028 | 44.4% |
| LICHSGFIN | ma_alignment | 2202 | 0.046 | 0.061 | 44.3% |
| DLF | ma_alignment | 2227 | 0.071 | 0.095 | 44.3% |
| EXIDEIND | ma_alignment | 2190 | 0.032 | 0.043 | 44.2% |
| MPHASIS | ma_alignment | 2289 | 0.035 | 0.046 | 44.2% |
| DIXON | ma_alignment | 2352 | 0.076 | 0.101 | 44.0% |
| IDFCFIRSTB | ma_alignment | 2155 | 0.054 | 0.072 | 44.0% |
| BPCL | ma_alignment | 2139 | 0.023 | 0.031 | 44.0% |
| GRASIM | ma_alignment | 2110 | -0.015 | -0.019 | 44.0% |
| BOSCHLTD | ma_alignment | 2117 | 0.018 | 0.024 | 43.9% |
| AUBANK | ma_alignment | 2239 | 0.046 | 0.061 | 43.9% |
| BIOCON | ma_alignment | 2182 | 0.033 | 0.044 | 43.9% |
| GLENMARK | ma_alignment | 2276 | 0.026 | 0.034 | 43.8% |
| PAGEIND | ma_alignment | 2085 | -0.002 | -0.003 | 43.8% |
| HINDZINC | ma_alignment | 2205 | 0.035 | 0.046 | 43.8% |
| INDIGO | ma_alignment | 2181 | 0.034 | 0.045 | 43.8% |
| HDFCAMC | ma_alignment | 2176 | 0.009 | 0.012 | 43.7% |
| HAVELLS | ma_alignment | 2129 | 0.003 | 0.004 | 43.7% |
| AWL | ma_alignment | 1993 | 0.043 | 0.058 | 43.7% |
| HINDALCO | ma_alignment | 2252 | -0.004 | -0.005 | 43.7% |
| INDUSINDBK | ma_alignment | 2193 | 0.026 | 0.035 | 43.6% |
| OIL | ma_alignment | 2342 | 0.088 | 0.117 | 43.6% |
| JUBLFOOD | ma_alignment | 2290 | 0.020 | 0.027 | 43.6% |
| COROMANDEL | ma_alignment | 2352 | -0.004 | -0.005 | 43.6% |
| APOLLOTYRE | ma_alignment | 2278 | 0.024 | 0.033 | 43.6% |
| ABCAPITAL | ma_alignment | 2336 | 0.051 | 0.068 | 43.6% |
| CHOLAFIN | ma_alignment | 2321 | 0.021 | 0.029 | 43.5% |
| KEI | ma_alignment | 2338 | 0.102 | 0.136 | 43.5% |
| ALKEM | ma_alignment | 2129 | -0.021 | -0.028 | 43.4% |
| AMBUJACEM | ma_alignment | 2224 | 0.049 | 0.066 | 43.4% |
| FORTIS | ma_alignment | 2319 | 0.032 | 0.043 | 43.4% |
| APLAPOLLO | ma_alignment | 2352 | 0.042 | 0.056 | 43.3% |
| AUROPHARMA | ma_alignment | 2221 | 0.005 | 0.007 | 43.3% |
| ADANIENT | ma_alignment | 2412 | 0.107 | 0.142 | 43.2% |
| ADANIPORTS | ma_alignment | 2264 | 0.065 | 0.087 | 43.2% |
| ICICIGI | ma_alignment | 2167 | -0.045 | -0.060 | 43.2% |
| JSWSTEEL | ma_alignment | 2186 | -0.011 | -0.015 | 43.2% |
| BEL | ma_alignment | 2203 | 0.035 | 0.047 | 43.2% |
| MFSL | ma_alignment | 2278 | 0.005 | 0.007 | 43.2% |
| CUMMINSIND | ma_alignment | 2246 | 0.005 | 0.006 | 43.1% |
| MEDANTA | ma_alignment | 1697 | 0.036 | 0.049 | 43.1% |
| NYKAA | ma_alignment | 2045 | 0.074 | 0.099 | 43.1% |
| GODREJPROP | ma_alignment | 2401 | 0.073 | 0.097 | 43.1% |
| LINDEINDIA | ma_alignment | 2415 | 0.082 | 0.109 | 43.0% |
| AJANTPHARM | ma_alignment | 2299 | 0.024 | 0.032 | 43.0% |
| GAIL | ma_alignment | 2225 | 0.008 | 0.011 | 43.0% |
| ICICIPRULI | ma_alignment | 2194 | 0.007 | 0.010 | 43.0% |
| NLCINDIA | ma_alignment | 2543 | 0.123 | 0.164 | 43.0% |
| JKCEMENT | ma_alignment | 2262 | 0.032 | 0.043 | 43.0% |
| MOTHERSON | ma_alignment | 2289 | 0.038 | 0.051 | 42.9% |
| KPRMILL | ma_alignment | 2452 | 0.102 | 0.135 | 42.8% |
| CONCOR | ma_alignment | 2221 | 0.030 | 0.039 | 42.8% |
