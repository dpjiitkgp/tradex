# exit-tuning/mrm-6.0 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 6.0x risk (4.500%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 165 instruments, 2021-03-23..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_late_vwapslope_lb20 | 39734 | 0.043 | 0.058 | 45.5% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LICI | vwap_trend_short_late_vwapslope_lb20 | 199 | 0.180 | 0.239 | 65.3% |
| AWL | vwap_trend_short_late_vwapslope_lb20 | 222 | 0.179 | 0.239 | 58.1% |
| EXIDEIND | vwap_trend_short_late_vwapslope_lb20 | 254 | 0.125 | 0.167 | 55.9% |
| INDUSINDBK | vwap_trend_short_late_vwapslope_lb20 | 197 | 0.223 | 0.298 | 54.3% |
| ITC | vwap_trend_short_late_vwapslope_lb20 | 194 | 0.121 | 0.162 | 54.1% |
| MARICO | vwap_trend_short_late_vwapslope_lb20 | 198 | 0.052 | 0.069 | 53.5% |
| DIVISLAB | vwap_trend_short_late_vwapslope_lb20 | 210 | 0.087 | 0.116 | 53.3% |
| LINDEINDIA | vwap_trend_short_late_vwapslope_lb20 | 301 | 0.157 | 0.209 | 52.8% |
| COCHINSHIP | vwap_trend_short_late_vwapslope_lb20 | 324 | 0.137 | 0.183 | 52.8% |
| HAVELLS | vwap_trend_short_late_vwapslope_lb20 | 242 | 0.092 | 0.123 | 51.7% |
| HONAUT | vwap_trend_short_late_vwapslope_lb20 | 323 | 0.071 | 0.094 | 51.4% |
| HUDCO | vwap_trend_short_late_vwapslope_lb20 | 312 | 0.088 | 0.117 | 51.3% |
| HAL | vwap_trend_short_late_vwapslope_lb20 | 247 | 0.153 | 0.204 | 51.0% |
| 3MINDIA | vwap_trend_short_late_vwapslope_lb20 | 364 | 0.085 | 0.113 | 50.8% |
| HDFCLIFE | vwap_trend_short_late_vwapslope_lb20 | 200 | 0.061 | 0.082 | 50.5% |
| HEROMOTOCO | vwap_trend_short_late_vwapslope_lb20 | 208 | 0.108 | 0.145 | 50.5% |
| GLAXO | vwap_trend_short_late_vwapslope_lb20 | 321 | 0.103 | 0.137 | 50.5% |
| DLF | vwap_trend_short_late_vwapslope_lb20 | 232 | 0.162 | 0.216 | 50.4% |
| GODFRYPHLP | vwap_trend_short_late_vwapslope_lb20 | 288 | 0.240 | 0.320 | 50.3% |
| BIOCON | vwap_trend_short_late_vwapslope_lb20 | 229 | 0.128 | 0.171 | 50.2% |
| DIXON | vwap_trend_short_late_vwapslope_lb20 | 243 | 0.212 | 0.282 | 50.2% |
| BAJAJ-AUTO | vwap_trend_short_late_vwapslope_lb20 | 222 | 0.078 | 0.104 | 50.0% |
| NLCINDIA | vwap_trend_short_late_vwapslope_lb20 | 301 | 0.163 | 0.217 | 49.8% |
| APARINDS | vwap_trend_short_late_vwapslope_lb20 | 294 | 0.201 | 0.268 | 49.7% |
| ABB | vwap_trend_short_late_vwapslope_lb20 | 218 | 0.088 | 0.117 | 49.5% |
| BDL | vwap_trend_short_late_vwapslope_lb20 | 289 | 0.114 | 0.152 | 49.5% |
| MAZDOCK | vwap_trend_short_late_vwapslope_lb20 | 269 | 0.185 | 0.247 | 49.4% |
| CRISIL | vwap_trend_short_late_vwapslope_lb20 | 304 | 0.105 | 0.141 | 49.3% |
| IRCTC | vwap_trend_short_late_vwapslope_lb20 | 227 | 0.052 | 0.069 | 49.3% |
| BANKINDIA | vwap_trend_short_late_vwapslope_lb20 | 302 | 0.145 | 0.193 | 49.3% |
| CANBK | vwap_trend_short_late_vwapslope_lb20 | 217 | 0.124 | 0.165 | 49.3% |
| DMART | vwap_trend_short_late_vwapslope_lb20 | 256 | 0.092 | 0.123 | 49.2% |
| HDFCBANK | vwap_trend_short_late_vwapslope_lb20 | 210 | 0.031 | 0.041 | 49.0% |
| INDIGO | vwap_trend_short_late_vwapslope_lb20 | 225 | 0.051 | 0.069 | 48.9% |
| NYKAA | vwap_trend_short_late_vwapslope_lb20 | 221 | 0.126 | 0.168 | 48.9% |
| KPITTECH | vwap_trend_short_late_vwapslope_lb20 | 260 | 0.116 | 0.155 | 48.8% |
| LTM | vwap_trend_short_late_vwapslope_lb20 | 209 | 0.081 | 0.108 | 48.8% |
| OIL | vwap_trend_short_late_vwapslope_lb20 | 287 | 0.255 | 0.340 | 48.8% |
| CIPLA | vwap_trend_short_late_vwapslope_lb20 | 199 | -0.026 | -0.035 | 48.7% |
| HDFCAMC | vwap_trend_short_late_vwapslope_lb20 | 245 | 0.067 | 0.089 | 48.6% |
| MPHASIS | vwap_trend_short_late_vwapslope_lb20 | 208 | 0.175 | 0.233 | 48.6% |
| LUPIN | vwap_trend_short_late_vwapslope_lb20 | 221 | 0.104 | 0.139 | 48.4% |
| IOC | vwap_trend_short_late_vwapslope_lb20 | 246 | 0.116 | 0.155 | 48.4% |
| BERGEPAINT | vwap_trend_short_late_vwapslope_lb20 | 238 | 0.029 | 0.039 | 48.3% |
| ATGL | vwap_trend_short_late_vwapslope_lb20 | 267 | 0.106 | 0.141 | 48.3% |
| APOLLOHOSP | vwap_trend_short_late_vwapslope_lb20 | 203 | 0.062 | 0.083 | 48.3% |
| DABUR | vwap_trend_short_late_vwapslope_lb20 | 197 | 0.095 | 0.127 | 48.2% |
| AXISBANK | vwap_trend_short_late_vwapslope_lb20 | 194 | -0.046 | -0.061 | 47.9% |
| INFY | vwap_trend_short_late_vwapslope_lb20 | 182 | -0.005 | -0.007 | 47.8% |
| GAIL | vwap_trend_short_late_vwapslope_lb20 | 226 | 0.078 | 0.104 | 47.8% |
| ACC | vwap_trend_short_late_vwapslope_lb20 | 241 | 0.049 | 0.065 | 47.7% |
| COALINDIA | vwap_trend_short_late_vwapslope_lb20 | 193 | 0.108 | 0.144 | 47.7% |
| LICHSGFIN | vwap_trend_short_late_vwapslope_lb20 | 212 | 0.057 | 0.076 | 47.6% |
| NESTLEIND | vwap_trend_short_late_vwapslope_lb20 | 225 | 0.023 | 0.031 | 47.6% |
| MANKIND | vwap_trend_short_late_vwapslope_lb20 | 160 | 0.132 | 0.176 | 47.5% |
| ONGC | vwap_trend_short_late_vwapslope_lb20 | 215 | 0.047 | 0.062 | 47.4% |
| JKCEMENT | vwap_trend_short_late_vwapslope_lb20 | 253 | 0.037 | 0.049 | 47.4% |
| KPRMILL | vwap_trend_short_late_vwapslope_lb20 | 308 | 0.080 | 0.107 | 47.4% |
| BHARTIARTL | vwap_trend_short_late_vwapslope_lb20 | 229 | -0.006 | -0.008 | 47.2% |
| MOTILALOFS | vwap_trend_short_late_vwapslope_lb20 | 280 | 0.072 | 0.096 | 47.1% |
| GICRE | vwap_trend_short_late_vwapslope_lb20 | 344 | 0.082 | 0.110 | 47.1% |
| AUROPHARMA | vwap_trend_short_late_vwapslope_lb20 | 223 | 0.014 | 0.018 | 47.1% |
| PAGEIND | vwap_trend_short_late_vwapslope_lb20 | 217 | 0.032 | 0.042 | 47.0% |
| JSWSTEEL | vwap_trend_short_late_vwapslope_lb20 | 200 | 0.048 | 0.063 | 47.0% |
| NAUKRI | vwap_trend_short_late_vwapslope_lb20 | 230 | 0.118 | 0.157 | 47.0% |
| APLAPOLLO | vwap_trend_short_late_vwapslope_lb20 | 252 | 0.060 | 0.080 | 46.8% |
| NIACL | vwap_trend_short_late_vwapslope_lb20 | 329 | 0.121 | 0.162 | 46.8% |
| APOLLOTYRE | vwap_trend_short_late_vwapslope_lb20 | 235 | 0.062 | 0.082 | 46.8% |
| DRREDDY | vwap_trend_short_late_vwapslope_lb20 | 203 | 0.004 | 0.005 | 46.8% |
| JUBLFOOD | vwap_trend_short_late_vwapslope_lb20 | 210 | -0.020 | -0.027 | 46.7% |
| GODREJIND | vwap_trend_short_late_vwapslope_lb20 | 322 | -0.027 | -0.035 | 46.6% |
| HCLTECH | vwap_trend_short_late_vwapslope_lb20 | 219 | 0.015 | 0.020 | 46.6% |
| ASIANPAINT | vwap_trend_short_late_vwapslope_lb20 | 198 | 0.050 | 0.066 | 46.5% |
| KALYANKJIL | vwap_trend_short_late_vwapslope_lb20 | 308 | 0.030 | 0.040 | 46.4% |
| LTTS | vwap_trend_short_late_vwapslope_lb20 | 237 | 0.080 | 0.107 | 46.4% |
| BRITANNIA | vwap_trend_short_late_vwapslope_lb20 | 233 | 0.003 | 0.004 | 46.4% |
| OFSS | vwap_trend_short_late_vwapslope_lb20 | 275 | 0.025 | 0.034 | 46.2% |
| NHPC | vwap_trend_short_late_vwapslope_lb20 | 310 | 0.103 | 0.137 | 46.1% |
| BLUESTARCO | vwap_trend_short_late_vwapslope_lb20 | 276 | 0.065 | 0.087 | 46.0% |
| MARUTI | vwap_trend_short_late_vwapslope_lb20 | 224 | 0.030 | 0.040 | 46.0% |
| EICHERMOT | vwap_trend_short_late_vwapslope_lb20 | 220 | 0.015 | 0.020 | 45.9% |
| NAM-INDIA | vwap_trend_short_late_vwapslope_lb20 | 264 | 0.068 | 0.091 | 45.8% |
| M&M | vwap_trend_short_late_vwapslope_lb20 | 230 | 0.054 | 0.072 | 45.7% |
| ENDURANCE | vwap_trend_short_late_vwapslope_lb20 | 296 | 0.067 | 0.090 | 45.6% |
| ICICIPRULI | vwap_trend_short_late_vwapslope_lb20 | 202 | 0.086 | 0.114 | 45.5% |
| INDHOTEL | vwap_trend_short_late_vwapslope_lb20 | 235 | 0.096 | 0.128 | 45.5% |
| LAURUSLABS | vwap_trend_short_late_vwapslope_lb20 | 223 | 0.076 | 0.101 | 45.3% |
| JSWENERGY | vwap_trend_short_late_vwapslope_lb20 | 259 | 0.129 | 0.172 | 45.2% |
| MAHABANK | vwap_trend_short_late_vwapslope_lb20 | 328 | 0.069 | 0.092 | 45.1% |
| ABBOTINDIA | vwap_trend_short_late_vwapslope_lb20 | 315 | -0.091 | -0.121 | 45.1% |
| BAJAJFINSV | vwap_trend_short_late_vwapslope_lb20 | 189 | 0.063 | 0.084 | 45.0% |
| IRFC | vwap_trend_short_late_vwapslope_lb20 | 348 | 0.085 | 0.114 | 44.8% |
| BHEL | vwap_trend_short_late_vwapslope_lb20 | 241 | 0.100 | 0.134 | 44.8% |
| KOTAKBANK | vwap_trend_short_late_vwapslope_lb20 | 207 | -0.040 | -0.053 | 44.4% |
| ADANIGREEN | vwap_trend_short_late_vwapslope_lb20 | 264 | 0.094 | 0.126 | 44.3% |
| CONCOR | vwap_trend_short_late_vwapslope_lb20 | 217 | 0.026 | 0.035 | 44.2% |
| BANKBARODA | vwap_trend_short_late_vwapslope_lb20 | 231 | -0.016 | -0.021 | 44.2% |
| MEDANTA | vwap_trend_short_late_vwapslope_lb20 | 186 | 0.079 | 0.105 | 44.1% |
| ETERNAL | vwap_trend_short_late_vwapslope_lb20 | 218 | 0.057 | 0.076 | 44.0% |
| BEL | vwap_trend_short_late_vwapslope_lb20 | 207 | 0.008 | 0.011 | 44.0% |
