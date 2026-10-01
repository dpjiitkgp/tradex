# ma-alignment-arm2x — exit-rule research

Exit rule: risk_pct=0.500% (hard stop), armed at 2.0x risk (1.000%), give-back trail 0.500% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 250 instruments, 2026-03-27..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| ma_alignment | 65878 | 0.009 | 0.019 | 41.0% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| IRCTC | ma_alignment | 192 | 0.092 | 0.183 | 51.6% |
| BAJAJHFL | ma_alignment | 200 | 0.069 | 0.139 | 51.0% |
| ICICIGI | ma_alignment | 232 | 0.064 | 0.128 | 50.4% |
| IOC | ma_alignment | 198 | 0.069 | 0.137 | 48.0% |
| UNITDSPR | ma_alignment | 213 | 0.095 | 0.190 | 47.9% |
| ATGL | ma_alignment | 311 | 0.148 | 0.296 | 47.3% |
| DABUR | ma_alignment | 214 | 0.061 | 0.122 | 47.2% |
| RELIANCE | ma_alignment | 179 | 0.084 | 0.168 | 46.9% |
| HCLTECH | ma_alignment | 231 | 0.131 | 0.263 | 46.3% |
| NESTLEIND | ma_alignment | 214 | 0.009 | 0.017 | 46.3% |
| WIPRO | ma_alignment | 191 | 0.044 | 0.089 | 46.1% |
| NLCINDIA | ma_alignment | 330 | 0.033 | 0.065 | 46.1% |
| PIIND | ma_alignment | 260 | 0.087 | 0.173 | 45.8% |
| GICRE | ma_alignment | 247 | 0.029 | 0.058 | 45.7% |
| BERGEPAINT | ma_alignment | 280 | 0.066 | 0.131 | 45.7% |
| ONGC | ma_alignment | 197 | 0.039 | 0.078 | 45.7% |
| DLF | ma_alignment | 240 | 0.142 | 0.283 | 45.4% |
| APOLLOHOSP | ma_alignment | 185 | -0.009 | -0.018 | 45.4% |
| RVNL | ma_alignment | 256 | 0.059 | 0.119 | 45.3% |
| HONAUT | ma_alignment | 274 | 0.067 | 0.134 | 45.3% |
| MPHASIS | ma_alignment | 283 | 0.103 | 0.205 | 45.2% |
| RADICO | ma_alignment | 261 | 0.049 | 0.097 | 45.2% |
| APOLLOTYRE | ma_alignment | 257 | 0.001 | 0.001 | 45.1% |
| POLICYBZR | ma_alignment | 286 | 0.073 | 0.147 | 45.1% |
| HDFCBANK | ma_alignment | 187 | 0.059 | 0.118 | 44.9% |
| PETRONET | ma_alignment | 235 | 0.015 | 0.030 | 44.7% |
| MAHABANK | ma_alignment | 291 | 0.020 | 0.040 | 44.7% |
| ENRIN | ma_alignment | 332 | 0.091 | 0.182 | 44.6% |
| PRESTIGE | ma_alignment | 312 | 0.030 | 0.060 | 44.6% |
| HINDZINC | ma_alignment | 284 | 0.041 | 0.082 | 44.4% |
| SBIN | ma_alignment | 203 | 0.056 | 0.112 | 44.3% |
| JUBLFOOD | ma_alignment | 280 | 0.082 | 0.164 | 44.3% |
| 3MINDIA | ma_alignment | 280 | 0.002 | 0.004 | 44.3% |
| ABBOTINDIA | ma_alignment | 226 | 0.006 | 0.012 | 44.2% |
| KPRMILL | ma_alignment | 312 | 0.029 | 0.057 | 44.2% |
| ASTRAL | ma_alignment | 251 | 0.075 | 0.149 | 44.2% |
| TCS | ma_alignment | 222 | 0.088 | 0.177 | 44.1% |
| BHARTIARTL | ma_alignment | 186 | 0.006 | 0.013 | 44.1% |
| EXIDEIND | ma_alignment | 304 | 0.071 | 0.142 | 44.1% |
| MFSL | ma_alignment | 276 | 0.062 | 0.124 | 43.8% |
| APARINDS | ma_alignment | 365 | 0.093 | 0.187 | 43.8% |
| LENSKART | ma_alignment | 308 | -0.005 | -0.010 | 43.8% |
| CRISIL | ma_alignment | 274 | -0.032 | -0.064 | 43.8% |
| GLAXO | ma_alignment | 281 | 0.024 | 0.048 | 43.8% |
| THERMAX | ma_alignment | 311 | 0.099 | 0.198 | 43.7% |
| BOSCHLTD | ma_alignment | 247 | 0.063 | 0.126 | 43.7% |
| ITC | ma_alignment | 167 | 0.020 | 0.040 | 43.7% |
| VMM | ma_alignment | 261 | 0.045 | 0.089 | 43.7% |
| DMART | ma_alignment | 229 | 0.056 | 0.112 | 43.7% |
| ACC | ma_alignment | 234 | -0.012 | -0.023 | 43.6% |
| DRREDDY | ma_alignment | 218 | 0.017 | 0.034 | 43.6% |
| HEXT | ma_alignment | 319 | 0.051 | 0.102 | 43.6% |
| SUNPHARMA | ma_alignment | 202 | -0.009 | -0.018 | 43.6% |
| GODREJCP | ma_alignment | 239 | -0.034 | -0.068 | 43.5% |
| DIVISLAB | ma_alignment | 214 | 0.063 | 0.125 | 43.5% |
| KOTAKBANK | ma_alignment | 214 | 0.015 | 0.030 | 43.5% |
| MAXHEALTH | ma_alignment | 242 | 0.054 | 0.108 | 43.4% |
| BSE | ma_alignment | 279 | 0.093 | 0.186 | 43.4% |
| PNB | ma_alignment | 217 | 0.034 | 0.068 | 43.3% |
| LICHSGFIN | ma_alignment | 248 | 0.072 | 0.144 | 43.1% |
| BANKINDIA | ma_alignment | 255 | 0.003 | 0.006 | 43.1% |
| ZYDUSLIFE | ma_alignment | 262 | 0.042 | 0.084 | 43.1% |
| TATACOMM | ma_alignment | 341 | 0.013 | 0.026 | 43.1% |
| LAURUSLABS | ma_alignment | 253 | 0.045 | 0.090 | 43.1% |
| 360ONE | ma_alignment | 265 | 0.058 | 0.115 | 43.0% |
| LINDEINDIA | ma_alignment | 284 | 0.035 | 0.070 | 43.0% |
| LICI | ma_alignment | 212 | 0.031 | 0.063 | 42.9% |
| PHOENIXLTD | ma_alignment | 282 | 0.025 | 0.049 | 42.9% |
| ITCHOTELS | ma_alignment | 259 | 0.055 | 0.110 | 42.9% |
| ICICIPRULI | ma_alignment | 266 | 0.049 | 0.099 | 42.9% |
| KALYANKJIL | ma_alignment | 341 | 0.045 | 0.090 | 42.8% |
| ADANIPOWER | ma_alignment | 304 | 0.053 | 0.105 | 42.8% |
| HAL | ma_alignment | 241 | 0.072 | 0.144 | 42.7% |
| TATASTEEL | ma_alignment | 234 | 0.023 | 0.046 | 42.7% |
| M&MFIN | ma_alignment | 337 | 0.053 | 0.106 | 42.7% |
| ADANIGREEN | ma_alignment | 323 | 0.083 | 0.165 | 42.7% |
| SBILIFE | ma_alignment | 216 | 0.031 | 0.062 | 42.6% |
| HINDUNILVR | ma_alignment | 202 | -0.019 | -0.038 | 42.6% |
| FEDERALBNK | ma_alignment | 233 | 0.017 | 0.033 | 42.5% |
| BANKBARODA | ma_alignment | 219 | 0.037 | 0.074 | 42.5% |
| ICICIAMC | ma_alignment | 285 | 0.036 | 0.072 | 42.5% |
| ASIANPAINT | ma_alignment | 205 | -0.016 | -0.032 | 42.4% |
| EICHERMOT | ma_alignment | 231 | -0.034 | -0.068 | 42.4% |
| ICICIBANK | ma_alignment | 191 | 0.001 | 0.002 | 42.4% |
| JKCEMENT | ma_alignment | 269 | 0.006 | 0.012 | 42.4% |
| BLUESTARCO | ma_alignment | 319 | -0.008 | -0.015 | 42.3% |
| HDBFS | ma_alignment | 272 | -0.042 | -0.084 | 42.3% |
| COLPAL | ma_alignment | 232 | 0.051 | 0.102 | 42.2% |
| TMPV | ma_alignment | 237 | 0.043 | 0.086 | 42.2% |
| ALKEM | ma_alignment | 237 | 0.024 | 0.049 | 42.2% |
| LGEINDIA | ma_alignment | 306 | -0.013 | -0.026 | 42.2% |
| COFORGE | ma_alignment | 304 | -0.000 | -0.001 | 42.1% |
| MUTHOOTFIN | ma_alignment | 259 | -0.012 | -0.024 | 42.1% |
| NHPC | ma_alignment | 221 | 0.034 | 0.067 | 42.1% |
| SHRIRAMFIN | ma_alignment | 252 | 0.085 | 0.170 | 42.1% |
| JIOFIN | ma_alignment | 219 | 0.030 | 0.060 | 42.0% |
| CANBK | ma_alignment | 241 | 0.056 | 0.112 | 41.9% |
| IPCALAB | ma_alignment | 315 | 0.026 | 0.053 | 41.9% |
| TITAN | ma_alignment | 210 | 0.020 | 0.039 | 41.9% |
| ASHOKLEY | ma_alignment | 284 | 0.052 | 0.104 | 41.9% |
