# eodfix — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 3.0x risk (2.250%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 3 instruments, 2018-11-09..2026-09-29.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_late_vwapslope_lb20 | 1023 | 0.180 | 0.241 | 51.4% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| HAL | vwap_trend_short_late_vwapslope_lb20 | 404 | 0.166 | 0.221 | 53.7% |
| DMART | vwap_trend_short_late_vwapslope_lb20 | 393 | 0.175 | 0.234 | 51.9% |
| POLICYBZR | vwap_trend_short_late_vwapslope_lb20 | 226 | 0.215 | 0.287 | 46.5% |
