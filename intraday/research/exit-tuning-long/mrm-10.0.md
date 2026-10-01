# exit-tuning-long/mrm-10.0 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 10.0x risk (7.500%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 165 instruments, 2021-03-23..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| ma_alignment | 364682 | 0.033 | 0.044 | 43.0% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| HONAUT | ma_alignment | 1953 | 0.072 | 0.096 | 49.0% |
| LICI | ma_alignment | 1537 | 0.107 | 0.143 | 48.8% |
| ASIANPAINT | ma_alignment | 1871 | 0.034 | 0.045 | 48.4% |
| APOLLOHOSP | ma_alignment | 1959 | 0.057 | 0.077 | 48.4% |
| AXISBANK | ma_alignment | 1947 | 0.032 | 0.043 | 48.3% |
| ONGC | ma_alignment | 2014 | 0.064 | 0.085 | 48.0% |
| BRITANNIA | ma_alignment | 1907 | 0.015 | 0.020 | 48.0% |
| HINDUNILVR | ma_alignment | 1857 | 0.020 | 0.026 | 47.7% |
| INFY | ma_alignment | 1861 | -0.001 | -0.001 | 47.6% |
| BAJAJ-AUTO | ma_alignment | 1955 | 0.020 | 0.027 | 47.5% |
| HCLTECH | ma_alignment | 1964 | 0.005 | 0.006 | 47.5% |
| BAJAJHLDNG | ma_alignment | 2053 | 0.105 | 0.141 | 47.4% |
| BHARTIARTL | ma_alignment | 1930 | 0.027 | 0.037 | 47.4% |
| DABUR | ma_alignment | 1902 | 0.025 | 0.033 | 47.2% |
| EICHERMOT | ma_alignment | 2024 | 0.024 | 0.032 | 46.9% |
| NESTLEIND | ma_alignment | 1944 | -0.008 | -0.011 | 46.9% |
| MARICO | ma_alignment | 2015 | -0.013 | -0.017 | 46.7% |
| NTPC | ma_alignment | 2053 | 0.024 | 0.032 | 46.7% |
| ITC | ma_alignment | 1852 | 0.025 | 0.034 | 46.7% |
| GODREJCP | ma_alignment | 2061 | 0.019 | 0.026 | 46.6% |
| 3MINDIA | ma_alignment | 2183 | 0.058 | 0.077 | 46.4% |
| LT | ma_alignment | 1937 | 0.023 | 0.030 | 46.3% |
| ABBOTINDIA | ma_alignment | 2131 | 0.027 | 0.035 | 46.2% |
| MANKIND | ma_alignment | 1305 | 0.088 | 0.117 | 46.1% |
| LTM | ma_alignment | 2060 | 0.052 | 0.069 | 46.0% |
| IRCTC | ma_alignment | 2061 | 0.079 | 0.105 | 46.0% |
| HDFCBANK | ma_alignment | 1857 | -0.003 | -0.004 | 46.0% |
| HEROMOTOCO | ma_alignment | 2010 | 0.022 | 0.029 | 46.0% |
| INDHOTEL | ma_alignment | 2152 | 0.081 | 0.108 | 46.0% |
| ICICIBANK | ma_alignment | 1909 | -0.010 | -0.013 | 45.9% |
| HDFCLIFE | ma_alignment | 1992 | 0.029 | 0.039 | 45.8% |
| ACC | ma_alignment | 2102 | 0.042 | 0.056 | 45.8% |
| LUPIN | ma_alignment | 2049 | 0.018 | 0.024 | 45.8% |
| CIPLA | ma_alignment | 1968 | -0.023 | -0.031 | 45.7% |
| DMART | ma_alignment | 2026 | 0.062 | 0.083 | 45.6% |
| MARUTI | ma_alignment | 1957 | -0.008 | -0.011 | 45.4% |
| BAJAJFINSV | ma_alignment | 2083 | 0.031 | 0.042 | 45.4% |
| COLPAL | ma_alignment | 2011 | -0.011 | -0.014 | 45.4% |
| HAL | ma_alignment | 2205 | 0.044 | 0.059 | 45.2% |
| M&M | ma_alignment | 2101 | 0.024 | 0.032 | 44.9% |
| COALINDIA | ma_alignment | 2093 | 0.028 | 0.038 | 44.8% |
| DIVISLAB | ma_alignment | 2051 | 0.018 | 0.024 | 44.8% |
| IOC | ma_alignment | 2075 | 0.059 | 0.078 | 44.7% |
| CRISIL | ma_alignment | 2302 | 0.063 | 0.084 | 44.7% |
| DRREDDY | ma_alignment | 1964 | -0.032 | -0.043 | 44.7% |
| BERGEPAINT | ma_alignment | 2192 | 0.013 | 0.017 | 44.5% |
| ASTRAL | ma_alignment | 2213 | 0.027 | 0.036 | 44.4% |
| ABB | ma_alignment | 2241 | 0.044 | 0.059 | 44.4% |
| KOTAKBANK | ma_alignment | 1946 | -0.021 | -0.028 | 44.4% |
| LICHSGFIN | ma_alignment | 2181 | 0.041 | 0.055 | 44.3% |
| DIXON | ma_alignment | 2311 | 0.072 | 0.096 | 44.3% |
| MPHASIS | ma_alignment | 2271 | 0.039 | 0.052 | 44.3% |
| DLF | ma_alignment | 2226 | 0.077 | 0.102 | 44.2% |
| EXIDEIND | ma_alignment | 2177 | 0.030 | 0.040 | 44.2% |
| PAGEIND | ma_alignment | 2066 | 0.008 | 0.010 | 44.1% |
| BPCL | ma_alignment | 2126 | 0.027 | 0.036 | 44.1% |
| IDFCFIRSTB | ma_alignment | 2149 | 0.053 | 0.071 | 44.1% |
| GRASIM | ma_alignment | 2108 | -0.014 | -0.019 | 44.0% |
| HINDZINC | ma_alignment | 2179 | 0.041 | 0.055 | 43.9% |
| HINDALCO | ma_alignment | 2236 | 0.001 | 0.001 | 43.9% |
| AUBANK | ma_alignment | 2230 | 0.047 | 0.062 | 43.9% |
| GLENMARK | ma_alignment | 2269 | 0.029 | 0.039 | 43.9% |
| BIOCON | ma_alignment | 2170 | 0.034 | 0.046 | 43.8% |
| ABCAPITAL | ma_alignment | 2316 | 0.054 | 0.072 | 43.8% |
| INDUSINDBK | ma_alignment | 2175 | 0.023 | 0.030 | 43.8% |
| INDIGO | ma_alignment | 2169 | 0.029 | 0.038 | 43.8% |
| JUBLFOOD | ma_alignment | 2277 | 0.028 | 0.038 | 43.7% |
| BOSCHLTD | ma_alignment | 2117 | 0.014 | 0.019 | 43.7% |
| CHOLAFIN | ma_alignment | 2303 | 0.028 | 0.038 | 43.7% |
| HAVELLS | ma_alignment | 2122 | -0.000 | -0.001 | 43.7% |
| OIL | ma_alignment | 2314 | 0.084 | 0.111 | 43.6% |
| HDFCAMC | ma_alignment | 2170 | 0.007 | 0.009 | 43.6% |
| KEI | ma_alignment | 2302 | 0.111 | 0.148 | 43.6% |
| FORTIS | ma_alignment | 2284 | 0.035 | 0.047 | 43.6% |
| ADANIENT | ma_alignment | 2342 | 0.143 | 0.191 | 43.6% |
| AWL | ma_alignment | 1959 | 0.040 | 0.054 | 43.5% |
| COROMANDEL | ma_alignment | 2336 | -0.005 | -0.006 | 43.5% |
| APLAPOLLO | ma_alignment | 2322 | 0.048 | 0.064 | 43.5% |
| APOLLOTYRE | ma_alignment | 2268 | 0.020 | 0.027 | 43.5% |
| AMBUJACEM | ma_alignment | 2189 | 0.049 | 0.066 | 43.4% |
| ALKEM | ma_alignment | 2119 | -0.028 | -0.037 | 43.4% |
| ICICIGI | ma_alignment | 2155 | -0.044 | -0.058 | 43.3% |
| ADANIPORTS | ma_alignment | 2229 | 0.078 | 0.104 | 43.3% |
| CUMMINSIND | ma_alignment | 2237 | 0.009 | 0.012 | 43.3% |
| MEDANTA | ma_alignment | 1680 | 0.029 | 0.039 | 43.2% |
| AUROPHARMA | ma_alignment | 2213 | 0.005 | 0.007 | 43.2% |
| JSWSTEEL | ma_alignment | 2179 | -0.012 | -0.016 | 43.2% |
| MFSL | ma_alignment | 2272 | 0.008 | 0.011 | 43.2% |
| LINDEINDIA | ma_alignment | 2386 | 0.100 | 0.133 | 43.2% |
| BEL | ma_alignment | 2199 | 0.033 | 0.044 | 43.2% |
| GODREJPROP | ma_alignment | 2378 | 0.074 | 0.098 | 43.1% |
| GAIL | ma_alignment | 2217 | 0.007 | 0.009 | 43.0% |
| AJANTPHARM | ma_alignment | 2286 | 0.020 | 0.027 | 43.0% |
| JKCEMENT | ma_alignment | 2254 | 0.032 | 0.043 | 43.0% |
| NLCINDIA | ma_alignment | 2493 | 0.130 | 0.173 | 43.0% |
| CONCOR | ma_alignment | 2198 | 0.032 | 0.043 | 42.9% |
| NYKAA | ma_alignment | 2020 | 0.074 | 0.099 | 42.9% |
| ICICIPRULI | ma_alignment | 2187 | 0.004 | 0.005 | 42.9% |
| MOTHERSON | ma_alignment | 2280 | 0.032 | 0.043 | 42.9% |
| KPRMILL | ma_alignment | 2415 | 0.096 | 0.128 | 42.8% |
