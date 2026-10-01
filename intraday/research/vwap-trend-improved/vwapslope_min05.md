# vwap-trend-improved/vwapslope_min05 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 3.0x risk (2.250%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 165 instruments, 2021-03-23..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_late_vwapslope_min05 | 5580 | -0.005 | -0.007 | 41.5% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| ASIANPAINT | vwap_trend_short_late_vwapslope_min05 | 12 | 0.844 | 1.125 | 75.0% |
| LICI | vwap_trend_short_late_vwapslope_min05 | 9 | 0.014 | 0.018 | 66.7% |
| IRCTC | vwap_trend_short_late_vwapslope_min05 | 14 | 0.561 | 0.748 | 64.3% |
| MARICO | vwap_trend_short_late_vwapslope_min05 | 25 | 0.081 | 0.109 | 64.0% |
| DRREDDY | vwap_trend_short_late_vwapslope_min05 | 16 | 0.280 | 0.374 | 62.5% |
| HDFCLIFE | vwap_trend_short_late_vwapslope_min05 | 13 | 0.263 | 0.351 | 61.5% |
| HONAUT | vwap_trend_short_late_vwapslope_min05 | 36 | 0.117 | 0.156 | 61.1% |
| AWL | vwap_trend_short_late_vwapslope_min05 | 30 | 0.272 | 0.362 | 60.0% |
| INDIGO | vwap_trend_short_late_vwapslope_min05 | 32 | 0.155 | 0.206 | 59.4% |
| AXISBANK | vwap_trend_short_late_vwapslope_min05 | 22 | 0.068 | 0.090 | 59.1% |
| EXIDEIND | vwap_trend_short_late_vwapslope_min05 | 24 | 0.183 | 0.244 | 58.3% |
| HEROMOTOCO | vwap_trend_short_late_vwapslope_min05 | 21 | 0.169 | 0.225 | 57.1% |
| BERGEPAINT | vwap_trend_short_late_vwapslope_min05 | 32 | 0.177 | 0.236 | 56.2% |
| APARINDS | vwap_trend_short_late_vwapslope_min05 | 61 | 0.488 | 0.650 | 55.7% |
| INDUSINDBK | vwap_trend_short_late_vwapslope_min05 | 27 | 0.290 | 0.387 | 55.6% |
| GRASIM | vwap_trend_short_late_vwapslope_min05 | 27 | 0.104 | 0.138 | 55.6% |
| DIVISLAB | vwap_trend_short_late_vwapslope_min05 | 18 | 0.056 | 0.075 | 55.6% |
| BAJAJHLDNG | vwap_trend_short_late_vwapslope_min05 | 22 | 0.189 | 0.252 | 54.5% |
| ONGC | vwap_trend_short_late_vwapslope_min05 | 24 | 0.150 | 0.200 | 54.2% |
| BANKBARODA | vwap_trend_short_late_vwapslope_min05 | 24 | 0.087 | 0.116 | 54.2% |
| MARUTI | vwap_trend_short_late_vwapslope_min05 | 13 | 0.119 | 0.159 | 53.8% |
| BOSCHLTD | vwap_trend_short_late_vwapslope_min05 | 28 | 0.156 | 0.208 | 53.6% |
| LTM | vwap_trend_short_late_vwapslope_min05 | 21 | 0.263 | 0.350 | 52.4% |
| GICRE | vwap_trend_short_late_vwapslope_min05 | 41 | 0.311 | 0.415 | 51.2% |
| GODREJPROP | vwap_trend_short_late_vwapslope_min05 | 28 | 0.244 | 0.326 | 50.0% |
| NTPC | vwap_trend_short_late_vwapslope_min05 | 28 | 0.192 | 0.256 | 50.0% |
| JUBLFOOD | vwap_trend_short_late_vwapslope_min05 | 38 | 0.172 | 0.230 | 50.0% |
| APOLLOHOSP | vwap_trend_short_late_vwapslope_min05 | 16 | 0.119 | 0.159 | 50.0% |
| ADANIENT | vwap_trend_short_late_vwapslope_min05 | 22 | 0.082 | 0.109 | 50.0% |
| GLAXO | vwap_trend_short_late_vwapslope_min05 | 44 | 0.059 | 0.078 | 50.0% |
| EICHERMOT | vwap_trend_short_late_vwapslope_min05 | 22 | 0.038 | 0.050 | 50.0% |
| HINDUNILVR | vwap_trend_short_late_vwapslope_min05 | 14 | -0.057 | -0.076 | 50.0% |
| ENDURANCE | vwap_trend_short_late_vwapslope_min05 | 59 | 0.114 | 0.152 | 49.2% |
| LICHSGFIN | vwap_trend_short_late_vwapslope_min05 | 37 | 0.003 | 0.003 | 48.6% |
| IDFCFIRSTB | vwap_trend_short_late_vwapslope_min05 | 35 | 0.081 | 0.107 | 48.6% |
| ABB | vwap_trend_short_late_vwapslope_min05 | 33 | 0.289 | 0.385 | 48.5% |
| MEDANTA | vwap_trend_short_late_vwapslope_min05 | 31 | 0.071 | 0.094 | 48.4% |
| DLF | vwap_trend_short_late_vwapslope_min05 | 27 | 0.202 | 0.270 | 48.1% |
| IOC | vwap_trend_short_late_vwapslope_min05 | 27 | 0.178 | 0.238 | 48.1% |
| GAIL | vwap_trend_short_late_vwapslope_min05 | 25 | 0.050 | 0.067 | 48.0% |
| CONCOR | vwap_trend_short_late_vwapslope_min05 | 25 | 0.013 | 0.017 | 48.0% |
| PAGEIND | vwap_trend_short_late_vwapslope_min05 | 25 | 0.005 | 0.006 | 48.0% |
| BHARTIARTL | vwap_trend_short_late_vwapslope_min05 | 23 | -0.026 | -0.035 | 47.8% |
| LTF | vwap_trend_short_late_vwapslope_min05 | 21 | 0.341 | 0.454 | 47.6% |
| M&M | vwap_trend_short_late_vwapslope_min05 | 19 | 0.051 | 0.067 | 47.4% |
| DALBHARAT | vwap_trend_short_late_vwapslope_min05 | 38 | 0.030 | 0.041 | 47.4% |
| DIXON | vwap_trend_short_late_vwapslope_min05 | 34 | 0.261 | 0.348 | 47.1% |
| LTTS | vwap_trend_short_late_vwapslope_min05 | 32 | -0.017 | -0.022 | 46.9% |
| MOTILALOFS | vwap_trend_short_late_vwapslope_min05 | 62 | 0.027 | 0.036 | 46.8% |
| MAHABANK | vwap_trend_short_late_vwapslope_min05 | 60 | 0.183 | 0.244 | 46.7% |
| BAJAJFINSV | vwap_trend_short_late_vwapslope_min05 | 30 | 0.061 | 0.082 | 46.7% |
| HUDCO | vwap_trend_short_late_vwapslope_min05 | 43 | 0.075 | 0.100 | 46.5% |
| FORTIS | vwap_trend_short_late_vwapslope_min05 | 41 | 0.230 | 0.307 | 46.3% |
| ADANIENSOL | vwap_trend_short_late_vwapslope_min05 | 54 | 0.139 | 0.185 | 46.3% |
| BANKINDIA | vwap_trend_short_late_vwapslope_min05 | 39 | 0.038 | 0.050 | 46.2% |
| BRITANNIA | vwap_trend_short_late_vwapslope_min05 | 13 | 0.014 | 0.018 | 46.2% |
| ESCORTS | vwap_trend_short_late_vwapslope_min05 | 26 | -0.008 | -0.010 | 46.2% |
| M&MFIN | vwap_trend_short_late_vwapslope_min05 | 37 | 0.061 | 0.082 | 45.9% |
| AMBUJACEM | vwap_trend_short_late_vwapslope_min05 | 35 | 0.136 | 0.181 | 45.7% |
| COROMANDEL | vwap_trend_short_late_vwapslope_min05 | 35 | 0.116 | 0.155 | 45.7% |
| FLUOROCHEM | vwap_trend_short_late_vwapslope_min05 | 62 | -0.110 | -0.146 | 45.2% |
| NAM-INDIA | vwap_trend_short_late_vwapslope_min05 | 49 | 0.079 | 0.106 | 44.9% |
| COFORGE | vwap_trend_short_late_vwapslope_min05 | 29 | 0.095 | 0.127 | 44.8% |
| MPHASIS | vwap_trend_short_late_vwapslope_min05 | 29 | 0.082 | 0.110 | 44.8% |
| APOLLOTYRE | vwap_trend_short_late_vwapslope_min05 | 29 | 0.035 | 0.046 | 44.8% |
| KALYANKJIL | vwap_trend_short_late_vwapslope_min05 | 38 | 0.009 | 0.012 | 44.7% |
| JSL | vwap_trend_short_late_vwapslope_min05 | 54 | 0.340 | 0.453 | 44.4% |
| JKCEMENT | vwap_trend_short_late_vwapslope_min05 | 54 | 0.010 | 0.013 | 44.4% |
| JSWENERGY | vwap_trend_short_late_vwapslope_min05 | 61 | 0.167 | 0.223 | 44.3% |
| ABCAPITAL | vwap_trend_short_late_vwapslope_min05 | 43 | 0.046 | 0.061 | 44.2% |
| 3MINDIA | vwap_trend_short_late_vwapslope_min05 | 62 | -0.069 | -0.092 | 43.5% |
| ATGL | vwap_trend_short_late_vwapslope_min05 | 37 | 0.045 | 0.061 | 43.2% |
| LUPIN | vwap_trend_short_late_vwapslope_min05 | 21 | -0.042 | -0.056 | 42.9% |
| LT | vwap_trend_short_late_vwapslope_min05 | 14 | -0.060 | -0.080 | 42.9% |
| HAVELLS | vwap_trend_short_late_vwapslope_min05 | 35 | -0.139 | -0.186 | 42.9% |
| MANKIND | vwap_trend_short_late_vwapslope_min05 | 40 | 0.095 | 0.126 | 42.5% |
| ABBOTINDIA | vwap_trend_short_late_vwapslope_min05 | 33 | -0.118 | -0.157 | 42.4% |
| NIACL | vwap_trend_short_late_vwapslope_min05 | 52 | 0.086 | 0.114 | 42.3% |
| BLUESTARCO | vwap_trend_short_late_vwapslope_min05 | 52 | 0.032 | 0.042 | 42.3% |
| INDUSTOWER | vwap_trend_short_late_vwapslope_min05 | 38 | 0.045 | 0.061 | 42.1% |
| KPRMILL | vwap_trend_short_late_vwapslope_min05 | 69 | 0.100 | 0.133 | 42.0% |
| NYKAA | vwap_trend_short_late_vwapslope_min05 | 24 | 0.084 | 0.113 | 41.7% |
| ASHOKLEY | vwap_trend_short_late_vwapslope_min05 | 24 | 0.032 | 0.042 | 41.7% |
| GODREJIND | vwap_trend_short_late_vwapslope_min05 | 60 | -0.147 | -0.196 | 41.7% |
| ICICIPRULI | vwap_trend_short_late_vwapslope_min05 | 24 | -0.166 | -0.221 | 41.7% |
| HCLTECH | vwap_trend_short_late_vwapslope_min05 | 12 | -0.179 | -0.239 | 41.7% |
| HINDALCO | vwap_trend_short_late_vwapslope_min05 | 17 | 0.076 | 0.101 | 41.2% |
| OIL | vwap_trend_short_late_vwapslope_min05 | 34 | -0.032 | -0.042 | 41.2% |
| FEDERALBNK | vwap_trend_short_late_vwapslope_min05 | 39 | -0.055 | -0.074 | 41.0% |
| JSWSTEEL | vwap_trend_short_late_vwapslope_min05 | 22 | 0.023 | 0.030 | 40.9% |
| GLENMARK | vwap_trend_short_late_vwapslope_min05 | 27 | -0.050 | -0.067 | 40.7% |
| BSE | vwap_trend_short_late_vwapslope_min05 | 32 | 0.284 | 0.379 | 40.6% |
| AJANTPHARM | vwap_trend_short_late_vwapslope_min05 | 32 | 0.175 | 0.234 | 40.6% |
| HINDZINC | vwap_trend_short_late_vwapslope_min05 | 32 | 0.141 | 0.188 | 40.6% |
| NLCINDIA | vwap_trend_short_late_vwapslope_min05 | 69 | -0.023 | -0.031 | 40.6% |
| BHEL | vwap_trend_short_late_vwapslope_min05 | 37 | 0.031 | 0.041 | 40.5% |
| KPITTECH | vwap_trend_short_late_vwapslope_min05 | 37 | 0.026 | 0.035 | 40.5% |
| JINDALSTEL | vwap_trend_short_late_vwapslope_min05 | 25 | 0.106 | 0.141 | 40.0% |
| OFSS | vwap_trend_short_late_vwapslope_min05 | 40 | -0.019 | -0.025 | 40.0% |
| GODFRYPHLP | vwap_trend_short_late_vwapslope_min05 | 55 | -0.079 | -0.106 | 40.0% |
