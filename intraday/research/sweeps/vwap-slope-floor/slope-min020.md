# slope-min020 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 3.0x risk (2.250%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 25 instruments, 2018-11-09..2026-09-30.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwapslope_lb20_min020 | 224 | -0.097 | -0.129 | 30.4% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LLOYDSME | vwapslope_lb20_min020 | 7 | 0.159 | 0.212 | 57.1% |
| ICICIPRULI | vwapslope_lb20_min020 | 6 | 0.228 | 0.304 | 50.0% |
| DIXON | vwapslope_lb20_min020 | 13 | 0.237 | 0.316 | 46.2% |
| GMRAIRPORT | vwapslope_lb20_min020 | 9 | 0.243 | 0.324 | 44.4% |
| TVSMOTOR | vwapslope_lb20_min020 | 5 | 0.098 | 0.130 | 40.0% |
| APARINDS | vwapslope_lb20_min020 | 21 | -0.094 | -0.126 | 38.1% |
| HAL | vwapslope_lb20_min020 | 8 | 0.006 | 0.008 | 37.5% |
| TIINDIA | vwapslope_lb20_min020 | 20 | -0.379 | -0.506 | 35.0% |
| CANBK | vwapslope_lb20_min020 | 9 | -0.226 | -0.301 | 33.3% |
| ATGL | vwapslope_lb20_min020 | 11 | -0.300 | -0.400 | 27.3% |
| YESBANK | vwapslope_lb20_min020 | 12 | -0.104 | -0.138 | 25.0% |
| PRESTIGE | vwapslope_lb20_min020 | 17 | -0.312 | -0.417 | 23.5% |
| NMDC | vwapslope_lb20_min020 | 10 | -0.268 | -0.357 | 20.0% |
| IDEA | vwapslope_lb20_min020 | 41 | 0.102 | 0.135 | 19.5% |
| LODHA | vwapslope_lb20_min020 | 14 | -0.738 | -0.984 | 0.0% |
