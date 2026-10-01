# vwap-trend-improved/vwapslope — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 3.0x risk (2.250%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 250 instruments, 2021-03-23..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_late_vwapslope | 42207 | 0.041 | 0.055 | 45.9% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| POWERGRID | vwap_trend_short_late_vwapslope | 19 | 0.350 | 0.466 | 68.4% |
| VOLTAS | vwap_trend_short_late_vwapslope | 15 | 0.829 | 1.105 | 66.7% |
| BAJAJHFL | vwap_trend_short_late_vwapslope | 108 | 0.228 | 0.303 | 66.7% |
| LICI | vwap_trend_short_late_vwapslope | 194 | 0.178 | 0.238 | 64.9% |
| ZYDUSLIFE | vwap_trend_short_late_vwapslope | 14 | 0.363 | 0.484 | 64.3% |
| TORNTPOWER | vwap_trend_short_late_vwapslope | 19 | 0.425 | 0.566 | 63.2% |
| TCS | vwap_trend_short_late_vwapslope | 16 | 0.678 | 0.904 | 62.5% |
| PRESTIGE | vwap_trend_short_late_vwapslope | 18 | 0.530 | 0.707 | 61.1% |
| ITCHOTELS | vwap_trend_short_late_vwapslope | 71 | 0.213 | 0.284 | 60.6% |
| WIPRO | vwap_trend_short_late_vwapslope | 15 | 0.694 | 0.925 | 60.0% |
| TATACONSUM | vwap_trend_short_late_vwapslope | 20 | 0.170 | 0.227 | 60.0% |
| UNITDSPR | vwap_trend_short_late_vwapslope | 15 | 0.128 | 0.171 | 60.0% |
| TATAINVEST | vwap_trend_short_late_vwapslope | 29 | 0.140 | 0.187 | 58.6% |
| AWL | vwap_trend_short_late_vwapslope | 216 | 0.161 | 0.215 | 57.9% |
| LENSKART | vwap_trend_short_late_vwapslope | 45 | 0.291 | 0.388 | 57.8% |
| SAIL | vwap_trend_short_late_vwapslope | 14 | 0.731 | 0.975 | 57.1% |
| UBL | vwap_trend_short_late_vwapslope | 28 | 0.165 | 0.220 | 57.1% |
| SUNDARMFIN | vwap_trend_short_late_vwapslope | 25 | 0.068 | 0.091 | 56.0% |
| EXIDEIND | vwap_trend_short_late_vwapslope | 248 | 0.117 | 0.155 | 55.6% |
| DIVISLAB | vwap_trend_short_late_vwapslope | 205 | 0.124 | 0.165 | 55.6% |
| PIIND | vwap_trend_short_late_vwapslope | 18 | 0.121 | 0.161 | 55.6% |
| JIOFIN | vwap_trend_short_late_vwapslope | 134 | 0.168 | 0.223 | 55.2% |
| ITC | vwap_trend_short_late_vwapslope | 187 | 0.091 | 0.121 | 54.5% |
| INDUSINDBK | vwap_trend_short_late_vwapslope | 201 | 0.193 | 0.257 | 54.2% |
| TMCV | vwap_trend_short_late_vwapslope | 24 | 0.343 | 0.457 | 54.2% |
| TATACOMM | vwap_trend_short_late_vwapslope | 24 | 0.055 | 0.073 | 54.2% |
| TMPV | vwap_trend_short_late_vwapslope | 13 | 0.407 | 0.542 | 53.8% |
| PNB | vwap_trend_short_late_vwapslope | 13 | 0.114 | 0.152 | 53.8% |
| COCHINSHIP | vwap_trend_short_late_vwapslope | 314 | 0.153 | 0.205 | 53.8% |
| MARICO | vwap_trend_short_late_vwapslope | 197 | 0.060 | 0.079 | 53.8% |
| NTPCGREEN | vwap_trend_short_late_vwapslope | 93 | 0.074 | 0.099 | 53.8% |
| PIDILITIND | vwap_trend_short_late_vwapslope | 15 | 0.145 | 0.194 | 53.3% |
| IREDA | vwap_trend_short_late_vwapslope | 117 | 0.194 | 0.258 | 53.0% |
| UPL | vwap_trend_short_late_vwapslope | 19 | 0.147 | 0.196 | 52.6% |
| JSWINFRA | vwap_trend_short_late_vwapslope | 137 | 0.154 | 0.206 | 52.6% |
| HONAUT | vwap_trend_short_late_vwapslope | 328 | 0.070 | 0.094 | 52.4% |
| TIINDIA | vwap_trend_short_late_vwapslope | 23 | 0.346 | 0.462 | 52.2% |
| SUNPHARMA | vwap_trend_short_late_vwapslope | 23 | 0.231 | 0.308 | 52.2% |
| VEDL | vwap_trend_short_late_vwapslope | 25 | 0.322 | 0.430 | 52.0% |
| LINDEINDIA | vwap_trend_short_late_vwapslope | 295 | 0.127 | 0.169 | 51.9% |
| DIXON | vwap_trend_short_late_vwapslope | 234 | 0.221 | 0.294 | 51.7% |
| NYKAA | vwap_trend_short_late_vwapslope | 213 | 0.195 | 0.260 | 51.6% |
| HEROMOTOCO | vwap_trend_short_late_vwapslope | 215 | 0.098 | 0.130 | 51.6% |
| HAVELLS | vwap_trend_short_late_vwapslope | 241 | 0.057 | 0.076 | 51.5% |
| HUDCO | vwap_trend_short_late_vwapslope | 311 | 0.086 | 0.114 | 51.4% |
| GLAXO | vwap_trend_short_late_vwapslope | 328 | 0.111 | 0.148 | 50.9% |
| APARINDS | vwap_trend_short_late_vwapslope | 294 | 0.232 | 0.309 | 50.7% |
| BIOCON | vwap_trend_short_late_vwapslope | 223 | 0.125 | 0.167 | 50.7% |
| DLF | vwap_trend_short_late_vwapslope | 229 | 0.173 | 0.230 | 50.7% |
| HDFCLIFE | vwap_trend_short_late_vwapslope | 190 | 0.072 | 0.096 | 50.5% |
| 3MINDIA | vwap_trend_short_late_vwapslope | 371 | 0.067 | 0.090 | 50.4% |
| CRISIL | vwap_trend_short_late_vwapslope | 308 | 0.117 | 0.155 | 50.3% |
| LLOYDSME | vwap_trend_short_late_vwapslope | 161 | 0.146 | 0.195 | 50.3% |
| GODFRYPHLP | vwap_trend_short_late_vwapslope | 289 | 0.169 | 0.225 | 50.2% |
| ICICIAMC | vwap_trend_short_late_vwapslope | 28 | 0.190 | 0.253 | 50.0% |
| MAZDOCK | vwap_trend_short_late_vwapslope | 274 | 0.161 | 0.215 | 50.0% |
| NLCINDIA | vwap_trend_short_late_vwapslope | 308 | 0.150 | 0.201 | 50.0% |
| SBIN | vwap_trend_short_late_vwapslope | 14 | 0.112 | 0.150 | 50.0% |
| SHREECEM | vwap_trend_short_late_vwapslope | 18 | 0.099 | 0.132 | 50.0% |
| SJVN | vwap_trend_short_late_vwapslope | 26 | -0.112 | -0.149 | 50.0% |
| MANKIND | vwap_trend_short_late_vwapslope | 161 | 0.169 | 0.225 | 49.7% |
| HAL | vwap_trend_short_late_vwapslope | 242 | 0.134 | 0.178 | 49.6% |
| LTM | vwap_trend_short_late_vwapslope | 208 | 0.090 | 0.120 | 49.5% |
| DRREDDY | vwap_trend_short_late_vwapslope | 194 | 0.024 | 0.032 | 49.5% |
| ATGL | vwap_trend_short_late_vwapslope | 273 | 0.086 | 0.115 | 49.5% |
| KPITTECH | vwap_trend_short_late_vwapslope | 247 | 0.119 | 0.159 | 49.4% |
| NESTLEIND | vwap_trend_short_late_vwapslope | 223 | 0.022 | 0.029 | 49.3% |
| BDL | vwap_trend_short_late_vwapslope | 284 | 0.121 | 0.162 | 49.3% |
| APOLLOHOSP | vwap_trend_short_late_vwapslope | 199 | 0.093 | 0.124 | 49.2% |
| HDBFS | vwap_trend_short_late_vwapslope | 59 | 0.008 | 0.010 | 49.2% |
| LUPIN | vwap_trend_short_late_vwapslope | 222 | 0.102 | 0.135 | 49.1% |
| CANBK | vwap_trend_short_late_vwapslope | 214 | 0.122 | 0.162 | 49.1% |
| IRCTC | vwap_trend_short_late_vwapslope | 217 | 0.046 | 0.061 | 48.8% |
| BAJAJ-AUTO | vwap_trend_short_late_vwapslope | 215 | 0.069 | 0.092 | 48.8% |
| IOC | vwap_trend_short_late_vwapslope | 246 | 0.122 | 0.163 | 48.8% |
| ACC | vwap_trend_short_late_vwapslope | 240 | 0.048 | 0.064 | 48.8% |
| BERGEPAINT | vwap_trend_short_late_vwapslope | 236 | 0.032 | 0.042 | 48.7% |
| KPRMILL | vwap_trend_short_late_vwapslope | 312 | 0.095 | 0.127 | 48.7% |
| COALINDIA | vwap_trend_short_late_vwapslope | 191 | 0.088 | 0.117 | 48.7% |
| ABB | vwap_trend_short_late_vwapslope | 224 | 0.050 | 0.067 | 48.7% |
| GAIL | vwap_trend_short_late_vwapslope | 220 | 0.076 | 0.102 | 48.6% |
| DMART | vwap_trend_short_late_vwapslope | 247 | 0.083 | 0.111 | 48.6% |
| KALYANKJIL | vwap_trend_short_late_vwapslope | 305 | 0.081 | 0.108 | 48.5% |
| OIL | vwap_trend_short_late_vwapslope | 291 | 0.219 | 0.292 | 48.5% |
| ANTHEM | vwap_trend_short_late_vwapslope | 56 | 0.081 | 0.108 | 48.2% |
| AXISBANK | vwap_trend_short_late_vwapslope | 193 | -0.028 | -0.037 | 48.2% |
| HDFCAMC | vwap_trend_short_late_vwapslope | 237 | 0.027 | 0.037 | 48.1% |
| APOLLOTYRE | vwap_trend_short_late_vwapslope | 233 | 0.088 | 0.117 | 48.1% |
| HDFCBANK | vwap_trend_short_late_vwapslope | 206 | 0.013 | 0.018 | 48.1% |
| BHARTIARTL | vwap_trend_short_late_vwapslope | 225 | -0.003 | -0.004 | 48.0% |
| CIPLA | vwap_trend_short_late_vwapslope | 192 | -0.041 | -0.055 | 47.9% |
| TATAELXSI | vwap_trend_short_late_vwapslope | 23 | 0.327 | 0.436 | 47.8% |
| BANKINDIA | vwap_trend_short_late_vwapslope | 297 | 0.095 | 0.127 | 47.8% |
| INDIGO | vwap_trend_short_late_vwapslope | 224 | 0.009 | 0.012 | 47.8% |
| JUBLFOOD | vwap_trend_short_late_vwapslope | 201 | 0.003 | 0.004 | 47.8% |
| MOTILALOFS | vwap_trend_short_late_vwapslope | 279 | 0.073 | 0.097 | 47.7% |
| LGEINDIA | vwap_trend_short_late_vwapslope | 42 | -0.001 | -0.001 | 47.6% |
| INFY | vwap_trend_short_late_vwapslope | 186 | -0.012 | -0.016 | 47.3% |
| DABUR | vwap_trend_short_late_vwapslope | 195 | 0.091 | 0.121 | 47.2% |
| JKCEMENT | vwap_trend_short_late_vwapslope | 259 | 0.065 | 0.086 | 47.1% |
