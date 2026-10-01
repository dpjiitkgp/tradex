# exit-tuning/mrm-2.5 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 2.5x risk (1.875%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 165 instruments, 2021-03-23..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_late_vwapslope_lb20 | 39875 | 0.039 | 0.052 | 46.2% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LICI | vwap_trend_short_late_vwapslope_lb20 | 199 | 0.172 | 0.229 | 65.3% |
| AWL | vwap_trend_short_late_vwapslope_lb20 | 222 | 0.145 | 0.193 | 58.6% |
| EXIDEIND | vwap_trend_short_late_vwapslope_lb20 | 254 | 0.129 | 0.172 | 55.9% |
| ITC | vwap_trend_short_late_vwapslope_lb20 | 196 | 0.114 | 0.152 | 54.6% |
| INDUSINDBK | vwap_trend_short_late_vwapslope_lb20 | 200 | 0.183 | 0.243 | 54.0% |
| COCHINSHIP | vwap_trend_short_late_vwapslope_lb20 | 325 | 0.144 | 0.191 | 53.8% |
| DIVISLAB | vwap_trend_short_late_vwapslope_lb20 | 210 | 0.079 | 0.106 | 53.8% |
| MARICO | vwap_trend_short_late_vwapslope_lb20 | 199 | 0.046 | 0.061 | 53.8% |
| LINDEINDIA | vwap_trend_short_late_vwapslope_lb20 | 301 | 0.158 | 0.211 | 53.2% |
| APARINDS | vwap_trend_short_late_vwapslope_lb20 | 294 | 0.221 | 0.295 | 52.4% |
| HAVELLS | vwap_trend_short_late_vwapslope_lb20 | 243 | 0.062 | 0.083 | 52.3% |
| HONAUT | vwap_trend_short_late_vwapslope_lb20 | 323 | 0.069 | 0.092 | 52.0% |
| HUDCO | vwap_trend_short_late_vwapslope_lb20 | 312 | 0.111 | 0.149 | 51.9% |
| HDFCLIFE | vwap_trend_short_late_vwapslope_lb20 | 200 | 0.074 | 0.098 | 51.5% |
| HAL | vwap_trend_short_late_vwapslope_lb20 | 247 | 0.168 | 0.224 | 51.4% |
| DLF | vwap_trend_short_late_vwapslope_lb20 | 232 | 0.172 | 0.229 | 51.3% |
| NLCINDIA | vwap_trend_short_late_vwapslope_lb20 | 301 | 0.167 | 0.223 | 51.2% |
| 3MINDIA | vwap_trend_short_late_vwapslope_lb20 | 364 | 0.058 | 0.077 | 50.8% |
| DIXON | vwap_trend_short_late_vwapslope_lb20 | 244 | 0.194 | 0.259 | 50.8% |
| MAZDOCK | vwap_trend_short_late_vwapslope_lb20 | 269 | 0.148 | 0.197 | 50.6% |
| GLAXO | vwap_trend_short_late_vwapslope_lb20 | 321 | 0.092 | 0.123 | 50.5% |
| CANBK | vwap_trend_short_late_vwapslope_lb20 | 222 | 0.129 | 0.172 | 50.5% |
| BAJAJ-AUTO | vwap_trend_short_late_vwapslope_lb20 | 222 | 0.089 | 0.119 | 50.5% |
| NYKAA | vwap_trend_short_late_vwapslope_lb20 | 221 | 0.158 | 0.210 | 50.2% |
| BIOCON | vwap_trend_short_late_vwapslope_lb20 | 231 | 0.116 | 0.155 | 50.2% |
| GODFRYPHLP | vwap_trend_short_late_vwapslope_lb20 | 289 | 0.159 | 0.212 | 50.2% |
| BDL | vwap_trend_short_late_vwapslope_lb20 | 289 | 0.133 | 0.178 | 50.2% |
| KPITTECH | vwap_trend_short_late_vwapslope_lb20 | 260 | 0.122 | 0.162 | 50.0% |
| CRISIL | vwap_trend_short_late_vwapslope_lb20 | 304 | 0.105 | 0.141 | 50.0% |
| BANKINDIA | vwap_trend_short_late_vwapslope_lb20 | 305 | 0.117 | 0.157 | 49.8% |
| HEROMOTOCO | vwap_trend_short_late_vwapslope_lb20 | 211 | 0.083 | 0.110 | 49.8% |
| HDFCBANK | vwap_trend_short_late_vwapslope_lb20 | 211 | 0.039 | 0.051 | 49.8% |
| INDIGO | vwap_trend_short_late_vwapslope_lb20 | 228 | 0.051 | 0.068 | 49.6% |
| DMART | vwap_trend_short_late_vwapslope_lb20 | 259 | 0.102 | 0.136 | 49.4% |
| IRCTC | vwap_trend_short_late_vwapslope_lb20 | 227 | 0.049 | 0.066 | 49.3% |
| ABB | vwap_trend_short_late_vwapslope_lb20 | 221 | 0.048 | 0.065 | 49.3% |
| LTM | vwap_trend_short_late_vwapslope_lb20 | 209 | 0.078 | 0.104 | 49.3% |
| CIPLA | vwap_trend_short_late_vwapslope_lb20 | 199 | -0.013 | -0.017 | 49.2% |
| OIL | vwap_trend_short_late_vwapslope_lb20 | 289 | 0.208 | 0.278 | 49.1% |
| ATGL | vwap_trend_short_late_vwapslope_lb20 | 267 | 0.089 | 0.119 | 49.1% |
| APOLLOHOSP | vwap_trend_short_late_vwapslope_lb20 | 204 | 0.076 | 0.102 | 49.0% |
| MPHASIS | vwap_trend_short_late_vwapslope_lb20 | 209 | 0.155 | 0.207 | 48.8% |
| IOC | vwap_trend_short_late_vwapslope_lb20 | 248 | 0.130 | 0.173 | 48.8% |
| HDFCAMC | vwap_trend_short_late_vwapslope_lb20 | 246 | 0.041 | 0.054 | 48.8% |
| BERGEPAINT | vwap_trend_short_late_vwapslope_lb20 | 238 | 0.029 | 0.038 | 48.7% |
| AXISBANK | vwap_trend_short_late_vwapslope_lb20 | 195 | -0.018 | -0.023 | 48.7% |
| KALYANKJIL | vwap_trend_short_late_vwapslope_lb20 | 310 | 0.093 | 0.124 | 48.7% |
| KPRMILL | vwap_trend_short_late_vwapslope_lb20 | 308 | 0.087 | 0.115 | 48.7% |
| INFY | vwap_trend_short_late_vwapslope_lb20 | 183 | 0.004 | 0.006 | 48.6% |
| LUPIN | vwap_trend_short_late_vwapslope_lb20 | 223 | 0.070 | 0.094 | 48.4% |
| MANKIND | vwap_trend_short_late_vwapslope_lb20 | 162 | 0.112 | 0.149 | 48.1% |
| NESTLEIND | vwap_trend_short_late_vwapslope_lb20 | 227 | 0.018 | 0.024 | 48.0% |
| DABUR | vwap_trend_short_late_vwapslope_lb20 | 198 | 0.085 | 0.114 | 48.0% |
| COALINDIA | vwap_trend_short_late_vwapslope_lb20 | 194 | 0.053 | 0.070 | 47.9% |
| LICHSGFIN | vwap_trend_short_late_vwapslope_lb20 | 213 | 0.036 | 0.048 | 47.9% |
| MOTILALOFS | vwap_trend_short_late_vwapslope_lb20 | 280 | 0.089 | 0.119 | 47.9% |
| GAIL | vwap_trend_short_late_vwapslope_lb20 | 226 | 0.059 | 0.078 | 47.8% |
| ACC | vwap_trend_short_late_vwapslope_lb20 | 241 | 0.025 | 0.033 | 47.7% |
| GICRE | vwap_trend_short_late_vwapslope_lb20 | 344 | 0.066 | 0.088 | 47.7% |
| APOLLOTYRE | vwap_trend_short_late_vwapslope_lb20 | 235 | 0.079 | 0.105 | 47.7% |
| OFSS | vwap_trend_short_late_vwapslope_lb20 | 277 | 0.038 | 0.051 | 47.7% |
| BHARTIARTL | vwap_trend_short_late_vwapslope_lb20 | 229 | -0.012 | -0.016 | 47.6% |
| AUROPHARMA | vwap_trend_short_late_vwapslope_lb20 | 225 | 0.029 | 0.039 | 47.6% |
| JSWSTEEL | vwap_trend_short_late_vwapslope_lb20 | 202 | 0.029 | 0.039 | 47.5% |
| HCLTECH | vwap_trend_short_late_vwapslope_lb20 | 221 | 0.038 | 0.051 | 47.5% |
| JKCEMENT | vwap_trend_short_late_vwapslope_lb20 | 253 | 0.052 | 0.069 | 47.4% |
| ASIANPAINT | vwap_trend_short_late_vwapslope_lb20 | 201 | 0.036 | 0.047 | 47.3% |
| APLAPOLLO | vwap_trend_short_late_vwapslope_lb20 | 252 | 0.063 | 0.084 | 47.2% |
| NIACL | vwap_trend_short_late_vwapslope_lb20 | 329 | 0.109 | 0.145 | 47.1% |
| ONGC | vwap_trend_short_late_vwapslope_lb20 | 219 | 0.012 | 0.016 | 47.0% |
| PAGEIND | vwap_trend_short_late_vwapslope_lb20 | 217 | 0.021 | 0.028 | 47.0% |
| NAUKRI | vwap_trend_short_late_vwapslope_lb20 | 230 | 0.104 | 0.139 | 47.0% |
| BLUESTARCO | vwap_trend_short_late_vwapslope_lb20 | 277 | 0.059 | 0.078 | 46.9% |
| GODREJIND | vwap_trend_short_late_vwapslope_lb20 | 322 | -0.027 | -0.036 | 46.9% |
| LTTS | vwap_trend_short_late_vwapslope_lb20 | 237 | 0.063 | 0.084 | 46.8% |
| DRREDDY | vwap_trend_short_late_vwapslope_lb20 | 203 | -0.009 | -0.012 | 46.8% |
| JUBLFOOD | vwap_trend_short_late_vwapslope_lb20 | 210 | -0.021 | -0.028 | 46.7% |
| NAM-INDIA | vwap_trend_short_late_vwapslope_lb20 | 264 | 0.073 | 0.098 | 46.6% |
| MARUTI | vwap_trend_short_late_vwapslope_lb20 | 226 | 0.041 | 0.055 | 46.5% |
| EICHERMOT | vwap_trend_short_late_vwapslope_lb20 | 220 | 0.021 | 0.028 | 46.4% |
| BRITANNIA | vwap_trend_short_late_vwapslope_lb20 | 233 | 0.003 | 0.003 | 46.4% |
| MAHABANK | vwap_trend_short_late_vwapslope_lb20 | 328 | 0.026 | 0.035 | 46.3% |
| JSWENERGY | vwap_trend_short_late_vwapslope_lb20 | 259 | 0.109 | 0.145 | 46.3% |
| ETERNAL | vwap_trend_short_late_vwapslope_lb20 | 218 | 0.058 | 0.078 | 46.3% |
| ICICIPRULI | vwap_trend_short_late_vwapslope_lb20 | 203 | 0.058 | 0.077 | 46.3% |
| BHEL | vwap_trend_short_late_vwapslope_lb20 | 242 | 0.105 | 0.140 | 46.3% |
| M&M | vwap_trend_short_late_vwapslope_lb20 | 232 | 0.040 | 0.054 | 46.1% |
| ENDURANCE | vwap_trend_short_late_vwapslope_lb20 | 296 | 0.068 | 0.091 | 45.9% |
| ADANIGREEN | vwap_trend_short_late_vwapslope_lb20 | 264 | 0.120 | 0.159 | 45.8% |
| INDHOTEL | vwap_trend_short_late_vwapslope_lb20 | 236 | 0.072 | 0.096 | 45.8% |
| NHPC | vwap_trend_short_late_vwapslope_lb20 | 318 | 0.069 | 0.093 | 45.6% |
| LAURUSLABS | vwap_trend_short_late_vwapslope_lb20 | 224 | 0.042 | 0.056 | 45.5% |
| ABBOTINDIA | vwap_trend_short_late_vwapslope_lb20 | 315 | -0.083 | -0.110 | 45.4% |
| CUMMINSIND | vwap_trend_short_late_vwapslope_lb20 | 217 | 0.010 | 0.013 | 45.2% |
| BAJAJFINSV | vwap_trend_short_late_vwapslope_lb20 | 191 | 0.033 | 0.044 | 45.0% |
| BANKBARODA | vwap_trend_short_late_vwapslope_lb20 | 231 | -0.002 | -0.002 | 45.0% |
| IRFC | vwap_trend_short_late_vwapslope_lb20 | 349 | 0.034 | 0.045 | 45.0% |
| BEL | vwap_trend_short_late_vwapslope_lb20 | 207 | 0.013 | 0.018 | 44.9% |
| BPCL | vwap_trend_short_late_vwapslope_lb20 | 221 | -0.011 | -0.014 | 44.8% |
| KOTAKBANK | vwap_trend_short_late_vwapslope_lb20 | 208 | -0.025 | -0.034 | 44.7% |
