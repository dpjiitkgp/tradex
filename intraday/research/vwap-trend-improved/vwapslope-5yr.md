# vwap-trend-improved/vwapslope-5yr — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 3.0x risk (2.250%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 165 instruments, 2021-03-23..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_late_vwapslope | 39418 | 0.039 | 0.052 | 45.9% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LICI | vwap_trend_short_late_vwapslope | 194 | 0.178 | 0.238 | 64.9% |
| AWL | vwap_trend_short_late_vwapslope | 216 | 0.161 | 0.215 | 57.9% |
| EXIDEIND | vwap_trend_short_late_vwapslope | 248 | 0.117 | 0.155 | 55.6% |
| DIVISLAB | vwap_trend_short_late_vwapslope | 205 | 0.124 | 0.165 | 55.6% |
| ITC | vwap_trend_short_late_vwapslope | 187 | 0.091 | 0.121 | 54.5% |
| INDUSINDBK | vwap_trend_short_late_vwapslope | 201 | 0.193 | 0.257 | 54.2% |
| COCHINSHIP | vwap_trend_short_late_vwapslope | 314 | 0.153 | 0.205 | 53.8% |
| MARICO | vwap_trend_short_late_vwapslope | 197 | 0.060 | 0.079 | 53.8% |
| HONAUT | vwap_trend_short_late_vwapslope | 328 | 0.070 | 0.094 | 52.4% |
| LINDEINDIA | vwap_trend_short_late_vwapslope | 295 | 0.127 | 0.169 | 51.9% |
| DIXON | vwap_trend_short_late_vwapslope | 234 | 0.221 | 0.294 | 51.7% |
| NYKAA | vwap_trend_short_late_vwapslope | 213 | 0.195 | 0.260 | 51.6% |
| HEROMOTOCO | vwap_trend_short_late_vwapslope | 215 | 0.098 | 0.130 | 51.6% |
| HAVELLS | vwap_trend_short_late_vwapslope | 241 | 0.057 | 0.076 | 51.5% |
| HUDCO | vwap_trend_short_late_vwapslope | 311 | 0.086 | 0.114 | 51.4% |
| GLAXO | vwap_trend_short_late_vwapslope | 328 | 0.111 | 0.148 | 50.9% |
| APARINDS | vwap_trend_short_late_vwapslope | 294 | 0.232 | 0.309 | 50.7% |
| BIOCON | vwap_trend_short_late_vwapslope | 223 | 0.125 | 0.167 | 50.7% |
| DLF | vwap_trend_short_late_vwapslope | 229 | 0.173 | 0.230 | 50.7% |
| HDFCLIFE | vwap_trend_short_late_vwapslope | 190 | 0.072 | 0.096 | 50.5% |
| 3MINDIA | vwap_trend_short_late_vwapslope | 371 | 0.067 | 0.090 | 50.4% |
| CRISIL | vwap_trend_short_late_vwapslope | 308 | 0.117 | 0.155 | 50.3% |
| GODFRYPHLP | vwap_trend_short_late_vwapslope | 289 | 0.169 | 0.225 | 50.2% |
| MAZDOCK | vwap_trend_short_late_vwapslope | 274 | 0.161 | 0.215 | 50.0% |
| NLCINDIA | vwap_trend_short_late_vwapslope | 308 | 0.150 | 0.201 | 50.0% |
| MANKIND | vwap_trend_short_late_vwapslope | 161 | 0.169 | 0.225 | 49.7% |
| HAL | vwap_trend_short_late_vwapslope | 242 | 0.134 | 0.178 | 49.6% |
| LTM | vwap_trend_short_late_vwapslope | 208 | 0.090 | 0.120 | 49.5% |
| DRREDDY | vwap_trend_short_late_vwapslope | 194 | 0.024 | 0.032 | 49.5% |
| ATGL | vwap_trend_short_late_vwapslope | 273 | 0.086 | 0.115 | 49.5% |
| KPITTECH | vwap_trend_short_late_vwapslope | 247 | 0.119 | 0.159 | 49.4% |
| NESTLEIND | vwap_trend_short_late_vwapslope | 223 | 0.022 | 0.029 | 49.3% |
| BDL | vwap_trend_short_late_vwapslope | 284 | 0.121 | 0.162 | 49.3% |
| APOLLOHOSP | vwap_trend_short_late_vwapslope | 199 | 0.093 | 0.124 | 49.2% |
| LUPIN | vwap_trend_short_late_vwapslope | 222 | 0.102 | 0.135 | 49.1% |
| CANBK | vwap_trend_short_late_vwapslope | 214 | 0.122 | 0.162 | 49.1% |
| IRCTC | vwap_trend_short_late_vwapslope | 217 | 0.046 | 0.061 | 48.8% |
| BAJAJ-AUTO | vwap_trend_short_late_vwapslope | 215 | 0.069 | 0.092 | 48.8% |
| IOC | vwap_trend_short_late_vwapslope | 246 | 0.122 | 0.163 | 48.8% |
| ACC | vwap_trend_short_late_vwapslope | 240 | 0.048 | 0.064 | 48.8% |
| BERGEPAINT | vwap_trend_short_late_vwapslope | 236 | 0.032 | 0.042 | 48.7% |
| KPRMILL | vwap_trend_short_late_vwapslope | 312 | 0.095 | 0.127 | 48.7% |
| COALINDIA | vwap_trend_short_late_vwapslope | 191 | 0.088 | 0.117 | 48.7% |
| ABB | vwap_trend_short_late_vwapslope | 224 | 0.050 | 0.067 | 48.7% |
| GAIL | vwap_trend_short_late_vwapslope | 220 | 0.076 | 0.102 | 48.6% |
| DMART | vwap_trend_short_late_vwapslope | 247 | 0.083 | 0.111 | 48.6% |
| KALYANKJIL | vwap_trend_short_late_vwapslope | 305 | 0.081 | 0.108 | 48.5% |
| OIL | vwap_trend_short_late_vwapslope | 291 | 0.219 | 0.292 | 48.5% |
| AXISBANK | vwap_trend_short_late_vwapslope | 193 | -0.028 | -0.037 | 48.2% |
| HDFCAMC | vwap_trend_short_late_vwapslope | 237 | 0.027 | 0.037 | 48.1% |
| APOLLOTYRE | vwap_trend_short_late_vwapslope | 233 | 0.088 | 0.117 | 48.1% |
| HDFCBANK | vwap_trend_short_late_vwapslope | 206 | 0.013 | 0.018 | 48.1% |
| BHARTIARTL | vwap_trend_short_late_vwapslope | 225 | -0.003 | -0.004 | 48.0% |
| CIPLA | vwap_trend_short_late_vwapslope | 192 | -0.041 | -0.055 | 47.9% |
| BANKINDIA | vwap_trend_short_late_vwapslope | 297 | 0.095 | 0.127 | 47.8% |
| INDIGO | vwap_trend_short_late_vwapslope | 224 | 0.009 | 0.012 | 47.8% |
| JUBLFOOD | vwap_trend_short_late_vwapslope | 201 | 0.003 | 0.004 | 47.8% |
| MOTILALOFS | vwap_trend_short_late_vwapslope | 279 | 0.073 | 0.097 | 47.7% |
| INFY | vwap_trend_short_late_vwapslope | 186 | -0.012 | -0.016 | 47.3% |
| DABUR | vwap_trend_short_late_vwapslope | 195 | 0.091 | 0.121 | 47.2% |
| JKCEMENT | vwap_trend_short_late_vwapslope | 259 | 0.065 | 0.086 | 47.1% |
| GICRE | vwap_trend_short_late_vwapslope | 338 | 0.059 | 0.079 | 47.0% |
| LICHSGFIN | vwap_trend_short_late_vwapslope | 215 | 0.039 | 0.053 | 47.0% |
| BAJAJFINSV | vwap_trend_short_late_vwapslope | 184 | 0.076 | 0.101 | 46.7% |
| ONGC | vwap_trend_short_late_vwapslope | 214 | 0.018 | 0.024 | 46.7% |
| JSWSTEEL | vwap_trend_short_late_vwapslope | 197 | 0.042 | 0.056 | 46.7% |
| MPHASIS | vwap_trend_short_late_vwapslope | 210 | 0.145 | 0.193 | 46.7% |
| NIACL | vwap_trend_short_late_vwapslope | 328 | 0.105 | 0.141 | 46.6% |
| HCLTECH | vwap_trend_short_late_vwapslope | 217 | 0.035 | 0.047 | 46.5% |
| APLAPOLLO | vwap_trend_short_late_vwapslope | 245 | 0.029 | 0.038 | 46.5% |
| ASIANPAINT | vwap_trend_short_late_vwapslope | 198 | 0.047 | 0.062 | 46.5% |
| GODREJIND | vwap_trend_short_late_vwapslope | 325 | -0.043 | -0.057 | 46.5% |
| ENDURANCE | vwap_trend_short_late_vwapslope | 291 | 0.084 | 0.111 | 46.4% |
| MAHABANK | vwap_trend_short_late_vwapslope | 328 | 0.042 | 0.055 | 46.3% |
| NAUKRI | vwap_trend_short_late_vwapslope | 223 | 0.099 | 0.131 | 46.2% |
| BRITANNIA | vwap_trend_short_late_vwapslope | 232 | -0.005 | -0.007 | 46.1% |
| BLUESTARCO | vwap_trend_short_late_vwapslope | 287 | 0.044 | 0.058 | 46.0% |
| OFSS | vwap_trend_short_late_vwapslope | 274 | 0.039 | 0.052 | 46.0% |
| NHPC | vwap_trend_short_late_vwapslope | 318 | 0.098 | 0.130 | 45.9% |
| MARUTI | vwap_trend_short_late_vwapslope | 220 | 0.026 | 0.035 | 45.9% |
| LAURUSLABS | vwap_trend_short_late_vwapslope | 216 | 0.065 | 0.087 | 45.8% |
| COLPAL | vwap_trend_short_late_vwapslope | 214 | 0.061 | 0.081 | 45.8% |
| PAGEIND | vwap_trend_short_late_vwapslope | 223 | 0.002 | 0.003 | 45.7% |
| MEDANTA | vwap_trend_short_late_vwapslope | 184 | 0.111 | 0.148 | 45.7% |
| INDHOTEL | vwap_trend_short_late_vwapslope | 228 | 0.088 | 0.118 | 45.6% |
| AUROPHARMA | vwap_trend_short_late_vwapslope | 215 | 0.008 | 0.011 | 45.6% |
| M&M | vwap_trend_short_late_vwapslope | 224 | 0.024 | 0.033 | 45.5% |
| LTTS | vwap_trend_short_late_vwapslope | 233 | 0.045 | 0.060 | 45.5% |
| NAM-INDIA | vwap_trend_short_late_vwapslope | 266 | 0.083 | 0.110 | 45.5% |
| BOSCHLTD | vwap_trend_short_late_vwapslope | 253 | -0.001 | -0.001 | 45.5% |
| ETERNAL | vwap_trend_short_late_vwapslope | 218 | 0.057 | 0.076 | 45.4% |
| JSWENERGY | vwap_trend_short_late_vwapslope | 258 | 0.095 | 0.127 | 45.3% |
| BANKBARODA | vwap_trend_short_late_vwapslope | 225 | 0.007 | 0.009 | 45.3% |
| ABBOTINDIA | vwap_trend_short_late_vwapslope | 309 | -0.084 | -0.112 | 45.3% |
| KEI | vwap_trend_short_late_vwapslope | 241 | 0.047 | 0.062 | 45.2% |
| CUMMINSIND | vwap_trend_short_late_vwapslope | 213 | 0.004 | 0.005 | 45.1% |
| GRASIM | vwap_trend_short_late_vwapslope | 202 | -0.036 | -0.048 | 45.0% |
| EICHERMOT | vwap_trend_short_late_vwapslope | 220 | 0.016 | 0.021 | 45.0% |
| FEDERALBNK | vwap_trend_short_late_vwapslope | 241 | 0.047 | 0.063 | 44.8% |
| BHEL | vwap_trend_short_late_vwapslope | 238 | 0.086 | 0.115 | 44.5% |
