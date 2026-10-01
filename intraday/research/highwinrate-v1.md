# highwinrate-v1 — exit-rule research

Exit rule: risk_pct=0.500% (hard stop), armed at 3.0x risk (1.500%), give-back trail 0.500% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 250 instruments, 2026-03-27..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| momentum_breakout | 47746 | -0.001 | -0.003 | 38.9% |
| orb | 10151 | -0.052 | -0.105 | 29.2% |
| vwap_trend | 21154 | -0.005 | -0.009 | 34.2% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| DABUR | vwap_trend | 75 | 0.297 | 0.595 | 57.3% |
| IRCTC | orb | 37 | 0.244 | 0.488 | 56.8% |
| HDFCBANK | vwap_trend | 68 | 0.335 | 0.670 | 54.4% |
| BEL | momentum_breakout | 141 | 0.149 | 0.298 | 53.2% |
| SBIN | momentum_breakout | 136 | 0.176 | 0.352 | 51.5% |
| IRCTC | vwap_trend | 68 | 0.123 | 0.247 | 51.5% |
| LICHSGFIN | vwap_trend | 74 | 0.341 | 0.682 | 51.4% |
| COLPAL | vwap_trend | 78 | 0.207 | 0.413 | 51.3% |
| VEDL | orb | 34 | 0.378 | 0.756 | 50.0% |
| BAJAJHFL | vwap_trend | 86 | -0.003 | -0.006 | 48.8% |
| IOC | vwap_trend | 72 | 0.100 | 0.200 | 48.6% |
| HDFCBANK | momentum_breakout | 130 | 0.026 | 0.053 | 48.5% |
| GROWW | momentum_breakout | 224 | 0.075 | 0.149 | 47.8% |
| UNOMINDA | momentum_breakout | 197 | 0.093 | 0.186 | 47.2% |
| ONGC | momentum_breakout | 148 | 0.066 | 0.131 | 46.6% |
| PFC | orb | 41 | 0.370 | 0.740 | 46.3% |
| ALKEM | vwap_trend | 78 | 0.157 | 0.315 | 46.2% |
| COCHINSHIP | momentum_breakout | 206 | 0.173 | 0.347 | 46.1% |
| DIXON | momentum_breakout | 189 | 0.119 | 0.237 | 46.0% |
| SUNDARMFIN | vwap_trend | 87 | 0.177 | 0.354 | 46.0% |
| HDFCBANK | orb | 37 | 0.151 | 0.302 | 45.9% |
| APOLLOTYRE | vwap_trend | 85 | 0.066 | 0.132 | 45.9% |
| BHARTIARTL | momentum_breakout | 140 | -0.014 | -0.028 | 45.7% |
| HDBFS | vwap_trend | 90 | 0.048 | 0.096 | 45.6% |
| RECLTD | momentum_breakout | 167 | -0.016 | -0.032 | 45.5% |
| APLAPOLLO | orb | 33 | 0.341 | 0.681 | 45.5% |
| JIOFIN | momentum_breakout | 154 | 0.151 | 0.303 | 45.5% |
| APOLLOHOSP | vwap_trend | 73 | 0.000 | 0.001 | 45.2% |
| SUNPHARMA | momentum_breakout | 131 | 0.030 | 0.059 | 45.0% |
| COALINDIA | momentum_breakout | 147 | 0.105 | 0.210 | 44.9% |
| ICICIAMC | momentum_breakout | 239 | 0.088 | 0.176 | 44.8% |
| 3MINDIA | vwap_trend | 94 | 0.040 | 0.080 | 44.7% |
| TATACAP | momentum_breakout | 215 | 0.029 | 0.058 | 44.7% |
| ITC | momentum_breakout | 112 | 0.091 | 0.183 | 44.6% |
| GLAXO | momentum_breakout | 229 | 0.027 | 0.054 | 44.5% |
| NTPC | orb | 36 | 0.206 | 0.412 | 44.4% |
| FEDERALBNK | momentum_breakout | 162 | 0.100 | 0.200 | 44.4% |
| GICRE | vwap_trend | 104 | 0.127 | 0.254 | 44.2% |
| PNB | momentum_breakout | 147 | 0.008 | 0.016 | 44.2% |
| COFORGE | orb | 34 | 0.360 | 0.721 | 44.1% |
| PRESTIGE | vwap_trend | 93 | 0.231 | 0.462 | 44.1% |
| ABB | momentum_breakout | 184 | 0.079 | 0.158 | 44.0% |
| ACC | vwap_trend | 91 | 0.004 | 0.007 | 44.0% |
| BRITANNIA | momentum_breakout | 148 | -0.015 | -0.030 | 43.9% |
| INDUSINDBK | momentum_breakout | 164 | 0.064 | 0.128 | 43.9% |
| NESTLEIND | momentum_breakout | 164 | -0.014 | -0.029 | 43.9% |
| PFC | momentum_breakout | 155 | 0.119 | 0.239 | 43.9% |
| UNIONBANK | momentum_breakout | 176 | 0.095 | 0.189 | 43.8% |
| GROWW | vwap_trend | 87 | 0.269 | 0.538 | 43.7% |
| AWL | vwap_trend | 87 | 0.019 | 0.037 | 43.7% |
| MARUTI | momentum_breakout | 142 | 0.023 | 0.046 | 43.7% |
| TITAN | vwap_trend | 78 | 0.110 | 0.221 | 43.6% |
| MARICO | momentum_breakout | 161 | 0.041 | 0.082 | 43.5% |
| NTPC | vwap_trend | 69 | 0.028 | 0.057 | 43.5% |
| JSWSTEEL | momentum_breakout | 161 | 0.008 | 0.016 | 43.5% |
| IRCTC | momentum_breakout | 168 | -0.001 | -0.002 | 43.5% |
| ASTRAL | momentum_breakout | 182 | 0.070 | 0.140 | 43.4% |
| GICRE | momentum_breakout | 212 | 0.035 | 0.069 | 43.4% |
| TMPV | momentum_breakout | 171 | 0.079 | 0.157 | 43.3% |
| OFSS | momentum_breakout | 185 | 0.086 | 0.172 | 43.2% |
| SJVN | vwap_trend | 81 | -0.025 | -0.050 | 43.2% |
| CANBK | orb | 44 | 0.206 | 0.413 | 43.2% |
| INFY | momentum_breakout | 139 | 0.032 | 0.064 | 43.2% |
| THERMAX | momentum_breakout | 218 | 0.063 | 0.125 | 43.1% |
| DLF | momentum_breakout | 174 | 0.068 | 0.136 | 43.1% |
| AWL | momentum_breakout | 216 | -0.008 | -0.016 | 43.1% |
| MPHASIS | momentum_breakout | 193 | 0.143 | 0.286 | 43.0% |
| HCLTECH | vwap_trend | 84 | 0.145 | 0.290 | 42.9% |
| CRISIL | vwap_trend | 91 | 0.063 | 0.126 | 42.9% |
| HCLTECH | momentum_breakout | 154 | 0.010 | 0.020 | 42.9% |
| TATACONSUM | momentum_breakout | 161 | -0.015 | -0.029 | 42.9% |
| COROMANDEL | momentum_breakout | 236 | -0.013 | -0.026 | 42.8% |
| VMM | momentum_breakout | 180 | 0.029 | 0.058 | 42.8% |
| TCS | momentum_breakout | 152 | 0.044 | 0.088 | 42.8% |
| OBEROIRLTY | momentum_breakout | 192 | 0.108 | 0.216 | 42.7% |
| HAL | momentum_breakout | 178 | 0.111 | 0.222 | 42.7% |
| ADANIENT | momentum_breakout | 178 | 0.089 | 0.178 | 42.7% |
| GVT&D | vwap_trend | 82 | 0.375 | 0.750 | 42.7% |
| IDFCFIRSTB | momentum_breakout | 148 | 0.007 | 0.014 | 42.6% |
| BHARATFORG | momentum_breakout | 188 | 0.088 | 0.176 | 42.6% |
| WIPRO | momentum_breakout | 139 | -0.004 | -0.008 | 42.4% |
| ABCAPITAL | momentum_breakout | 172 | 0.088 | 0.175 | 42.4% |
| RELIANCE | momentum_breakout | 132 | 0.029 | 0.058 | 42.4% |
| LGEINDIA | vwap_trend | 85 | 0.108 | 0.215 | 42.4% |
| NMDC | momentum_breakout | 170 | 0.044 | 0.088 | 42.4% |
| BSE | momentum_breakout | 185 | 0.068 | 0.135 | 42.2% |
| SHRIRAMFIN | orb | 38 | 0.262 | 0.525 | 42.1% |
| POLYCAB | orb | 38 | 0.099 | 0.197 | 42.1% |
| ICICIBANK | orb | 38 | 0.073 | 0.146 | 42.1% |
| ONGC | vwap_trend | 76 | 0.049 | 0.099 | 42.1% |
| SHRIRAMFIN | momentum_breakout | 164 | 0.078 | 0.155 | 42.1% |
| APOLLOHOSP | momentum_breakout | 145 | -0.003 | -0.005 | 42.1% |
| CUMMINSIND | momentum_breakout | 174 | 0.050 | 0.101 | 42.0% |
| HDBFS | momentum_breakout | 203 | 0.070 | 0.141 | 41.9% |
| TCS | orb | 43 | 0.260 | 0.521 | 41.9% |
| ANTHEM | momentum_breakout | 227 | -0.023 | -0.047 | 41.9% |
| INDIGO | momentum_breakout | 165 | 0.075 | 0.149 | 41.8% |
| TORNTPOWER | momentum_breakout | 220 | 0.009 | 0.017 | 41.8% |
| ADANIPOWER | momentum_breakout | 189 | 0.097 | 0.194 | 41.8% |
| ESCORTS | vwap_trend | 79 | 0.062 | 0.124 | 41.8% |
