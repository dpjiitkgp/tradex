# exit-tuning/mrm-1.0 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 1.0x risk (0.750%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 165 instruments, 2021-03-23..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_late_vwapslope_lb20 | 39980 | 0.028 | 0.037 | 49.6% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LICI | vwap_trend_short_late_vwapslope_lb20 | 200 | 0.176 | 0.235 | 66.5% |
| AWL | vwap_trend_short_late_vwapslope_lb20 | 222 | 0.106 | 0.141 | 59.0% |
| NLCINDIA | vwap_trend_short_late_vwapslope_lb20 | 301 | 0.205 | 0.273 | 58.5% |
| DIXON | vwap_trend_short_late_vwapslope_lb20 | 245 | 0.182 | 0.242 | 58.4% |
| EXIDEIND | vwap_trend_short_late_vwapslope_lb20 | 255 | 0.120 | 0.160 | 57.3% |
| HUDCO | vwap_trend_short_late_vwapslope_lb20 | 312 | 0.128 | 0.170 | 57.1% |
| HAL | vwap_trend_short_late_vwapslope_lb20 | 247 | 0.179 | 0.239 | 56.7% |
| INDUSINDBK | vwap_trend_short_late_vwapslope_lb20 | 201 | 0.137 | 0.183 | 56.2% |
| MPHASIS | vwap_trend_short_late_vwapslope_lb20 | 209 | 0.180 | 0.241 | 56.0% |
| MAZDOCK | vwap_trend_short_late_vwapslope_lb20 | 270 | 0.112 | 0.150 | 55.9% |
| HAVELLS | vwap_trend_short_late_vwapslope_lb20 | 245 | 0.064 | 0.085 | 55.9% |
| ITC | vwap_trend_short_late_vwapslope_lb20 | 197 | 0.109 | 0.146 | 55.8% |
| LINDEINDIA | vwap_trend_short_late_vwapslope_lb20 | 301 | 0.153 | 0.204 | 55.5% |
| BDL | vwap_trend_short_late_vwapslope_lb20 | 289 | 0.127 | 0.170 | 55.4% |
| DIVISLAB | vwap_trend_short_late_vwapslope_lb20 | 210 | 0.056 | 0.075 | 55.2% |
| DLF | vwap_trend_short_late_vwapslope_lb20 | 232 | 0.128 | 0.171 | 55.2% |
| BIOCON | vwap_trend_short_late_vwapslope_lb20 | 232 | 0.093 | 0.124 | 55.2% |
| APARINDS | vwap_trend_short_late_vwapslope_lb20 | 294 | 0.155 | 0.207 | 55.1% |
| COCHINSHIP | vwap_trend_short_late_vwapslope_lb20 | 325 | 0.100 | 0.134 | 55.1% |
| 3MINDIA | vwap_trend_short_late_vwapslope_lb20 | 364 | 0.057 | 0.076 | 54.7% |
| GLAXO | vwap_trend_short_late_vwapslope_lb20 | 321 | 0.075 | 0.100 | 54.5% |
| ABB | vwap_trend_short_late_vwapslope_lb20 | 222 | 0.039 | 0.052 | 54.5% |
| OIL | vwap_trend_short_late_vwapslope_lb20 | 290 | 0.225 | 0.300 | 54.5% |
| MARICO | vwap_trend_short_late_vwapslope_lb20 | 201 | 0.049 | 0.065 | 54.2% |
| HONAUT | vwap_trend_short_late_vwapslope_lb20 | 324 | 0.065 | 0.087 | 54.0% |
| HDFCLIFE | vwap_trend_short_late_vwapslope_lb20 | 200 | 0.057 | 0.076 | 54.0% |
| ATGL | vwap_trend_short_late_vwapslope_lb20 | 267 | 0.063 | 0.084 | 53.9% |
| NYKAA | vwap_trend_short_late_vwapslope_lb20 | 221 | 0.130 | 0.173 | 53.8% |
| HEROMOTOCO | vwap_trend_short_late_vwapslope_lb20 | 211 | 0.087 | 0.115 | 53.6% |
| CIPLA | vwap_trend_short_late_vwapslope_lb20 | 198 | 0.008 | 0.010 | 53.5% |
| DMART | vwap_trend_short_late_vwapslope_lb20 | 260 | 0.104 | 0.138 | 53.5% |
| BAJAJ-AUTO | vwap_trend_short_late_vwapslope_lb20 | 227 | 0.081 | 0.108 | 53.3% |
| COALINDIA | vwap_trend_short_late_vwapslope_lb20 | 197 | 0.071 | 0.095 | 53.3% |
| ETERNAL | vwap_trend_short_late_vwapslope_lb20 | 218 | 0.084 | 0.112 | 53.2% |
| CANBK | vwap_trend_short_late_vwapslope_lb20 | 222 | 0.096 | 0.128 | 53.2% |
| OFSS | vwap_trend_short_late_vwapslope_lb20 | 278 | 0.045 | 0.061 | 52.9% |
| LTM | vwap_trend_short_late_vwapslope_lb20 | 210 | 0.076 | 0.102 | 52.9% |
| GODREJIND | vwap_trend_short_late_vwapslope_lb20 | 322 | -0.009 | -0.012 | 52.8% |
| MANKIND | vwap_trend_short_late_vwapslope_lb20 | 163 | 0.096 | 0.128 | 52.8% |
| IPCALAB | vwap_trend_short_late_vwapslope_lb20 | 241 | 0.067 | 0.089 | 52.7% |
| BHEL | vwap_trend_short_late_vwapslope_lb20 | 242 | 0.052 | 0.069 | 52.5% |
| ADANIGREEN | vwap_trend_short_late_vwapslope_lb20 | 265 | 0.117 | 0.156 | 52.5% |
| HDFCAMC | vwap_trend_short_late_vwapslope_lb20 | 246 | 0.042 | 0.057 | 52.4% |
| AUROPHARMA | vwap_trend_short_late_vwapslope_lb20 | 227 | 0.032 | 0.042 | 52.4% |
| INDIGO | vwap_trend_short_late_vwapslope_lb20 | 229 | 0.014 | 0.018 | 52.4% |
| LICHSGFIN | vwap_trend_short_late_vwapslope_lb20 | 213 | 0.016 | 0.022 | 52.1% |
| PAGEIND | vwap_trend_short_late_vwapslope_lb20 | 217 | 0.024 | 0.033 | 52.1% |
| APLAPOLLO | vwap_trend_short_late_vwapslope_lb20 | 254 | 0.057 | 0.076 | 52.0% |
| APOLLOHOSP | vwap_trend_short_late_vwapslope_lb20 | 206 | 0.079 | 0.105 | 51.9% |
| LUPIN | vwap_trend_short_late_vwapslope_lb20 | 224 | 0.078 | 0.104 | 51.8% |
| LAURUSLABS | vwap_trend_short_late_vwapslope_lb20 | 224 | 0.041 | 0.054 | 51.8% |
| JKCEMENT | vwap_trend_short_late_vwapslope_lb20 | 253 | 0.014 | 0.019 | 51.8% |
| KPITTECH | vwap_trend_short_late_vwapslope_lb20 | 261 | 0.064 | 0.085 | 51.7% |
| GODREJPROP | vwap_trend_short_late_vwapslope_lb20 | 207 | 0.071 | 0.095 | 51.7% |
| APOLLOTYRE | vwap_trend_short_late_vwapslope_lb20 | 235 | 0.070 | 0.094 | 51.5% |
| BLUESTARCO | vwap_trend_short_late_vwapslope_lb20 | 278 | 0.033 | 0.044 | 51.4% |
| HDFCBANK | vwap_trend_short_late_vwapslope_lb20 | 212 | 0.028 | 0.037 | 51.4% |
| EICHERMOT | vwap_trend_short_late_vwapslope_lb20 | 220 | 0.015 | 0.020 | 51.4% |
| NAM-INDIA | vwap_trend_short_late_vwapslope_lb20 | 265 | 0.053 | 0.071 | 51.3% |
| JSWSTEEL | vwap_trend_short_late_vwapslope_lb20 | 203 | 0.048 | 0.064 | 51.2% |
| GICRE | vwap_trend_short_late_vwapslope_lb20 | 344 | 0.047 | 0.063 | 51.2% |
| GAIL | vwap_trend_short_late_vwapslope_lb20 | 225 | 0.073 | 0.098 | 51.1% |
| LTTS | vwap_trend_short_late_vwapslope_lb20 | 237 | 0.061 | 0.081 | 51.1% |
| CRISIL | vwap_trend_short_late_vwapslope_lb20 | 304 | 0.085 | 0.114 | 51.0% |
| JUBLFOOD | vwap_trend_short_late_vwapslope_lb20 | 210 | 0.009 | 0.011 | 51.0% |
| NTPC | vwap_trend_short_late_vwapslope_lb20 | 230 | 0.035 | 0.046 | 50.9% |
| INDHOTEL | vwap_trend_short_late_vwapslope_lb20 | 236 | 0.091 | 0.121 | 50.8% |
| BERGEPAINT | vwap_trend_short_late_vwapslope_lb20 | 238 | -0.003 | -0.004 | 50.8% |
| BEL | vwap_trend_short_late_vwapslope_lb20 | 207 | 0.028 | 0.038 | 50.7% |
| HCLTECH | vwap_trend_short_late_vwapslope_lb20 | 223 | 0.030 | 0.040 | 50.7% |
| IRCTC | vwap_trend_short_late_vwapslope_lb20 | 227 | 0.039 | 0.052 | 50.7% |
| BANKINDIA | vwap_trend_short_late_vwapslope_lb20 | 306 | 0.072 | 0.096 | 50.7% |
| JSWENERGY | vwap_trend_short_late_vwapslope_lb20 | 260 | 0.044 | 0.059 | 50.4% |
| KPRMILL | vwap_trend_short_late_vwapslope_lb20 | 308 | 0.054 | 0.072 | 50.3% |
| ICICIPRULI | vwap_trend_short_late_vwapslope_lb20 | 203 | 0.045 | 0.059 | 50.2% |
| BANKBARODA | vwap_trend_short_late_vwapslope_lb20 | 231 | -0.011 | -0.015 | 50.2% |
| KEI | vwap_trend_short_late_vwapslope_lb20 | 247 | 0.045 | 0.060 | 50.2% |
| IOC | vwap_trend_short_late_vwapslope_lb20 | 248 | 0.109 | 0.145 | 50.0% |
| KALYANKJIL | vwap_trend_short_late_vwapslope_lb20 | 310 | 0.074 | 0.099 | 50.0% |
| INDIANB | vwap_trend_short_late_vwapslope_lb20 | 280 | -0.016 | -0.021 | 50.0% |
| BHARTIARTL | vwap_trend_short_late_vwapslope_lb20 | 230 | -0.019 | -0.025 | 50.0% |
| JSL | vwap_trend_short_late_vwapslope_lb20 | 241 | 0.051 | 0.068 | 49.8% |
| ONGC | vwap_trend_short_late_vwapslope_lb20 | 219 | 0.023 | 0.031 | 49.8% |
| NIACL | vwap_trend_short_late_vwapslope_lb20 | 329 | 0.076 | 0.101 | 49.5% |
| AIAENG | vwap_trend_short_late_vwapslope_lb20 | 281 | -0.009 | -0.011 | 49.5% |
| ACC | vwap_trend_short_late_vwapslope_lb20 | 241 | 0.016 | 0.021 | 49.4% |
| ENDURANCE | vwap_trend_short_late_vwapslope_lb20 | 296 | 0.068 | 0.091 | 49.3% |
| CUMMINSIND | vwap_trend_short_late_vwapslope_lb20 | 217 | 0.001 | 0.001 | 49.3% |
| CHOLAFIN | vwap_trend_short_late_vwapslope_lb20 | 205 | 0.023 | 0.030 | 49.3% |
| AXISBANK | vwap_trend_short_late_vwapslope_lb20 | 195 | -0.031 | -0.042 | 49.2% |
| FEDERALBNK | vwap_trend_short_late_vwapslope_lb20 | 244 | 0.051 | 0.069 | 49.2% |
| COROMANDEL | vwap_trend_short_late_vwapslope_lb20 | 236 | 0.045 | 0.061 | 49.2% |
| GODFRYPHLP | vwap_trend_short_late_vwapslope_lb20 | 289 | 0.102 | 0.136 | 49.1% |
| NAUKRI | vwap_trend_short_late_vwapslope_lb20 | 230 | 0.065 | 0.087 | 49.1% |
| MOTILALOFS | vwap_trend_short_late_vwapslope_lb20 | 281 | 0.030 | 0.040 | 49.1% |
| GODREJCP | vwap_trend_short_late_vwapslope_lb20 | 208 | -0.007 | -0.009 | 49.0% |
| DRREDDY | vwap_trend_short_late_vwapslope_lb20 | 204 | -0.012 | -0.016 | 49.0% |
| CONCOR | vwap_trend_short_late_vwapslope_lb20 | 223 | 0.005 | 0.007 | 48.9% |
| M&M | vwap_trend_short_late_vwapslope_lb20 | 232 | 0.038 | 0.050 | 48.7% |
| NESTLEIND | vwap_trend_short_late_vwapslope_lb20 | 228 | 0.024 | 0.032 | 48.7% |
