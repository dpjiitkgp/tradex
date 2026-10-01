# vwap-trend-improved/vwapslope_lb20 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 3.0x risk (2.250%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 165 instruments, 2021-03-23..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_late_vwapslope_lb20 | 39854 | 0.040 | 0.053 | 45.8% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LICI | vwap_trend_short_late_vwapslope_lb20 | 199 | 0.168 | 0.224 | 65.3% |
| AWL | vwap_trend_short_late_vwapslope_lb20 | 222 | 0.162 | 0.216 | 58.6% |
| EXIDEIND | vwap_trend_short_late_vwapslope_lb20 | 254 | 0.123 | 0.164 | 55.9% |
| ITC | vwap_trend_short_late_vwapslope_lb20 | 196 | 0.101 | 0.134 | 54.1% |
| INDUSINDBK | vwap_trend_short_late_vwapslope_lb20 | 200 | 0.189 | 0.252 | 54.0% |
| DIVISLAB | vwap_trend_short_late_vwapslope_lb20 | 210 | 0.103 | 0.137 | 53.8% |
| MARICO | vwap_trend_short_late_vwapslope_lb20 | 199 | 0.055 | 0.073 | 53.8% |
| COCHINSHIP | vwap_trend_short_late_vwapslope_lb20 | 325 | 0.149 | 0.199 | 53.5% |
| LINDEINDIA | vwap_trend_short_late_vwapslope_lb20 | 301 | 0.166 | 0.221 | 53.2% |
| HAVELLS | vwap_trend_short_late_vwapslope_lb20 | 243 | 0.052 | 0.069 | 51.9% |
| HUDCO | vwap_trend_short_late_vwapslope_lb20 | 312 | 0.107 | 0.142 | 51.6% |
| HAL | vwap_trend_short_late_vwapslope_lb20 | 247 | 0.172 | 0.230 | 51.4% |
| HONAUT | vwap_trend_short_late_vwapslope_lb20 | 323 | 0.065 | 0.087 | 51.4% |
| 3MINDIA | vwap_trend_short_late_vwapslope_lb20 | 364 | 0.062 | 0.083 | 50.8% |
| DIXON | vwap_trend_short_late_vwapslope_lb20 | 244 | 0.205 | 0.273 | 50.8% |
| APARINDS | vwap_trend_short_late_vwapslope_lb20 | 294 | 0.220 | 0.294 | 50.7% |
| HDFCLIFE | vwap_trend_short_late_vwapslope_lb20 | 200 | 0.062 | 0.082 | 50.5% |
| NLCINDIA | vwap_trend_short_late_vwapslope_lb20 | 301 | 0.162 | 0.216 | 50.5% |
| GLAXO | vwap_trend_short_late_vwapslope_lb20 | 321 | 0.093 | 0.124 | 50.5% |
| DLF | vwap_trend_short_late_vwapslope_lb20 | 232 | 0.168 | 0.224 | 50.4% |
| NYKAA | vwap_trend_short_late_vwapslope_lb20 | 221 | 0.159 | 0.212 | 50.2% |
| CANBK | vwap_trend_short_late_vwapslope_lb20 | 221 | 0.125 | 0.167 | 50.2% |
| BIOCON | vwap_trend_short_late_vwapslope_lb20 | 231 | 0.117 | 0.155 | 50.2% |
| GODFRYPHLP | vwap_trend_short_late_vwapslope_lb20 | 289 | 0.167 | 0.222 | 50.2% |
| HEROMOTOCO | vwap_trend_short_late_vwapslope_lb20 | 210 | 0.089 | 0.119 | 50.0% |
| BAJAJ-AUTO | vwap_trend_short_late_vwapslope_lb20 | 222 | 0.078 | 0.104 | 50.0% |
| BDL | vwap_trend_short_late_vwapslope_lb20 | 289 | 0.130 | 0.173 | 49.8% |
| CRISIL | vwap_trend_short_late_vwapslope_lb20 | 304 | 0.107 | 0.143 | 49.7% |
| KPITTECH | vwap_trend_short_late_vwapslope_lb20 | 260 | 0.134 | 0.178 | 49.6% |
| BANKINDIA | vwap_trend_short_late_vwapslope_lb20 | 305 | 0.119 | 0.159 | 49.5% |
| MAZDOCK | vwap_trend_short_late_vwapslope_lb20 | 269 | 0.149 | 0.198 | 49.4% |
| DMART | vwap_trend_short_late_vwapslope_lb20 | 259 | 0.103 | 0.137 | 49.4% |
| IRCTC | vwap_trend_short_late_vwapslope_lb20 | 227 | 0.049 | 0.066 | 49.3% |
| ABB | vwap_trend_short_late_vwapslope_lb20 | 221 | 0.050 | 0.067 | 49.3% |
| HDFCBANK | vwap_trend_short_late_vwapslope_lb20 | 211 | 0.027 | 0.036 | 49.3% |
| LTM | vwap_trend_short_late_vwapslope_lb20 | 209 | 0.094 | 0.125 | 49.3% |
| OIL | vwap_trend_short_late_vwapslope_lb20 | 289 | 0.234 | 0.312 | 49.1% |
| INDIGO | vwap_trend_short_late_vwapslope_lb20 | 228 | 0.039 | 0.052 | 49.1% |
| ATGL | vwap_trend_short_late_vwapslope_lb20 | 267 | 0.103 | 0.137 | 49.1% |
| CIPLA | vwap_trend_short_late_vwapslope_lb20 | 199 | -0.022 | -0.029 | 48.7% |
| AXISBANK | vwap_trend_short_late_vwapslope_lb20 | 195 | -0.020 | -0.027 | 48.7% |
| LUPIN | vwap_trend_short_late_vwapslope_lb20 | 222 | 0.089 | 0.118 | 48.6% |
| INFY | vwap_trend_short_late_vwapslope_lb20 | 183 | 0.004 | 0.005 | 48.6% |
| IOC | vwap_trend_short_late_vwapslope_lb20 | 247 | 0.120 | 0.161 | 48.6% |
| MPHASIS | vwap_trend_short_late_vwapslope_lb20 | 208 | 0.166 | 0.222 | 48.6% |
| APOLLOHOSP | vwap_trend_short_late_vwapslope_lb20 | 204 | 0.073 | 0.097 | 48.5% |
| HDFCAMC | vwap_trend_short_late_vwapslope_lb20 | 246 | 0.037 | 0.049 | 48.4% |
| BERGEPAINT | vwap_trend_short_late_vwapslope_lb20 | 238 | 0.028 | 0.037 | 48.3% |
| KALYANKJIL | vwap_trend_short_late_vwapslope_lb20 | 310 | 0.085 | 0.114 | 48.1% |
| KPRMILL | vwap_trend_short_late_vwapslope_lb20 | 308 | 0.095 | 0.127 | 48.1% |
| NESTLEIND | vwap_trend_short_late_vwapslope_lb20 | 227 | 0.019 | 0.025 | 48.0% |
| DABUR | vwap_trend_short_late_vwapslope_lb20 | 198 | 0.085 | 0.114 | 48.0% |
| LICHSGFIN | vwap_trend_short_late_vwapslope_lb20 | 213 | 0.033 | 0.044 | 47.9% |
| MANKIND | vwap_trend_short_late_vwapslope_lb20 | 161 | 0.116 | 0.155 | 47.8% |
| GAIL | vwap_trend_short_late_vwapslope_lb20 | 226 | 0.067 | 0.089 | 47.8% |
| ACC | vwap_trend_short_late_vwapslope_lb20 | 241 | 0.039 | 0.051 | 47.7% |
| COALINDIA | vwap_trend_short_late_vwapslope_lb20 | 193 | 0.069 | 0.091 | 47.7% |
| APOLLOTYRE | vwap_trend_short_late_vwapslope_lb20 | 235 | 0.081 | 0.108 | 47.7% |
| BHARTIARTL | vwap_trend_short_late_vwapslope_lb20 | 229 | -0.005 | -0.007 | 47.6% |
| AUROPHARMA | vwap_trend_short_late_vwapslope_lb20 | 225 | 0.032 | 0.042 | 47.6% |
| JKCEMENT | vwap_trend_short_late_vwapslope_lb20 | 253 | 0.048 | 0.064 | 47.4% |
| GICRE | vwap_trend_short_late_vwapslope_lb20 | 344 | 0.069 | 0.092 | 47.4% |
| HCLTECH | vwap_trend_short_late_vwapslope_lb20 | 220 | 0.037 | 0.049 | 47.3% |
| ASIANPAINT | vwap_trend_short_late_vwapslope_lb20 | 201 | 0.055 | 0.074 | 47.3% |
| APLAPOLLO | vwap_trend_short_late_vwapslope_lb20 | 252 | 0.058 | 0.077 | 47.2% |
| MOTILALOFS | vwap_trend_short_late_vwapslope_lb20 | 280 | 0.063 | 0.083 | 47.1% |
| ONGC | vwap_trend_short_late_vwapslope_lb20 | 219 | 0.015 | 0.020 | 47.0% |
| JSWSTEEL | vwap_trend_short_late_vwapslope_lb20 | 202 | 0.024 | 0.032 | 47.0% |
| PAGEIND | vwap_trend_short_late_vwapslope_lb20 | 217 | 0.024 | 0.033 | 47.0% |
| NAUKRI | vwap_trend_short_late_vwapslope_lb20 | 230 | 0.106 | 0.141 | 47.0% |
| BLUESTARCO | vwap_trend_short_late_vwapslope_lb20 | 277 | 0.063 | 0.085 | 46.9% |
| NIACL | vwap_trend_short_late_vwapslope_lb20 | 329 | 0.114 | 0.152 | 46.8% |
| DRREDDY | vwap_trend_short_late_vwapslope_lb20 | 203 | -0.000 | -0.000 | 46.8% |
| JUBLFOOD | vwap_trend_short_late_vwapslope_lb20 | 210 | -0.019 | -0.025 | 46.7% |
| GODREJIND | vwap_trend_short_late_vwapslope_lb20 | 322 | -0.035 | -0.046 | 46.6% |
| OFSS | vwap_trend_short_late_vwapslope_lb20 | 277 | 0.031 | 0.041 | 46.6% |
| LTTS | vwap_trend_short_late_vwapslope_lb20 | 237 | 0.059 | 0.079 | 46.4% |
| BRITANNIA | vwap_trend_short_late_vwapslope_lb20 | 233 | 0.003 | 0.004 | 46.4% |
| BHEL | vwap_trend_short_late_vwapslope_lb20 | 242 | 0.120 | 0.160 | 46.3% |
| NAM-INDIA | vwap_trend_short_late_vwapslope_lb20 | 264 | 0.073 | 0.097 | 46.2% |
| M&M | vwap_trend_short_late_vwapslope_lb20 | 232 | 0.045 | 0.060 | 46.1% |
| MARUTI | vwap_trend_short_late_vwapslope_lb20 | 224 | 0.023 | 0.030 | 46.0% |
| EICHERMOT | vwap_trend_short_late_vwapslope_lb20 | 220 | 0.015 | 0.020 | 45.9% |
| MAHABANK | vwap_trend_short_late_vwapslope_lb20 | 328 | 0.049 | 0.066 | 45.7% |
| ENDURANCE | vwap_trend_short_late_vwapslope_lb20 | 296 | 0.068 | 0.090 | 45.6% |
| NHPC | vwap_trend_short_late_vwapslope_lb20 | 318 | 0.090 | 0.120 | 45.6% |
| JSWENERGY | vwap_trend_short_late_vwapslope_lb20 | 259 | 0.091 | 0.121 | 45.6% |
| ICICIPRULI | vwap_trend_short_late_vwapslope_lb20 | 202 | 0.057 | 0.076 | 45.5% |
| LAURUSLABS | vwap_trend_short_late_vwapslope_lb20 | 224 | 0.070 | 0.093 | 45.5% |
| INDHOTEL | vwap_trend_short_late_vwapslope_lb20 | 235 | 0.083 | 0.111 | 45.5% |
| ABBOTINDIA | vwap_trend_short_late_vwapslope_lb20 | 315 | -0.089 | -0.119 | 45.1% |
| ADANIGREEN | vwap_trend_short_late_vwapslope_lb20 | 264 | 0.110 | 0.147 | 45.1% |
| BAJAJFINSV | vwap_trend_short_late_vwapslope_lb20 | 191 | 0.044 | 0.059 | 45.0% |
| BANKBARODA | vwap_trend_short_late_vwapslope_lb20 | 231 | 0.001 | 0.001 | 45.0% |
| IRFC | vwap_trend_short_late_vwapslope_lb20 | 349 | 0.041 | 0.054 | 45.0% |
| KOTAKBANK | vwap_trend_short_late_vwapslope_lb20 | 208 | -0.029 | -0.039 | 44.7% |
| CUMMINSIND | vwap_trend_short_late_vwapslope_lb20 | 217 | 0.001 | 0.002 | 44.7% |
| ETERNAL | vwap_trend_short_late_vwapslope_lb20 | 218 | 0.031 | 0.041 | 44.5% |
| BEL | vwap_trend_short_late_vwapslope_lb20 | 207 | 0.016 | 0.021 | 44.4% |
| HINDPETRO | vwap_trend_short_late_vwapslope_lb20 | 223 | 0.081 | 0.108 | 44.4% |
