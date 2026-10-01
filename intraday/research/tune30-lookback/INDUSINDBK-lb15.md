# tune30-lookback/INDUSINDBK-lb15 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 10.0x risk (7.500%), give-back trail 0.250% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 1 instruments, 2018-12-04..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_late_vwapslope | 291 | 0.284 | 0.378 | 52.9% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| INDUSINDBK | vwap_trend_short_late_vwapslope | 291 | 0.284 | 0.378 | 52.9% |
