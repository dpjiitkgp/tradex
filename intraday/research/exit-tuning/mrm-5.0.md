# exit-tuning/mrm-5.0 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 5.0x risk (3.750%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 165 instruments, 2021-03-23..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_late_vwapslope_lb20 | 39766 | 0.042 | 0.056 | 45.5% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LICI | vwap_trend_short_late_vwapslope_lb20 | 199 | 0.174 | 0.232 | 65.3% |
| AWL | vwap_trend_short_late_vwapslope_lb20 | 222 | 0.175 | 0.233 | 58.1% |
| EXIDEIND | vwap_trend_short_late_vwapslope_lb20 | 254 | 0.125 | 0.167 | 55.9% |
| INDUSINDBK | vwap_trend_short_late_vwapslope_lb20 | 197 | 0.219 | 0.292 | 54.3% |
| ITC | vwap_trend_short_late_vwapslope_lb20 | 195 | 0.112 | 0.149 | 53.8% |
| MARICO | vwap_trend_short_late_vwapslope_lb20 | 199 | 0.064 | 0.085 | 53.8% |
| DIVISLAB | vwap_trend_short_late_vwapslope_lb20 | 210 | 0.087 | 0.116 | 53.3% |
| LINDEINDIA | vwap_trend_short_late_vwapslope_lb20 | 301 | 0.164 | 0.218 | 52.8% |
| COCHINSHIP | vwap_trend_short_late_vwapslope_lb20 | 324 | 0.137 | 0.183 | 52.8% |
| HAVELLS | vwap_trend_short_late_vwapslope_lb20 | 242 | 0.092 | 0.123 | 51.7% |
| HONAUT | vwap_trend_short_late_vwapslope_lb20 | 323 | 0.071 | 0.094 | 51.4% |
| HUDCO | vwap_trend_short_late_vwapslope_lb20 | 312 | 0.089 | 0.119 | 51.3% |
| HAL | vwap_trend_short_late_vwapslope_lb20 | 247 | 0.153 | 0.204 | 51.0% |
| 3MINDIA | vwap_trend_short_late_vwapslope_lb20 | 364 | 0.085 | 0.113 | 50.8% |
| HDFCLIFE | vwap_trend_short_late_vwapslope_lb20 | 200 | 0.061 | 0.082 | 50.5% |
| HEROMOTOCO | vwap_trend_short_late_vwapslope_lb20 | 208 | 0.108 | 0.145 | 50.5% |
| GLAXO | vwap_trend_short_late_vwapslope_lb20 | 321 | 0.103 | 0.137 | 50.5% |
| DLF | vwap_trend_short_late_vwapslope_lb20 | 232 | 0.164 | 0.219 | 50.4% |
| GODFRYPHLP | vwap_trend_short_late_vwapslope_lb20 | 288 | 0.222 | 0.297 | 50.3% |
| DIXON | vwap_trend_short_late_vwapslope_lb20 | 243 | 0.214 | 0.285 | 50.2% |
| APARINDS | vwap_trend_short_late_vwapslope_lb20 | 294 | 0.207 | 0.276 | 50.0% |
| BIOCON | vwap_trend_short_late_vwapslope_lb20 | 230 | 0.118 | 0.158 | 50.0% |
| BAJAJ-AUTO | vwap_trend_short_late_vwapslope_lb20 | 222 | 0.078 | 0.104 | 50.0% |
| NLCINDIA | vwap_trend_short_late_vwapslope_lb20 | 301 | 0.163 | 0.217 | 49.8% |
| CANBK | vwap_trend_short_late_vwapslope_lb20 | 218 | 0.127 | 0.169 | 49.5% |
| BANKINDIA | vwap_trend_short_late_vwapslope_lb20 | 303 | 0.140 | 0.186 | 49.5% |
| BDL | vwap_trend_short_late_vwapslope_lb20 | 289 | 0.108 | 0.144 | 49.5% |
| MAZDOCK | vwap_trend_short_late_vwapslope_lb20 | 269 | 0.179 | 0.239 | 49.4% |
| CRISIL | vwap_trend_short_late_vwapslope_lb20 | 304 | 0.105 | 0.141 | 49.3% |
| IRCTC | vwap_trend_short_late_vwapslope_lb20 | 227 | 0.052 | 0.069 | 49.3% |
| NYKAA | vwap_trend_short_late_vwapslope_lb20 | 221 | 0.153 | 0.203 | 49.3% |
| DMART | vwap_trend_short_late_vwapslope_lb20 | 256 | 0.092 | 0.123 | 49.2% |
| ABB | vwap_trend_short_late_vwapslope_lb20 | 220 | 0.065 | 0.086 | 49.1% |
| OIL | vwap_trend_short_late_vwapslope_lb20 | 288 | 0.254 | 0.339 | 49.0% |
| INDIGO | vwap_trend_short_late_vwapslope_lb20 | 225 | 0.043 | 0.057 | 48.9% |
| KPITTECH | vwap_trend_short_late_vwapslope_lb20 | 260 | 0.113 | 0.150 | 48.8% |
| HDFCBANK | vwap_trend_short_late_vwapslope_lb20 | 211 | 0.022 | 0.030 | 48.8% |
| LTM | vwap_trend_short_late_vwapslope_lb20 | 209 | 0.074 | 0.099 | 48.8% |
| CIPLA | vwap_trend_short_late_vwapslope_lb20 | 199 | -0.026 | -0.035 | 48.7% |
| HDFCAMC | vwap_trend_short_late_vwapslope_lb20 | 245 | 0.067 | 0.089 | 48.6% |
| MPHASIS | vwap_trend_short_late_vwapslope_lb20 | 208 | 0.180 | 0.240 | 48.6% |
| LUPIN | vwap_trend_short_late_vwapslope_lb20 | 221 | 0.096 | 0.128 | 48.4% |
| IOC | vwap_trend_short_late_vwapslope_lb20 | 246 | 0.110 | 0.146 | 48.4% |
| BERGEPAINT | vwap_trend_short_late_vwapslope_lb20 | 238 | 0.029 | 0.039 | 48.3% |
| ATGL | vwap_trend_short_late_vwapslope_lb20 | 267 | 0.098 | 0.130 | 48.3% |
| APOLLOHOSP | vwap_trend_short_late_vwapslope_lb20 | 203 | 0.066 | 0.087 | 48.3% |
| INFY | vwap_trend_short_late_vwapslope_lb20 | 183 | -0.007 | -0.009 | 48.1% |
| DABUR | vwap_trend_short_late_vwapslope_lb20 | 198 | 0.086 | 0.115 | 48.0% |
| AXISBANK | vwap_trend_short_late_vwapslope_lb20 | 194 | -0.046 | -0.061 | 47.9% |
| MANKIND | vwap_trend_short_late_vwapslope_lb20 | 161 | 0.138 | 0.183 | 47.8% |
| GAIL | vwap_trend_short_late_vwapslope_lb20 | 226 | 0.078 | 0.104 | 47.8% |
| ACC | vwap_trend_short_late_vwapslope_lb20 | 241 | 0.046 | 0.061 | 47.7% |
| COALINDIA | vwap_trend_short_late_vwapslope_lb20 | 193 | 0.092 | 0.123 | 47.7% |
| LICHSGFIN | vwap_trend_short_late_vwapslope_lb20 | 212 | 0.054 | 0.072 | 47.6% |
| NESTLEIND | vwap_trend_short_late_vwapslope_lb20 | 225 | 0.017 | 0.023 | 47.6% |
| ONGC | vwap_trend_short_late_vwapslope_lb20 | 215 | 0.047 | 0.062 | 47.4% |
| JKCEMENT | vwap_trend_short_late_vwapslope_lb20 | 253 | 0.042 | 0.056 | 47.4% |
| KPRMILL | vwap_trend_short_late_vwapslope_lb20 | 308 | 0.081 | 0.108 | 47.4% |
| AUROPHARMA | vwap_trend_short_late_vwapslope_lb20 | 224 | 0.025 | 0.033 | 47.3% |
| JSWSTEEL | vwap_trend_short_late_vwapslope_lb20 | 201 | 0.056 | 0.074 | 47.3% |
| BHARTIARTL | vwap_trend_short_late_vwapslope_lb20 | 229 | -0.011 | -0.014 | 47.2% |
| MOTILALOFS | vwap_trend_short_late_vwapslope_lb20 | 280 | 0.066 | 0.087 | 47.1% |
| GICRE | vwap_trend_short_late_vwapslope_lb20 | 344 | 0.078 | 0.104 | 47.1% |
| PAGEIND | vwap_trend_short_late_vwapslope_lb20 | 217 | 0.032 | 0.042 | 47.0% |
| NAUKRI | vwap_trend_short_late_vwapslope_lb20 | 230 | 0.123 | 0.164 | 47.0% |
| APLAPOLLO | vwap_trend_short_late_vwapslope_lb20 | 252 | 0.062 | 0.082 | 46.8% |
| HCLTECH | vwap_trend_short_late_vwapslope_lb20 | 220 | 0.029 | 0.039 | 46.8% |
| NIACL | vwap_trend_short_late_vwapslope_lb20 | 329 | 0.116 | 0.155 | 46.8% |
| APOLLOTYRE | vwap_trend_short_late_vwapslope_lb20 | 235 | 0.062 | 0.082 | 46.8% |
| DRREDDY | vwap_trend_short_late_vwapslope_lb20 | 203 | 0.004 | 0.005 | 46.8% |
| ASIANPAINT | vwap_trend_short_late_vwapslope_lb20 | 199 | 0.070 | 0.094 | 46.7% |
| JUBLFOOD | vwap_trend_short_late_vwapslope_lb20 | 210 | -0.020 | -0.027 | 46.7% |
| GODREJIND | vwap_trend_short_late_vwapslope_lb20 | 322 | -0.027 | -0.035 | 46.6% |
| OFSS | vwap_trend_short_late_vwapslope_lb20 | 275 | 0.036 | 0.048 | 46.5% |
| KALYANKJIL | vwap_trend_short_late_vwapslope_lb20 | 308 | 0.030 | 0.040 | 46.4% |
| LTTS | vwap_trend_short_late_vwapslope_lb20 | 237 | 0.089 | 0.118 | 46.4% |
| BRITANNIA | vwap_trend_short_late_vwapslope_lb20 | 233 | 0.003 | 0.004 | 46.4% |
| BLUESTARCO | vwap_trend_short_late_vwapslope_lb20 | 276 | 0.059 | 0.078 | 46.0% |
| MARUTI | vwap_trend_short_late_vwapslope_lb20 | 224 | 0.030 | 0.040 | 46.0% |
| EICHERMOT | vwap_trend_short_late_vwapslope_lb20 | 220 | 0.015 | 0.020 | 45.9% |
| NHPC | vwap_trend_short_late_vwapslope_lb20 | 314 | 0.108 | 0.144 | 45.9% |
| NAM-INDIA | vwap_trend_short_late_vwapslope_lb20 | 264 | 0.066 | 0.088 | 45.8% |
| M&M | vwap_trend_short_late_vwapslope_lb20 | 230 | 0.054 | 0.072 | 45.7% |
| ENDURANCE | vwap_trend_short_late_vwapslope_lb20 | 296 | 0.068 | 0.091 | 45.6% |
| ICICIPRULI | vwap_trend_short_late_vwapslope_lb20 | 202 | 0.082 | 0.109 | 45.5% |
| LAURUSLABS | vwap_trend_short_late_vwapslope_lb20 | 224 | 0.083 | 0.111 | 45.5% |
| INDHOTEL | vwap_trend_short_late_vwapslope_lb20 | 235 | 0.099 | 0.132 | 45.5% |
| MAHABANK | vwap_trend_short_late_vwapslope_lb20 | 328 | 0.071 | 0.095 | 45.4% |
| JSWENERGY | vwap_trend_short_late_vwapslope_lb20 | 259 | 0.140 | 0.186 | 45.2% |
| ABBOTINDIA | vwap_trend_short_late_vwapslope_lb20 | 315 | -0.091 | -0.121 | 45.1% |
| BHEL | vwap_trend_short_late_vwapslope_lb20 | 242 | 0.110 | 0.146 | 45.0% |
| BAJAJFINSV | vwap_trend_short_late_vwapslope_lb20 | 189 | 0.060 | 0.081 | 45.0% |
| IRFC | vwap_trend_short_late_vwapslope_lb20 | 348 | 0.080 | 0.106 | 44.8% |
| ADANIGREEN | vwap_trend_short_late_vwapslope_lb20 | 264 | 0.107 | 0.143 | 44.7% |
| ETERNAL | vwap_trend_short_late_vwapslope_lb20 | 218 | 0.064 | 0.085 | 44.5% |
| KOTAKBANK | vwap_trend_short_late_vwapslope_lb20 | 207 | -0.040 | -0.053 | 44.4% |
| BANKBARODA | vwap_trend_short_late_vwapslope_lb20 | 231 | -0.019 | -0.025 | 44.2% |
| MEDANTA | vwap_trend_short_late_vwapslope_lb20 | 186 | 0.079 | 0.105 | 44.1% |
| CONCOR | vwap_trend_short_late_vwapslope_lb20 | 218 | 0.017 | 0.023 | 44.0% |
| BEL | vwap_trend_short_late_vwapslope_lb20 | 207 | 0.008 | 0.011 | 44.0% |
