# tune30-lookback/ATGL-lb30 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 6.0x risk (4.500%), give-back trail 0.250% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 1 instruments, 2018-12-04..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_late_vwapslope_lb30 | 387 | 0.129 | 0.172 | 49.4% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| ATGL | vwap_trend_short_late_vwapslope_lb30 | 387 | 0.129 | 0.172 | 49.4% |
