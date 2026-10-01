# exit-tuning/mrm-10.0 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 10.0x risk (7.500%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 165 instruments, 2021-03-23..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_late_vwapslope_lb20 | 39622 | 0.043 | 0.057 | 45.5% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LICI | vwap_trend_short_late_vwapslope_lb20 | 196 | 0.168 | 0.224 | 65.8% |
| AWL | vwap_trend_short_late_vwapslope_lb20 | 222 | 0.179 | 0.239 | 58.1% |
| EXIDEIND | vwap_trend_short_late_vwapslope_lb20 | 254 | 0.125 | 0.167 | 55.9% |
| INDUSINDBK | vwap_trend_short_late_vwapslope_lb20 | 196 | 0.235 | 0.314 | 54.1% |
| MARICO | vwap_trend_short_late_vwapslope_lb20 | 197 | 0.068 | 0.091 | 53.8% |
| ITC | vwap_trend_short_late_vwapslope_lb20 | 194 | 0.101 | 0.135 | 53.6% |
| DIVISLAB | vwap_trend_short_late_vwapslope_lb20 | 208 | 0.069 | 0.092 | 53.4% |
| COCHINSHIP | vwap_trend_short_late_vwapslope_lb20 | 322 | 0.161 | 0.215 | 53.1% |
| LINDEINDIA | vwap_trend_short_late_vwapslope_lb20 | 301 | 0.157 | 0.209 | 52.8% |
| HAVELLS | vwap_trend_short_late_vwapslope_lb20 | 239 | 0.072 | 0.096 | 51.9% |
| HONAUT | vwap_trend_short_late_vwapslope_lb20 | 323 | 0.064 | 0.085 | 51.4% |
| HUDCO | vwap_trend_short_late_vwapslope_lb20 | 312 | 0.083 | 0.111 | 51.3% |
| HEROMOTOCO | vwap_trend_short_late_vwapslope_lb20 | 205 | 0.157 | 0.210 | 51.2% |
| HAL | vwap_trend_short_late_vwapslope_lb20 | 247 | 0.159 | 0.212 | 51.0% |
| 3MINDIA | vwap_trend_short_late_vwapslope_lb20 | 364 | 0.089 | 0.118 | 50.8% |
| HDFCLIFE | vwap_trend_short_late_vwapslope_lb20 | 200 | 0.061 | 0.082 | 50.5% |
| GLAXO | vwap_trend_short_late_vwapslope_lb20 | 321 | 0.103 | 0.137 | 50.5% |
| DLF | vwap_trend_short_late_vwapslope_lb20 | 232 | 0.162 | 0.217 | 50.4% |
| GODFRYPHLP | vwap_trend_short_late_vwapslope_lb20 | 286 | 0.251 | 0.334 | 50.3% |
| BIOCON | vwap_trend_short_late_vwapslope_lb20 | 229 | 0.135 | 0.180 | 50.2% |
| DIXON | vwap_trend_short_late_vwapslope_lb20 | 242 | 0.188 | 0.251 | 50.0% |
| BAJAJ-AUTO | vwap_trend_short_late_vwapslope_lb20 | 222 | 0.078 | 0.104 | 50.0% |
| NLCINDIA | vwap_trend_short_late_vwapslope_lb20 | 301 | 0.161 | 0.215 | 49.8% |
| APARINDS | vwap_trend_short_late_vwapslope_lb20 | 294 | 0.201 | 0.269 | 49.7% |
| ABB | vwap_trend_short_late_vwapslope_lb20 | 218 | 0.100 | 0.133 | 49.5% |
| HDFCBANK | vwap_trend_short_late_vwapslope_lb20 | 208 | 0.058 | 0.078 | 49.5% |
| BDL | vwap_trend_short_late_vwapslope_lb20 | 289 | 0.123 | 0.164 | 49.5% |
| MAZDOCK | vwap_trend_short_late_vwapslope_lb20 | 269 | 0.215 | 0.286 | 49.4% |
| CRISIL | vwap_trend_short_late_vwapslope_lb20 | 304 | 0.105 | 0.141 | 49.3% |
| IRCTC | vwap_trend_short_late_vwapslope_lb20 | 227 | 0.052 | 0.069 | 49.3% |
| BANKINDIA | vwap_trend_short_late_vwapslope_lb20 | 300 | 0.130 | 0.173 | 49.3% |
| CANBK | vwap_trend_short_late_vwapslope_lb20 | 217 | 0.137 | 0.183 | 49.3% |
| LTM | vwap_trend_short_late_vwapslope_lb20 | 207 | 0.102 | 0.136 | 49.3% |
| DMART | vwap_trend_short_late_vwapslope_lb20 | 256 | 0.092 | 0.123 | 49.2% |
| INDIGO | vwap_trend_short_late_vwapslope_lb20 | 223 | 0.043 | 0.058 | 48.9% |
| NYKAA | vwap_trend_short_late_vwapslope_lb20 | 221 | 0.141 | 0.188 | 48.9% |
| KPITTECH | vwap_trend_short_late_vwapslope_lb20 | 260 | 0.127 | 0.169 | 48.8% |
| MPHASIS | vwap_trend_short_late_vwapslope_lb20 | 207 | 0.197 | 0.263 | 48.8% |
| HDFCAMC | vwap_trend_short_late_vwapslope_lb20 | 242 | 0.071 | 0.095 | 48.8% |
| CIPLA | vwap_trend_short_late_vwapslope_lb20 | 199 | -0.026 | -0.035 | 48.7% |
| DABUR | vwap_trend_short_late_vwapslope_lb20 | 196 | 0.111 | 0.148 | 48.5% |
| IOC | vwap_trend_short_late_vwapslope_lb20 | 244 | 0.104 | 0.139 | 48.4% |
| BERGEPAINT | vwap_trend_short_late_vwapslope_lb20 | 238 | 0.029 | 0.039 | 48.3% |
| ATGL | vwap_trend_short_late_vwapslope_lb20 | 267 | 0.107 | 0.143 | 48.3% |
| APOLLOHOSP | vwap_trend_short_late_vwapslope_lb20 | 203 | 0.062 | 0.083 | 48.3% |
| OIL | vwap_trend_short_late_vwapslope_lb20 | 286 | 0.225 | 0.300 | 48.3% |
| LUPIN | vwap_trend_short_late_vwapslope_lb20 | 220 | 0.098 | 0.131 | 48.2% |
| AXISBANK | vwap_trend_short_late_vwapslope_lb20 | 194 | -0.046 | -0.061 | 47.9% |
| INFY | vwap_trend_short_late_vwapslope_lb20 | 182 | -0.005 | -0.007 | 47.8% |
| GAIL | vwap_trend_short_late_vwapslope_lb20 | 226 | 0.078 | 0.104 | 47.8% |
| NESTLEIND | vwap_trend_short_late_vwapslope_lb20 | 222 | 0.025 | 0.033 | 47.7% |
| ACC | vwap_trend_short_late_vwapslope_lb20 | 241 | 0.056 | 0.075 | 47.7% |
| MANKIND | vwap_trend_short_late_vwapslope_lb20 | 160 | 0.145 | 0.194 | 47.5% |
| JKCEMENT | vwap_trend_short_late_vwapslope_lb20 | 253 | 0.037 | 0.049 | 47.4% |
| ONGC | vwap_trend_short_late_vwapslope_lb20 | 213 | 0.032 | 0.043 | 47.4% |
| KPRMILL | vwap_trend_short_late_vwapslope_lb20 | 308 | 0.080 | 0.107 | 47.4% |
| COALINDIA | vwap_trend_short_late_vwapslope_lb20 | 192 | 0.087 | 0.115 | 47.4% |
| PAGEIND | vwap_trend_short_late_vwapslope_lb20 | 216 | 0.049 | 0.066 | 47.2% |
| LICHSGFIN | vwap_trend_short_late_vwapslope_lb20 | 212 | 0.033 | 0.044 | 47.2% |
| NAUKRI | vwap_trend_short_late_vwapslope_lb20 | 229 | 0.126 | 0.168 | 47.2% |
| MOTILALOFS | vwap_trend_short_late_vwapslope_lb20 | 280 | 0.077 | 0.102 | 47.1% |
| AUROPHARMA | vwap_trend_short_late_vwapslope_lb20 | 223 | 0.015 | 0.020 | 47.1% |
| HCLTECH | vwap_trend_short_late_vwapslope_lb20 | 217 | 0.049 | 0.065 | 47.0% |
| JSWSTEEL | vwap_trend_short_late_vwapslope_lb20 | 200 | 0.057 | 0.076 | 47.0% |
| BHARTIARTL | vwap_trend_short_late_vwapslope_lb20 | 228 | -0.026 | -0.035 | 46.9% |
| APLAPOLLO | vwap_trend_short_late_vwapslope_lb20 | 252 | 0.055 | 0.074 | 46.8% |
| NIACL | vwap_trend_short_late_vwapslope_lb20 | 329 | 0.133 | 0.177 | 46.8% |
| APOLLOTYRE | vwap_trend_short_late_vwapslope_lb20 | 235 | 0.062 | 0.082 | 46.8% |
| GICRE | vwap_trend_short_late_vwapslope_lb20 | 344 | 0.075 | 0.100 | 46.8% |
| DRREDDY | vwap_trend_short_late_vwapslope_lb20 | 203 | 0.026 | 0.034 | 46.8% |
| JUBLFOOD | vwap_trend_short_late_vwapslope_lb20 | 210 | -0.020 | -0.027 | 46.7% |
| GODREJIND | vwap_trend_short_late_vwapslope_lb20 | 322 | -0.027 | -0.036 | 46.6% |
| ASIANPAINT | vwap_trend_short_late_vwapslope_lb20 | 196 | 0.035 | 0.046 | 46.4% |
| KALYANKJIL | vwap_trend_short_late_vwapslope_lb20 | 308 | 0.030 | 0.040 | 46.4% |
| LTTS | vwap_trend_short_late_vwapslope_lb20 | 237 | 0.080 | 0.107 | 46.4% |
| BRITANNIA | vwap_trend_short_late_vwapslope_lb20 | 233 | 0.012 | 0.016 | 46.4% |
| INDHOTEL | vwap_trend_short_late_vwapslope_lb20 | 229 | 0.104 | 0.139 | 46.3% |
| OFSS | vwap_trend_short_late_vwapslope_lb20 | 273 | 0.023 | 0.031 | 46.2% |
| NHPC | vwap_trend_short_late_vwapslope_lb20 | 310 | 0.112 | 0.149 | 46.1% |
| BLUESTARCO | vwap_trend_short_late_vwapslope_lb20 | 276 | 0.065 | 0.087 | 46.0% |
| MARUTI | vwap_trend_short_late_vwapslope_lb20 | 224 | 0.038 | 0.051 | 46.0% |
| EICHERMOT | vwap_trend_short_late_vwapslope_lb20 | 220 | 0.015 | 0.020 | 45.9% |
| NAM-INDIA | vwap_trend_short_late_vwapslope_lb20 | 264 | 0.068 | 0.091 | 45.8% |
| M&M | vwap_trend_short_late_vwapslope_lb20 | 228 | 0.038 | 0.050 | 45.6% |
| ENDURANCE | vwap_trend_short_late_vwapslope_lb20 | 296 | 0.067 | 0.090 | 45.6% |
| LAURUSLABS | vwap_trend_short_late_vwapslope_lb20 | 223 | 0.071 | 0.095 | 45.3% |
| MAHABANK | vwap_trend_short_late_vwapslope_lb20 | 328 | 0.065 | 0.087 | 45.1% |
| ABBOTINDIA | vwap_trend_short_late_vwapslope_lb20 | 315 | -0.091 | -0.121 | 45.1% |
| BHEL | vwap_trend_short_late_vwapslope_lb20 | 241 | 0.115 | 0.154 | 44.8% |
| IRFC | vwap_trend_short_late_vwapslope_lb20 | 347 | 0.079 | 0.105 | 44.7% |
| JSWENERGY | vwap_trend_short_late_vwapslope_lb20 | 258 | 0.051 | 0.069 | 44.6% |
| KOTAKBANK | vwap_trend_short_late_vwapslope_lb20 | 207 | -0.040 | -0.053 | 44.4% |
| CONCOR | vwap_trend_short_late_vwapslope_lb20 | 214 | 0.034 | 0.046 | 44.4% |
| BAJAJFINSV | vwap_trend_short_late_vwapslope_lb20 | 187 | 0.029 | 0.039 | 44.4% |
| ADANIGREEN | vwap_trend_short_late_vwapslope_lb20 | 264 | 0.084 | 0.112 | 44.3% |
| BAJAJHLDNG | vwap_trend_short_late_vwapslope_lb20 | 260 | 0.066 | 0.088 | 44.2% |
| GODREJCP | vwap_trend_short_late_vwapslope_lb20 | 206 | -0.012 | -0.016 | 44.2% |
| BANKBARODA | vwap_trend_short_late_vwapslope_lb20 | 231 | -0.016 | -0.021 | 44.2% |
| MEDANTA | vwap_trend_short_late_vwapslope_lb20 | 186 | 0.079 | 0.105 | 44.1% |
| ICICIPRULI | vwap_trend_short_late_vwapslope_lb20 | 202 | 0.007 | 0.009 | 44.1% |
