# Documentation

| document | what it covers |
|---|---|
| [strategies.md](strategies.md) | The live strategy — hypothesis, exact entry and exit rules, universe selection, sizing, backtested results. Plus the numbers for every other strategy tested. |
| [architecture.md](architecture.md) | The live session's data path and safety loop, the 16 workspace crates, the database schema. |
| [research-method.md](research-method.md) | The bar a result clears before reaching the live config: the six-point protocol, the random-entry control, R units, the scale of the noise, and the comparison pitfalls specific to this setup. |
| [operations.md](operations.md) | Setup, daily commands, Telegram alerting, the watchdog, logging. |

Raw research output lives in [`../research/`](../research/):

* [`stock-selection/`](../research/stock-selection/) — the walk-forward
  selection studies and the square-off correction. Start here.
* [`stock-analysis/`](../research/stock-analysis/) — the scripts behind the
  entry-filter and regime work.
* [`sweeps/`](../research/sweeps/) — 157 parameter-sweep runs, one file each.
* [`swing/`](../research/swing/) — the daily-timeframe simulator and setups.
* [`long-baseline/`](../research/long-baseline/) — long-side results.
