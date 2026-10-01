# vwap-trend-improved/vol5x-morning — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 3.0x risk (2.250%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 250 instruments, 2021-03-23..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_late_vol5x | 16738 | 0.030 | 0.040 | 45.5% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| PHOENIXLTD | vwap_trend_short_late_vol5x | 5 | 1.147 | 1.529 | 80.0% |
| JIOFIN | vwap_trend_short_late_vol5x | 32 | 0.317 | 0.423 | 78.1% |
| THERMAX | vwap_trend_short_late_vol5x | 13 | 0.440 | 0.587 | 76.9% |
| TATACONSUM | vwap_trend_short_late_vol5x | 8 | 0.590 | 0.786 | 75.0% |
| ZYDUSLIFE | vwap_trend_short_late_vol5x | 12 | 0.545 | 0.727 | 75.0% |
| SJVN | vwap_trend_short_late_vol5x | 15 | 0.128 | 0.171 | 73.3% |
| VEDL | vwap_trend_short_late_vol5x | 6 | 1.010 | 1.347 | 66.7% |
| TITAN | vwap_trend_short_late_vol5x | 6 | 0.748 | 0.997 | 66.7% |
| UNITDSPR | vwap_trend_short_late_vol5x | 6 | 0.058 | 0.077 | 66.7% |
| TATAINVEST | vwap_trend_short_late_vol5x | 11 | 0.266 | 0.355 | 63.6% |
| BPCL | vwap_trend_short_late_vol5x | 60 | 0.198 | 0.264 | 63.3% |
| LENSKART | vwap_trend_short_late_vol5x | 16 | 0.560 | 0.746 | 62.5% |
| HCLTECH | vwap_trend_short_late_vol5x | 40 | 0.205 | 0.273 | 62.5% |
| BAJAJHFL | vwap_trend_short_late_vol5x | 21 | 0.071 | 0.095 | 61.9% |
| ITC | vwap_trend_short_late_vol5x | 61 | 0.197 | 0.262 | 60.7% |
| LTM | vwap_trend_short_late_vol5x | 63 | 0.310 | 0.413 | 60.3% |
| TIINDIA | vwap_trend_short_late_vol5x | 10 | 0.630 | 0.841 | 60.0% |
| TMPV | vwap_trend_short_late_vol5x | 5 | 0.280 | 0.373 | 60.0% |
| JSWINFRA | vwap_trend_short_late_vol5x | 59 | 0.162 | 0.216 | 59.3% |
| TORNTPOWER | vwap_trend_short_late_vol5x | 12 | 0.315 | 0.421 | 58.3% |
| INDUSINDBK | vwap_trend_short_late_vol5x | 48 | 0.246 | 0.327 | 58.3% |
| TMCV | vwap_trend_short_late_vol5x | 12 | 0.218 | 0.290 | 58.3% |
| LICI | vwap_trend_short_late_vol5x | 64 | 0.007 | 0.009 | 57.8% |
| MARICO | vwap_trend_short_late_vol5x | 80 | 0.093 | 0.124 | 57.5% |
| APOLLOTYRE | vwap_trend_short_late_vol5x | 89 | 0.266 | 0.355 | 57.3% |
| RADICO | vwap_trend_short_late_vol5x | 7 | 0.800 | 1.067 | 57.1% |
| AWL | vwap_trend_short_late_vol5x | 66 | 0.135 | 0.180 | 56.1% |
| JKCEMENT | vwap_trend_short_late_vol5x | 145 | 0.202 | 0.270 | 55.9% |
| BANKINDIA | vwap_trend_short_late_vol5x | 106 | 0.185 | 0.247 | 55.7% |
| SWIGGY | vwap_trend_short_late_vol5x | 9 | 0.707 | 0.943 | 55.6% |
| UPL | vwap_trend_short_late_vol5x | 9 | 0.218 | 0.290 | 55.6% |
| ASIANPAINT | vwap_trend_short_late_vol5x | 54 | 0.140 | 0.186 | 55.6% |
| CIPLA | vwap_trend_short_late_vol5x | 49 | 0.047 | 0.063 | 55.1% |
| LINDEINDIA | vwap_trend_short_late_vol5x | 169 | 0.177 | 0.236 | 55.0% |
| PREMIERENE | vwap_trend_short_late_vol5x | 11 | 0.302 | 0.403 | 54.5% |
| ANTHEM | vwap_trend_short_late_vol5x | 22 | 0.209 | 0.279 | 54.5% |
| SCHAEFFLER | vwap_trend_short_late_vol5x | 11 | 0.037 | 0.050 | 54.5% |
| ITCHOTELS | vwap_trend_short_late_vol5x | 46 | 0.279 | 0.372 | 54.3% |
| DIVISLAB | vwap_trend_short_late_vol5x | 70 | 0.129 | 0.173 | 54.3% |
| INFY | vwap_trend_short_late_vol5x | 35 | 0.058 | 0.078 | 54.3% |
| HONAUT | vwap_trend_short_late_vol5x | 243 | 0.088 | 0.117 | 53.5% |
| TATACOMM | vwap_trend_short_late_vol5x | 15 | 0.409 | 0.546 | 53.3% |
| INDUSTOWER | vwap_trend_short_late_vol5x | 64 | 0.149 | 0.199 | 53.1% |
| ETERNAL | vwap_trend_short_late_vol5x | 55 | 0.184 | 0.245 | 52.7% |
| COALINDIA | vwap_trend_short_late_vol5x | 55 | 0.093 | 0.124 | 52.7% |
| ABB | vwap_trend_short_late_vol5x | 97 | 0.214 | 0.285 | 52.6% |
| MARUTI | vwap_trend_short_late_vol5x | 61 | 0.095 | 0.127 | 52.5% |
| NATIONALUM | vwap_trend_short_late_vol5x | 67 | 0.227 | 0.302 | 52.2% |
| 3MINDIA | vwap_trend_short_late_vol5x | 272 | 0.060 | 0.080 | 52.2% |
| IREDA | vwap_trend_short_late_vol5x | 25 | 0.142 | 0.189 | 52.0% |
| COCHINSHIP | vwap_trend_short_late_vol5x | 120 | 0.036 | 0.048 | 51.7% |
| LLOYDSME | vwap_trend_short_late_vol5x | 93 | 0.319 | 0.425 | 51.6% |
| APARINDS | vwap_trend_short_late_vol5x | 165 | 0.291 | 0.388 | 51.5% |
| HINDALCO | vwap_trend_short_late_vol5x | 47 | 0.055 | 0.074 | 51.1% |
| CRISIL | vwap_trend_short_late_vol5x | 208 | 0.106 | 0.141 | 51.0% |
| MUTHOOTFIN | vwap_trend_short_late_vol5x | 67 | 0.044 | 0.058 | 50.7% |
| HUDCO | vwap_trend_short_late_vol5x | 136 | -0.060 | -0.080 | 50.7% |
| MANKIND | vwap_trend_short_late_vol5x | 81 | 0.156 | 0.208 | 50.6% |
| KPRMILL | vwap_trend_short_late_vol5x | 165 | 0.133 | 0.177 | 50.3% |
| POWERINDIA | vwap_trend_short_late_vol5x | 6 | 0.476 | 0.635 | 50.0% |
| SUNPHARMA | vwap_trend_short_late_vol5x | 6 | 0.473 | 0.630 | 50.0% |
| TORNTPHARM | vwap_trend_short_late_vol5x | 6 | 0.388 | 0.517 | 50.0% |
| PFC | vwap_trend_short_late_vol5x | 6 | 0.369 | 0.491 | 50.0% |
| OIL | vwap_trend_short_late_vol5x | 126 | 0.203 | 0.270 | 50.0% |
| ICICIPRULI | vwap_trend_short_late_vol5x | 82 | 0.198 | 0.264 | 50.0% |
| DMART | vwap_trend_short_late_vol5x | 80 | 0.191 | 0.255 | 50.0% |
| PIDILITIND | vwap_trend_short_late_vol5x | 10 | 0.165 | 0.221 | 50.0% |
| PATANJALI | vwap_trend_short_late_vol5x | 14 | 0.158 | 0.210 | 50.0% |
| POWERGRID | vwap_trend_short_late_vol5x | 10 | 0.070 | 0.093 | 50.0% |
| ENRIN | vwap_trend_short_late_vol5x | 28 | 0.053 | 0.070 | 50.0% |
| MAHABANK | vwap_trend_short_late_vol5x | 168 | 0.035 | 0.046 | 50.0% |
| HEROMOTOCO | vwap_trend_short_late_vol5x | 36 | 0.024 | 0.032 | 50.0% |
| ADANIPORTS | vwap_trend_short_late_vol5x | 38 | 0.018 | 0.024 | 50.0% |
| PRESTIGE | vwap_trend_short_late_vol5x | 6 | -0.174 | -0.231 | 50.0% |
| DIXON | vwap_trend_short_late_vol5x | 61 | 0.047 | 0.063 | 49.2% |
| AUBANK | vwap_trend_short_late_vol5x | 74 | 0.139 | 0.186 | 48.6% |
| BOSCHLTD | vwap_trend_short_late_vol5x | 148 | 0.062 | 0.083 | 48.6% |
| DABUR | vwap_trend_short_late_vol5x | 72 | 0.135 | 0.181 | 48.6% |
| AXISBANK | vwap_trend_short_late_vol5x | 35 | 0.041 | 0.054 | 48.6% |
| BAJAJFINSV | vwap_trend_short_late_vol5x | 64 | 0.122 | 0.163 | 48.4% |
| NLCINDIA | vwap_trend_short_late_vol5x | 122 | 0.129 | 0.171 | 48.4% |
| INDIGO | vwap_trend_short_late_vol5x | 56 | 0.007 | 0.009 | 48.2% |
| HDBFS | vwap_trend_short_late_vol5x | 27 | 0.017 | 0.023 | 48.1% |
| ABBOTINDIA | vwap_trend_short_late_vol5x | 219 | 0.017 | 0.022 | 47.9% |
| JSWSTEEL | vwap_trend_short_late_vol5x | 65 | 0.079 | 0.105 | 47.7% |
| KOTAKBANK | vwap_trend_short_late_vol5x | 65 | 0.007 | 0.010 | 47.7% |
| DLF | vwap_trend_short_late_vol5x | 42 | 0.076 | 0.102 | 47.6% |
| HAL | vwap_trend_short_late_vol5x | 63 | 0.029 | 0.039 | 47.6% |
| CONCOR | vwap_trend_short_late_vol5x | 82 | 0.080 | 0.107 | 47.6% |
| BDL | vwap_trend_short_late_vol5x | 97 | 0.062 | 0.082 | 47.4% |
| M&MFIN | vwap_trend_short_late_vol5x | 76 | 0.114 | 0.152 | 47.4% |
| NESTLEIND | vwap_trend_short_late_vol5x | 76 | -0.040 | -0.053 | 47.4% |
| LICHSGFIN | vwap_trend_short_late_vol5x | 76 | -0.053 | -0.071 | 47.4% |
| LT | vwap_trend_short_late_vol5x | 55 | 0.103 | 0.138 | 47.3% |
| AIAENG | vwap_trend_short_late_vol5x | 199 | 0.001 | 0.001 | 47.2% |
| LGEINDIA | vwap_trend_short_late_vol5x | 17 | -0.008 | -0.010 | 47.1% |
| HAVELLS | vwap_trend_short_late_vol5x | 96 | -0.061 | -0.081 | 46.9% |
| BHARTIHEXA | vwap_trend_short_late_vol5x | 64 | -0.080 | -0.107 | 46.9% |
| ATGL | vwap_trend_short_late_vol5x | 94 | -0.073 | -0.097 | 46.8% |
| INDHOTEL | vwap_trend_short_late_vol5x | 62 | 0.055 | 0.073 | 46.8% |
