# intraday — NSE intraday equity (Rust)

A live short book trading real money through Kite Connect, and the research
that produced it. Sibling to the Python `tradex/` EOD options pipeline in this
repo — separate concern, separate language, not merged.

**Start with the [root README](../README.md)** for the strategy, the
architecture diagram and the backtested numbers. Then:

| | |
|---|---|
| [docs/strategies.md](docs/strategies.md) | The live strategy in full, and the numbers for everything else tested |
| [docs/architecture.md](docs/architecture.md) | Run modes, the 16 crates, the database schema |
| [docs/research-method.md](docs/research-method.md) | How a result earns belief here |
| [docs/operations.md](docs/operations.md) | Setup, daily commands, Telegram alerts, watchdog |

## Quick reference

```sh
cargo build --release
cargo test                                              # 240 tests, no network

papertrader collect                                     # minute-candle collector
papertrader live-trade check                            # read-only pre-flight
papertrader shadow --config config/tune30_live.toml     # simulated fills, live prices
papertrader live-trade run --live --config config/tune30_live.toml
papertrader live-trade halt                             # stop new entries
papertrader research --help                             # backtests (never logs in)
papertrader-web                                         # local dashboard
```

`--live` is never a default. The engine refuses to start when this machine's
egress IP is not in `config/ip_whitelist.toml`.

**Do not run anything that touches Kite while a live session is up** —
`backfill`, `market quote` and `live-trade check` each mint a fresh access
token and invalidate the one the live process holds.

## Layout

```
crates/
  core-types/      the traits every adapter implements, plus domain types
  kite-auth/       headless login + TOTP -> daily access token
  kite-client/     Kite REST + WebSocket, IPv4-bound
  market-calendar/ NSE holiday-aware session open/close
  universe/        NSE 50/100/250 seed lists + liquidity screening
  indicators/      EMA, VWAP, relative volume, opening range
  strategy-core/   IndicatorEngine + StrategyRunner (bar plumbing)
  strategies/      strategy impls, incl. the RandomEntry control
  risk/            DefaultExitManager: stop -> arm -> trail
  storage/         SQLite schema and queries
  paper-engine/    simulated fills (backtest + shadow)
  backtest/        backtest driver + CacheOnlyFeed
  live-trading/    LiveEngine, ExecutionPolicy, session clock, IP check, alerts
  scheduler/       session scheduling
cli/               the `papertrader` binary
web/               local read-only research dashboard
config/            default.toml, tune30_live.toml (the live book), templates
docs/              see the table above
research/          every run's output, kept for audit
scripts/           watchdog, Telegram setup
secrets/           credentials.toml (gitignored) + template
```

## Config

| file | tracked? | holds |
|---|---|---|
| `config/default.toml` | yes | universe, windows, risk defaults |
| `config/tune30_live.toml` | yes | the live 25-stock book and its parameters |
| `config/alerts.toml` | **no** | Telegram token / ntfy topic |
| `config/ip_whitelist.toml` | **no** | this machine's egress IP |
| `secrets/credentials.toml` | **no** | Kite api_key, api_secret, client id, password, TOTP seed |

Each untracked file has a committed `.example` template beside it.
