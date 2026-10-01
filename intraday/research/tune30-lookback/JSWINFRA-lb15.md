# tune30-lookback/JSWINFRA-lb15 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 1.0x risk (0.750%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 1 instruments, 2018-12-04..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_late_vwapslope | 137 | 0.248 | 0.331 | 62.0% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| JSWINFRA | vwap_trend_short_late_vwapslope | 137 | 0.248 | 0.331 | 62.0% |
