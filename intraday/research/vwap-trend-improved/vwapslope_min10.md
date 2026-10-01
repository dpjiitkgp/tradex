# vwap-trend-improved/vwapslope_min10 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 3.0x risk (2.250%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 165 instruments, 2021-03-23..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_late_vwapslope_min10 | 1455 | -0.068 | -0.090 | 37.5% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LICHSGFIN | vwap_trend_short_late_vwapslope_min10 | 9 | 0.068 | 0.090 | 77.8% |
| HONAUT | vwap_trend_short_late_vwapslope_min10 | 12 | 0.293 | 0.391 | 75.0% |
| IDFCFIRSTB | vwap_trend_short_late_vwapslope_min10 | 7 | 0.827 | 1.103 | 71.4% |
| ABBOTINDIA | vwap_trend_short_late_vwapslope_min10 | 7 | 0.409 | 0.546 | 71.4% |
| BERGEPAINT | vwap_trend_short_late_vwapslope_min10 | 6 | 0.615 | 0.820 | 66.7% |
| JKCEMENT | vwap_trend_short_late_vwapslope_min10 | 20 | 0.318 | 0.424 | 65.0% |
| ICICIPRULI | vwap_trend_short_late_vwapslope_min10 | 8 | 0.222 | 0.296 | 62.5% |
| APARINDS | vwap_trend_short_late_vwapslope_min10 | 15 | 0.616 | 0.822 | 60.0% |
| ESCORTS | vwap_trend_short_late_vwapslope_min10 | 5 | 0.590 | 0.786 | 60.0% |
| INDIGO | vwap_trend_short_late_vwapslope_min10 | 10 | 0.171 | 0.229 | 60.0% |
| NIACL | vwap_trend_short_late_vwapslope_min10 | 21 | 0.168 | 0.225 | 57.1% |
| LTTS | vwap_trend_short_late_vwapslope_min10 | 7 | -0.106 | -0.141 | 57.1% |
| LINDEINDIA | vwap_trend_short_late_vwapslope_min10 | 9 | 0.057 | 0.076 | 55.6% |
| M&MFIN | vwap_trend_short_late_vwapslope_min10 | 11 | 0.602 | 0.802 | 54.5% |
| JSWENERGY | vwap_trend_short_late_vwapslope_min10 | 24 | 0.212 | 0.283 | 54.2% |
| ENDURANCE | vwap_trend_short_late_vwapslope_min10 | 13 | 0.175 | 0.233 | 53.8% |
| AUBANK | vwap_trend_short_late_vwapslope_min10 | 8 | 0.751 | 1.002 | 50.0% |
| NYKAA | vwap_trend_short_late_vwapslope_min10 | 6 | 0.615 | 0.819 | 50.0% |
| BOSCHLTD | vwap_trend_short_late_vwapslope_min10 | 6 | 0.207 | 0.276 | 50.0% |
| ADANIENSOL | vwap_trend_short_late_vwapslope_min10 | 18 | 0.124 | 0.165 | 50.0% |
| INDHOTEL | vwap_trend_short_late_vwapslope_min10 | 8 | 0.081 | 0.108 | 50.0% |
| MFSL | vwap_trend_short_late_vwapslope_min10 | 8 | -0.013 | -0.017 | 50.0% |
| HAVELLS | vwap_trend_short_late_vwapslope_min10 | 10 | -0.031 | -0.041 | 50.0% |
| COLPAL | vwap_trend_short_late_vwapslope_min10 | 8 | -0.107 | -0.143 | 50.0% |
| HDFCAMC | vwap_trend_short_late_vwapslope_min10 | 6 | -0.175 | -0.234 | 50.0% |
| GLAXO | vwap_trend_short_late_vwapslope_min10 | 13 | -0.042 | -0.056 | 46.2% |
| MOTHERSON | vwap_trend_short_late_vwapslope_min10 | 11 | -0.042 | -0.056 | 45.5% |
| AIAENG | vwap_trend_short_late_vwapslope_min10 | 22 | -0.109 | -0.146 | 45.5% |
| MOTILALOFS | vwap_trend_short_late_vwapslope_min10 | 20 | 0.077 | 0.102 | 45.0% |
| BALKRISIND | vwap_trend_short_late_vwapslope_min10 | 9 | 0.632 | 0.843 | 44.4% |
| GICRE | vwap_trend_short_late_vwapslope_min10 | 9 | 0.394 | 0.526 | 44.4% |
| MAZDOCK | vwap_trend_short_late_vwapslope_min10 | 9 | 0.263 | 0.351 | 44.4% |
| AMBUJACEM | vwap_trend_short_late_vwapslope_min10 | 9 | 0.061 | 0.081 | 44.4% |
| MEDANTA | vwap_trend_short_late_vwapslope_min10 | 9 | 0.043 | 0.057 | 44.4% |
| 3MINDIA | vwap_trend_short_late_vwapslope_min10 | 27 | -0.047 | -0.063 | 44.4% |
| DALBHARAT | vwap_trend_short_late_vwapslope_min10 | 18 | -0.170 | -0.226 | 44.4% |
| MAHABANK | vwap_trend_short_late_vwapslope_min10 | 16 | 0.181 | 0.242 | 43.8% |
| ABB | vwap_trend_short_late_vwapslope_min10 | 7 | 0.773 | 1.030 | 42.9% |
| BSE | vwap_trend_short_late_vwapslope_min10 | 7 | 0.471 | 0.628 | 42.9% |
| FEDERALBNK | vwap_trend_short_late_vwapslope_min10 | 7 | 0.102 | 0.136 | 42.9% |
| NLCINDIA | vwap_trend_short_late_vwapslope_min10 | 14 | -0.061 | -0.081 | 42.9% |
| KEI | vwap_trend_short_late_vwapslope_min10 | 12 | 0.508 | 0.677 | 41.7% |
| GMRAIRPORT | vwap_trend_short_late_vwapslope_min10 | 10 | 0.265 | 0.353 | 40.0% |
| IRCTC | vwap_trend_short_late_vwapslope_min10 | 5 | 0.085 | 0.113 | 40.0% |
| LTF | vwap_trend_short_late_vwapslope_min10 | 5 | -0.262 | -0.349 | 40.0% |
| JSL | vwap_trend_short_late_vwapslope_min10 | 18 | 0.257 | 0.343 | 38.9% |
| BLUESTARCO | vwap_trend_short_late_vwapslope_min10 | 13 | 0.151 | 0.201 | 38.5% |
| OFSS | vwap_trend_short_late_vwapslope_min10 | 13 | 0.068 | 0.090 | 38.5% |
| INDIANB | vwap_trend_short_late_vwapslope_min10 | 13 | -0.128 | -0.171 | 38.5% |
| FLUOROCHEM | vwap_trend_short_late_vwapslope_min10 | 26 | -0.270 | -0.359 | 38.5% |
| DIXON | vwap_trend_short_late_vwapslope_min10 | 8 | 0.153 | 0.204 | 37.5% |
| COROMANDEL | vwap_trend_short_late_vwapslope_min10 | 8 | 0.010 | 0.014 | 37.5% |
| COFORGE | vwap_trend_short_late_vwapslope_min10 | 8 | 0.005 | 0.007 | 37.5% |
| ASHOKLEY | vwap_trend_short_late_vwapslope_min10 | 8 | -0.130 | -0.173 | 37.5% |
| IPCALAB | vwap_trend_short_late_vwapslope_min10 | 8 | -0.131 | -0.175 | 37.5% |
| MPHASIS | vwap_trend_short_late_vwapslope_min10 | 8 | -0.164 | -0.218 | 37.5% |
| BEL | vwap_trend_short_late_vwapslope_min10 | 8 | -0.230 | -0.307 | 37.5% |
| BDL | vwap_trend_short_late_vwapslope_min10 | 11 | 0.076 | 0.101 | 36.4% |
| BANKINDIA | vwap_trend_short_late_vwapslope_min10 | 11 | -0.270 | -0.360 | 36.4% |
| KPRMILL | vwap_trend_short_late_vwapslope_min10 | 17 | 0.080 | 0.107 | 35.3% |
| AJANTPHARM | vwap_trend_short_late_vwapslope_min10 | 9 | 0.442 | 0.589 | 33.3% |
| BIOCON | vwap_trend_short_late_vwapslope_min10 | 12 | 0.303 | 0.404 | 33.3% |
| BHEL | vwap_trend_short_late_vwapslope_min10 | 9 | 0.080 | 0.107 | 33.3% |
| NATIONALUM | vwap_trend_short_late_vwapslope_min10 | 9 | 0.027 | 0.036 | 33.3% |
| ATGL | vwap_trend_short_late_vwapslope_min10 | 9 | -0.026 | -0.034 | 33.3% |
| JINDALSTEL | vwap_trend_short_late_vwapslope_min10 | 9 | -0.080 | -0.106 | 33.3% |
| PAGEIND | vwap_trend_short_late_vwapslope_min10 | 9 | -0.145 | -0.194 | 33.3% |
| HAL | vwap_trend_short_late_vwapslope_min10 | 6 | -0.182 | -0.243 | 33.3% |
| MCX | vwap_trend_short_late_vwapslope_min10 | 6 | -0.284 | -0.379 | 33.3% |
| ADANIENT | vwap_trend_short_late_vwapslope_min10 | 6 | -0.289 | -0.386 | 33.3% |
| GAIL | vwap_trend_short_late_vwapslope_min10 | 9 | -0.347 | -0.463 | 33.3% |
| AUROPHARMA | vwap_trend_short_late_vwapslope_min10 | 9 | -0.352 | -0.469 | 33.3% |
| CUMMINSIND | vwap_trend_short_late_vwapslope_min10 | 9 | -0.434 | -0.579 | 33.3% |
| ADANIPORTS | vwap_trend_short_late_vwapslope_min10 | 6 | -0.677 | -0.903 | 33.3% |
| NAM-INDIA | vwap_trend_short_late_vwapslope_min10 | 16 | -0.112 | -0.149 | 31.2% |
| COCHINSHIP | vwap_trend_short_late_vwapslope_min10 | 13 | -0.131 | -0.175 | 30.8% |
| ASTRAL | vwap_trend_short_late_vwapslope_min10 | 13 | -0.416 | -0.555 | 30.8% |
| KALYANKJIL | vwap_trend_short_late_vwapslope_min10 | 10 | -0.309 | -0.412 | 30.0% |
| IDEA | vwap_trend_short_late_vwapslope_min10 | 44 | 0.061 | 0.081 | 29.5% |
| JUBLFOOD | vwap_trend_short_late_vwapslope_min10 | 7 | -0.017 | -0.022 | 28.6% |
| FORTIS | vwap_trend_short_late_vwapslope_min10 | 14 | -0.115 | -0.153 | 28.6% |
| HUDCO | vwap_trend_short_late_vwapslope_min10 | 7 | -0.240 | -0.320 | 28.6% |
| CANBK | vwap_trend_short_late_vwapslope_min10 | 7 | -0.276 | -0.367 | 28.6% |
| BAJAJHLDNG | vwap_trend_short_late_vwapslope_min10 | 7 | -0.363 | -0.484 | 28.6% |
| AWL | vwap_trend_short_late_vwapslope_min10 | 7 | -0.370 | -0.494 | 28.6% |
| GODREJPROP | vwap_trend_short_late_vwapslope_min10 | 7 | -0.379 | -0.505 | 28.6% |
| ACC | vwap_trend_short_late_vwapslope_min10 | 7 | -0.579 | -0.772 | 28.6% |
| GVT&D | vwap_trend_short_late_vwapslope_min10 | 60 | -0.475 | -0.634 | 28.3% |
| OIL | vwap_trend_short_late_vwapslope_min10 | 11 | -0.023 | -0.030 | 27.3% |
| OBEROIRLTY | vwap_trend_short_late_vwapslope_min10 | 11 | -0.056 | -0.075 | 27.3% |
| MANKIND | vwap_trend_short_late_vwapslope_min10 | 12 | -0.140 | -0.187 | 25.0% |
| NAUKRI | vwap_trend_short_late_vwapslope_min10 | 8 | -0.163 | -0.218 | 25.0% |
| NHPC | vwap_trend_short_late_vwapslope_min10 | 16 | -0.351 | -0.469 | 25.0% |
| HINDPETRO | vwap_trend_short_late_vwapslope_min10 | 8 | -0.363 | -0.485 | 25.0% |
| GODREJCP | vwap_trend_short_late_vwapslope_min10 | 8 | -0.371 | -0.494 | 25.0% |
| KPITTECH | vwap_trend_short_late_vwapslope_min10 | 8 | -0.402 | -0.536 | 25.0% |
| GODREJIND | vwap_trend_short_late_vwapslope_min10 | 25 | -0.265 | -0.354 | 24.0% |
| CRISIL | vwap_trend_short_late_vwapslope_min10 | 13 | -0.287 | -0.383 | 23.1% |
| GODFRYPHLP | vwap_trend_short_late_vwapslope_min10 | 13 | -0.429 | -0.572 | 23.1% |
| HINDZINC | vwap_trend_short_late_vwapslope_min10 | 9 | -0.137 | -0.183 | 22.2% |
