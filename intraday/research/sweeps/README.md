# Parameter sweeps — raw run output

Mechanical output, one file per configuration, kept for audit rather than
reading. The conclusions drawn from them live in
[`../stock-selection/`](../stock-selection/) and
[`../../docs/strategies.md`](../../docs/strategies.md).

| directory | files | what varied |
|---|---|---|
| `stop-mrm-trail-grid/` | 140 | hard stop x MRM arm multiple x give-back trail, named `g<stop>m<mrm>t<trail>` — e.g. `g050m25t075` is a 0.50% stop, armed at 2.5x risk, 0.75% give-back |
| `stop-only/` | 12 | hard stop alone, 0.50%-0.75%, holding MRM and trail fixed so the stop's effect is isolated. `stopS*` in percent, `stopRn*` in R units |
| `vwap-slope-floor/` | 5 | minimum VWAP slope required to take a signal, 0.05%-0.50% |

## What the grid showed

Nothing beat the live `(stop 0.75%, MRM 3.0, trail 0.75%)` by more than noise,
and the grid's best cells do not hold at neighbouring parameters — the
agreement test in [`../../docs/research-method.md`](../../docs/research-method.md).
MRM 1.5 is measurably worse (t = 2.03); MRM 6.0 and a 0.25% trail are within
noise.

**Read these files as R-multiples at your peril.** 1 R = `risk_pct x entry
price`, so a row at a 0.50% stop and a row at a 0.75% stop are quoting
different rupee amounts per R and are not comparable. The `Avg PnL%` column is
the one that compares across the grid.
