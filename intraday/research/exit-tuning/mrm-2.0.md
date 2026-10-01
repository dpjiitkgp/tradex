# exit-tuning/mrm-2.0 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 2.0x risk (1.500%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 165 instruments, 2021-03-23..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_late_vwapslope_lb20 | 39905 | 0.036 | 0.048 | 46.8% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LICI | vwap_trend_short_late_vwapslope_lb20 | 200 | 0.173 | 0.230 | 65.5% |
| AWL | vwap_trend_short_late_vwapslope_lb20 | 222 | 0.128 | 0.171 | 58.6% |
| EXIDEIND | vwap_trend_short_late_vwapslope_lb20 | 254 | 0.130 | 0.173 | 56.3% |
| DIVISLAB | vwap_trend_short_late_vwapslope_lb20 | 210 | 0.072 | 0.096 | 54.8% |
| ITC | vwap_trend_short_late_vwapslope_lb20 | 196 | 0.114 | 0.152 | 54.6% |
| INDUSINDBK | vwap_trend_short_late_vwapslope_lb20 | 200 | 0.180 | 0.240 | 54.5% |
| COCHINSHIP | vwap_trend_short_late_vwapslope_lb20 | 325 | 0.130 | 0.173 | 54.5% |
| LINDEINDIA | vwap_trend_short_late_vwapslope_lb20 | 301 | 0.159 | 0.212 | 54.2% |
| MARICO | vwap_trend_short_late_vwapslope_lb20 | 200 | 0.052 | 0.070 | 54.0% |
| MAZDOCK | vwap_trend_short_late_vwapslope_lb20 | 269 | 0.172 | 0.229 | 53.5% |
| HUDCO | vwap_trend_short_late_vwapslope_lb20 | 312 | 0.110 | 0.147 | 53.5% |
| APARINDS | vwap_trend_short_late_vwapslope_lb20 | 294 | 0.188 | 0.251 | 52.7% |
| HAVELLS | vwap_trend_short_late_vwapslope_lb20 | 243 | 0.051 | 0.068 | 52.3% |
| HAL | vwap_trend_short_late_vwapslope_lb20 | 247 | 0.176 | 0.234 | 52.2% |
| DLF | vwap_trend_short_late_vwapslope_lb20 | 232 | 0.149 | 0.199 | 52.2% |
| DIXON | vwap_trend_short_late_vwapslope_lb20 | 244 | 0.192 | 0.256 | 52.0% |
| HONAUT | vwap_trend_short_late_vwapslope_lb20 | 323 | 0.067 | 0.089 | 52.0% |
| NLCINDIA | vwap_trend_short_late_vwapslope_lb20 | 301 | 0.165 | 0.220 | 51.8% |
| GODFRYPHLP | vwap_trend_short_late_vwapslope_lb20 | 289 | 0.160 | 0.214 | 51.6% |
| HDFCLIFE | vwap_trend_short_late_vwapslope_lb20 | 200 | 0.062 | 0.083 | 51.5% |
| 3MINDIA | vwap_trend_short_late_vwapslope_lb20 | 364 | 0.060 | 0.080 | 51.4% |
| BDL | vwap_trend_short_late_vwapslope_lb20 | 289 | 0.111 | 0.149 | 51.2% |
| HEROMOTOCO | vwap_trend_short_late_vwapslope_lb20 | 211 | 0.094 | 0.126 | 51.2% |
| GLAXO | vwap_trend_short_late_vwapslope_lb20 | 321 | 0.084 | 0.112 | 51.1% |
| CANBK | vwap_trend_short_late_vwapslope_lb20 | 222 | 0.135 | 0.180 | 50.9% |
| BAJAJ-AUTO | vwap_trend_short_late_vwapslope_lb20 | 224 | 0.095 | 0.127 | 50.9% |
| OIL | vwap_trend_short_late_vwapslope_lb20 | 289 | 0.218 | 0.290 | 50.9% |
| BANKINDIA | vwap_trend_short_late_vwapslope_lb20 | 305 | 0.114 | 0.152 | 50.8% |
| NYKAA | vwap_trend_short_late_vwapslope_lb20 | 221 | 0.157 | 0.210 | 50.7% |
| CRISIL | vwap_trend_short_late_vwapslope_lb20 | 304 | 0.110 | 0.146 | 50.7% |
| BIOCON | vwap_trend_short_late_vwapslope_lb20 | 231 | 0.112 | 0.149 | 50.6% |
| DMART | vwap_trend_short_late_vwapslope_lb20 | 259 | 0.107 | 0.143 | 50.6% |
| ATGL | vwap_trend_short_late_vwapslope_lb20 | 267 | 0.083 | 0.111 | 50.6% |
| KPITTECH | vwap_trend_short_late_vwapslope_lb20 | 260 | 0.110 | 0.147 | 50.0% |
| HDFCBANK | vwap_trend_short_late_vwapslope_lb20 | 212 | 0.035 | 0.046 | 50.0% |
| IRCTC | vwap_trend_short_late_vwapslope_lb20 | 227 | 0.064 | 0.085 | 49.8% |
| MPHASIS | vwap_trend_short_late_vwapslope_lb20 | 209 | 0.174 | 0.232 | 49.8% |
| LTM | vwap_trend_short_late_vwapslope_lb20 | 209 | 0.072 | 0.096 | 49.8% |
| KPRMILL | vwap_trend_short_late_vwapslope_lb20 | 308 | 0.082 | 0.109 | 49.7% |
| INDIGO | vwap_trend_short_late_vwapslope_lb20 | 228 | 0.023 | 0.031 | 49.6% |
| KALYANKJIL | vwap_trend_short_late_vwapslope_lb20 | 310 | 0.088 | 0.118 | 49.4% |
| LUPIN | vwap_trend_short_late_vwapslope_lb20 | 223 | 0.090 | 0.120 | 49.3% |
| ABB | vwap_trend_short_late_vwapslope_lb20 | 221 | 0.051 | 0.068 | 49.3% |
| CIPLA | vwap_trend_short_late_vwapslope_lb20 | 199 | -0.011 | -0.015 | 49.2% |
| IOC | vwap_trend_short_late_vwapslope_lb20 | 248 | 0.132 | 0.176 | 49.2% |
| HDFCAMC | vwap_trend_short_late_vwapslope_lb20 | 246 | 0.044 | 0.059 | 49.2% |
| APOLLOHOSP | vwap_trend_short_late_vwapslope_lb20 | 204 | 0.070 | 0.093 | 49.0% |
| MANKIND | vwap_trend_short_late_vwapslope_lb20 | 162 | 0.103 | 0.137 | 48.8% |
| BERGEPAINT | vwap_trend_short_late_vwapslope_lb20 | 238 | 0.025 | 0.033 | 48.7% |
| OFSS | vwap_trend_short_late_vwapslope_lb20 | 277 | 0.044 | 0.059 | 48.7% |
| AXISBANK | vwap_trend_short_late_vwapslope_lb20 | 195 | -0.021 | -0.027 | 48.7% |
| BHARTIARTL | vwap_trend_short_late_vwapslope_lb20 | 229 | 0.007 | 0.010 | 48.5% |
| APLAPOLLO | vwap_trend_short_late_vwapslope_lb20 | 252 | 0.081 | 0.108 | 48.4% |
| INFY | vwap_trend_short_late_vwapslope_lb20 | 184 | -0.004 | -0.006 | 48.4% |
| LICHSGFIN | vwap_trend_short_late_vwapslope_lb20 | 213 | 0.035 | 0.046 | 48.4% |
| NESTLEIND | vwap_trend_short_late_vwapslope_lb20 | 228 | 0.008 | 0.011 | 48.2% |
| DABUR | vwap_trend_short_late_vwapslope_lb20 | 199 | 0.076 | 0.101 | 48.2% |
| GAIL | vwap_trend_short_late_vwapslope_lb20 | 226 | 0.063 | 0.085 | 48.2% |
| MOTILALOFS | vwap_trend_short_late_vwapslope_lb20 | 280 | 0.073 | 0.097 | 48.2% |
| ETERNAL | vwap_trend_short_late_vwapslope_lb20 | 218 | 0.082 | 0.109 | 48.2% |
| JKCEMENT | vwap_trend_short_late_vwapslope_lb20 | 253 | 0.031 | 0.041 | 47.8% |
| GODREJIND | vwap_trend_short_late_vwapslope_lb20 | 322 | -0.026 | -0.035 | 47.8% |
| ACC | vwap_trend_short_late_vwapslope_lb20 | 241 | 0.013 | 0.017 | 47.7% |
| COALINDIA | vwap_trend_short_late_vwapslope_lb20 | 195 | 0.038 | 0.050 | 47.7% |
| LTTS | vwap_trend_short_late_vwapslope_lb20 | 237 | 0.073 | 0.097 | 47.7% |
| GICRE | vwap_trend_short_late_vwapslope_lb20 | 344 | 0.062 | 0.082 | 47.7% |
| APOLLOTYRE | vwap_trend_short_late_vwapslope_lb20 | 235 | 0.068 | 0.090 | 47.7% |
| BLUESTARCO | vwap_trend_short_late_vwapslope_lb20 | 277 | 0.057 | 0.076 | 47.7% |
| AUROPHARMA | vwap_trend_short_late_vwapslope_lb20 | 225 | 0.029 | 0.039 | 47.6% |
| HCLTECH | vwap_trend_short_late_vwapslope_lb20 | 223 | 0.025 | 0.034 | 47.5% |
| ONGC | vwap_trend_short_late_vwapslope_lb20 | 219 | 0.020 | 0.027 | 47.5% |
| MAHABANK | vwap_trend_short_late_vwapslope_lb20 | 329 | 0.033 | 0.044 | 47.4% |
| NAUKRI | vwap_trend_short_late_vwapslope_lb20 | 230 | 0.083 | 0.111 | 47.4% |
| ADANIGREEN | vwap_trend_short_late_vwapslope_lb20 | 264 | 0.114 | 0.152 | 47.3% |
| NAM-INDIA | vwap_trend_short_late_vwapslope_lb20 | 264 | 0.074 | 0.099 | 47.3% |
| JSWSTEEL | vwap_trend_short_late_vwapslope_lb20 | 203 | 0.012 | 0.016 | 47.3% |
| ASIANPAINT | vwap_trend_short_late_vwapslope_lb20 | 201 | 0.034 | 0.046 | 47.3% |
| JUBLFOOD | vwap_trend_short_late_vwapslope_lb20 | 210 | -0.007 | -0.010 | 47.1% |
| NIACL | vwap_trend_short_late_vwapslope_lb20 | 329 | 0.101 | 0.134 | 47.1% |
| BRITANNIA | vwap_trend_short_late_vwapslope_lb20 | 234 | -0.001 | -0.001 | 47.0% |
| PAGEIND | vwap_trend_short_late_vwapslope_lb20 | 217 | 0.015 | 0.019 | 47.0% |
| ENDURANCE | vwap_trend_short_late_vwapslope_lb20 | 296 | 0.071 | 0.095 | 47.0% |
| NHPC | vwap_trend_short_late_vwapslope_lb20 | 318 | 0.072 | 0.096 | 46.9% |
| DRREDDY | vwap_trend_short_late_vwapslope_lb20 | 203 | -0.018 | -0.024 | 46.8% |
| JSWENERGY | vwap_trend_short_late_vwapslope_lb20 | 259 | 0.089 | 0.119 | 46.7% |
| BHEL | vwap_trend_short_late_vwapslope_lb20 | 242 | 0.074 | 0.099 | 46.7% |
| INDHOTEL | vwap_trend_short_late_vwapslope_lb20 | 236 | 0.082 | 0.109 | 46.6% |
| KEI | vwap_trend_short_late_vwapslope_lb20 | 247 | 0.048 | 0.064 | 46.6% |
| MARUTI | vwap_trend_short_late_vwapslope_lb20 | 226 | 0.018 | 0.024 | 46.5% |
| BEL | vwap_trend_short_late_vwapslope_lb20 | 207 | 0.030 | 0.040 | 46.4% |
| EICHERMOT | vwap_trend_short_late_vwapslope_lb20 | 220 | 0.003 | 0.004 | 46.4% |
| ICICIPRULI | vwap_trend_short_late_vwapslope_lb20 | 203 | 0.051 | 0.067 | 46.3% |
| M&M | vwap_trend_short_late_vwapslope_lb20 | 232 | 0.033 | 0.044 | 46.1% |
| CUMMINSIND | vwap_trend_short_late_vwapslope_lb20 | 217 | 0.006 | 0.009 | 46.1% |
| ABBOTINDIA | vwap_trend_short_late_vwapslope_lb20 | 315 | -0.077 | -0.102 | 46.0% |
| BAJAJFINSV | vwap_trend_short_late_vwapslope_lb20 | 192 | 0.039 | 0.052 | 45.8% |
| AIAENG | vwap_trend_short_late_vwapslope_lb20 | 280 | -0.022 | -0.030 | 45.7% |
| GODREJCP | vwap_trend_short_late_vwapslope_lb20 | 208 | -0.024 | -0.032 | 45.7% |
| IPCALAB | vwap_trend_short_late_vwapslope_lb20 | 241 | 0.043 | 0.058 | 45.6% |
| LAURUSLABS | vwap_trend_short_late_vwapslope_lb20 | 224 | 0.029 | 0.038 | 45.5% |
