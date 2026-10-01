# vwap-trend-filtered — exit-rule research

Exit rule: risk_pct=0.500% (hard stop), armed at 3.0x risk (1.500%), give-back trail 0.500% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 250 instruments, 2026-03-27..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| momentum_breakout | 47746 | -0.001 | -0.003 | 38.9% |
| orb | 10151 | -0.052 | -0.105 | 29.2% |
| trend_pullback | 57358 | 0.002 | 0.005 | 39.6% |
| vwap_reversion | 31282 | -0.009 | -0.019 | 33.8% |
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
| M&M | trend_pullback | 178 | 0.053 | 0.106 | 48.9% |
| BAJAJHFL | vwap_trend | 86 | -0.003 | -0.006 | 48.8% |
| WIPRO | trend_pullback | 162 | 0.069 | 0.137 | 48.8% |
| IOC | vwap_trend | 72 | 0.100 | 0.200 | 48.6% |
| HDFCBANK | momentum_breakout | 130 | 0.026 | 0.053 | 48.5% |
| HEROMOTOCO | trend_pullback | 211 | 0.084 | 0.168 | 48.3% |
| BHEL | vwap_reversion | 129 | 0.391 | 0.782 | 48.1% |
| GROWW | momentum_breakout | 224 | 0.075 | 0.149 | 47.8% |
| INDIGO | trend_pullback | 191 | 0.118 | 0.236 | 47.6% |
| SUNPHARMA | trend_pullback | 167 | 0.004 | 0.008 | 47.3% |
| UNOMINDA | momentum_breakout | 197 | 0.093 | 0.186 | 47.2% |
| ENDURANCE | vwap_reversion | 125 | 0.203 | 0.406 | 47.2% |
| ACC | vwap_reversion | 123 | 0.080 | 0.159 | 47.2% |
| ONGC | momentum_breakout | 148 | 0.066 | 0.131 | 46.6% |
| UPL | trend_pullback | 209 | 0.109 | 0.218 | 46.4% |
| PFC | orb | 41 | 0.370 | 0.740 | 46.3% |
| ALKEM | vwap_trend | 78 | 0.157 | 0.315 | 46.2% |
| COCHINSHIP | momentum_breakout | 206 | 0.173 | 0.347 | 46.1% |
| MANKIND | trend_pullback | 206 | 0.073 | 0.146 | 46.1% |
| DIXON | momentum_breakout | 189 | 0.119 | 0.237 | 46.0% |
| SUNDARMFIN | vwap_trend | 87 | 0.177 | 0.354 | 46.0% |
| HDFCBANK | orb | 37 | 0.151 | 0.302 | 45.9% |
| APOLLOTYRE | vwap_trend | 85 | 0.066 | 0.132 | 45.9% |
| MPHASIS | trend_pullback | 238 | 0.062 | 0.124 | 45.8% |
| BHARTIARTL | momentum_breakout | 140 | -0.014 | -0.028 | 45.7% |
| HDFCBANK | trend_pullback | 169 | 0.109 | 0.219 | 45.6% |
| HDBFS | vwap_trend | 90 | 0.048 | 0.096 | 45.6% |
| RECLTD | momentum_breakout | 167 | -0.016 | -0.032 | 45.5% |
| APLAPOLLO | orb | 33 | 0.341 | 0.681 | 45.5% |
| JIOFIN | momentum_breakout | 154 | 0.151 | 0.303 | 45.5% |
| NMDC | trend_pullback | 207 | 0.051 | 0.101 | 45.4% |
| BEL | trend_pullback | 172 | 0.086 | 0.172 | 45.3% |
| TVSMOTOR | trend_pullback | 201 | 0.100 | 0.199 | 45.3% |
| SAIL | trend_pullback | 252 | 0.101 | 0.202 | 45.2% |
| APOLLOHOSP | vwap_trend | 73 | 0.000 | 0.001 | 45.2% |
| JIOFIN | trend_pullback | 195 | 0.121 | 0.241 | 45.1% |
| SUNPHARMA | momentum_breakout | 131 | 0.030 | 0.059 | 45.0% |
| FEDERALBNK | trend_pullback | 191 | 0.060 | 0.121 | 45.0% |
| TATAELXSI | trend_pullback | 209 | 0.067 | 0.134 | 45.0% |
| BAJAJHFL | vwap_reversion | 109 | 0.026 | 0.052 | 45.0% |
| COALINDIA | momentum_breakout | 147 | 0.105 | 0.210 | 44.9% |
| ICICIAMC | momentum_breakout | 239 | 0.088 | 0.176 | 44.8% |
| ICICIAMC | trend_pullback | 257 | 0.114 | 0.227 | 44.7% |
| HINDALCO | trend_pullback | 199 | -0.009 | -0.019 | 44.7% |
| PFC | trend_pullback | 188 | 0.092 | 0.185 | 44.7% |
| 3MINDIA | vwap_trend | 94 | 0.040 | 0.080 | 44.7% |
| TATACAP | momentum_breakout | 215 | 0.029 | 0.058 | 44.7% |
| ITC | momentum_breakout | 112 | 0.091 | 0.183 | 44.6% |
| HINDALCO | vwap_reversion | 121 | 0.081 | 0.162 | 44.6% |
| BHARTIARTL | trend_pullback | 157 | 0.010 | 0.020 | 44.6% |
| UNITDSPR | trend_pullback | 193 | -0.007 | -0.014 | 44.6% |
| GLAXO | momentum_breakout | 229 | 0.027 | 0.054 | 44.5% |
| NTPC | orb | 36 | 0.206 | 0.412 | 44.4% |
| FEDERALBNK | momentum_breakout | 162 | 0.100 | 0.200 | 44.4% |
| HDFCLIFE | trend_pullback | 205 | 0.081 | 0.163 | 44.4% |
| JUBLFOOD | trend_pullback | 246 | 0.080 | 0.161 | 44.3% |
| ENRIN | trend_pullback | 289 | 0.056 | 0.111 | 44.3% |
| THERMAX | trend_pullback | 253 | 0.053 | 0.105 | 44.3% |
| GICRE | vwap_trend | 104 | 0.127 | 0.254 | 44.2% |
| PNB | momentum_breakout | 147 | 0.008 | 0.016 | 44.2% |
| SRF | trend_pullback | 190 | 0.075 | 0.151 | 44.2% |
| COALINDIA | trend_pullback | 179 | 0.040 | 0.079 | 44.1% |
| LT | trend_pullback | 179 | 0.007 | 0.014 | 44.1% |
| BAJAJFINSV | trend_pullback | 213 | 0.004 | 0.008 | 44.1% |
| COFORGE | orb | 34 | 0.360 | 0.721 | 44.1% |
| KALYANKJIL | trend_pullback | 263 | 0.080 | 0.159 | 44.1% |
| ADANIGREEN | trend_pullback | 254 | 0.140 | 0.280 | 44.1% |
| PRESTIGE | vwap_trend | 93 | 0.231 | 0.462 | 44.1% |
| TATACAP | vwap_reversion | 118 | 0.053 | 0.106 | 44.1% |
| ABB | momentum_breakout | 184 | 0.079 | 0.158 | 44.0% |
| RELIANCE | trend_pullback | 166 | 0.061 | 0.122 | 44.0% |
| ACC | vwap_trend | 91 | 0.004 | 0.007 | 44.0% |
| BRITANNIA | momentum_breakout | 148 | -0.015 | -0.030 | 43.9% |
| INDUSINDBK | momentum_breakout | 164 | 0.064 | 0.128 | 43.9% |
| NESTLEIND | momentum_breakout | 164 | -0.014 | -0.029 | 43.9% |
| PFC | momentum_breakout | 155 | 0.119 | 0.239 | 43.9% |
| MAZDOCK | trend_pullback | 217 | 0.056 | 0.112 | 43.8% |
| UNIONBANK | momentum_breakout | 176 | 0.095 | 0.189 | 43.8% |
| SBILIFE | trend_pullback | 199 | 0.022 | 0.043 | 43.7% |
| TCS | trend_pullback | 190 | 0.027 | 0.055 | 43.7% |
| GROWW | vwap_trend | 87 | 0.269 | 0.538 | 43.7% |
| AWL | vwap_trend | 87 | 0.019 | 0.037 | 43.7% |
| MARUTI | momentum_breakout | 142 | 0.023 | 0.046 | 43.7% |
| TITAN | vwap_trend | 78 | 0.110 | 0.221 | 43.6% |
| HDBFS | trend_pullback | 241 | 0.021 | 0.042 | 43.6% |
| ITCHOTELS | trend_pullback | 264 | 0.013 | 0.025 | 43.6% |
| LUPIN | trend_pullback | 200 | -0.010 | -0.020 | 43.5% |
| ABBOTINDIA | vwap_reversion | 115 | 0.058 | 0.117 | 43.5% |
| MARICO | momentum_breakout | 161 | 0.041 | 0.082 | 43.5% |
| NTPC | vwap_trend | 69 | 0.028 | 0.057 | 43.5% |
| JSWSTEEL | momentum_breakout | 161 | 0.008 | 0.016 | 43.5% |
