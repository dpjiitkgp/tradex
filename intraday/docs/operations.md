# Running it

## Setup

```sh
# 1. Credentials
cp secrets/credentials.example.toml secrets/credentials.toml
chmod 600 secrets/credentials.toml          # api_key, api_secret, client id,
                                            # password, TOTP seed
# 2. IP whitelist mirror (live trading only)
cp config/ip_whitelist.example.toml config/ip_whitelist.toml
#    Enter the same address at developers.kite.trade -> Profile -> IP Whitelist
#    FIRST, then mirror it here. Kite allows 2 addresses, one change per week.

# 3. Alerts (see below)
cp config/alerts.example.toml config/alerts.toml
chmod 600 config/alerts.toml

cargo build --release
```

All three are gitignored. `secrets/`, `config/alerts.toml` and
`config/ip_whitelist.toml` are listed in both `.gitignore` files on purpose.

## Daily

```sh
# Minute-candle collector — run it for the whole session
papertrader collect

# Live, real money. --live is never a default; the engine refuses to start if
# this machine's egress IP is not in config/ip_whitelist.toml
papertrader live-trade run --live --config config/tune30_live.toml

# Same book, simulated fills, independent of the live process
papertrader shadow --config config/tune30_live.toml

# Pause new entries; an open position is still managed to a clean exit
papertrader live-trade halt
papertrader live-trade resume
```

**Do not run anything that touches Kite while a live session is up** —
`backfill`, `market quote` and `live-trade check` all mint a fresh access
token, which invalidates the one the live process is holding. Logging into
Kite Web does the same thing. `papertrader research` is safe: it reads through
`CacheOnlyFeed`, which cannot make a network call.

## Telegram alerting

The live engine never sends anything itself. It *queues* typed `Alert` values
and the CLI drains them, so a network failure in the alerting path can never
affect a trading decision.

### Setting it up

```sh
./scripts/setup_telegram_alerts.sh      # walks through the steps below
```

1. Message **@BotFather** on Telegram, `/newbot`, and keep the token.
2. Message your new bot once, or add it to a group and post once — Telegram
   will not let a bot open a conversation.
3. Find the chat id at
   `https://api.telegram.org/bot<TOKEN>/getUpdates`. A group id is negative.
4. Put both in `config/alerts.toml`:

```toml
[channel]
kind = "telegram"
bot_token = "123456:ABC-DEF..."
chat_id = "-987654321"
```

5. `papertrader test-alert` to confirm delivery.

A group is worth preferring over a direct chat: you can add a second device or
another person without reissuing the token.

### What gets sent

| alert | when | critical |
|---|---|---|
| `Entry` | a position opened, with symbol, side, qty, price, stop, target | no |
| `Exit` | a position closed, with reason and realised R | no |
| `OrderRejected` | the broker refused an order, with its reason | **yes** |
| `EntryUnfilled` | an entry limit timed out after 60s and the signal was dropped | no |
| `ExitEscalated` | a stop-loss limit did not fill and became a market order | **yes** |
| `Halted` | the kill switch engaged, or reconciliation found a mismatch | **yes** |
| `Critical` | anything that could leave a position unaccounted for | **yes** |

Plus a **heartbeat** at session start, periodically through the day, and at
session end with a tally.

### Two independent halves

`config/alerts.toml` configures both, and a live session wants both because
they fail differently:

* **`[channel]`** is the process reporting what it noticed. It dies with the
  process, so it cannot tell you the process died.
* **`heartbeat_url`** is a dead-man's switch — the *absence* of pings raises
  the alarm, from a service running elsewhere. healthchecks.io has a free tier
  that does this: set the period to 1 minute and the grace to ~5 minutes. This
  is the half that survives a crash, the Mac sleeping, and a power cut.

The dead-man's switch is currently **not configured**.

### Delivery details

Worth knowing because both were live failures:

* **Messages are split at 3,900 characters** on line boundaries (counting
  characters, not bytes), with 1.1s pacing between parts and one retry.
  Telegram throttles a burst and silently dropped part 3 of 4 before the
  pacing was added.
* **URLs are stripped from error messages** before logging. A failed request
  put the bot token into a `reqwest` error that was printed to the terminal and
  the log file. There is a regression test for this. If a token is ever
  exposed, revoke it via @BotFather → `/mybots` → Revoke.

## Watchdog

```sh
sed "s|__REPO__|$(pwd)|g" scripts/com.papertrader.watchdog.plist.template \
  > ~/Library/LaunchAgents/com.papertrader.watchdog.plist
launchctl load ~/Library/LaunchAgents/com.papertrader.watchdog.plist
```

Fires every 60s; `watchdog.sh` decides whether the current time is in a
session and mostly exits immediately.

Two limits to be aware of:

* It checks that the process **exists**, not that it is doing anything. A
  collector that is alive but no longer writing candles passes — this has
  happened twice.
* launchd sleeps when the Mac does, so it cannot report a machine that is
  asleep or off. That needs the dead-man's switch above.

Related: the collector's overnight wait uses `tokio::time::sleep`, which does
not advance while macOS is asleep. It needs to be launchd-scheduled rather
than sleeping in-process.

## Logging

`RUST_LOG` defaults to `info` if unset. This matters: it was previously unset
by default, which silenced *everything* including `error!`, so the entire audit
trail was off. Logs go to stdout and to a daily file under `data/live-trades/`.
