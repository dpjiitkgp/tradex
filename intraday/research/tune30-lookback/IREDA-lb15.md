# tune30-lookback/IREDA-lb15 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 10.0x risk (7.500%), give-back trail 1.500% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 1 instruments, 2018-12-04..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_late_vwapslope | 111 | 0.280 | 0.374 | 53.2% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| IREDA | vwap_trend_short_late_vwapslope | 111 | 0.280 | 0.374 | 53.2% |
