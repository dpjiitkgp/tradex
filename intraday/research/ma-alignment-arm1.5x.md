# ma-alignment-arm1.5x — exit-rule research

Exit rule: risk_pct=0.500% (hard stop), armed at 1.5x risk (0.750%), give-back trail 0.500% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 250 instruments, 2026-03-27..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| ma_alignment | 70469 | 0.007 | 0.015 | 43.9% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| IRCTC | ma_alignment | 196 | 0.121 | 0.242 | 55.6% |
| BAJAJHFL | ma_alignment | 206 | 0.066 | 0.132 | 52.9% |
| HDFCBANK | ma_alignment | 198 | 0.098 | 0.196 | 50.5% |
| ICICIGI | ma_alignment | 246 | 0.046 | 0.093 | 50.4% |
| UNITDSPR | ma_alignment | 228 | 0.066 | 0.131 | 50.0% |
| DLF | ma_alignment | 259 | 0.146 | 0.292 | 49.8% |
| ATGL | ma_alignment | 326 | 0.140 | 0.279 | 49.7% |
| HINDZINC | ma_alignment | 294 | 0.080 | 0.160 | 49.7% |
| RELIANCE | ma_alignment | 193 | 0.074 | 0.147 | 49.2% |
| BANKBARODA | ma_alignment | 234 | 0.078 | 0.156 | 49.1% |
| IOC | ma_alignment | 212 | 0.053 | 0.105 | 49.1% |
| HCLTECH | ma_alignment | 252 | 0.082 | 0.164 | 48.8% |
| APOLLOHOSP | ma_alignment | 204 | -0.009 | -0.018 | 48.5% |
| HONAUT | ma_alignment | 287 | 0.060 | 0.121 | 48.4% |
| IDFCFIRSTB | ma_alignment | 246 | 0.033 | 0.066 | 48.4% |
| SBIN | ma_alignment | 218 | 0.064 | 0.128 | 48.2% |
| POWERGRID | ma_alignment | 216 | 0.011 | 0.022 | 48.1% |
| JUBLFOOD | ma_alignment | 295 | 0.093 | 0.186 | 48.1% |
| NLCINDIA | ma_alignment | 341 | 0.014 | 0.028 | 48.1% |
| BHARTIARTL | ma_alignment | 204 | 0.020 | 0.040 | 48.0% |
| PIIND | ma_alignment | 277 | 0.063 | 0.126 | 48.0% |
| FEDERALBNK | ma_alignment | 242 | 0.054 | 0.108 | 47.9% |
| DABUR | ma_alignment | 224 | 0.044 | 0.089 | 47.8% |
| JKCEMENT | ma_alignment | 287 | 0.024 | 0.047 | 47.7% |
| TCS | ma_alignment | 237 | 0.050 | 0.101 | 47.7% |
| M&MFIN | ma_alignment | 357 | 0.061 | 0.122 | 47.6% |
| RVNL | ma_alignment | 273 | 0.054 | 0.107 | 47.6% |
| CRISIL | ma_alignment | 294 | -0.017 | -0.033 | 47.6% |
| HEXT | ma_alignment | 334 | 0.077 | 0.154 | 47.6% |
| VMM | ma_alignment | 290 | 0.052 | 0.104 | 47.6% |
| ALKEM | ma_alignment | 244 | 0.043 | 0.086 | 47.5% |
| ITC | ma_alignment | 183 | 0.036 | 0.071 | 47.5% |
| MOTILALOFS | ma_alignment | 343 | 0.005 | 0.009 | 47.5% |
| BSE | ma_alignment | 300 | 0.084 | 0.167 | 47.3% |
| MAXHEALTH | ma_alignment | 262 | 0.063 | 0.125 | 47.3% |
| WIPRO | ma_alignment | 203 | 0.027 | 0.054 | 47.3% |
| MFSL | ma_alignment | 294 | 0.045 | 0.089 | 47.3% |
| BANKINDIA | ma_alignment | 288 | 0.011 | 0.022 | 47.2% |
| ICICIPRULI | ma_alignment | 282 | 0.068 | 0.136 | 47.2% |
| MUTHOOTFIN | ma_alignment | 282 | 0.005 | 0.009 | 47.2% |
| APOLLOTYRE | ma_alignment | 272 | -0.013 | -0.025 | 47.1% |
| PNB | ma_alignment | 232 | 0.029 | 0.058 | 47.0% |
| TATACOMM | ma_alignment | 362 | 0.026 | 0.052 | 47.0% |
| ASTRAL | ma_alignment | 279 | 0.062 | 0.124 | 47.0% |
| BAJFINANCE | ma_alignment | 263 | 0.028 | 0.056 | 46.8% |
| NESTLEIND | ma_alignment | 234 | -0.012 | -0.025 | 46.6% |
| PETRONET | ma_alignment | 247 | 0.015 | 0.030 | 46.6% |
| HAL | ma_alignment | 256 | 0.039 | 0.079 | 46.5% |
| LAURUSLABS | ma_alignment | 284 | 0.034 | 0.067 | 46.5% |
| ITCHOTELS | ma_alignment | 276 | 0.052 | 0.105 | 46.4% |
| POLICYBZR | ma_alignment | 298 | 0.060 | 0.120 | 46.3% |
| ICICIAMC | ma_alignment | 298 | 0.049 | 0.098 | 46.3% |
| APARINDS | ma_alignment | 389 | 0.078 | 0.156 | 46.3% |
| BERGEPAINT | ma_alignment | 294 | 0.042 | 0.085 | 46.3% |
| PRESTIGE | ma_alignment | 346 | 0.024 | 0.048 | 46.2% |
| BOSCHLTD | ma_alignment | 264 | 0.041 | 0.082 | 46.2% |
| KALYANKJIL | ma_alignment | 368 | 0.032 | 0.064 | 46.2% |
| SAIL | ma_alignment | 334 | 0.082 | 0.165 | 46.1% |
| MPHASIS | ma_alignment | 308 | 0.066 | 0.133 | 46.1% |
| GODREJCP | ma_alignment | 252 | -0.027 | -0.054 | 46.0% |
| ENRIN | ma_alignment | 350 | 0.071 | 0.142 | 46.0% |
| IPCALAB | ma_alignment | 335 | 0.013 | 0.026 | 46.0% |
| GICRE | ma_alignment | 259 | 0.022 | 0.043 | 45.9% |
| THERMAX | ma_alignment | 331 | 0.078 | 0.155 | 45.9% |
| APLAPOLLO | ma_alignment | 294 | 0.064 | 0.128 | 45.9% |
| UNIONBANK | ma_alignment | 288 | 0.027 | 0.053 | 45.8% |
| SCHAEFFLER | ma_alignment | 312 | 0.006 | 0.012 | 45.8% |
| DIVISLAB | ma_alignment | 234 | 0.046 | 0.092 | 45.7% |
| UPL | ma_alignment | 243 | 0.039 | 0.078 | 45.7% |
| PHOENIXLTD | ma_alignment | 300 | 0.011 | 0.022 | 45.7% |
| VBL | ma_alignment | 309 | -0.017 | -0.035 | 45.6% |
| LENSKART | ma_alignment | 331 | -0.007 | -0.013 | 45.6% |
| ACC | ma_alignment | 246 | 0.002 | 0.003 | 45.5% |
| ONGC | ma_alignment | 209 | 0.034 | 0.069 | 45.5% |
| PAGEIND | ma_alignment | 264 | 0.011 | 0.022 | 45.5% |
| DRREDDY | ma_alignment | 242 | 0.004 | 0.009 | 45.5% |
| SOLARINDS | ma_alignment | 304 | 0.072 | 0.145 | 45.4% |
| 3MINDIA | ma_alignment | 289 | -0.004 | -0.008 | 45.3% |
| DIXON | ma_alignment | 307 | 0.054 | 0.108 | 45.3% |
| JIOFIN | ma_alignment | 232 | 0.045 | 0.091 | 45.3% |
| LUPIN | ma_alignment | 241 | -0.063 | -0.127 | 45.2% |
| BAJAJFINSV | ma_alignment | 230 | 0.013 | 0.027 | 45.2% |
| BEL | ma_alignment | 219 | 0.015 | 0.030 | 45.2% |
| COLPAL | ma_alignment | 250 | 0.060 | 0.120 | 45.2% |
| DMART | ma_alignment | 250 | 0.035 | 0.070 | 45.2% |
| RADICO | ma_alignment | 290 | 0.009 | 0.017 | 45.2% |
| GLAXO | ma_alignment | 288 | 0.016 | 0.032 | 45.1% |
| AUBANK | ma_alignment | 275 | 0.026 | 0.051 | 45.1% |
| 360ONE | ma_alignment | 295 | 0.040 | 0.079 | 45.1% |
| SBILIFE | ma_alignment | 227 | 0.019 | 0.038 | 44.9% |
| ADANIGREEN | ma_alignment | 345 | 0.059 | 0.119 | 44.9% |
| EXIDEIND | ma_alignment | 325 | 0.036 | 0.073 | 44.9% |
| ANTHEM | ma_alignment | 317 | -0.013 | -0.026 | 44.8% |
| ABBOTINDIA | ma_alignment | 237 | 0.008 | 0.016 | 44.7% |
| TATASTEEL | ma_alignment | 255 | 0.003 | 0.006 | 44.7% |
| BLUESTARCO | ma_alignment | 338 | -0.009 | -0.017 | 44.7% |
| COCHINSHIP | ma_alignment | 336 | 0.029 | 0.057 | 44.6% |
| RECLTD | ma_alignment | 260 | 0.022 | 0.045 | 44.6% |
| TMCV | ma_alignment | 361 | 0.028 | 0.056 | 44.6% |
| ICICIBANK | ma_alignment | 202 | -0.001 | -0.001 | 44.6% |
