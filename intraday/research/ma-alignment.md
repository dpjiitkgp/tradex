# ma-alignment — exit-rule research

Exit rule: risk_pct=0.500% (hard stop), armed at 3.0x risk (1.500%), give-back trail 0.500% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 250 instruments, 2026-03-27..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| ma_alignment | 59980 | 0.015 | 0.030 | 38.2% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| BAJAJHFL | ma_alignment | 187 | 0.081 | 0.162 | 49.7% |
| IRCTC | ma_alignment | 185 | 0.095 | 0.190 | 49.2% |
| ICICIGI | ma_alignment | 203 | 0.080 | 0.160 | 48.8% |
| RELIANCE | ma_alignment | 163 | 0.138 | 0.275 | 47.9% |
| DABUR | ma_alignment | 186 | 0.077 | 0.154 | 45.7% |
| IOC | ma_alignment | 185 | 0.071 | 0.141 | 45.4% |
| WIPRO | ma_alignment | 184 | 0.042 | 0.084 | 45.1% |
| BERGEPAINT | ma_alignment | 254 | 0.090 | 0.180 | 44.9% |
| HDFCBANK | ma_alignment | 175 | 0.110 | 0.221 | 44.6% |
| ONGC | ma_alignment | 180 | 0.031 | 0.061 | 44.4% |
| ATGL | ma_alignment | 286 | 0.141 | 0.281 | 44.4% |
| GICRE | ma_alignment | 219 | 0.066 | 0.131 | 44.3% |
| BHARTIARTL | ma_alignment | 165 | 0.030 | 0.061 | 44.2% |
| HCLTECH | ma_alignment | 207 | 0.128 | 0.257 | 44.0% |
| ABBOTINDIA | ma_alignment | 221 | 0.023 | 0.045 | 43.9% |
| UNITDSPR | ma_alignment | 187 | 0.085 | 0.170 | 43.9% |
| APOLLOHOSP | ma_alignment | 169 | -0.019 | -0.038 | 43.8% |
| ITC | ma_alignment | 158 | 0.035 | 0.069 | 43.7% |
| RVNL | ma_alignment | 239 | 0.054 | 0.107 | 43.5% |
| NESTLEIND | ma_alignment | 207 | 0.018 | 0.035 | 43.5% |
| DRREDDY | ma_alignment | 191 | 0.063 | 0.127 | 43.5% |
| KOTAKBANK | ma_alignment | 196 | 0.028 | 0.055 | 43.4% |
| ACC | ma_alignment | 226 | 0.002 | 0.004 | 43.4% |
| HINDUNILVR | ma_alignment | 186 | 0.012 | 0.025 | 43.0% |
| SBIN | ma_alignment | 185 | 0.068 | 0.135 | 42.7% |
| DIVISLAB | ma_alignment | 197 | 0.038 | 0.076 | 42.6% |
| THERMAX | ma_alignment | 291 | 0.099 | 0.199 | 42.6% |
| ICICIBANK | ma_alignment | 181 | 0.021 | 0.042 | 42.5% |
| NLCINDIA | ma_alignment | 294 | 0.055 | 0.111 | 42.5% |
| DLF | ma_alignment | 219 | 0.161 | 0.322 | 42.5% |
| HEXT | ma_alignment | 285 | 0.080 | 0.160 | 42.5% |
| AXISBANK | ma_alignment | 191 | 0.020 | 0.039 | 42.4% |
| GLAXO | ma_alignment | 255 | 0.027 | 0.054 | 42.4% |
| HDBFS | ma_alignment | 253 | 0.014 | 0.028 | 42.3% |
| BSE | ma_alignment | 246 | 0.150 | 0.300 | 42.3% |
| JKCEMENT | ma_alignment | 251 | 0.031 | 0.063 | 42.2% |
| APOLLOTYRE | ma_alignment | 244 | -0.010 | -0.020 | 42.2% |
| ITCHOTELS | ma_alignment | 240 | 0.070 | 0.139 | 42.1% |
| BOSCHLTD | ma_alignment | 226 | 0.098 | 0.195 | 42.0% |
| BEL | ma_alignment | 181 | 0.073 | 0.145 | 42.0% |
| COLPAL | ma_alignment | 212 | 0.085 | 0.170 | 42.0% |
| PETRONET | ma_alignment | 210 | 0.013 | 0.027 | 41.9% |
| VBL | ma_alignment | 246 | 0.053 | 0.106 | 41.9% |
| LAURUSLABS | ma_alignment | 232 | 0.051 | 0.102 | 41.8% |
| TCS | ma_alignment | 201 | 0.105 | 0.209 | 41.8% |
| LUPIN | ma_alignment | 201 | -0.048 | -0.096 | 41.8% |
| MAXHEALTH | ma_alignment | 211 | 0.083 | 0.166 | 41.7% |
| HONAUT | ma_alignment | 247 | 0.092 | 0.184 | 41.7% |
| 3MINDIA | ma_alignment | 252 | 0.016 | 0.032 | 41.7% |
| CRISIL | ma_alignment | 250 | 0.005 | 0.010 | 41.6% |
| APARINDS | ma_alignment | 327 | 0.115 | 0.231 | 41.6% |
| KPRMILL | ma_alignment | 277 | 0.032 | 0.065 | 41.5% |
| HAL | ma_alignment | 210 | 0.078 | 0.157 | 41.4% |
| LICHSGFIN | ma_alignment | 221 | 0.086 | 0.171 | 41.2% |
| APLAPOLLO | ma_alignment | 229 | 0.112 | 0.224 | 41.0% |
| GODREJCP | ma_alignment | 217 | -0.045 | -0.089 | 41.0% |
| BAJAJFINSV | ma_alignment | 193 | 0.057 | 0.113 | 40.9% |
| NHPC | ma_alignment | 198 | 0.040 | 0.081 | 40.9% |
| HDFCLIFE | ma_alignment | 203 | 0.073 | 0.147 | 40.9% |
| ENRIN | ma_alignment | 301 | 0.082 | 0.164 | 40.9% |
| HEROMOTOCO | ma_alignment | 216 | 0.095 | 0.190 | 40.7% |
| SUNPHARMA | ma_alignment | 189 | -0.015 | -0.029 | 40.7% |
| LINDEINDIA | ma_alignment | 253 | 0.044 | 0.087 | 40.7% |
| NMDC | ma_alignment | 222 | 0.108 | 0.216 | 40.5% |
| VMM | ma_alignment | 222 | 0.048 | 0.096 | 40.5% |
| ASIANPAINT | ma_alignment | 190 | -0.031 | -0.063 | 40.5% |
| TORNTPOWER | ma_alignment | 269 | 0.030 | 0.059 | 40.5% |
| ESCORTS | ma_alignment | 232 | -0.004 | -0.008 | 40.5% |
| MAHABANK | ma_alignment | 262 | 0.019 | 0.038 | 40.5% |
| PIIND | ma_alignment | 235 | 0.068 | 0.135 | 40.4% |
| ZYDUSLIFE | ma_alignment | 235 | 0.040 | 0.080 | 40.4% |
| LGEINDIA | ma_alignment | 282 | 0.007 | 0.014 | 40.4% |
| IDFCFIRSTB | ma_alignment | 208 | -0.007 | -0.014 | 40.4% |
| ASTRAL | ma_alignment | 218 | 0.097 | 0.194 | 40.4% |
| MFSL | ma_alignment | 238 | 0.056 | 0.112 | 40.3% |
| TATACAP | ma_alignment | 244 | 0.024 | 0.047 | 40.2% |
| EXIDEIND | ma_alignment | 284 | 0.062 | 0.124 | 40.1% |
| INFY | ma_alignment | 192 | -0.026 | -0.052 | 40.1% |
| ABB | ma_alignment | 257 | 0.046 | 0.092 | 40.1% |
| LENSKART | ma_alignment | 268 | -0.016 | -0.032 | 39.9% |
| TITAN | ma_alignment | 198 | 0.014 | 0.028 | 39.9% |
| HAVELLS | ma_alignment | 211 | 0.009 | 0.018 | 39.8% |
| LTTS | ma_alignment | 282 | -0.002 | -0.005 | 39.7% |
| SHRIRAMFIN | ma_alignment | 232 | 0.088 | 0.176 | 39.7% |
| TMPV | ma_alignment | 217 | 0.059 | 0.118 | 39.6% |
| ICICIAMC | ma_alignment | 270 | 0.057 | 0.113 | 39.6% |
| DIXON | ma_alignment | 265 | 0.072 | 0.144 | 39.6% |
| RADICO | ma_alignment | 220 | 0.040 | 0.081 | 39.5% |
| FEDERALBNK | ma_alignment | 220 | -0.000 | -0.001 | 39.5% |
| M&MFIN | ma_alignment | 304 | 0.066 | 0.131 | 39.5% |
| TATAINVEST | ma_alignment | 256 | -0.058 | -0.115 | 39.5% |
| DMART | ma_alignment | 213 | 0.013 | 0.026 | 39.4% |
| BANKBARODA | ma_alignment | 198 | 0.061 | 0.122 | 39.4% |
| UBL | ma_alignment | 231 | -0.026 | -0.052 | 39.4% |
| BAJAJ-AUTO | ma_alignment | 193 | -0.020 | -0.040 | 39.4% |
| ADANIPOWER | ma_alignment | 272 | 0.065 | 0.130 | 39.3% |
| NTPCGREEN | ma_alignment | 239 | -0.006 | -0.012 | 39.3% |
| PHOENIXLTD | ma_alignment | 262 | 0.017 | 0.033 | 39.3% |
| SUPREMEIND | ma_alignment | 247 | 0.038 | 0.076 | 39.3% |
| JINDALSTEL | ma_alignment | 219 | 0.014 | 0.028 | 39.3% |
