# vwap-trend-improved/vwapslope_lb10 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 3.0x risk (2.250%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 165 instruments, 2021-03-23..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_late_vwapslope_lb10 | 40870 | 0.039 | 0.052 | 45.8% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LICI | vwap_trend_short_late_vwapslope_lb10 | 203 | 0.147 | 0.196 | 62.6% |
| AWL | vwap_trend_short_late_vwapslope_lb10 | 231 | 0.142 | 0.190 | 57.1% |
| ITC | vwap_trend_short_late_vwapslope_lb10 | 193 | 0.097 | 0.130 | 54.4% |
| EXIDEIND | vwap_trend_short_late_vwapslope_lb10 | 251 | 0.116 | 0.155 | 54.2% |
| MARICO | vwap_trend_short_late_vwapslope_lb10 | 202 | 0.060 | 0.080 | 54.0% |
| DIVISLAB | vwap_trend_short_late_vwapslope_lb10 | 217 | 0.093 | 0.124 | 53.9% |
| COCHINSHIP | vwap_trend_short_late_vwapslope_lb10 | 329 | 0.136 | 0.181 | 53.8% |
| INDUSINDBK | vwap_trend_short_late_vwapslope_lb10 | 212 | 0.166 | 0.222 | 53.3% |
| LINDEINDIA | vwap_trend_short_late_vwapslope_lb10 | 315 | 0.138 | 0.185 | 52.7% |
| NYKAA | vwap_trend_short_late_vwapslope_lb10 | 225 | 0.221 | 0.294 | 52.4% |
| HAVELLS | vwap_trend_short_late_vwapslope_lb10 | 251 | 0.074 | 0.099 | 52.2% |
| HEROMOTOCO | vwap_trend_short_late_vwapslope_lb10 | 215 | 0.093 | 0.124 | 51.6% |
| GLAXO | vwap_trend_short_late_vwapslope_lb10 | 342 | 0.121 | 0.162 | 51.2% |
| HONAUT | vwap_trend_short_late_vwapslope_lb10 | 344 | 0.055 | 0.074 | 51.2% |
| LTM | vwap_trend_short_late_vwapslope_lb10 | 216 | 0.114 | 0.152 | 50.9% |
| HUDCO | vwap_trend_short_late_vwapslope_lb10 | 327 | 0.085 | 0.113 | 50.8% |
| BIOCON | vwap_trend_short_late_vwapslope_lb10 | 233 | 0.116 | 0.154 | 50.6% |
| 3MINDIA | vwap_trend_short_late_vwapslope_lb10 | 399 | 0.059 | 0.079 | 50.6% |
| BDL | vwap_trend_short_late_vwapslope_lb10 | 298 | 0.165 | 0.221 | 50.3% |
| COALINDIA | vwap_trend_short_late_vwapslope_lb10 | 201 | 0.110 | 0.147 | 50.2% |
| CANBK | vwap_trend_short_late_vwapslope_lb10 | 213 | 0.139 | 0.185 | 50.2% |
| BAJAJ-AUTO | vwap_trend_short_late_vwapslope_lb10 | 219 | 0.087 | 0.116 | 50.2% |
| BANKINDIA | vwap_trend_short_late_vwapslope_lb10 | 308 | 0.124 | 0.165 | 50.0% |
| KALYANKJIL | vwap_trend_short_late_vwapslope_lb10 | 310 | 0.123 | 0.163 | 50.0% |
| CRISIL | vwap_trend_short_late_vwapslope_lb10 | 318 | 0.112 | 0.149 | 50.0% |
| HDFCLIFE | vwap_trend_short_late_vwapslope_lb10 | 196 | 0.067 | 0.089 | 50.0% |
| IRCTC | vwap_trend_short_late_vwapslope_lb10 | 221 | 0.073 | 0.097 | 49.8% |
| CIPLA | vwap_trend_short_late_vwapslope_lb10 | 205 | -0.026 | -0.034 | 49.8% |
| NLCINDIA | vwap_trend_short_late_vwapslope_lb10 | 324 | 0.151 | 0.202 | 49.7% |
| APARINDS | vwap_trend_short_late_vwapslope_lb10 | 320 | 0.215 | 0.287 | 49.7% |
| DLF | vwap_trend_short_late_vwapslope_lb10 | 238 | 0.155 | 0.207 | 49.6% |
| INDIGO | vwap_trend_short_late_vwapslope_lb10 | 226 | 0.046 | 0.061 | 49.6% |
| ONGC | vwap_trend_short_late_vwapslope_lb10 | 226 | 0.045 | 0.061 | 49.6% |
| MAZDOCK | vwap_trend_short_late_vwapslope_lb10 | 280 | 0.146 | 0.194 | 49.3% |
| DMART | vwap_trend_short_late_vwapslope_lb10 | 250 | 0.097 | 0.130 | 49.2% |
| GODFRYPHLP | vwap_trend_short_late_vwapslope_lb10 | 309 | 0.133 | 0.178 | 49.2% |
| OIL | vwap_trend_short_late_vwapslope_lb10 | 294 | 0.226 | 0.301 | 49.0% |
| HAL | vwap_trend_short_late_vwapslope_lb10 | 243 | 0.129 | 0.172 | 49.0% |
| DIXON | vwap_trend_short_late_vwapslope_lb10 | 235 | 0.189 | 0.251 | 48.9% |
| MANKIND | vwap_trend_short_late_vwapslope_lb10 | 168 | 0.153 | 0.204 | 48.8% |
| HDFCBANK | vwap_trend_short_late_vwapslope_lb10 | 205 | 0.013 | 0.017 | 48.8% |
| APOLLOTYRE | vwap_trend_short_late_vwapslope_lb10 | 236 | 0.094 | 0.125 | 48.7% |
| BERGEPAINT | vwap_trend_short_late_vwapslope_lb10 | 251 | 0.020 | 0.027 | 48.6% |
| ABB | vwap_trend_short_late_vwapslope_lb10 | 225 | 0.051 | 0.068 | 48.4% |
| APOLLOHOSP | vwap_trend_short_late_vwapslope_lb10 | 209 | 0.069 | 0.092 | 48.3% |
| ATGL | vwap_trend_short_late_vwapslope_lb10 | 290 | 0.069 | 0.092 | 48.3% |
| KPRMILL | vwap_trend_short_late_vwapslope_lb10 | 317 | 0.103 | 0.138 | 48.3% |
| GAIL | vwap_trend_short_late_vwapslope_lb10 | 224 | 0.074 | 0.098 | 48.2% |
| LUPIN | vwap_trend_short_late_vwapslope_lb10 | 220 | 0.073 | 0.097 | 48.2% |
| KPITTECH | vwap_trend_short_late_vwapslope_lb10 | 247 | 0.093 | 0.123 | 48.2% |
| JUBLFOOD | vwap_trend_short_late_vwapslope_lb10 | 208 | 0.015 | 0.020 | 48.1% |
| IOC | vwap_trend_short_late_vwapslope_lb10 | 250 | 0.099 | 0.132 | 48.0% |
| BHARTIARTL | vwap_trend_short_late_vwapslope_lb10 | 225 | -0.007 | -0.009 | 48.0% |
| LICHSGFIN | vwap_trend_short_late_vwapslope_lb10 | 219 | 0.049 | 0.065 | 47.9% |
| APLAPOLLO | vwap_trend_short_late_vwapslope_lb10 | 261 | 0.074 | 0.099 | 47.9% |
| MPHASIS | vwap_trend_short_late_vwapslope_lb10 | 209 | 0.169 | 0.225 | 47.8% |
| ACC | vwap_trend_short_late_vwapslope_lb10 | 253 | 0.029 | 0.038 | 47.8% |
| DABUR | vwap_trend_short_late_vwapslope_lb10 | 199 | 0.100 | 0.133 | 47.7% |
| HCLTECH | vwap_trend_short_late_vwapslope_lb10 | 220 | 0.043 | 0.057 | 47.7% |
| MOTILALOFS | vwap_trend_short_late_vwapslope_lb10 | 296 | 0.068 | 0.090 | 47.6% |
| AXISBANK | vwap_trend_short_late_vwapslope_lb10 | 196 | -0.029 | -0.039 | 47.4% |
| HDFCAMC | vwap_trend_short_late_vwapslope_lb10 | 247 | 0.020 | 0.026 | 47.4% |
| INFY | vwap_trend_short_late_vwapslope_lb10 | 188 | 0.004 | 0.006 | 47.3% |
| ETERNAL | vwap_trend_short_late_vwapslope_lb10 | 214 | 0.104 | 0.139 | 47.2% |
| JSWSTEEL | vwap_trend_short_late_vwapslope_lb10 | 208 | 0.047 | 0.062 | 47.1% |
| DRREDDY | vwap_trend_short_late_vwapslope_lb10 | 208 | -0.007 | -0.010 | 47.1% |
| GICRE | vwap_trend_short_late_vwapslope_lb10 | 366 | 0.074 | 0.099 | 47.0% |
| NESTLEIND | vwap_trend_short_late_vwapslope_lb10 | 218 | 0.008 | 0.011 | 46.8% |
| GODREJIND | vwap_trend_short_late_vwapslope_lb10 | 342 | -0.036 | -0.048 | 46.8% |
| NIACL | vwap_trend_short_late_vwapslope_lb10 | 343 | 0.110 | 0.146 | 46.6% |
| MARUTI | vwap_trend_short_late_vwapslope_lb10 | 232 | 0.031 | 0.042 | 46.6% |
| ENDURANCE | vwap_trend_short_late_vwapslope_lb10 | 303 | 0.109 | 0.146 | 46.5% |
| JKCEMENT | vwap_trend_short_late_vwapslope_lb10 | 265 | 0.044 | 0.059 | 46.4% |
| BLUESTARCO | vwap_trend_short_late_vwapslope_lb10 | 287 | 0.045 | 0.060 | 46.3% |
| LAURUSLABS | vwap_trend_short_late_vwapslope_lb10 | 223 | 0.082 | 0.109 | 46.2% |
| AUROPHARMA | vwap_trend_short_late_vwapslope_lb10 | 223 | 0.009 | 0.011 | 46.2% |
| JSWENERGY | vwap_trend_short_late_vwapslope_lb10 | 284 | 0.095 | 0.126 | 46.1% |
| BAJAJFINSV | vwap_trend_short_late_vwapslope_lb10 | 193 | 0.048 | 0.063 | 46.1% |
| COLPAL | vwap_trend_short_late_vwapslope_lb10 | 226 | 0.075 | 0.100 | 46.0% |
| MAHABANK | vwap_trend_short_late_vwapslope_lb10 | 346 | 0.029 | 0.038 | 46.0% |
| NHPC | vwap_trend_short_late_vwapslope_lb10 | 333 | 0.118 | 0.157 | 45.9% |
| BANKBARODA | vwap_trend_short_late_vwapslope_lb10 | 222 | 0.020 | 0.027 | 45.9% |
| LTTS | vwap_trend_short_late_vwapslope_lb10 | 242 | 0.050 | 0.066 | 45.9% |
| BRITANNIA | vwap_trend_short_late_vwapslope_lb10 | 236 | -0.014 | -0.019 | 45.8% |
| CUMMINSIND | vwap_trend_short_late_vwapslope_lb10 | 219 | 0.018 | 0.024 | 45.7% |
| ICICIPRULI | vwap_trend_short_late_vwapslope_lb10 | 204 | 0.051 | 0.068 | 45.6% |
| MEDANTA | vwap_trend_short_late_vwapslope_lb10 | 185 | 0.092 | 0.122 | 45.4% |
| BHEL | vwap_trend_short_late_vwapslope_lb10 | 236 | 0.111 | 0.148 | 45.3% |
| PAGEIND | vwap_trend_short_late_vwapslope_lb10 | 234 | 0.016 | 0.021 | 45.3% |
| OFSS | vwap_trend_short_late_vwapslope_lb10 | 287 | 0.027 | 0.036 | 45.3% |
| AIAENG | vwap_trend_short_late_vwapslope_lb10 | 296 | -0.023 | -0.031 | 45.3% |
| ASIANPAINT | vwap_trend_short_late_vwapslope_lb10 | 199 | 0.025 | 0.033 | 45.2% |
| EICHERMOT | vwap_trend_short_late_vwapslope_lb10 | 219 | 0.005 | 0.006 | 45.2% |
| NTPC | vwap_trend_short_late_vwapslope_lb10 | 236 | 0.011 | 0.014 | 44.9% |
| ABBOTINDIA | vwap_trend_short_late_vwapslope_lb10 | 323 | -0.092 | -0.122 | 44.9% |
| INDHOTEL | vwap_trend_short_late_vwapslope_lb10 | 239 | 0.073 | 0.098 | 44.8% |
| M&M | vwap_trend_short_late_vwapslope_lb10 | 222 | 0.014 | 0.019 | 44.6% |
| MUTHOOTFIN | vwap_trend_short_late_vwapslope_lb10 | 222 | -0.059 | -0.079 | 44.6% |
| IPCALAB | vwap_trend_short_late_vwapslope_lb10 | 256 | 0.049 | 0.065 | 44.5% |
| INDIANB | vwap_trend_short_late_vwapslope_lb10 | 283 | -0.014 | -0.019 | 44.5% |
