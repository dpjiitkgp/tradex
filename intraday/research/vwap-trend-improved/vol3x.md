# vwap-trend-improved/vol3x — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 3.0x risk (2.250%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 250 instruments, 2021-03-23..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_allday_vol3x | 77665 | 0.026 | 0.034 | 46.3% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| TCS | vwap_trend_short_allday_vol3x | 15 | 0.360 | 0.480 | 66.7% |
| UBL | vwap_trend_short_allday_vol3x | 63 | 0.126 | 0.168 | 63.5% |
| NTPCGREEN | vwap_trend_short_allday_vol3x | 185 | 0.123 | 0.164 | 60.0% |
| JIOFIN | vwap_trend_short_allday_vol3x | 185 | 0.223 | 0.298 | 59.5% |
| RELIANCE | vwap_trend_short_allday_vol3x | 27 | 0.388 | 0.517 | 59.3% |
| UNITDSPR | vwap_trend_short_allday_vol3x | 27 | 0.274 | 0.365 | 59.3% |
| TMCV | vwap_trend_short_allday_vol3x | 49 | 0.122 | 0.162 | 57.1% |
| ITCHOTELS | vwap_trend_short_allday_vol3x | 165 | 0.196 | 0.261 | 57.0% |
| LICI | vwap_trend_short_allday_vol3x | 376 | 0.097 | 0.129 | 56.6% |
| AWL | vwap_trend_short_allday_vol3x | 433 | 0.122 | 0.163 | 56.4% |
| MANKIND | vwap_trend_short_allday_vol3x | 296 | 0.146 | 0.195 | 55.4% |
| PREMIERENE | vwap_trend_short_allday_vol3x | 47 | 0.196 | 0.262 | 55.3% |
| BAJAJHFL | vwap_trend_short_allday_vol3x | 193 | 0.100 | 0.134 | 54.9% |
| TORNTPOWER | vwap_trend_short_allday_vol3x | 46 | 0.179 | 0.239 | 54.3% |
| RVNL | vwap_trend_short_allday_vol3x | 35 | 0.457 | 0.610 | 54.3% |
| SJVN | vwap_trend_short_allday_vol3x | 61 | -0.030 | -0.040 | 54.1% |
| TMPV | vwap_trend_short_allday_vol3x | 26 | 0.484 | 0.645 | 53.8% |
| DIXON | vwap_trend_short_allday_vol3x | 377 | 0.088 | 0.117 | 53.3% |
| IRCTC | vwap_trend_short_allday_vol3x | 360 | 0.084 | 0.112 | 53.1% |
| ITC | vwap_trend_short_allday_vol3x | 282 | 0.067 | 0.090 | 52.8% |
| CIPLA | vwap_trend_short_allday_vol3x | 311 | 0.040 | 0.053 | 52.7% |
| ZYDUSLIFE | vwap_trend_short_allday_vol3x | 38 | 0.193 | 0.257 | 52.6% |
| COALINDIA | vwap_trend_short_allday_vol3x | 330 | 0.087 | 0.116 | 52.4% |
| TIINDIA | vwap_trend_short_allday_vol3x | 42 | 0.288 | 0.384 | 52.4% |
| BERGEPAINT | vwap_trend_short_allday_vol3x | 476 | 0.072 | 0.096 | 52.3% |
| LINDEINDIA | vwap_trend_short_allday_vol3x | 727 | 0.132 | 0.176 | 52.3% |
| BPCL | vwap_trend_short_allday_vol3x | 324 | 0.099 | 0.132 | 52.2% |
| IREDA | vwap_trend_short_allday_vol3x | 186 | 0.175 | 0.234 | 52.2% |
| LENSKART | vwap_trend_short_allday_vol3x | 73 | 0.210 | 0.280 | 52.1% |
| HUDCO | vwap_trend_short_allday_vol3x | 638 | 0.098 | 0.131 | 52.0% |
| MAZDOCK | vwap_trend_short_allday_vol3x | 513 | 0.108 | 0.144 | 51.9% |
| LGEINDIA | vwap_trend_short_allday_vol3x | 81 | 0.069 | 0.092 | 51.9% |
| HONAUT | vwap_trend_short_allday_vol3x | 742 | 0.079 | 0.105 | 51.8% |
| SUZLON | vwap_trend_short_allday_vol3x | 29 | 0.377 | 0.502 | 51.7% |
| PRESTIGE | vwap_trend_short_allday_vol3x | 29 | 0.317 | 0.422 | 51.7% |
| PHOENIXLTD | vwap_trend_short_allday_vol3x | 29 | 0.205 | 0.274 | 51.7% |
| SHRIRAMFIN | vwap_trend_short_allday_vol3x | 29 | 0.175 | 0.233 | 51.7% |
| ANTHEM | vwap_trend_short_allday_vol3x | 122 | 0.087 | 0.116 | 51.6% |
| MARUTI | vwap_trend_short_allday_vol3x | 305 | 0.069 | 0.093 | 51.5% |
| SBICARD | vwap_trend_short_allday_vol3x | 39 | 0.240 | 0.321 | 51.3% |
| ACC | vwap_trend_short_allday_vol3x | 444 | 0.049 | 0.065 | 51.1% |
| HCLTECH | vwap_trend_short_allday_vol3x | 239 | 0.077 | 0.102 | 51.0% |
| JSWINFRA | vwap_trend_short_allday_vol3x | 288 | 0.098 | 0.130 | 51.0% |
| GAIL | vwap_trend_short_allday_vol3x | 362 | 0.078 | 0.104 | 50.8% |
| HINDALCO | vwap_trend_short_allday_vol3x | 248 | 0.029 | 0.039 | 50.8% |
| HEROMOTOCO | vwap_trend_short_allday_vol3x | 298 | 0.070 | 0.094 | 50.7% |
| GLAXO | vwap_trend_short_allday_vol3x | 737 | 0.086 | 0.114 | 50.6% |
| HAL | vwap_trend_short_allday_vol3x | 368 | 0.078 | 0.104 | 50.5% |
| BDL | vwap_trend_short_allday_vol3x | 539 | 0.143 | 0.191 | 50.5% |
| CONCOR | vwap_trend_short_allday_vol3x | 359 | 0.087 | 0.116 | 50.4% |
| HDFCLIFE | vwap_trend_short_allday_vol3x | 308 | 0.016 | 0.021 | 50.3% |
| ETERNAL | vwap_trend_short_allday_vol3x | 328 | 0.180 | 0.239 | 50.3% |
| DIVISLAB | vwap_trend_short_allday_vol3x | 386 | 0.034 | 0.046 | 50.3% |
| KALYANKJIL | vwap_trend_short_allday_vol3x | 524 | 0.073 | 0.098 | 50.2% |
| WIPRO | vwap_trend_short_allday_vol3x | 20 | 0.487 | 0.650 | 50.0% |
| SBIN | vwap_trend_short_allday_vol3x | 28 | 0.348 | 0.464 | 50.0% |
| TATAELXSI | vwap_trend_short_allday_vol3x | 36 | 0.213 | 0.284 | 50.0% |
| SWIGGY | vwap_trend_short_allday_vol3x | 48 | 0.065 | 0.087 | 50.0% |
| HDBFS | vwap_trend_short_allday_vol3x | 120 | 0.027 | 0.036 | 50.0% |
| JSWSTEEL | vwap_trend_short_allday_vol3x | 343 | 0.065 | 0.087 | 49.9% |
| ADANIPORTS | vwap_trend_short_allday_vol3x | 285 | 0.061 | 0.082 | 49.8% |
| LTM | vwap_trend_short_allday_vol3x | 338 | 0.047 | 0.063 | 49.7% |
| DLF | vwap_trend_short_allday_vol3x | 310 | 0.124 | 0.165 | 49.7% |
| 3MINDIA | vwap_trend_short_allday_vol3x | 807 | 0.051 | 0.068 | 49.6% |
| NLCINDIA | vwap_trend_short_allday_vol3x | 600 | 0.103 | 0.137 | 49.5% |
| PAGEIND | vwap_trend_short_allday_vol3x | 455 | 0.023 | 0.031 | 49.5% |
| LLOYDSME | vwap_trend_short_allday_vol3x | 362 | 0.102 | 0.136 | 49.4% |
| BAJAJ-AUTO | vwap_trend_short_allday_vol3x | 351 | 0.058 | 0.077 | 49.3% |
| GICRE | vwap_trend_short_allday_vol3x | 700 | 0.073 | 0.097 | 49.3% |
| OIL | vwap_trend_short_allday_vol3x | 478 | 0.109 | 0.145 | 49.2% |
| KPITTECH | vwap_trend_short_allday_vol3x | 450 | 0.127 | 0.169 | 49.1% |
| BRITANNIA | vwap_trend_short_allday_vol3x | 373 | 0.007 | 0.010 | 49.1% |
| ATGL | vwap_trend_short_allday_vol3x | 518 | 0.018 | 0.023 | 48.8% |
| TATACONSUM | vwap_trend_short_allday_vol3x | 39 | 0.141 | 0.188 | 48.7% |
| AUROPHARMA | vwap_trend_short_allday_vol3x | 372 | 0.036 | 0.048 | 48.7% |
| EICHERMOT | vwap_trend_short_allday_vol3x | 338 | -0.005 | -0.006 | 48.5% |
| APOLLOHOSP | vwap_trend_short_allday_vol3x | 326 | 0.048 | 0.065 | 48.5% |
| APOLLOTYRE | vwap_trend_short_allday_vol3x | 435 | 0.078 | 0.104 | 48.3% |
| VBL | vwap_trend_short_allday_vol3x | 29 | 0.050 | 0.066 | 48.3% |
| GROWW | vwap_trend_short_allday_vol3x | 58 | 0.029 | 0.038 | 48.3% |
| HAVELLS | vwap_trend_short_allday_vol3x | 375 | 0.055 | 0.073 | 48.3% |
| BIOCON | vwap_trend_short_allday_vol3x | 446 | 0.066 | 0.088 | 48.2% |
| COCHINSHIP | vwap_trend_short_allday_vol3x | 554 | 0.024 | 0.031 | 48.2% |
| MEDANTA | vwap_trend_short_allday_vol3x | 379 | 0.042 | 0.057 | 48.0% |
| DMART | vwap_trend_short_allday_vol3x | 463 | 0.051 | 0.069 | 47.9% |
| JSWENERGY | vwap_trend_short_allday_vol3x | 522 | 0.067 | 0.090 | 47.9% |
| EXIDEIND | vwap_trend_short_allday_vol3x | 449 | 0.006 | 0.008 | 47.9% |
| GODFRYPHLP | vwap_trend_short_allday_vol3x | 665 | 0.083 | 0.111 | 47.8% |
| ONGC | vwap_trend_short_allday_vol3x | 354 | -0.001 | -0.001 | 47.7% |
| AXISBANK | vwap_trend_short_allday_vol3x | 237 | -0.016 | -0.021 | 47.7% |
| SHREECEM | vwap_trend_short_allday_vol3x | 42 | 0.224 | 0.298 | 47.6% |
| BOSCHLTD | vwap_trend_short_allday_vol3x | 538 | 0.017 | 0.023 | 47.6% |
| IOC | vwap_trend_short_allday_vol3x | 427 | 0.038 | 0.051 | 47.5% |
| NIACL | vwap_trend_short_allday_vol3x | 716 | 0.050 | 0.066 | 47.5% |
| BHARTIARTL | vwap_trend_short_allday_vol3x | 253 | 0.017 | 0.023 | 47.4% |
| APARINDS | vwap_trend_short_allday_vol3x | 639 | 0.107 | 0.143 | 47.4% |
| PIIND | vwap_trend_short_allday_vol3x | 38 | 0.037 | 0.050 | 47.4% |
| ESCORTS | vwap_trend_short_allday_vol3x | 492 | 0.039 | 0.052 | 47.4% |
| ADANIGREEN | vwap_trend_short_allday_vol3x | 395 | 0.055 | 0.074 | 47.3% |
| AMBUJACEM | vwap_trend_short_allday_vol3x | 376 | 0.059 | 0.079 | 47.3% |
