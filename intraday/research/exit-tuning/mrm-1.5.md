# exit-tuning/mrm-1.5 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 1.5x risk (1.125%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 165 instruments, 2021-03-23..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_late_vwapslope_lb20 | 39940 | 0.031 | 0.042 | 48.1% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LICI | vwap_trend_short_late_vwapslope_lb20 | 200 | 0.176 | 0.235 | 66.0% |
| AWL | vwap_trend_short_late_vwapslope_lb20 | 222 | 0.126 | 0.168 | 59.9% |
| EXIDEIND | vwap_trend_short_late_vwapslope_lb20 | 254 | 0.139 | 0.186 | 57.9% |
| NLCINDIA | vwap_trend_short_late_vwapslope_lb20 | 301 | 0.195 | 0.260 | 55.8% |
| HUDCO | vwap_trend_short_late_vwapslope_lb20 | 312 | 0.116 | 0.155 | 55.8% |
| LINDEINDIA | vwap_trend_short_late_vwapslope_lb20 | 301 | 0.157 | 0.209 | 55.1% |
| INDUSINDBK | vwap_trend_short_late_vwapslope_lb20 | 200 | 0.141 | 0.187 | 55.0% |
| ITC | vwap_trend_short_late_vwapslope_lb20 | 197 | 0.101 | 0.135 | 54.8% |
| COCHINSHIP | vwap_trend_short_late_vwapslope_lb20 | 325 | 0.111 | 0.147 | 54.8% |
| DIVISLAB | vwap_trend_short_late_vwapslope_lb20 | 210 | 0.061 | 0.082 | 54.8% |
| MAZDOCK | vwap_trend_short_late_vwapslope_lb20 | 269 | 0.138 | 0.184 | 54.6% |
| DIXON | vwap_trend_short_late_vwapslope_lb20 | 244 | 0.186 | 0.248 | 54.5% |
| MARICO | vwap_trend_short_late_vwapslope_lb20 | 201 | 0.056 | 0.075 | 54.2% |
| ATGL | vwap_trend_short_late_vwapslope_lb20 | 267 | 0.095 | 0.127 | 53.9% |
| DLF | vwap_trend_short_late_vwapslope_lb20 | 232 | 0.155 | 0.206 | 53.9% |
| APARINDS | vwap_trend_short_late_vwapslope_lb20 | 294 | 0.180 | 0.240 | 53.7% |
| HONAUT | vwap_trend_short_late_vwapslope_lb20 | 323 | 0.069 | 0.092 | 53.6% |
| HAL | vwap_trend_short_late_vwapslope_lb20 | 247 | 0.178 | 0.238 | 53.4% |
| 3MINDIA | vwap_trend_short_late_vwapslope_lb20 | 364 | 0.068 | 0.090 | 53.3% |
| MPHASIS | vwap_trend_short_late_vwapslope_lb20 | 209 | 0.166 | 0.221 | 53.1% |
| HAVELLS | vwap_trend_short_late_vwapslope_lb20 | 243 | 0.053 | 0.070 | 53.1% |
| OIL | vwap_trend_short_late_vwapslope_lb20 | 289 | 0.235 | 0.313 | 52.9% |
| NYKAA | vwap_trend_short_late_vwapslope_lb20 | 221 | 0.134 | 0.179 | 52.9% |
| CANBK | vwap_trend_short_late_vwapslope_lb20 | 222 | 0.142 | 0.190 | 52.7% |
| GLAXO | vwap_trend_short_late_vwapslope_lb20 | 321 | 0.088 | 0.118 | 52.6% |
| HEROMOTOCO | vwap_trend_short_late_vwapslope_lb20 | 211 | 0.094 | 0.126 | 52.1% |
| HDFCLIFE | vwap_trend_short_late_vwapslope_lb20 | 200 | 0.057 | 0.076 | 52.0% |
| INDIGO | vwap_trend_short_late_vwapslope_lb20 | 229 | 0.035 | 0.047 | 52.0% |
| GODFRYPHLP | vwap_trend_short_late_vwapslope_lb20 | 289 | 0.123 | 0.163 | 51.9% |
| BDL | vwap_trend_short_late_vwapslope_lb20 | 289 | 0.100 | 0.134 | 51.9% |
| LICHSGFIN | vwap_trend_short_late_vwapslope_lb20 | 213 | 0.029 | 0.039 | 51.6% |
| BAJAJ-AUTO | vwap_trend_short_late_vwapslope_lb20 | 227 | 0.084 | 0.112 | 51.5% |
| BIOCON | vwap_trend_short_late_vwapslope_lb20 | 231 | 0.099 | 0.132 | 51.5% |
| ETERNAL | vwap_trend_short_late_vwapslope_lb20 | 218 | 0.091 | 0.122 | 51.4% |
| CRISIL | vwap_trend_short_late_vwapslope_lb20 | 304 | 0.109 | 0.145 | 51.3% |
| KPITTECH | vwap_trend_short_late_vwapslope_lb20 | 260 | 0.110 | 0.146 | 51.2% |
| DMART | vwap_trend_short_late_vwapslope_lb20 | 259 | 0.102 | 0.136 | 51.0% |
| GODREJIND | vwap_trend_short_late_vwapslope_lb20 | 322 | -0.016 | -0.021 | 50.9% |
| BANKINDIA | vwap_trend_short_late_vwapslope_lb20 | 305 | 0.078 | 0.104 | 50.8% |
| HDFCAMC | vwap_trend_short_late_vwapslope_lb20 | 246 | 0.040 | 0.053 | 50.8% |
| LTM | vwap_trend_short_late_vwapslope_lb20 | 209 | 0.068 | 0.090 | 50.7% |
| ABB | vwap_trend_short_late_vwapslope_lb20 | 221 | 0.035 | 0.047 | 50.7% |
| KALYANKJIL | vwap_trend_short_late_vwapslope_lb20 | 310 | 0.069 | 0.092 | 50.3% |
| IRCTC | vwap_trend_short_late_vwapslope_lb20 | 227 | 0.053 | 0.071 | 50.2% |
| HDFCBANK | vwap_trend_short_late_vwapslope_lb20 | 212 | 0.032 | 0.043 | 50.0% |
| OFSS | vwap_trend_short_late_vwapslope_lb20 | 278 | 0.030 | 0.040 | 50.0% |
| APLAPOLLO | vwap_trend_short_late_vwapslope_lb20 | 253 | 0.093 | 0.124 | 49.8% |
| LUPIN | vwap_trend_short_late_vwapslope_lb20 | 223 | 0.089 | 0.118 | 49.8% |
| JSWSTEEL | vwap_trend_short_late_vwapslope_lb20 | 203 | 0.040 | 0.053 | 49.8% |
| KPRMILL | vwap_trend_short_late_vwapslope_lb20 | 308 | 0.054 | 0.073 | 49.7% |
| NAUKRI | vwap_trend_short_late_vwapslope_lb20 | 230 | 0.078 | 0.104 | 49.6% |
| AUROPHARMA | vwap_trend_short_late_vwapslope_lb20 | 226 | 0.029 | 0.039 | 49.6% |
| JUBLFOOD | vwap_trend_short_late_vwapslope_lb20 | 210 | -0.001 | -0.002 | 49.5% |
| APOLLOHOSP | vwap_trend_short_late_vwapslope_lb20 | 206 | 0.063 | 0.085 | 49.5% |
| COALINDIA | vwap_trend_short_late_vwapslope_lb20 | 196 | 0.051 | 0.068 | 49.5% |
| NAM-INDIA | vwap_trend_short_late_vwapslope_lb20 | 265 | 0.061 | 0.081 | 49.4% |
| JKCEMENT | vwap_trend_short_late_vwapslope_lb20 | 253 | 0.022 | 0.030 | 49.4% |
| MANKIND | vwap_trend_short_late_vwapslope_lb20 | 162 | 0.082 | 0.109 | 49.4% |
| GAIL | vwap_trend_short_late_vwapslope_lb20 | 225 | 0.061 | 0.082 | 49.3% |
| PAGEIND | vwap_trend_short_late_vwapslope_lb20 | 217 | 0.015 | 0.020 | 49.3% |
| MOTILALOFS | vwap_trend_short_late_vwapslope_lb20 | 280 | 0.044 | 0.059 | 49.3% |
| CIPLA | vwap_trend_short_late_vwapslope_lb20 | 199 | -0.015 | -0.019 | 49.2% |
| ADANIGREEN | vwap_trend_short_late_vwapslope_lb20 | 264 | 0.101 | 0.135 | 49.2% |
| IOC | vwap_trend_short_late_vwapslope_lb20 | 248 | 0.120 | 0.161 | 49.2% |
| BHEL | vwap_trend_short_late_vwapslope_lb20 | 242 | 0.076 | 0.101 | 49.2% |
| NESTLEIND | vwap_trend_short_late_vwapslope_lb20 | 228 | 0.015 | 0.020 | 49.1% |
| INFY | vwap_trend_short_late_vwapslope_lb20 | 184 | -0.010 | -0.014 | 48.9% |
| HCLTECH | vwap_trend_short_late_vwapslope_lb20 | 223 | 0.036 | 0.048 | 48.9% |
| BERGEPAINT | vwap_trend_short_late_vwapslope_lb20 | 238 | 0.008 | 0.011 | 48.7% |
| AXISBANK | vwap_trend_short_late_vwapslope_lb20 | 195 | -0.031 | -0.041 | 48.7% |
| MAHABANK | vwap_trend_short_late_vwapslope_lb20 | 329 | 0.008 | 0.011 | 48.6% |
| AIAENG | vwap_trend_short_late_vwapslope_lb20 | 280 | -0.008 | -0.010 | 48.6% |
| LTTS | vwap_trend_short_late_vwapslope_lb20 | 237 | 0.068 | 0.090 | 48.5% |
| BHARTIARTL | vwap_trend_short_late_vwapslope_lb20 | 229 | -0.001 | -0.001 | 48.5% |
| NHPC | vwap_trend_short_late_vwapslope_lb20 | 318 | 0.059 | 0.078 | 48.4% |
| ONGC | vwap_trend_short_late_vwapslope_lb20 | 219 | 0.015 | 0.020 | 48.4% |
| ENDURANCE | vwap_trend_short_late_vwapslope_lb20 | 296 | 0.073 | 0.097 | 48.3% |
| BEL | vwap_trend_short_late_vwapslope_lb20 | 207 | 0.026 | 0.035 | 48.3% |
| JSWENERGY | vwap_trend_short_late_vwapslope_lb20 | 259 | 0.065 | 0.087 | 48.3% |
| ASIANPAINT | vwap_trend_short_late_vwapslope_lb20 | 201 | 0.035 | 0.046 | 48.3% |
| DABUR | vwap_trend_short_late_vwapslope_lb20 | 199 | 0.075 | 0.100 | 48.2% |
| KEI | vwap_trend_short_late_vwapslope_lb20 | 247 | 0.049 | 0.066 | 48.2% |
| ACC | vwap_trend_short_late_vwapslope_lb20 | 241 | 0.017 | 0.023 | 48.1% |
| APOLLOTYRE | vwap_trend_short_late_vwapslope_lb20 | 235 | 0.066 | 0.089 | 48.1% |
| DRREDDY | vwap_trend_short_late_vwapslope_lb20 | 204 | -0.003 | -0.004 | 48.0% |
| NIACL | vwap_trend_short_late_vwapslope_lb20 | 329 | 0.091 | 0.121 | 48.0% |
| GICRE | vwap_trend_short_late_vwapslope_lb20 | 344 | 0.047 | 0.062 | 48.0% |
| BLUESTARCO | vwap_trend_short_late_vwapslope_lb20 | 278 | 0.031 | 0.041 | 47.8% |
| ICICIPRULI | vwap_trend_short_late_vwapslope_lb20 | 203 | 0.043 | 0.057 | 47.8% |
| LAURUSLABS | vwap_trend_short_late_vwapslope_lb20 | 224 | 0.043 | 0.057 | 47.3% |
| CHOLAFIN | vwap_trend_short_late_vwapslope_lb20 | 205 | 0.014 | 0.019 | 47.3% |
| IPCALAB | vwap_trend_short_late_vwapslope_lb20 | 241 | 0.053 | 0.070 | 47.3% |
| EICHERMOT | vwap_trend_short_late_vwapslope_lb20 | 220 | -0.001 | -0.002 | 47.3% |
| MARUTI | vwap_trend_short_late_vwapslope_lb20 | 227 | 0.020 | 0.027 | 47.1% |
| CONCOR | vwap_trend_short_late_vwapslope_lb20 | 221 | 0.005 | 0.007 | 47.1% |
| INDHOTEL | vwap_trend_short_late_vwapslope_lb20 | 236 | 0.074 | 0.098 | 47.0% |
| COROMANDEL | vwap_trend_short_late_vwapslope_lb20 | 236 | 0.035 | 0.046 | 47.0% |
| BRITANNIA | vwap_trend_short_late_vwapslope_lb20 | 234 | -0.002 | -0.003 | 47.0% |
| INDIANB | vwap_trend_short_late_vwapslope_lb20 | 279 | -0.021 | -0.028 | 47.0% |
| MOTHERSON | vwap_trend_short_late_vwapslope_lb20 | 258 | 0.042 | 0.057 | 46.9% |
