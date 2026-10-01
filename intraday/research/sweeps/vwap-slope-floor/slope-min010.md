# slope-min010 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 3.0x risk (2.250%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 25 instruments, 2018-11-09..2026-09-30.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwapslope_lb20_min010 | 927 | -0.030 | -0.040 | 37.1% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LICI | vwapslope_lb20_min010 | 5 | 0.436 | 0.581 | 80.0% |
| ASIANPAINT | vwapslope_lb20_min010 | 9 | 0.610 | 0.813 | 77.8% |
| DABUR | vwapslope_lb20_min010 | 9 | 0.126 | 0.168 | 66.7% |
| WAAREEENER | vwapslope_lb20_min010 | 7 | 1.118 | 1.490 | 57.1% |
| DIXON | vwapslope_lb20_min010 | 42 | 0.408 | 0.545 | 54.8% |
| VOLTAS | vwapslope_lb20_min010 | 21 | 0.209 | 0.278 | 52.4% |
| NAUKRI | vwapslope_lb20_min010 | 41 | 0.174 | 0.232 | 46.3% |
| ICICIPRULI | vwapslope_lb20_min010 | 30 | 0.013 | 0.018 | 43.3% |
| APARINDS | vwapslope_lb20_min010 | 74 | 0.058 | 0.077 | 43.2% |
| LLOYDSME | vwapslope_lb20_min010 | 26 | -0.042 | -0.056 | 42.3% |
| ATGL | vwapslope_lb20_min010 | 31 | 0.085 | 0.113 | 41.9% |
| HAL | vwapslope_lb20_min010 | 29 | -0.181 | -0.242 | 37.9% |
| PRESTIGE | vwapslope_lb20_min010 | 56 | 0.056 | 0.074 | 37.5% |
| GMRAIRPORT | vwapslope_lb20_min010 | 59 | -0.071 | -0.094 | 37.3% |
| CANBK | vwapslope_lb20_min010 | 40 | -0.109 | -0.145 | 35.0% |
| TMPV | vwapslope_lb20_min010 | 23 | -0.116 | -0.154 | 34.8% |
| TVSMOTOR | vwapslope_lb20_min010 | 29 | -0.121 | -0.162 | 34.5% |
| PNB | vwapslope_lb20_min010 | 34 | -0.176 | -0.235 | 32.4% |
| TIINDIA | vwapslope_lb20_min010 | 65 | -0.250 | -0.333 | 32.3% |
| NMDC | vwapslope_lb20_min010 | 51 | -0.166 | -0.221 | 31.4% |
| IDEA | vwapslope_lb20_min010 | 128 | -0.088 | -0.118 | 27.3% |
| ITC | vwapslope_lb20_min010 | 11 | -0.303 | -0.404 | 27.3% |
| YESBANK | vwapslope_lb20_min010 | 54 | -0.211 | -0.281 | 25.9% |
| LODHA | vwapslope_lb20_min010 | 51 | -0.189 | -0.252 | 25.5% |
