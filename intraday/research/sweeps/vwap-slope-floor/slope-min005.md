# slope-min005 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 3.0x risk (2.250%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 25 instruments, 2018-11-09..2026-09-30.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwapslope_lb20_min005 | 2457 | -0.004 | -0.006 | 40.2% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LICI | vwapslope_lb20_min005 | 26 | 0.236 | 0.315 | 65.4% |
| WAAREEENER | vwapslope_lb20_min005 | 20 | 0.472 | 0.629 | 60.0% |
| LENSKART | vwapslope_lb20_min005 | 7 | 0.454 | 0.605 | 57.1% |
| ASIANPAINT | vwapslope_lb20_min005 | 55 | 0.199 | 0.266 | 52.7% |
| LLOYDSME | vwapslope_lb20_min005 | 64 | 0.119 | 0.158 | 50.0% |
| DIXON | vwapslope_lb20_min005 | 115 | 0.230 | 0.307 | 48.7% |
| TMPV | vwapslope_lb20_min005 | 84 | 0.117 | 0.156 | 46.4% |
| DABUR | vwapslope_lb20_min005 | 67 | -0.051 | -0.068 | 46.3% |
| APARINDS | vwapslope_lb20_min005 | 152 | 0.091 | 0.121 | 45.4% |
| TIINDIA | vwapslope_lb20_min005 | 137 | 0.051 | 0.068 | 45.3% |
| HAL | vwapslope_lb20_min005 | 102 | 0.000 | 0.000 | 45.1% |
| PNB | vwapslope_lb20_min005 | 120 | 0.099 | 0.131 | 45.0% |
| NAUKRI | vwapslope_lb20_min005 | 99 | 0.107 | 0.143 | 43.4% |
| VOLTAS | vwapslope_lb20_min005 | 67 | 0.069 | 0.092 | 43.3% |
| ATGL | vwapslope_lb20_min005 | 126 | -0.021 | -0.029 | 42.9% |
| ITC | vwapslope_lb20_min005 | 47 | -0.108 | -0.144 | 40.4% |
| PRESTIGE | vwapslope_lb20_min005 | 127 | 0.048 | 0.065 | 37.8% |
| CANBK | vwapslope_lb20_min005 | 101 | -0.073 | -0.098 | 37.6% |
| ICICIPRULI | vwapslope_lb20_min005 | 89 | -0.111 | -0.149 | 37.1% |
| GMRAIRPORT | vwapslope_lb20_min005 | 135 | -0.079 | -0.106 | 35.6% |
| TVSMOTOR | vwapslope_lb20_min005 | 86 | -0.142 | -0.190 | 34.9% |
| NMDC | vwapslope_lb20_min005 | 124 | -0.189 | -0.252 | 33.9% |
| YESBANK | vwapslope_lb20_min005 | 145 | -0.054 | -0.073 | 33.8% |
| LODHA | vwapslope_lb20_min005 | 107 | -0.135 | -0.180 | 32.7% |
| IDEA | vwapslope_lb20_min005 | 255 | -0.162 | -0.216 | 27.1% |
