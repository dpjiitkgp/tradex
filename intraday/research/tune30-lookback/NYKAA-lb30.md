# tune30-lookback/NYKAA-lb30 — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 3.0x risk (2.250%), give-back trail 0.250% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 1 instruments, 2018-12-04..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_late_vwapslope_lb30 | 214 | 0.224 | 0.298 | 51.4% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| NYKAA | vwap_trend_short_late_vwapslope_lb30 | 214 | 0.224 | 0.298 | 51.4% |
