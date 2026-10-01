# rally-0.1pct — exit-rule research

Exit rule: risk_pct=0.500% (hard stop), armed at 3.0x risk (1.500%), give-back trail 0.500% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 250 instruments, 2026-03-27..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| strong_rally | 28084 | -0.000 | -0.000 | 35.2% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| HDBFS | strong_rally | 79 | 0.267 | 0.534 | 59.5% |
| HDFCBANK | strong_rally | 60 | 0.172 | 0.345 | 50.0% |
| RELIANCE | strong_rally | 54 | 0.131 | 0.262 | 50.0% |
| SBILIFE | strong_rally | 79 | 0.126 | 0.252 | 49.4% |
| WIPRO | strong_rally | 70 | 0.214 | 0.429 | 48.6% |
| ABBOTINDIA | strong_rally | 63 | 0.014 | 0.028 | 47.6% |
| NTPCGREEN | strong_rally | 78 | 0.068 | 0.136 | 46.2% |
| SUNPHARMA | strong_rally | 50 | 0.096 | 0.193 | 46.0% |
| CANBK | strong_rally | 99 | 0.154 | 0.309 | 45.5% |
| SBIN | strong_rally | 77 | 0.047 | 0.093 | 45.5% |
| BAJAJHFL | strong_rally | 69 | 0.051 | 0.102 | 44.9% |
| TORNTPHARM | strong_rally | 87 | 0.106 | 0.212 | 44.8% |
| BALKRISIND | strong_rally | 101 | 0.133 | 0.265 | 44.6% |
| DLF | strong_rally | 119 | 0.158 | 0.316 | 43.7% |
| ONGC | strong_rally | 71 | 0.142 | 0.284 | 43.7% |
| BHARTIARTL | strong_rally | 62 | 0.052 | 0.104 | 43.5% |
| LINDEINDIA | strong_rally | 108 | 0.113 | 0.227 | 43.5% |
| MUTHOOTFIN | strong_rally | 115 | 0.028 | 0.056 | 43.5% |
| ATGL | strong_rally | 165 | 0.185 | 0.369 | 43.0% |
| ADANIPOWER | strong_rally | 144 | 0.162 | 0.324 | 42.4% |
| INDIGO | strong_rally | 85 | 0.146 | 0.291 | 42.4% |
| BSE | strong_rally | 152 | 0.156 | 0.311 | 42.1% |
| TATAINVEST | strong_rally | 101 | 0.046 | 0.093 | 41.6% |
| ITCHOTELS | strong_rally | 111 | 0.058 | 0.116 | 41.4% |
| ICICIBANK | strong_rally | 58 | 0.008 | 0.015 | 41.4% |
| ESCORTS | strong_rally | 92 | 0.042 | 0.084 | 41.3% |
| SBICARD | strong_rally | 114 | 0.111 | 0.221 | 41.2% |
| IRFC | strong_rally | 78 | 0.008 | 0.017 | 41.0% |
| CRISIL | strong_rally | 100 | 0.025 | 0.051 | 41.0% |
| SUPREMEIND | strong_rally | 110 | 0.042 | 0.083 | 40.9% |
| BANKBARODA | strong_rally | 93 | 0.104 | 0.208 | 40.9% |
| BOSCHLTD | strong_rally | 98 | 0.133 | 0.266 | 40.8% |
| INFY | strong_rally | 86 | 0.075 | 0.151 | 40.7% |
| GLAXO | strong_rally | 96 | -0.017 | -0.035 | 40.6% |
| TATACOMM | strong_rally | 153 | 0.137 | 0.275 | 40.5% |
| TATACAP | strong_rally | 84 | 0.031 | 0.063 | 40.5% |
| TORNTPOWER | strong_rally | 109 | 0.021 | 0.042 | 40.4% |
| UPL | strong_rally | 100 | 0.073 | 0.146 | 40.0% |
| HINDUNILVR | strong_rally | 65 | 0.016 | 0.031 | 40.0% |
| ADANIENT | strong_rally | 143 | 0.127 | 0.254 | 39.9% |
| POLICYBZR | strong_rally | 138 | 0.050 | 0.100 | 39.9% |
| MAHABANK | strong_rally | 133 | 0.101 | 0.202 | 39.8% |
| INDHOTEL | strong_rally | 93 | -0.046 | -0.091 | 39.8% |
| JSWSTEEL | strong_rally | 88 | -0.036 | -0.072 | 39.8% |
| IOC | strong_rally | 68 | -0.022 | -0.044 | 39.7% |
| CGPOWER | strong_rally | 139 | 0.022 | 0.044 | 39.6% |
| IDFCFIRSTB | strong_rally | 86 | 0.123 | 0.245 | 39.5% |
| LICI | strong_rally | 81 | 0.022 | 0.044 | 39.5% |
| BERGEPAINT | strong_rally | 109 | 0.024 | 0.048 | 39.4% |
| HONAUT | strong_rally | 104 | 0.093 | 0.187 | 39.4% |
| POWERINDIA | strong_rally | 155 | 0.092 | 0.184 | 39.4% |
| TITAN | strong_rally | 84 | 0.133 | 0.266 | 39.3% |
| NTPC | strong_rally | 56 | -0.060 | -0.119 | 39.3% |
| APARINDS | strong_rally | 214 | 0.104 | 0.207 | 39.3% |
| BAJAJFINSV | strong_rally | 74 | 0.019 | 0.038 | 39.2% |
| LGEINDIA | strong_rally | 118 | -0.046 | -0.092 | 39.0% |
| BHEL | strong_rally | 162 | 0.136 | 0.272 | 38.9% |
| GODREJPROP | strong_rally | 157 | 0.029 | 0.059 | 38.9% |
| LTTS | strong_rally | 142 | 0.014 | 0.027 | 38.7% |
| TATACONSUM | strong_rally | 88 | 0.013 | 0.025 | 38.6% |
| HAL | strong_rally | 101 | 0.042 | 0.084 | 38.6% |
| GODREJCP | strong_rally | 96 | -0.025 | -0.050 | 38.5% |
| LICHSGFIN | strong_rally | 109 | 0.076 | 0.152 | 38.5% |
| ASTRAL | strong_rally | 122 | 0.054 | 0.109 | 38.5% |
| MEDANTA | strong_rally | 135 | 0.001 | 0.003 | 38.5% |
| BDL | strong_rally | 148 | 0.099 | 0.198 | 38.5% |
| IRCTC | strong_rally | 73 | 0.102 | 0.204 | 38.4% |
| TRENT | strong_rally | 120 | 0.031 | 0.063 | 38.3% |
| JKCEMENT | strong_rally | 107 | 0.005 | 0.010 | 38.3% |
| ICICIAMC | strong_rally | 110 | 0.039 | 0.078 | 38.2% |
| ADANIPORTS | strong_rally | 105 | 0.041 | 0.082 | 38.1% |
| THERMAX | strong_rally | 155 | -0.004 | -0.008 | 38.1% |
| LODHA | strong_rally | 177 | 0.157 | 0.314 | 37.9% |
| ASIANPAINT | strong_rally | 82 | -0.027 | -0.054 | 37.8% |
| APLAPOLLO | strong_rally | 114 | 0.093 | 0.186 | 37.7% |
| JIOFIN | strong_rally | 85 | 0.121 | 0.243 | 37.6% |
| GROWW | strong_rally | 165 | 0.092 | 0.184 | 37.6% |
| TMPV | strong_rally | 104 | 0.151 | 0.301 | 37.5% |
| JSL | strong_rally | 120 | 0.043 | 0.087 | 37.5% |
| POWERGRID | strong_rally | 64 | 0.007 | 0.015 | 37.5% |
| AJANTPHARM | strong_rally | 128 | -0.006 | -0.011 | 37.5% |
| LTM | strong_rally | 131 | 0.099 | 0.197 | 37.4% |
| MPHASIS | strong_rally | 131 | 0.070 | 0.139 | 37.4% |
| ZYDUSLIFE | strong_rally | 107 | 0.050 | 0.099 | 37.4% |
| FEDERALBNK | strong_rally | 91 | 0.028 | 0.056 | 37.4% |
| KPRMILL | strong_rally | 150 | -0.064 | -0.128 | 37.3% |
| CONCOR | strong_rally | 102 | 0.052 | 0.104 | 37.3% |
| BAJFINANCE | strong_rally | 102 | 0.020 | 0.039 | 37.3% |
| HINDZINC | strong_rally | 113 | 0.077 | 0.155 | 37.2% |
| HEXT | strong_rally | 140 | 0.091 | 0.182 | 37.1% |
| NMDC | strong_rally | 97 | 0.117 | 0.233 | 37.1% |
| VOLTAS | strong_rally | 135 | 0.096 | 0.193 | 37.0% |
| SUZLON | strong_rally | 135 | 0.061 | 0.123 | 37.0% |
| AXISBANK | strong_rally | 81 | -0.043 | -0.086 | 37.0% |
| ADANIGREEN | strong_rally | 173 | 0.124 | 0.247 | 37.0% |
| PFC | strong_rally | 122 | -0.048 | -0.096 | 36.9% |
| NIACL | strong_rally | 188 | 0.082 | 0.164 | 36.7% |
| ALKEM | strong_rally | 90 | 0.004 | 0.007 | 36.7% |
| VBL | strong_rally | 131 | -0.033 | -0.065 | 36.6% |
| JUBLFOOD | strong_rally | 123 | 0.053 | 0.106 | 36.6% |
