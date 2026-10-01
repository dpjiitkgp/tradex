# band-ceil005 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 3.0x risk (2.250%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 25 instruments, 2018-11-09..2026-09-30.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwapslope_lb20_ceil005 | 6296 | 0.064 | 0.086 | 46.7% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LENSKART | vwapslope_lb20_ceil005 | 32 | 0.281 | 0.374 | 62.5% |
| LICI | vwapslope_lb20_ceil005 | 181 | 0.112 | 0.150 | 61.3% |
| ITC | vwapslope_lb20_ceil005 | 243 | 0.131 | 0.174 | 58.0% |
| HAL | vwapslope_lb20_ceil005 | 315 | 0.204 | 0.272 | 57.1% |
| NAUKRI | vwapslope_lb20_ceil005 | 271 | 0.228 | 0.304 | 52.0% |
| APARINDS | vwapslope_lb20_ceil005 | 285 | 0.159 | 0.212 | 51.9% |
| VOLTAS | vwapslope_lb20_ceil005 | 242 | 0.084 | 0.113 | 50.8% |
| ATGL | vwapslope_lb20_ceil005 | 294 | 0.136 | 0.182 | 50.7% |
| CANBK | vwapslope_lb20_ceil005 | 247 | 0.188 | 0.251 | 50.2% |
| PNB | vwapslope_lb20_ceil005 | 300 | 0.132 | 0.177 | 49.7% |
| DABUR | vwapslope_lb20_ceil005 | 224 | 0.095 | 0.127 | 49.1% |
| TMPV | vwapslope_lb20_ceil005 | 278 | 0.116 | 0.155 | 48.9% |
| DIXON | vwapslope_lb20_ceil005 | 269 | 0.146 | 0.195 | 48.3% |
| ASIANPAINT | vwapslope_lb20_ceil005 | 255 | -0.026 | -0.034 | 47.1% |
| WAAREEENER | vwapslope_lb20_ceil005 | 66 | 0.066 | 0.088 | 45.5% |
| TVSMOTOR | vwapslope_lb20_ceil005 | 246 | 0.004 | 0.005 | 44.7% |
| PRESTIGE | vwapslope_lb20_ceil005 | 291 | 0.106 | 0.141 | 44.7% |
| ICICIPRULI | vwapslope_lb20_ceil005 | 229 | 0.033 | 0.044 | 43.7% |
| LLOYDSME | vwapslope_lb20_ceil005 | 109 | 0.020 | 0.027 | 43.1% |
| YESBANK | vwapslope_lb20_ceil005 | 447 | -0.025 | -0.033 | 42.3% |
| GMRAIRPORT | vwapslope_lb20_ceil005 | 273 | -0.002 | -0.003 | 41.8% |
| TIINDIA | vwapslope_lb20_ceil005 | 301 | -0.012 | -0.016 | 41.5% |
| LODHA | vwapslope_lb20_ceil005 | 193 | 0.022 | 0.030 | 40.9% |
| NMDC | vwapslope_lb20_ceil005 | 271 | -0.047 | -0.062 | 39.5% |
| IDEA | vwapslope_lb20_ceil005 | 434 | -0.189 | -0.253 | 29.7% |
