# vwap-trend-improved/vwapslope_lb20_250 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 3.0x risk (2.250%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 250 instruments, 2021-03-23..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_late_vwapslope_lb20 | 42657 | 0.042 | 0.056 | 45.9% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| ZYDUSLIFE | vwap_trend_short_late_vwapslope_lb20 | 13 | 0.450 | 0.600 | 69.2% |
| TCS | vwap_trend_short_late_vwapslope_lb20 | 15 | 0.778 | 1.037 | 66.7% |
| BAJAJHFL | vwap_trend_short_late_vwapslope_lb20 | 111 | 0.206 | 0.274 | 65.8% |
| LICI | vwap_trend_short_late_vwapslope_lb20 | 199 | 0.168 | 0.224 | 65.3% |
| VOLTAS | vwap_trend_short_late_vwapslope_lb20 | 14 | 0.846 | 1.128 | 64.3% |
| TORNTPOWER | vwap_trend_short_late_vwapslope_lb20 | 21 | 0.425 | 0.566 | 61.9% |
| LENSKART | vwap_trend_short_late_vwapslope_lb20 | 39 | 0.326 | 0.435 | 61.5% |
| POWERGRID | vwap_trend_short_late_vwapslope_lb20 | 23 | 0.252 | 0.337 | 60.9% |
| SUNDARMFIN | vwap_trend_short_late_vwapslope_lb20 | 23 | 0.146 | 0.195 | 60.9% |
| ITCHOTELS | vwap_trend_short_late_vwapslope_lb20 | 68 | 0.226 | 0.301 | 60.3% |
| PRESTIGE | vwap_trend_short_late_vwapslope_lb20 | 20 | 0.607 | 0.809 | 60.0% |
| TATACONSUM | vwap_trend_short_late_vwapslope_lb20 | 20 | 0.170 | 0.227 | 60.0% |
| AWL | vwap_trend_short_late_vwapslope_lb20 | 222 | 0.162 | 0.216 | 58.6% |
| PIIND | vwap_trend_short_late_vwapslope_lb20 | 19 | 0.234 | 0.311 | 57.9% |
| UPL | vwap_trend_short_late_vwapslope_lb20 | 19 | 0.205 | 0.273 | 57.9% |
| UNITDSPR | vwap_trend_short_late_vwapslope_lb20 | 14 | 0.075 | 0.101 | 57.1% |
| TATAINVEST | vwap_trend_short_late_vwapslope_lb20 | 30 | 0.134 | 0.178 | 56.7% |
| SAIL | vwap_trend_short_late_vwapslope_lb20 | 16 | 0.685 | 0.913 | 56.2% |
| TMCV | vwap_trend_short_late_vwapslope_lb20 | 25 | 0.323 | 0.431 | 56.0% |
| EXIDEIND | vwap_trend_short_late_vwapslope_lb20 | 254 | 0.123 | 0.164 | 55.9% |
| WIPRO | vwap_trend_short_late_vwapslope_lb20 | 18 | 0.620 | 0.827 | 55.6% |
| IREDA | vwap_trend_short_late_vwapslope_lb20 | 120 | 0.222 | 0.296 | 55.0% |
| NTPCGREEN | vwap_trend_short_late_vwapslope_lb20 | 95 | 0.111 | 0.148 | 54.7% |
| JIOFIN | vwap_trend_short_late_vwapslope_lb20 | 134 | 0.168 | 0.224 | 54.5% |
| ITC | vwap_trend_short_late_vwapslope_lb20 | 196 | 0.101 | 0.134 | 54.1% |
| INDUSINDBK | vwap_trend_short_late_vwapslope_lb20 | 200 | 0.189 | 0.252 | 54.0% |
| TMPV | vwap_trend_short_late_vwapslope_lb20 | 13 | 0.431 | 0.575 | 53.8% |
| PNB | vwap_trend_short_late_vwapslope_lb20 | 13 | 0.114 | 0.152 | 53.8% |
| DIVISLAB | vwap_trend_short_late_vwapslope_lb20 | 210 | 0.103 | 0.137 | 53.8% |
| MARICO | vwap_trend_short_late_vwapslope_lb20 | 199 | 0.055 | 0.073 | 53.8% |
| COCHINSHIP | vwap_trend_short_late_vwapslope_lb20 | 325 | 0.149 | 0.199 | 53.5% |
| SBIN | vwap_trend_short_late_vwapslope_lb20 | 15 | 0.163 | 0.218 | 53.3% |
| LINDEINDIA | vwap_trend_short_late_vwapslope_lb20 | 301 | 0.166 | 0.221 | 53.2% |
| SHREECEM | vwap_trend_short_late_vwapslope_lb20 | 17 | 0.157 | 0.209 | 52.9% |
| RELIANCE | vwap_trend_short_late_vwapslope_lb20 | 19 | 0.068 | 0.091 | 52.6% |
| JSWINFRA | vwap_trend_short_late_vwapslope_lb20 | 135 | 0.162 | 0.216 | 52.6% |
| VEDL | vwap_trend_short_late_vwapslope_lb20 | 25 | 0.342 | 0.456 | 52.0% |
| UBL | vwap_trend_short_late_vwapslope_lb20 | 27 | 0.058 | 0.078 | 51.9% |
| HAVELLS | vwap_trend_short_late_vwapslope_lb20 | 243 | 0.052 | 0.069 | 51.9% |
| HDBFS | vwap_trend_short_late_vwapslope_lb20 | 62 | 0.014 | 0.019 | 51.6% |
| HUDCO | vwap_trend_short_late_vwapslope_lb20 | 312 | 0.107 | 0.142 | 51.6% |
| HAL | vwap_trend_short_late_vwapslope_lb20 | 247 | 0.172 | 0.230 | 51.4% |
| HONAUT | vwap_trend_short_late_vwapslope_lb20 | 323 | 0.065 | 0.087 | 51.4% |
| 3MINDIA | vwap_trend_short_late_vwapslope_lb20 | 364 | 0.062 | 0.083 | 50.8% |
| DIXON | vwap_trend_short_late_vwapslope_lb20 | 244 | 0.205 | 0.273 | 50.8% |
| APARINDS | vwap_trend_short_late_vwapslope_lb20 | 294 | 0.220 | 0.294 | 50.7% |
| HDFCLIFE | vwap_trend_short_late_vwapslope_lb20 | 200 | 0.062 | 0.082 | 50.5% |
| NLCINDIA | vwap_trend_short_late_vwapslope_lb20 | 301 | 0.162 | 0.216 | 50.5% |
| GLAXO | vwap_trend_short_late_vwapslope_lb20 | 321 | 0.093 | 0.124 | 50.5% |
| DLF | vwap_trend_short_late_vwapslope_lb20 | 232 | 0.168 | 0.224 | 50.4% |
| NYKAA | vwap_trend_short_late_vwapslope_lb20 | 221 | 0.159 | 0.212 | 50.2% |
| CANBK | vwap_trend_short_late_vwapslope_lb20 | 221 | 0.125 | 0.167 | 50.2% |
| BIOCON | vwap_trend_short_late_vwapslope_lb20 | 231 | 0.117 | 0.155 | 50.2% |
| GODFRYPHLP | vwap_trend_short_late_vwapslope_lb20 | 289 | 0.167 | 0.222 | 50.2% |
| SUNPHARMA | vwap_trend_short_late_vwapslope_lb20 | 22 | 0.112 | 0.150 | 50.0% |
| HEROMOTOCO | vwap_trend_short_late_vwapslope_lb20 | 210 | 0.089 | 0.119 | 50.0% |
| BAJAJ-AUTO | vwap_trend_short_late_vwapslope_lb20 | 222 | 0.078 | 0.104 | 50.0% |
| TATACOMM | vwap_trend_short_late_vwapslope_lb20 | 26 | -0.002 | -0.003 | 50.0% |
| SJVN | vwap_trend_short_late_vwapslope_lb20 | 26 | -0.112 | -0.149 | 50.0% |
| BDL | vwap_trend_short_late_vwapslope_lb20 | 289 | 0.130 | 0.173 | 49.8% |
| CRISIL | vwap_trend_short_late_vwapslope_lb20 | 304 | 0.107 | 0.143 | 49.7% |
| KPITTECH | vwap_trend_short_late_vwapslope_lb20 | 260 | 0.134 | 0.178 | 49.6% |
| BANKINDIA | vwap_trend_short_late_vwapslope_lb20 | 305 | 0.119 | 0.159 | 49.5% |
| MAZDOCK | vwap_trend_short_late_vwapslope_lb20 | 269 | 0.149 | 0.198 | 49.4% |
| DMART | vwap_trend_short_late_vwapslope_lb20 | 259 | 0.103 | 0.137 | 49.4% |
| IRCTC | vwap_trend_short_late_vwapslope_lb20 | 227 | 0.049 | 0.066 | 49.3% |
| ABB | vwap_trend_short_late_vwapslope_lb20 | 221 | 0.050 | 0.067 | 49.3% |
| HDFCBANK | vwap_trend_short_late_vwapslope_lb20 | 211 | 0.027 | 0.036 | 49.3% |
| LTM | vwap_trend_short_late_vwapslope_lb20 | 209 | 0.094 | 0.125 | 49.3% |
| OIL | vwap_trend_short_late_vwapslope_lb20 | 289 | 0.234 | 0.312 | 49.1% |
| INDIGO | vwap_trend_short_late_vwapslope_lb20 | 228 | 0.039 | 0.052 | 49.1% |
| LLOYDSME | vwap_trend_short_late_vwapslope_lb20 | 165 | 0.122 | 0.162 | 49.1% |
| ATGL | vwap_trend_short_late_vwapslope_lb20 | 267 | 0.103 | 0.137 | 49.1% |
| LGEINDIA | vwap_trend_short_late_vwapslope_lb20 | 41 | 0.015 | 0.021 | 48.8% |
| CIPLA | vwap_trend_short_late_vwapslope_lb20 | 199 | -0.022 | -0.029 | 48.7% |
| AXISBANK | vwap_trend_short_late_vwapslope_lb20 | 195 | -0.020 | -0.027 | 48.7% |
| LUPIN | vwap_trend_short_late_vwapslope_lb20 | 222 | 0.089 | 0.118 | 48.6% |
| INFY | vwap_trend_short_late_vwapslope_lb20 | 183 | 0.004 | 0.005 | 48.6% |
| IOC | vwap_trend_short_late_vwapslope_lb20 | 247 | 0.120 | 0.161 | 48.6% |
| MPHASIS | vwap_trend_short_late_vwapslope_lb20 | 208 | 0.166 | 0.222 | 48.6% |
| APOLLOHOSP | vwap_trend_short_late_vwapslope_lb20 | 204 | 0.073 | 0.097 | 48.5% |
| HDFCAMC | vwap_trend_short_late_vwapslope_lb20 | 246 | 0.037 | 0.049 | 48.4% |
| BERGEPAINT | vwap_trend_short_late_vwapslope_lb20 | 238 | 0.028 | 0.037 | 48.3% |
| KALYANKJIL | vwap_trend_short_late_vwapslope_lb20 | 310 | 0.085 | 0.114 | 48.1% |
| KPRMILL | vwap_trend_short_late_vwapslope_lb20 | 308 | 0.095 | 0.127 | 48.1% |
| NESTLEIND | vwap_trend_short_late_vwapslope_lb20 | 227 | 0.019 | 0.025 | 48.0% |
| TIINDIA | vwap_trend_short_late_vwapslope_lb20 | 25 | 0.255 | 0.340 | 48.0% |
| DABUR | vwap_trend_short_late_vwapslope_lb20 | 198 | 0.085 | 0.114 | 48.0% |
| LICHSGFIN | vwap_trend_short_late_vwapslope_lb20 | 213 | 0.033 | 0.044 | 47.9% |
| TATAELXSI | vwap_trend_short_late_vwapslope_lb20 | 23 | 0.336 | 0.447 | 47.8% |
| MANKIND | vwap_trend_short_late_vwapslope_lb20 | 161 | 0.116 | 0.155 | 47.8% |
| THERMAX | vwap_trend_short_late_vwapslope_lb20 | 23 | 0.047 | 0.063 | 47.8% |
| GAIL | vwap_trend_short_late_vwapslope_lb20 | 226 | 0.067 | 0.089 | 47.8% |
| ACC | vwap_trend_short_late_vwapslope_lb20 | 241 | 0.039 | 0.051 | 47.7% |
| COALINDIA | vwap_trend_short_late_vwapslope_lb20 | 193 | 0.069 | 0.091 | 47.7% |
| APOLLOTYRE | vwap_trend_short_late_vwapslope_lb20 | 235 | 0.081 | 0.108 | 47.7% |
| PETRONET | vwap_trend_short_late_vwapslope_lb20 | 21 | -0.026 | -0.035 | 47.6% |
| BHARTIARTL | vwap_trend_short_late_vwapslope_lb20 | 229 | -0.005 | -0.007 | 47.6% |
| AUROPHARMA | vwap_trend_short_late_vwapslope_lb20 | 225 | 0.032 | 0.042 | 47.6% |
| JKCEMENT | vwap_trend_short_late_vwapslope_lb20 | 253 | 0.048 | 0.064 | 47.4% |
