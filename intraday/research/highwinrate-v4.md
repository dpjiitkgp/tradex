# highwinrate-v4 — exit-rule research

Exit rule: risk_pct=0.500% (hard stop), armed at 3.0x risk (1.500%), give-back trail 0.500% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 250 instruments, 2026-03-27..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| momentum_breakout | 47746 | -0.001 | -0.003 | 38.9% |
| orb | 10151 | -0.052 | -0.105 | 29.2% |
| strong_rally | 8083 | -0.017 | -0.034 | 32.8% |
| trend_pullback | 57358 | 0.002 | 0.005 | 39.6% |
| vwap_reversion | 31282 | -0.009 | -0.019 | 33.8% |
| vwap_trend | 21154 | -0.005 | -0.009 | 34.2% |
| vwap_trend | 8711 | 0.019 | 0.038 | 37.4% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| COALINDIA | vwap_trend | 27 | 0.125 | 0.249 | 66.7% |
| ZYDUSLIFE | vwap_trend | 26 | 0.267 | 0.534 | 65.4% |
| ACC | strong_rally | 11 | 0.496 | 0.993 | 63.6% |
| ATGL | vwap_trend | 42 | 0.292 | 0.584 | 61.9% |
| BAJAJHFL | vwap_trend | 39 | 0.076 | 0.151 | 61.5% |
| INDIGO | strong_rally | 22 | 0.323 | 0.645 | 59.1% |
| NMDC | vwap_trend | 39 | 0.490 | 0.980 | 59.0% |
| IDFCFIRSTB | strong_rally | 17 | 0.342 | 0.683 | 58.8% |
| NTPCGREEN | strong_rally | 24 | 0.381 | 0.762 | 58.3% |
| LINDEINDIA | vwap_trend | 36 | 0.287 | 0.574 | 58.3% |
| TATACAP | strong_rally | 26 | 0.143 | 0.287 | 57.7% |
| DABUR | vwap_trend | 75 | 0.297 | 0.595 | 57.3% |
| HDFCBANK | strong_rally | 7 | 0.099 | 0.199 | 57.1% |
| IRCTC | orb | 37 | 0.244 | 0.488 | 56.8% |
| ITCHOTELS | strong_rally | 23 | 0.254 | 0.508 | 56.5% |
| SJVN | vwap_trend | 41 | 0.060 | 0.120 | 56.1% |
| ITC | vwap_trend | 25 | 0.270 | 0.539 | 56.0% |
| RELIANCE | strong_rally | 9 | 0.377 | 0.754 | 55.6% |
| LICHSGFIN | vwap_trend | 27 | 0.294 | 0.588 | 55.6% |
| JIOFIN | vwap_trend | 36 | 0.148 | 0.295 | 55.6% |
| IOC | strong_rally | 9 | 0.044 | 0.088 | 55.6% |
| ICICIBANK | strong_rally | 9 | -0.021 | -0.043 | 55.6% |
| BERGEPAINT | strong_rally | 29 | 0.214 | 0.428 | 55.2% |
| DLF | strong_rally | 31 | 0.557 | 1.115 | 54.8% |
| TATACONSUM | vwap_trend | 31 | 0.299 | 0.599 | 54.8% |
| SBILIFE | strong_rally | 11 | 0.293 | 0.586 | 54.5% |
| HDFCBANK | vwap_trend | 68 | 0.335 | 0.670 | 54.4% |
| INFY | vwap_trend | 35 | 0.224 | 0.449 | 54.3% |
| IPCALAB | vwap_trend | 35 | 0.180 | 0.359 | 54.3% |
| SCHAEFFLER | vwap_trend | 35 | 0.111 | 0.223 | 54.3% |
| APOLLOTYRE | vwap_trend | 37 | 0.135 | 0.269 | 54.1% |
| ITCHOTELS | vwap_trend | 41 | 0.445 | 0.890 | 53.7% |
| TATAINVEST | vwap_trend | 43 | -0.016 | -0.033 | 53.5% |
| ALKEM | vwap_trend | 30 | 0.141 | 0.282 | 53.3% |
| BEL | momentum_breakout | 141 | 0.149 | 0.298 | 53.2% |
| BIOCON | vwap_trend | 32 | 0.318 | 0.636 | 53.1% |
| DABUR | vwap_trend | 34 | 0.327 | 0.655 | 52.9% |
| IRCTC | strong_rally | 19 | 0.342 | 0.685 | 52.6% |
| TMPV | vwap_trend | 25 | 0.340 | 0.679 | 52.0% |
| LGEINDIA | strong_rally | 25 | 0.301 | 0.603 | 52.0% |
| HDBFS | strong_rally | 25 | 0.095 | 0.191 | 52.0% |
| ACC | vwap_trend | 50 | 0.051 | 0.101 | 52.0% |
| SHREECEM | vwap_trend | 29 | 0.178 | 0.356 | 51.7% |
| ENRIN | vwap_trend | 33 | 0.253 | 0.505 | 51.5% |
| SBIN | momentum_breakout | 136 | 0.176 | 0.352 | 51.5% |
| IRCTC | vwap_trend | 68 | 0.123 | 0.247 | 51.5% |
| LICHSGFIN | vwap_trend | 74 | 0.341 | 0.682 | 51.4% |
| COLPAL | vwap_trend | 78 | 0.207 | 0.413 | 51.3% |
| VEDL | orb | 34 | 0.378 | 0.756 | 50.0% |
| HONAUT | strong_rally | 30 | 0.246 | 0.492 | 50.0% |
| THERMAX | vwap_trend | 36 | 0.243 | 0.486 | 50.0% |
| JSL | strong_rally | 38 | 0.242 | 0.484 | 50.0% |
| TORNTPHARM | strong_rally | 24 | 0.214 | 0.428 | 50.0% |
| JKCEMENT | vwap_trend | 34 | 0.179 | 0.358 | 50.0% |
| KOTAKBANK | strong_rally | 12 | 0.148 | 0.297 | 50.0% |
| TRENT | vwap_trend | 18 | 0.120 | 0.240 | 50.0% |
| IOC | vwap_trend | 40 | 0.056 | 0.113 | 50.0% |
| HUDCO | vwap_trend | 30 | 0.052 | 0.103 | 50.0% |
| IRCTC | vwap_trend | 30 | 0.048 | 0.096 | 50.0% |
| M&M | trend_pullback | 178 | 0.053 | 0.106 | 48.9% |
| BAJAJHFL | vwap_trend | 86 | -0.003 | -0.006 | 48.8% |
| WIPRO | trend_pullback | 162 | 0.069 | 0.137 | 48.8% |
| TATACOMM | vwap_trend | 39 | 0.325 | 0.651 | 48.7% |
| VOLTAS | vwap_trend | 37 | 0.250 | 0.500 | 48.6% |
| LUPIN | vwap_trend | 37 | 0.168 | 0.336 | 48.6% |
| AWL | vwap_trend | 37 | -0.002 | -0.003 | 48.6% |
| IOC | vwap_trend | 72 | 0.100 | 0.200 | 48.6% |
| BALKRISIND | vwap_trend | 33 | 0.270 | 0.541 | 48.5% |
| HAVELLS | vwap_trend | 33 | 0.097 | 0.193 | 48.5% |
| HDFCBANK | momentum_breakout | 130 | 0.026 | 0.053 | 48.5% |
| WIPRO | vwap_trend | 31 | 0.228 | 0.456 | 48.4% |
| HEROMOTOCO | trend_pullback | 211 | 0.084 | 0.168 | 48.3% |
| 3MINDIA | vwap_trend | 52 | 0.003 | 0.005 | 48.1% |
| BHEL | vwap_reversion | 129 | 0.391 | 0.782 | 48.1% |
| BOSCHLTD | strong_rally | 25 | 0.482 | 0.965 | 48.0% |
| GROWW | momentum_breakout | 224 | 0.075 | 0.149 | 47.8% |
| INDIGO | trend_pullback | 191 | 0.118 | 0.236 | 47.6% |
| INDIANB | vwap_trend | 38 | 0.258 | 0.517 | 47.4% |
| HDBFS | vwap_trend | 38 | 0.057 | 0.114 | 47.4% |
| SUNPHARMA | trend_pullback | 167 | 0.004 | 0.008 | 47.3% |
| UNOMINDA | momentum_breakout | 197 | 0.093 | 0.186 | 47.2% |
| ENDURANCE | vwap_reversion | 125 | 0.203 | 0.406 | 47.2% |
| ACC | vwap_reversion | 123 | 0.080 | 0.159 | 47.2% |
| ASIANPAINT | vwap_trend | 34 | 0.268 | 0.536 | 47.1% |
| SBIN | strong_rally | 17 | 0.067 | 0.134 | 47.1% |
| SBICARD | strong_rally | 30 | 0.382 | 0.764 | 46.7% |
| MARUTI | strong_rally | 15 | 0.275 | 0.550 | 46.7% |
| ONGC | momentum_breakout | 148 | 0.066 | 0.131 | 46.6% |
| UPL | trend_pullback | 209 | 0.109 | 0.218 | 46.4% |
| PFC | orb | 41 | 0.370 | 0.740 | 46.3% |
| TATAELXSI | vwap_trend | 41 | 0.235 | 0.471 | 46.3% |
| JIOFIN | strong_rally | 26 | 0.224 | 0.449 | 46.2% |
| ALKEM | vwap_trend | 78 | 0.157 | 0.315 | 46.2% |
| COCHINSHIP | momentum_breakout | 206 | 0.173 | 0.347 | 46.1% |
| MANKIND | trend_pullback | 206 | 0.073 | 0.146 | 46.1% |
| DIXON | momentum_breakout | 189 | 0.119 | 0.237 | 46.0% |
| SUNDARMFIN | vwap_trend | 87 | 0.177 | 0.354 | 46.0% |
| HDFCBANK | orb | 37 | 0.151 | 0.302 | 45.9% |
| TITAN | vwap_trend | 37 | 0.042 | 0.083 | 45.9% |
| APOLLOTYRE | vwap_trend | 85 | 0.066 | 0.132 | 45.9% |
