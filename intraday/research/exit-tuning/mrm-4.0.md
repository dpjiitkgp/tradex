# exit-tuning/mrm-4.0 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 4.0x risk (3.000%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 165 instruments, 2021-03-23..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_late_vwapslope_lb20 | 39805 | 0.041 | 0.055 | 45.6% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LICI | vwap_trend_short_late_vwapslope_lb20 | 199 | 0.171 | 0.228 | 65.3% |
| AWL | vwap_trend_short_late_vwapslope_lb20 | 222 | 0.168 | 0.225 | 58.1% |
| EXIDEIND | vwap_trend_short_late_vwapslope_lb20 | 254 | 0.124 | 0.165 | 55.9% |
| INDUSINDBK | vwap_trend_short_late_vwapslope_lb20 | 197 | 0.206 | 0.275 | 54.3% |
| ITC | vwap_trend_short_late_vwapslope_lb20 | 195 | 0.105 | 0.140 | 53.8% |
| MARICO | vwap_trend_short_late_vwapslope_lb20 | 199 | 0.064 | 0.085 | 53.8% |
| DIVISLAB | vwap_trend_short_late_vwapslope_lb20 | 210 | 0.087 | 0.116 | 53.3% |
| COCHINSHIP | vwap_trend_short_late_vwapslope_lb20 | 324 | 0.147 | 0.196 | 53.1% |
| LINDEINDIA | vwap_trend_short_late_vwapslope_lb20 | 301 | 0.166 | 0.221 | 52.8% |
| HAVELLS | vwap_trend_short_late_vwapslope_lb20 | 243 | 0.068 | 0.090 | 51.4% |
| HAL | vwap_trend_short_late_vwapslope_lb20 | 247 | 0.171 | 0.228 | 51.4% |
| HONAUT | vwap_trend_short_late_vwapslope_lb20 | 323 | 0.071 | 0.094 | 51.4% |
| HUDCO | vwap_trend_short_late_vwapslope_lb20 | 312 | 0.092 | 0.123 | 51.3% |
| 3MINDIA | vwap_trend_short_late_vwapslope_lb20 | 364 | 0.084 | 0.112 | 50.8% |
| HDFCLIFE | vwap_trend_short_late_vwapslope_lb20 | 200 | 0.061 | 0.082 | 50.5% |
| GLAXO | vwap_trend_short_late_vwapslope_lb20 | 321 | 0.102 | 0.135 | 50.5% |
| BIOCON | vwap_trend_short_late_vwapslope_lb20 | 230 | 0.123 | 0.164 | 50.4% |
| DLF | vwap_trend_short_late_vwapslope_lb20 | 232 | 0.164 | 0.219 | 50.4% |
| DIXON | vwap_trend_short_late_vwapslope_lb20 | 244 | 0.211 | 0.282 | 50.4% |
| GODFRYPHLP | vwap_trend_short_late_vwapslope_lb20 | 288 | 0.202 | 0.270 | 50.3% |
| APARINDS | vwap_trend_short_late_vwapslope_lb20 | 294 | 0.209 | 0.279 | 50.0% |
| CANBK | vwap_trend_short_late_vwapslope_lb20 | 220 | 0.149 | 0.199 | 50.0% |
| HEROMOTOCO | vwap_trend_short_late_vwapslope_lb20 | 210 | 0.093 | 0.124 | 50.0% |
| BAJAJ-AUTO | vwap_trend_short_late_vwapslope_lb20 | 222 | 0.078 | 0.104 | 50.0% |
| NLCINDIA | vwap_trend_short_late_vwapslope_lb20 | 301 | 0.158 | 0.210 | 49.8% |
| BDL | vwap_trend_short_late_vwapslope_lb20 | 289 | 0.121 | 0.162 | 49.8% |
| CRISIL | vwap_trend_short_late_vwapslope_lb20 | 304 | 0.106 | 0.142 | 49.7% |
| MAZDOCK | vwap_trend_short_late_vwapslope_lb20 | 269 | 0.166 | 0.221 | 49.4% |
| BANKINDIA | vwap_trend_short_late_vwapslope_lb20 | 304 | 0.123 | 0.164 | 49.3% |
| IRCTC | vwap_trend_short_late_vwapslope_lb20 | 227 | 0.052 | 0.069 | 49.3% |
| INDIGO | vwap_trend_short_late_vwapslope_lb20 | 227 | 0.049 | 0.066 | 49.3% |
| NYKAA | vwap_trend_short_late_vwapslope_lb20 | 221 | 0.163 | 0.217 | 49.3% |
| KPITTECH | vwap_trend_short_late_vwapslope_lb20 | 260 | 0.126 | 0.168 | 49.2% |
| ABB | vwap_trend_short_late_vwapslope_lb20 | 220 | 0.060 | 0.081 | 49.1% |
| DMART | vwap_trend_short_late_vwapslope_lb20 | 259 | 0.099 | 0.132 | 49.0% |
| HDFCBANK | vwap_trend_short_late_vwapslope_lb20 | 211 | 0.018 | 0.024 | 48.8% |
| LTM | vwap_trend_short_late_vwapslope_lb20 | 209 | 0.086 | 0.114 | 48.8% |
| OIL | vwap_trend_short_late_vwapslope_lb20 | 289 | 0.241 | 0.321 | 48.8% |
| CIPLA | vwap_trend_short_late_vwapslope_lb20 | 199 | -0.026 | -0.035 | 48.7% |
| LUPIN | vwap_trend_short_late_vwapslope_lb20 | 222 | 0.099 | 0.132 | 48.6% |
| HDFCAMC | vwap_trend_short_late_vwapslope_lb20 | 245 | 0.053 | 0.070 | 48.6% |
| MPHASIS | vwap_trend_short_late_vwapslope_lb20 | 208 | 0.172 | 0.230 | 48.6% |
| IOC | vwap_trend_short_late_vwapslope_lb20 | 246 | 0.110 | 0.147 | 48.4% |
| BERGEPAINT | vwap_trend_short_late_vwapslope_lb20 | 238 | 0.030 | 0.039 | 48.3% |
| ATGL | vwap_trend_short_late_vwapslope_lb20 | 267 | 0.092 | 0.122 | 48.3% |
| APOLLOHOSP | vwap_trend_short_late_vwapslope_lb20 | 203 | 0.066 | 0.087 | 48.3% |
| LICHSGFIN | vwap_trend_short_late_vwapslope_lb20 | 212 | 0.062 | 0.083 | 48.1% |
| INFY | vwap_trend_short_late_vwapslope_lb20 | 183 | -0.007 | -0.009 | 48.1% |
| DABUR | vwap_trend_short_late_vwapslope_lb20 | 198 | 0.086 | 0.115 | 48.0% |
| AXISBANK | vwap_trend_short_late_vwapslope_lb20 | 194 | -0.046 | -0.061 | 47.9% |
| MANKIND | vwap_trend_short_late_vwapslope_lb20 | 161 | 0.134 | 0.179 | 47.8% |
| GAIL | vwap_trend_short_late_vwapslope_lb20 | 226 | 0.073 | 0.097 | 47.8% |
| ACC | vwap_trend_short_late_vwapslope_lb20 | 241 | 0.039 | 0.052 | 47.7% |
| COALINDIA | vwap_trend_short_late_vwapslope_lb20 | 193 | 0.084 | 0.112 | 47.7% |
| BHARTIARTL | vwap_trend_short_late_vwapslope_lb20 | 229 | -0.000 | -0.000 | 47.6% |
| NESTLEIND | vwap_trend_short_late_vwapslope_lb20 | 227 | 0.005 | 0.006 | 47.6% |
| AUROPHARMA | vwap_trend_short_late_vwapslope_lb20 | 225 | 0.033 | 0.044 | 47.6% |
| JKCEMENT | vwap_trend_short_late_vwapslope_lb20 | 253 | 0.049 | 0.065 | 47.4% |
| KPRMILL | vwap_trend_short_late_vwapslope_lb20 | 308 | 0.069 | 0.092 | 47.4% |
| JSWSTEEL | vwap_trend_short_late_vwapslope_lb20 | 201 | 0.057 | 0.076 | 47.3% |
| MOTILALOFS | vwap_trend_short_late_vwapslope_lb20 | 280 | 0.070 | 0.093 | 47.1% |
| GICRE | vwap_trend_short_late_vwapslope_lb20 | 344 | 0.074 | 0.099 | 47.1% |
| KALYANKJIL | vwap_trend_short_late_vwapslope_lb20 | 308 | 0.062 | 0.083 | 47.1% |
| PAGEIND | vwap_trend_short_late_vwapslope_lb20 | 217 | 0.030 | 0.040 | 47.0% |
| ONGC | vwap_trend_short_late_vwapslope_lb20 | 217 | 0.028 | 0.037 | 47.0% |
| ASIANPAINT | vwap_trend_short_late_vwapslope_lb20 | 200 | 0.054 | 0.072 | 47.0% |
| NAUKRI | vwap_trend_short_late_vwapslope_lb20 | 230 | 0.109 | 0.146 | 47.0% |
| BLUESTARCO | vwap_trend_short_late_vwapslope_lb20 | 277 | 0.092 | 0.123 | 46.9% |
| APLAPOLLO | vwap_trend_short_late_vwapslope_lb20 | 252 | 0.061 | 0.081 | 46.8% |
| HCLTECH | vwap_trend_short_late_vwapslope_lb20 | 220 | 0.029 | 0.039 | 46.8% |
| NIACL | vwap_trend_short_late_vwapslope_lb20 | 329 | 0.114 | 0.152 | 46.8% |
| APOLLOTYRE | vwap_trend_short_late_vwapslope_lb20 | 235 | 0.060 | 0.080 | 46.8% |
| DRREDDY | vwap_trend_short_late_vwapslope_lb20 | 203 | 0.003 | 0.005 | 46.8% |
| JUBLFOOD | vwap_trend_short_late_vwapslope_lb20 | 210 | -0.019 | -0.025 | 46.7% |
| GODREJIND | vwap_trend_short_late_vwapslope_lb20 | 322 | -0.027 | -0.037 | 46.6% |
| OFSS | vwap_trend_short_late_vwapslope_lb20 | 275 | 0.033 | 0.044 | 46.5% |
| LTTS | vwap_trend_short_late_vwapslope_lb20 | 237 | 0.069 | 0.092 | 46.4% |
| BRITANNIA | vwap_trend_short_late_vwapslope_lb20 | 233 | 0.003 | 0.004 | 46.4% |
| M&M | vwap_trend_short_late_vwapslope_lb20 | 231 | 0.071 | 0.095 | 46.3% |
| NHPC | vwap_trend_short_late_vwapslope_lb20 | 314 | 0.116 | 0.155 | 46.2% |
| MARUTI | vwap_trend_short_late_vwapslope_lb20 | 224 | 0.030 | 0.040 | 46.0% |
| EICHERMOT | vwap_trend_short_late_vwapslope_lb20 | 220 | 0.015 | 0.020 | 45.9% |
| NAM-INDIA | vwap_trend_short_late_vwapslope_lb20 | 264 | 0.061 | 0.081 | 45.8% |
| ENDURANCE | vwap_trend_short_late_vwapslope_lb20 | 296 | 0.068 | 0.091 | 45.6% |
| ICICIPRULI | vwap_trend_short_late_vwapslope_lb20 | 202 | 0.082 | 0.109 | 45.5% |
| LAURUSLABS | vwap_trend_short_late_vwapslope_lb20 | 224 | 0.087 | 0.116 | 45.5% |
| INDHOTEL | vwap_trend_short_late_vwapslope_lb20 | 235 | 0.092 | 0.123 | 45.5% |
| BHEL | vwap_trend_short_late_vwapslope_lb20 | 242 | 0.119 | 0.159 | 45.5% |
| MAHABANK | vwap_trend_short_late_vwapslope_lb20 | 328 | 0.060 | 0.080 | 45.4% |
| JSWENERGY | vwap_trend_short_late_vwapslope_lb20 | 259 | 0.114 | 0.152 | 45.2% |
| ABBOTINDIA | vwap_trend_short_late_vwapslope_lb20 | 315 | -0.091 | -0.121 | 45.1% |
| IRFC | vwap_trend_short_late_vwapslope_lb20 | 348 | 0.067 | 0.089 | 44.8% |
| BAJAJFINSV | vwap_trend_short_late_vwapslope_lb20 | 190 | 0.042 | 0.056 | 44.7% |
| KOTAKBANK | vwap_trend_short_late_vwapslope_lb20 | 208 | -0.027 | -0.036 | 44.7% |
| ADANIGREEN | vwap_trend_short_late_vwapslope_lb20 | 264 | 0.096 | 0.128 | 44.7% |
| ETERNAL | vwap_trend_short_late_vwapslope_lb20 | 218 | 0.040 | 0.053 | 44.5% |
| BANKBARODA | vwap_trend_short_late_vwapslope_lb20 | 231 | -0.010 | -0.014 | 44.2% |
| MEDANTA | vwap_trend_short_late_vwapslope_lb20 | 186 | 0.079 | 0.105 | 44.1% |
| COROMANDEL | vwap_trend_short_late_vwapslope_lb20 | 236 | 0.009 | 0.011 | 44.1% |
| BEL | vwap_trend_short_late_vwapslope_lb20 | 207 | 0.008 | 0.011 | 44.0% |
