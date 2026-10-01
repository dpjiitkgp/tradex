# slope-min035 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 3.0x risk (2.250%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 25 instruments, 2018-11-09..2026-09-30.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwapslope_lb20_min035 | 47 | -0.119 | -0.158 | 25.5% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| ATGL | vwapslope_lb20_min035 | 5 | -0.089 | -0.118 | 40.0% |
| APARINDS | vwapslope_lb20_min035 | 5 | -0.092 | -0.122 | 40.0% |
| IDEA | vwapslope_lb20_min035 | 13 | 0.260 | 0.347 | 15.4% |
