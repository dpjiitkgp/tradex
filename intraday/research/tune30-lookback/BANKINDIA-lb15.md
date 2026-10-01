# tune30-lookback/BANKINDIA-lb15 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 6.0x risk (4.500%), give-back trail 0.250% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 1 instruments, 2018-12-04..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_late_vwapslope | 419 | 0.117 | 0.155 | 47.0% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| BANKINDIA | vwap_trend_short_late_vwapslope | 419 | 0.117 | 0.155 | 47.0% |
