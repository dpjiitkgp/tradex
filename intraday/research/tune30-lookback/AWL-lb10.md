# tune30-lookback/AWL-lb10 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 6.0x risk (4.500%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 1 instruments, 2018-12-04..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_late_vwapslope_lb10 | 231 | 0.159 | 0.212 | 56.7% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| AWL | vwap_trend_short_late_vwapslope_lb10 | 231 | 0.159 | 0.212 | 56.7% |
