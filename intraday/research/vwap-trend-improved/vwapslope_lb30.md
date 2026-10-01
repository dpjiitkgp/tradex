# vwap-trend-improved/vwapslope_lb30 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 3.0x risk (2.250%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 165 instruments, 2021-03-23..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_late_vwapslope_lb30 | 39092 | 0.039 | 0.052 | 45.8% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LICI | vwap_trend_short_late_vwapslope_lb30 | 193 | 0.167 | 0.223 | 63.2% |
| AWL | vwap_trend_short_late_vwapslope_lb30 | 231 | 0.148 | 0.197 | 57.6% |
| EXIDEIND | vwap_trend_short_late_vwapslope_lb30 | 250 | 0.117 | 0.156 | 56.4% |
| MARICO | vwap_trend_short_late_vwapslope_lb30 | 196 | 0.080 | 0.106 | 55.6% |
| LINDEINDIA | vwap_trend_short_late_vwapslope_lb30 | 281 | 0.155 | 0.207 | 53.7% |
| COCHINSHIP | vwap_trend_short_late_vwapslope_lb30 | 308 | 0.143 | 0.191 | 53.6% |
| DIVISLAB | vwap_trend_short_late_vwapslope_lb30 | 214 | 0.050 | 0.067 | 53.3% |
| HUDCO | vwap_trend_short_late_vwapslope_lb30 | 305 | 0.148 | 0.197 | 52.5% |
| HONAUT | vwap_trend_short_late_vwapslope_lb30 | 317 | 0.084 | 0.113 | 52.4% |
| ITC | vwap_trend_short_late_vwapslope_lb30 | 194 | 0.062 | 0.082 | 52.1% |
| CANBK | vwap_trend_short_late_vwapslope_lb30 | 218 | 0.193 | 0.257 | 51.8% |
| INDUSINDBK | vwap_trend_short_late_vwapslope_lb30 | 201 | 0.167 | 0.222 | 51.7% |
| NYKAA | vwap_trend_short_late_vwapslope_lb30 | 214 | 0.182 | 0.243 | 51.4% |
| HAVELLS | vwap_trend_short_late_vwapslope_lb30 | 227 | 0.064 | 0.085 | 51.1% |
| HAL | vwap_trend_short_late_vwapslope_lb30 | 245 | 0.182 | 0.243 | 51.0% |
| NLCINDIA | vwap_trend_short_late_vwapslope_lb30 | 308 | 0.171 | 0.229 | 51.0% |
| 3MINDIA | vwap_trend_short_late_vwapslope_lb30 | 316 | 0.071 | 0.095 | 50.9% |
| APARINDS | vwap_trend_short_late_vwapslope_lb30 | 291 | 0.216 | 0.288 | 50.9% |
| APOLLOHOSP | vwap_trend_short_late_vwapslope_lb30 | 211 | 0.107 | 0.142 | 50.7% |
| INDIGO | vwap_trend_short_late_vwapslope_lb30 | 223 | 0.072 | 0.096 | 50.7% |
| HEROMOTOCO | vwap_trend_short_late_vwapslope_lb30 | 216 | 0.095 | 0.126 | 50.5% |
| BAJAJ-AUTO | vwap_trend_short_late_vwapslope_lb30 | 226 | 0.097 | 0.129 | 50.4% |
| GLAXO | vwap_trend_short_late_vwapslope_lb30 | 302 | 0.088 | 0.118 | 50.3% |
| ATGL | vwap_trend_short_late_vwapslope_lb30 | 261 | 0.116 | 0.154 | 50.2% |
| BIOCON | vwap_trend_short_late_vwapslope_lb30 | 228 | 0.115 | 0.153 | 50.0% |
| CRISIL | vwap_trend_short_late_vwapslope_lb30 | 286 | 0.073 | 0.097 | 50.0% |
| DLF | vwap_trend_short_late_vwapslope_lb30 | 231 | 0.138 | 0.184 | 49.8% |
| INFY | vwap_trend_short_late_vwapslope_lb30 | 189 | 0.026 | 0.035 | 49.7% |
| DMART | vwap_trend_short_late_vwapslope_lb30 | 254 | 0.110 | 0.146 | 49.6% |
| GAIL | vwap_trend_short_late_vwapslope_lb30 | 216 | 0.112 | 0.149 | 49.5% |
| BANKINDIA | vwap_trend_short_late_vwapslope_lb30 | 315 | 0.138 | 0.184 | 49.5% |
| OIL | vwap_trend_short_late_vwapslope_lb30 | 285 | 0.237 | 0.316 | 49.5% |
| BDL | vwap_trend_short_late_vwapslope_lb30 | 285 | 0.108 | 0.144 | 49.5% |
| MAZDOCK | vwap_trend_short_late_vwapslope_lb30 | 265 | 0.145 | 0.193 | 49.4% |
| BRITANNIA | vwap_trend_short_late_vwapslope_lb30 | 221 | 0.002 | 0.002 | 49.3% |
| IRCTC | vwap_trend_short_late_vwapslope_lb30 | 219 | 0.042 | 0.057 | 49.3% |
| MPHASIS | vwap_trend_short_late_vwapslope_lb30 | 215 | 0.184 | 0.245 | 49.3% |
| LTM | vwap_trend_short_late_vwapslope_lb30 | 213 | 0.109 | 0.146 | 49.3% |
| LUPIN | vwap_trend_short_late_vwapslope_lb30 | 211 | 0.099 | 0.132 | 49.3% |
| JUBLFOOD | vwap_trend_short_late_vwapslope_lb30 | 211 | 0.035 | 0.047 | 49.3% |
| DIXON | vwap_trend_short_late_vwapslope_lb30 | 234 | 0.194 | 0.259 | 49.1% |
| IOC | vwap_trend_short_late_vwapslope_lb30 | 247 | 0.113 | 0.151 | 49.0% |
| KPITTECH | vwap_trend_short_late_vwapslope_lb30 | 242 | 0.125 | 0.167 | 48.8% |
| HDFCLIFE | vwap_trend_short_late_vwapslope_lb30 | 197 | 0.048 | 0.064 | 48.7% |
| GODFRYPHLP | vwap_trend_short_late_vwapslope_lb30 | 298 | 0.140 | 0.186 | 48.7% |
| COALINDIA | vwap_trend_short_late_vwapslope_lb30 | 202 | 0.051 | 0.068 | 48.5% |
| ABB | vwap_trend_short_late_vwapslope_lb30 | 225 | 0.043 | 0.058 | 48.4% |
| NESTLEIND | vwap_trend_short_late_vwapslope_lb30 | 211 | 0.035 | 0.046 | 48.3% |
| KPRMILL | vwap_trend_short_late_vwapslope_lb30 | 300 | 0.093 | 0.124 | 48.3% |
| KALYANKJIL | vwap_trend_short_late_vwapslope_lb30 | 298 | 0.084 | 0.112 | 48.3% |
| APOLLOTYRE | vwap_trend_short_late_vwapslope_lb30 | 233 | 0.079 | 0.105 | 48.1% |
| DRREDDY | vwap_trend_short_late_vwapslope_lb30 | 204 | 0.028 | 0.037 | 48.0% |
| APLAPOLLO | vwap_trend_short_late_vwapslope_lb30 | 238 | 0.089 | 0.119 | 47.9% |
| HDFCBANK | vwap_trend_short_late_vwapslope_lb30 | 207 | -0.026 | -0.034 | 47.8% |
| ACC | vwap_trend_short_late_vwapslope_lb30 | 247 | 0.036 | 0.047 | 47.8% |
| GICRE | vwap_trend_short_late_vwapslope_lb30 | 331 | 0.061 | 0.082 | 47.7% |
| MANKIND | vwap_trend_short_late_vwapslope_lb30 | 151 | 0.101 | 0.134 | 47.7% |
| LICHSGFIN | vwap_trend_short_late_vwapslope_lb30 | 206 | 0.036 | 0.048 | 47.6% |
| GODREJIND | vwap_trend_short_late_vwapslope_lb30 | 313 | -0.027 | -0.036 | 47.3% |
| BERGEPAINT | vwap_trend_short_late_vwapslope_lb30 | 235 | -0.009 | -0.012 | 47.2% |
| JKCEMENT | vwap_trend_short_late_vwapslope_lb30 | 249 | 0.080 | 0.107 | 47.0% |
| MAHABANK | vwap_trend_short_late_vwapslope_lb30 | 313 | 0.060 | 0.080 | 47.0% |
| AUROPHARMA | vwap_trend_short_late_vwapslope_lb30 | 226 | 0.008 | 0.011 | 46.9% |
| AXISBANK | vwap_trend_short_late_vwapslope_lb30 | 209 | -0.004 | -0.005 | 46.9% |
| JSWSTEEL | vwap_trend_short_late_vwapslope_lb30 | 203 | 0.038 | 0.051 | 46.8% |
| EICHERMOT | vwap_trend_short_late_vwapslope_lb30 | 221 | 0.023 | 0.031 | 46.6% |
| BANKBARODA | vwap_trend_short_late_vwapslope_lb30 | 219 | 0.006 | 0.007 | 46.6% |
| NIACL | vwap_trend_short_late_vwapslope_lb30 | 325 | 0.111 | 0.148 | 46.5% |
| LTTS | vwap_trend_short_late_vwapslope_lb30 | 235 | 0.049 | 0.065 | 46.4% |
| MOTILALOFS | vwap_trend_short_late_vwapslope_lb30 | 277 | 0.054 | 0.072 | 46.2% |
| LAURUSLABS | vwap_trend_short_late_vwapslope_lb30 | 210 | 0.086 | 0.115 | 46.2% |
| HCLTECH | vwap_trend_short_late_vwapslope_lb30 | 223 | 0.024 | 0.032 | 46.2% |
| PAGEIND | vwap_trend_short_late_vwapslope_lb30 | 221 | 0.027 | 0.036 | 46.2% |
| ASIANPAINT | vwap_trend_short_late_vwapslope_lb30 | 202 | 0.037 | 0.049 | 46.0% |
| DABUR | vwap_trend_short_late_vwapslope_lb30 | 207 | 0.052 | 0.070 | 45.9% |
| OFSS | vwap_trend_short_late_vwapslope_lb30 | 266 | 0.025 | 0.034 | 45.9% |
| CIPLA | vwap_trend_short_late_vwapslope_lb30 | 205 | -0.032 | -0.042 | 45.9% |
| IDFCFIRSTB | vwap_trend_short_late_vwapslope_lb30 | 236 | 0.006 | 0.008 | 45.8% |
| NAM-INDIA | vwap_trend_short_late_vwapslope_lb30 | 256 | 0.067 | 0.089 | 45.7% |
| JSWENERGY | vwap_trend_short_late_vwapslope_lb30 | 263 | 0.091 | 0.121 | 45.6% |
| BLUESTARCO | vwap_trend_short_late_vwapslope_lb30 | 263 | 0.034 | 0.045 | 45.6% |
| ADANIGREEN | vwap_trend_short_late_vwapslope_lb30 | 261 | 0.118 | 0.158 | 45.6% |
| CONCOR | vwap_trend_short_late_vwapslope_lb30 | 215 | 0.033 | 0.044 | 45.6% |
| NAUKRI | vwap_trend_short_late_vwapslope_lb30 | 224 | 0.105 | 0.140 | 45.5% |
| NHPC | vwap_trend_short_late_vwapslope_lb30 | 301 | 0.081 | 0.108 | 45.5% |
| BAJAJFINSV | vwap_trend_short_late_vwapslope_lb30 | 189 | 0.058 | 0.077 | 45.5% |
| MARUTI | vwap_trend_short_late_vwapslope_lb30 | 233 | 0.005 | 0.007 | 45.5% |
| ENDURANCE | vwap_trend_short_late_vwapslope_lb30 | 280 | 0.059 | 0.079 | 45.4% |
| BPCL | vwap_trend_short_late_vwapslope_lb30 | 232 | 0.014 | 0.018 | 45.3% |
| BAJAJHLDNG | vwap_trend_short_late_vwapslope_lb30 | 246 | 0.078 | 0.103 | 45.1% |
| HDFCAMC | vwap_trend_short_late_vwapslope_lb30 | 242 | 0.007 | 0.009 | 45.0% |
| ONGC | vwap_trend_short_late_vwapslope_lb30 | 212 | -0.010 | -0.013 | 44.8% |
| HINDPETRO | vwap_trend_short_late_vwapslope_lb30 | 219 | 0.056 | 0.075 | 44.7% |
| JSL | vwap_trend_short_late_vwapslope_lb30 | 237 | 0.179 | 0.239 | 44.7% |
| CUMMINSIND | vwap_trend_short_late_vwapslope_lb30 | 220 | -0.002 | -0.003 | 44.5% |
| M&M | vwap_trend_short_late_vwapslope_lb30 | 230 | 0.007 | 0.009 | 44.3% |
| COROMANDEL | vwap_trend_short_late_vwapslope_lb30 | 224 | -0.023 | -0.030 | 44.2% |
| IRFC | vwap_trend_short_late_vwapslope_lb30 | 356 | 0.022 | 0.029 | 44.1% |
| INDIANB | vwap_trend_short_late_vwapslope_lb30 | 277 | -0.030 | -0.040 | 44.0% |
| ETERNAL | vwap_trend_short_late_vwapslope_lb30 | 218 | 0.029 | 0.039 | 44.0% |
