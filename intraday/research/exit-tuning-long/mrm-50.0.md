# exit-tuning-long/mrm-50.0 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 50.0x risk (37.500%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 165 instruments, 2021-03-23..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| ma_alignment | 363299 | 0.030 | 0.040 | 43.1% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LICI | ma_alignment | 1527 | 0.106 | 0.141 | 49.0% |
| HONAUT | ma_alignment | 1952 | 0.077 | 0.103 | 48.9% |
| ASIANPAINT | ma_alignment | 1866 | 0.031 | 0.041 | 48.5% |
| APOLLOHOSP | ma_alignment | 1959 | 0.057 | 0.077 | 48.4% |
| AXISBANK | ma_alignment | 1947 | 0.032 | 0.043 | 48.3% |
| ONGC | ma_alignment | 2009 | 0.068 | 0.091 | 48.1% |
| BRITANNIA | ma_alignment | 1904 | 0.011 | 0.014 | 48.0% |
| HINDUNILVR | ma_alignment | 1851 | 0.018 | 0.024 | 47.8% |
| INFY | ma_alignment | 1858 | -0.002 | -0.003 | 47.6% |
| HCLTECH | ma_alignment | 1961 | 0.002 | 0.003 | 47.5% |
| BAJAJ-AUTO | ma_alignment | 1955 | 0.020 | 0.027 | 47.5% |
| BAJAJHLDNG | ma_alignment | 2053 | 0.100 | 0.133 | 47.4% |
| BHARTIARTL | ma_alignment | 1929 | 0.024 | 0.032 | 47.4% |
| NESTLEIND | ma_alignment | 1932 | -0.007 | -0.009 | 47.2% |
| DABUR | ma_alignment | 1901 | 0.021 | 0.028 | 47.1% |
| EICHERMOT | ma_alignment | 2021 | 0.021 | 0.028 | 47.0% |
| GODREJCP | ma_alignment | 2054 | 0.015 | 0.020 | 46.7% |
| ITC | ma_alignment | 1847 | 0.023 | 0.031 | 46.7% |
| NTPC | ma_alignment | 2049 | 0.027 | 0.035 | 46.7% |
| MARICO | ma_alignment | 2014 | -0.016 | -0.021 | 46.7% |
| 3MINDIA | ma_alignment | 2182 | 0.055 | 0.074 | 46.4% |
| MANKIND | ma_alignment | 1293 | 0.081 | 0.107 | 46.4% |
| LT | ma_alignment | 1935 | 0.022 | 0.030 | 46.3% |
| ABBOTINDIA | ma_alignment | 2131 | 0.027 | 0.035 | 46.2% |
| IRCTC | ma_alignment | 2059 | 0.079 | 0.106 | 46.0% |
| LTM | ma_alignment | 2055 | 0.042 | 0.056 | 46.0% |
| HEROMOTOCO | ma_alignment | 2010 | 0.022 | 0.029 | 46.0% |
| HDFCBANK | ma_alignment | 1854 | -0.011 | -0.014 | 46.0% |
| INDHOTEL | ma_alignment | 2150 | 0.075 | 0.101 | 45.9% |
| ICICIBANK | ma_alignment | 1909 | -0.010 | -0.013 | 45.9% |
| HDFCLIFE | ma_alignment | 1988 | 0.024 | 0.032 | 45.8% |
| ACC | ma_alignment | 2098 | 0.036 | 0.047 | 45.8% |
| CIPLA | ma_alignment | 1965 | -0.026 | -0.035 | 45.8% |
| LUPIN | ma_alignment | 2044 | 0.014 | 0.019 | 45.7% |
| DMART | ma_alignment | 2022 | 0.059 | 0.079 | 45.5% |
| HAL | ma_alignment | 2185 | 0.049 | 0.066 | 45.4% |
| BAJAJFINSV | ma_alignment | 2078 | 0.029 | 0.039 | 45.4% |
| MARUTI | ma_alignment | 1956 | -0.012 | -0.016 | 45.3% |
| COLPAL | ma_alignment | 2010 | -0.015 | -0.019 | 45.3% |
| M&M | ma_alignment | 2098 | 0.021 | 0.028 | 44.9% |
| COALINDIA | ma_alignment | 2092 | 0.027 | 0.035 | 44.8% |
| IOC | ma_alignment | 2069 | 0.056 | 0.075 | 44.8% |
| CRISIL | ma_alignment | 2300 | 0.062 | 0.083 | 44.7% |
| DIVISLAB | ma_alignment | 2052 | 0.011 | 0.015 | 44.7% |
| DRREDDY | ma_alignment | 1964 | -0.033 | -0.044 | 44.7% |
| BERGEPAINT | ma_alignment | 2192 | 0.013 | 0.017 | 44.5% |
| KOTAKBANK | ma_alignment | 1946 | -0.021 | -0.028 | 44.4% |
| LICHSGFIN | ma_alignment | 2177 | 0.036 | 0.048 | 44.4% |
| ASTRAL | ma_alignment | 2211 | 0.019 | 0.026 | 44.4% |
| MPHASIS | ma_alignment | 2259 | 0.036 | 0.048 | 44.4% |
| ABB | ma_alignment | 2239 | 0.041 | 0.055 | 44.4% |
| GRASIM | ma_alignment | 2091 | -0.011 | -0.015 | 44.3% |
| DLF | ma_alignment | 2218 | 0.074 | 0.098 | 44.3% |
| DIXON | ma_alignment | 2307 | 0.064 | 0.085 | 44.3% |
| PAGEIND | ma_alignment | 2060 | 0.002 | 0.003 | 44.2% |
| BPCL | ma_alignment | 2121 | 0.024 | 0.032 | 44.1% |
| EXIDEIND | ma_alignment | 2172 | 0.029 | 0.038 | 44.1% |
| HINDZINC | ma_alignment | 2154 | 0.037 | 0.049 | 44.1% |
| IDFCFIRSTB | ma_alignment | 2147 | 0.053 | 0.071 | 44.1% |
| GLENMARK | ma_alignment | 2256 | 0.028 | 0.038 | 44.1% |
| BOSCHLTD | ma_alignment | 2102 | 0.013 | 0.017 | 44.0% |
| JUBLFOOD | ma_alignment | 2261 | 0.020 | 0.027 | 43.9% |
| INDIGO | ma_alignment | 2161 | 0.028 | 0.037 | 43.9% |
| BIOCON | ma_alignment | 2166 | 0.032 | 0.042 | 43.9% |
| ABCAPITAL | ma_alignment | 2310 | 0.050 | 0.067 | 43.9% |
| HINDALCO | ma_alignment | 2229 | -0.009 | -0.012 | 43.8% |
| AUBANK | ma_alignment | 2226 | 0.042 | 0.055 | 43.8% |
| INDUSINDBK | ma_alignment | 2175 | 0.023 | 0.031 | 43.8% |
| HDFCAMC | ma_alignment | 2157 | 0.007 | 0.010 | 43.8% |
| CHOLAFIN | ma_alignment | 2297 | 0.026 | 0.034 | 43.8% |
| KEI | ma_alignment | 2287 | 0.098 | 0.131 | 43.7% |
| OIL | ma_alignment | 2294 | 0.066 | 0.088 | 43.7% |
| FORTIS | ma_alignment | 2272 | 0.034 | 0.045 | 43.7% |
| HAVELLS | ma_alignment | 2121 | -0.004 | -0.005 | 43.7% |
| APLAPOLLO | ma_alignment | 2308 | 0.041 | 0.055 | 43.6% |
| AWL | ma_alignment | 1954 | 0.037 | 0.049 | 43.6% |
| ADANIENT | ma_alignment | 2322 | 0.142 | 0.190 | 43.5% |
| COROMANDEL | ma_alignment | 2336 | -0.005 | -0.006 | 43.5% |
| AMBUJACEM | ma_alignment | 2179 | 0.045 | 0.061 | 43.5% |
| ALKEM | ma_alignment | 2113 | -0.029 | -0.039 | 43.5% |
| APOLLOTYRE | ma_alignment | 2266 | 0.020 | 0.026 | 43.5% |
| CUMMINSIND | ma_alignment | 2221 | 0.009 | 0.012 | 43.4% |
| ICICIGI | ma_alignment | 2150 | -0.045 | -0.060 | 43.4% |
| ADANIPORTS | ma_alignment | 2218 | 0.078 | 0.104 | 43.3% |
| MEDANTA | ma_alignment | 1676 | 0.029 | 0.039 | 43.3% |
| GODREJPROP | ma_alignment | 2364 | 0.069 | 0.092 | 43.2% |
| AUROPHARMA | ma_alignment | 2210 | 0.003 | 0.004 | 43.2% |
| JSWSTEEL | ma_alignment | 2178 | -0.013 | -0.017 | 43.2% |
| BEL | ma_alignment | 2189 | 0.029 | 0.038 | 43.2% |
| LINDEINDIA | ma_alignment | 2372 | 0.095 | 0.127 | 43.1% |
| MFSL | ma_alignment | 2270 | -0.002 | -0.003 | 43.1% |
| CONCOR | ma_alignment | 2186 | 0.030 | 0.040 | 43.1% |
| GAIL | ma_alignment | 2215 | 0.008 | 0.010 | 43.1% |
| AJANTPHARM | ma_alignment | 2284 | 0.016 | 0.022 | 43.0% |
| JKCEMENT | ma_alignment | 2254 | 0.031 | 0.041 | 43.0% |
| NLCINDIA | ma_alignment | 2485 | 0.128 | 0.171 | 43.0% |
| ICICIPRULI | ma_alignment | 2183 | -0.000 | -0.000 | 42.9% |
| NYKAA | ma_alignment | 2011 | 0.066 | 0.089 | 42.9% |
| COFORGE | ma_alignment | 2242 | 0.050 | 0.067 | 42.9% |
| MOTHERSON | ma_alignment | 2277 | 0.035 | 0.047 | 42.9% |
