# tune30-lookback/LENSKART-lb30 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 1.0x risk (0.750%), give-back trail 0.250% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 1 instruments, 2018-12-04..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_late_vwapslope_lb30 | 38 | 0.245 | 0.327 | 63.2% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LENSKART | vwap_trend_short_late_vwapslope_lb30 | 38 | 0.245 | 0.327 | 63.2% |
