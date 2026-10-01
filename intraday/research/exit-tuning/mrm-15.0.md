# exit-tuning/mrm-15.0 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 15.0x risk (11.250%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 165 instruments, 2021-03-23..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_late_vwapslope_lb20 | 39576 | 0.040 | 0.053 | 45.4% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LICI | vwap_trend_short_late_vwapslope_lb20 | 196 | 0.168 | 0.224 | 65.8% |
| AWL | vwap_trend_short_late_vwapslope_lb20 | 222 | 0.179 | 0.239 | 58.1% |
| EXIDEIND | vwap_trend_short_late_vwapslope_lb20 | 254 | 0.125 | 0.167 | 55.9% |
| MARICO | vwap_trend_short_late_vwapslope_lb20 | 194 | 0.043 | 0.058 | 54.1% |
| INDUSINDBK | vwap_trend_short_late_vwapslope_lb20 | 195 | 0.197 | 0.263 | 53.8% |
| ITC | vwap_trend_short_late_vwapslope_lb20 | 192 | 0.072 | 0.096 | 53.6% |
| DIVISLAB | vwap_trend_short_late_vwapslope_lb20 | 208 | 0.069 | 0.092 | 53.4% |
| COCHINSHIP | vwap_trend_short_late_vwapslope_lb20 | 322 | 0.166 | 0.222 | 53.1% |
| LINDEINDIA | vwap_trend_short_late_vwapslope_lb20 | 301 | 0.157 | 0.209 | 52.8% |
| HAVELLS | vwap_trend_short_late_vwapslope_lb20 | 241 | 0.081 | 0.108 | 51.5% |
| HONAUT | vwap_trend_short_late_vwapslope_lb20 | 323 | 0.064 | 0.085 | 51.4% |
| HUDCO | vwap_trend_short_late_vwapslope_lb20 | 312 | 0.079 | 0.106 | 51.3% |
| HEROMOTOCO | vwap_trend_short_late_vwapslope_lb20 | 205 | 0.157 | 0.210 | 51.2% |
| HAL | vwap_trend_short_late_vwapslope_lb20 | 247 | 0.159 | 0.212 | 51.0% |
| 3MINDIA | vwap_trend_short_late_vwapslope_lb20 | 364 | 0.074 | 0.099 | 50.8% |
| HDFCLIFE | vwap_trend_short_late_vwapslope_lb20 | 200 | 0.061 | 0.082 | 50.5% |
| GLAXO | vwap_trend_short_late_vwapslope_lb20 | 321 | 0.103 | 0.137 | 50.5% |
| BIOCON | vwap_trend_short_late_vwapslope_lb20 | 228 | 0.161 | 0.215 | 50.4% |
| DLF | vwap_trend_short_late_vwapslope_lb20 | 232 | 0.162 | 0.217 | 50.4% |
| GODFRYPHLP | vwap_trend_short_late_vwapslope_lb20 | 286 | 0.248 | 0.330 | 50.3% |
| DIXON | vwap_trend_short_late_vwapslope_lb20 | 242 | 0.200 | 0.267 | 50.0% |
| BAJAJ-AUTO | vwap_trend_short_late_vwapslope_lb20 | 222 | 0.078 | 0.104 | 50.0% |
| NLCINDIA | vwap_trend_short_late_vwapslope_lb20 | 301 | 0.161 | 0.215 | 49.8% |
| APARINDS | vwap_trend_short_late_vwapslope_lb20 | 294 | 0.201 | 0.269 | 49.7% |
| ABB | vwap_trend_short_late_vwapslope_lb20 | 216 | 0.074 | 0.098 | 49.5% |
| HDFCBANK | vwap_trend_short_late_vwapslope_lb20 | 206 | 0.023 | 0.031 | 49.5% |
| BDL | vwap_trend_short_late_vwapslope_lb20 | 289 | 0.123 | 0.164 | 49.5% |
| CRISIL | vwap_trend_short_late_vwapslope_lb20 | 304 | 0.105 | 0.141 | 49.3% |
| IRCTC | vwap_trend_short_late_vwapslope_lb20 | 227 | 0.052 | 0.069 | 49.3% |
| BANKINDIA | vwap_trend_short_late_vwapslope_lb20 | 300 | 0.130 | 0.173 | 49.3% |
| LTM | vwap_trend_short_late_vwapslope_lb20 | 207 | 0.125 | 0.166 | 49.3% |
| MAZDOCK | vwap_trend_short_late_vwapslope_lb20 | 268 | 0.211 | 0.282 | 49.3% |
| DMART | vwap_trend_short_late_vwapslope_lb20 | 256 | 0.092 | 0.123 | 49.2% |
| CANBK | vwap_trend_short_late_vwapslope_lb20 | 216 | 0.106 | 0.141 | 49.1% |
| NYKAA | vwap_trend_short_late_vwapslope_lb20 | 221 | 0.141 | 0.188 | 48.9% |
| KPITTECH | vwap_trend_short_late_vwapslope_lb20 | 260 | 0.127 | 0.169 | 48.8% |
| CIPLA | vwap_trend_short_late_vwapslope_lb20 | 199 | -0.026 | -0.035 | 48.7% |
| INDIGO | vwap_trend_short_late_vwapslope_lb20 | 224 | 0.058 | 0.077 | 48.7% |
| HDFCAMC | vwap_trend_short_late_vwapslope_lb20 | 241 | 0.043 | 0.057 | 48.5% |
| MPHASIS | vwap_trend_short_late_vwapslope_lb20 | 206 | 0.162 | 0.216 | 48.5% |
| LUPIN | vwap_trend_short_late_vwapslope_lb20 | 217 | 0.075 | 0.100 | 48.4% |
| IOC | vwap_trend_short_late_vwapslope_lb20 | 244 | 0.104 | 0.139 | 48.4% |
| BERGEPAINT | vwap_trend_short_late_vwapslope_lb20 | 238 | 0.029 | 0.039 | 48.3% |
| ATGL | vwap_trend_short_late_vwapslope_lb20 | 267 | 0.107 | 0.143 | 48.3% |
| APOLLOHOSP | vwap_trend_short_late_vwapslope_lb20 | 203 | 0.062 | 0.083 | 48.3% |
| OIL | vwap_trend_short_late_vwapslope_lb20 | 286 | 0.232 | 0.309 | 48.3% |
| DABUR | vwap_trend_short_late_vwapslope_lb20 | 195 | 0.077 | 0.102 | 48.2% |
| AXISBANK | vwap_trend_short_late_vwapslope_lb20 | 194 | -0.046 | -0.061 | 47.9% |
| GAIL | vwap_trend_short_late_vwapslope_lb20 | 226 | 0.078 | 0.104 | 47.8% |
| INFY | vwap_trend_short_late_vwapslope_lb20 | 180 | -0.019 | -0.026 | 47.8% |
| ACC | vwap_trend_short_late_vwapslope_lb20 | 241 | 0.041 | 0.055 | 47.7% |
| NESTLEIND | vwap_trend_short_late_vwapslope_lb20 | 223 | 0.035 | 0.047 | 47.5% |
| MANKIND | vwap_trend_short_late_vwapslope_lb20 | 158 | 0.107 | 0.143 | 47.5% |
| PAGEIND | vwap_trend_short_late_vwapslope_lb20 | 215 | 0.075 | 0.099 | 47.4% |
| JKCEMENT | vwap_trend_short_late_vwapslope_lb20 | 253 | 0.037 | 0.049 | 47.4% |
| ONGC | vwap_trend_short_late_vwapslope_lb20 | 213 | 0.032 | 0.043 | 47.4% |
| KPRMILL | vwap_trend_short_late_vwapslope_lb20 | 308 | 0.080 | 0.107 | 47.4% |
| COALINDIA | vwap_trend_short_late_vwapslope_lb20 | 192 | 0.087 | 0.115 | 47.4% |
| HCLTECH | vwap_trend_short_late_vwapslope_lb20 | 216 | 0.065 | 0.086 | 47.2% |
| LICHSGFIN | vwap_trend_short_late_vwapslope_lb20 | 212 | 0.033 | 0.044 | 47.2% |
| MOTILALOFS | vwap_trend_short_late_vwapslope_lb20 | 280 | 0.080 | 0.107 | 47.1% |
| AUROPHARMA | vwap_trend_short_late_vwapslope_lb20 | 223 | 0.015 | 0.020 | 47.1% |
| NAUKRI | vwap_trend_short_late_vwapslope_lb20 | 228 | 0.095 | 0.126 | 46.9% |
| BHARTIARTL | vwap_trend_short_late_vwapslope_lb20 | 228 | -0.026 | -0.035 | 46.9% |
| APLAPOLLO | vwap_trend_short_late_vwapslope_lb20 | 252 | 0.055 | 0.074 | 46.8% |
| NIACL | vwap_trend_short_late_vwapslope_lb20 | 329 | 0.133 | 0.177 | 46.8% |
| APOLLOTYRE | vwap_trend_short_late_vwapslope_lb20 | 235 | 0.062 | 0.082 | 46.8% |
| GICRE | vwap_trend_short_late_vwapslope_lb20 | 344 | 0.075 | 0.100 | 46.8% |
| DRREDDY | vwap_trend_short_late_vwapslope_lb20 | 203 | 0.024 | 0.032 | 46.8% |
| JSWSTEEL | vwap_trend_short_late_vwapslope_lb20 | 199 | 0.023 | 0.030 | 46.7% |
| JUBLFOOD | vwap_trend_short_late_vwapslope_lb20 | 210 | -0.020 | -0.027 | 46.7% |
| GODREJIND | vwap_trend_short_late_vwapslope_lb20 | 322 | -0.027 | -0.036 | 46.6% |
| ASIANPAINT | vwap_trend_short_late_vwapslope_lb20 | 196 | 0.057 | 0.076 | 46.4% |
| KALYANKJIL | vwap_trend_short_late_vwapslope_lb20 | 308 | 0.030 | 0.040 | 46.4% |
| LTTS | vwap_trend_short_late_vwapslope_lb20 | 237 | 0.080 | 0.107 | 46.4% |
| MARUTI | vwap_trend_short_late_vwapslope_lb20 | 222 | 0.063 | 0.084 | 46.4% |
| INDHOTEL | vwap_trend_short_late_vwapslope_lb20 | 229 | 0.104 | 0.139 | 46.3% |
| OFSS | vwap_trend_short_late_vwapslope_lb20 | 273 | 0.030 | 0.040 | 46.2% |
| NHPC | vwap_trend_short_late_vwapslope_lb20 | 310 | 0.112 | 0.149 | 46.1% |
| BRITANNIA | vwap_trend_short_late_vwapslope_lb20 | 232 | -0.019 | -0.026 | 46.1% |
| BLUESTARCO | vwap_trend_short_late_vwapslope_lb20 | 276 | 0.084 | 0.111 | 46.0% |
| EICHERMOT | vwap_trend_short_late_vwapslope_lb20 | 220 | 0.015 | 0.020 | 45.9% |
| NAM-INDIA | vwap_trend_short_late_vwapslope_lb20 | 264 | 0.068 | 0.091 | 45.8% |
| M&M | vwap_trend_short_late_vwapslope_lb20 | 228 | 0.054 | 0.072 | 45.6% |
| ENDURANCE | vwap_trend_short_late_vwapslope_lb20 | 296 | 0.067 | 0.090 | 45.6% |
| LAURUSLABS | vwap_trend_short_late_vwapslope_lb20 | 223 | 0.071 | 0.095 | 45.3% |
| MAHABANK | vwap_trend_short_late_vwapslope_lb20 | 328 | 0.065 | 0.087 | 45.1% |
| ABBOTINDIA | vwap_trend_short_late_vwapslope_lb20 | 315 | -0.091 | -0.121 | 45.1% |
| BHEL | vwap_trend_short_late_vwapslope_lb20 | 241 | 0.124 | 0.166 | 44.8% |
| IRFC | vwap_trend_short_late_vwapslope_lb20 | 345 | 0.057 | 0.076 | 44.6% |
| JSWENERGY | vwap_trend_short_late_vwapslope_lb20 | 258 | 0.051 | 0.069 | 44.6% |
| KOTAKBANK | vwap_trend_short_late_vwapslope_lb20 | 207 | -0.040 | -0.053 | 44.4% |
| CONCOR | vwap_trend_short_late_vwapslope_lb20 | 214 | 0.050 | 0.066 | 44.4% |
| BAJAJFINSV | vwap_trend_short_late_vwapslope_lb20 | 187 | 0.050 | 0.067 | 44.4% |
| ADANIGREEN | vwap_trend_short_late_vwapslope_lb20 | 264 | 0.089 | 0.119 | 44.3% |
| BAJAJHLDNG | vwap_trend_short_late_vwapslope_lb20 | 260 | 0.066 | 0.088 | 44.2% |
| BANKBARODA | vwap_trend_short_late_vwapslope_lb20 | 231 | -0.016 | -0.021 | 44.2% |
| GODREJCP | vwap_trend_short_late_vwapslope_lb20 | 204 | -0.044 | -0.059 | 44.1% |
| MEDANTA | vwap_trend_short_late_vwapslope_lb20 | 186 | 0.079 | 0.105 | 44.1% |
| ICICIPRULI | vwap_trend_short_late_vwapslope_lb20 | 202 | 0.007 | 0.009 | 44.1% |
