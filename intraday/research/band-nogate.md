# band-nogate — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 3.0x risk (2.250%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 25 instruments, 2018-11-09..2026-09-30.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwapslope_lb20_nogate | 13862 | 0.036 | 0.047 | 44.3% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LICI | vwapslope_lb20_nogate | 315 | 0.104 | 0.139 | 56.5% |
| ITC | vwapslope_lb20_nogate | 508 | 0.095 | 0.127 | 54.5% |
| LENSKART | vwapslope_lb20_nogate | 76 | 0.185 | 0.247 | 53.9% |
| HAL | vwapslope_lb20_nogate | 646 | 0.116 | 0.155 | 51.1% |
| ASIANPAINT | vwapslope_lb20_nogate | 556 | 0.056 | 0.075 | 50.4% |
| DABUR | vwapslope_lb20_nogate | 535 | 0.064 | 0.086 | 48.4% |
| ATGL | vwapslope_lb20_nogate | 658 | 0.074 | 0.099 | 47.4% |
| TMPV | vwapslope_lb20_nogate | 578 | 0.087 | 0.116 | 47.4% |
| PNB | vwapslope_lb20_nogate | 639 | 0.120 | 0.160 | 46.9% |
| APARINDS | vwapslope_lb20_nogate | 665 | 0.095 | 0.127 | 46.9% |
| NAUKRI | vwapslope_lb20_nogate | 607 | 0.115 | 0.153 | 46.8% |
| VOLTAS | vwapslope_lb20_nogate | 562 | 0.032 | 0.043 | 46.1% |
| DIXON | vwapslope_lb20_nogate | 605 | 0.107 | 0.143 | 45.5% |
| WAAREEENER | vwapslope_lb20_nogate | 145 | 0.078 | 0.104 | 44.8% |
| ICICIPRULI | vwapslope_lb20_nogate | 529 | 0.045 | 0.060 | 44.8% |
| TVSMOTOR | vwapslope_lb20_nogate | 565 | 0.013 | 0.017 | 44.1% |
| LLOYDSME | vwapslope_lb20_nogate | 257 | 0.073 | 0.098 | 44.0% |
| CANBK | vwapslope_lb20_nogate | 573 | 0.066 | 0.089 | 43.5% |
| PRESTIGE | vwapslope_lb20_nogate | 646 | 0.115 | 0.153 | 42.7% |
| TIINDIA | vwapslope_lb20_nogate | 664 | -0.010 | -0.013 | 41.9% |
| LODHA | vwapslope_lb20_nogate | 466 | 0.016 | 0.021 | 40.8% |
| GMRAIRPORT | vwapslope_lb20_nogate | 638 | -0.056 | -0.074 | 39.5% |
| NMDC | vwapslope_lb20_nogate | 611 | -0.059 | -0.079 | 39.4% |
| YESBANK | vwapslope_lb20_nogate | 837 | -0.034 | -0.046 | 39.4% |
| IDEA | vwapslope_lb20_nogate | 981 | -0.212 | -0.283 | 28.8% |
