use chrono::TimeZone;
use clap::{Parser, Subcommand};
use futures::StreamExt;

mod alerts;

#[derive(Parser)]
#[command(name = "papertrader", about = "NSE intraday equity paper-trading tool")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Market-calendar checks (trading day / session times).
    Calendar {
        #[command(subcommand)]
        action: CalendarAction,
    },
    /// Kite Connect headless-login checks.
    Auth {
        #[command(subcommand)]
        action: AuthAction,
    },
    /// Kite market-data checks (instruments, quotes, historical candles).
    Market {
        #[command(subcommand)]
        action: MarketAction,
    },
    /// Curated stock universe (Nifty 50, liquidity/volatility screened).
    Universe {
        #[command(subcommand)]
        action: UniverseAction,
    },
    /// Replay the three strategies over historical data through PaperEngine.
    Backtest {
        #[command(subcommand)]
        action: BacktestAction,
    },
    /// Print the strategy-performance report from the local database.
    Report {
        /// Show only this run (numeric id, as printed at the end of a
        /// backtest run). Omit to see every run — each row still carries
        /// its own run id/label, so runs never get silently pooled.
        #[arg(long)]
        run: Option<i64>,
        /// Break down by instrument too (the default view rolls up across
        /// instruments per strategy) — this is where "strategy X works
        /// better on stock Y" shows up.
        #[arg(long)]
        by_instrument: bool,
    },
    /// Put two or more backtest runs side by side, per strategy — the tool
    /// for "did that parameter change actually help."
    Compare {
        /// Run ids to compare, e.g. `compare 5 6 7`. Omit to compare every
        /// run on record.
        runs: Vec<i64>,
    },
    /// Send an ad-hoc message to the configured alert channel.
    ///
    /// For notes to yourself during a session — a finding, a query result, a
    /// reminder — landing in the same place as the trade alerts so the day
    /// reads as one thread.
    ///
    ///   papertrader notify "IDEA and YESBANK are 14% of all trades"
    ///   papertrader notify --title Finding "..."
    ///   sqlite3 data/papertrader.db "SELECT ..." | papertrader notify
    Notify {
        /// The message. Omit to read it from stdin instead, which is what
        /// makes piping query output in work.
        message: Vec<String>,
        /// Prefix, so an ad-hoc note is distinguishable at a glance from an
        /// engine alert in the same chat.
        #[arg(long, default_value = "Note")]
        title: String,
    },
    /// Paper-trade the LIVE book with no position limit, against live ticks.
    ///
    /// Same 25 stocks and same strategy as `live-trade run --live`, but every
    /// signal is taken instead of only the one that wins the single slot.
    /// Runs whether or not a live session is running, and records to the same
    /// database, so the cost of slot contention can be measured against real
    /// sessions rather than assumed.
    Shadow {
        /// Rupees per position (sizing only — nothing is sent to a broker).
        #[arg(long, default_value_t = 100_000.0)]
        capital: f64,
        /// Stop after this many seconds. Testing aid.
        #[arg(long)]
        max_seconds: Option<u64>,
    },
    /// Run the live paper-trading daemon: sleeps until market open, trades
    /// the curated universe through market close, repeats daily.
    Run {
        /// Testing/validation only: stop after this many seconds instead of
        /// running forever. Omit for real unattended operation.
        #[arg(long)]
        max_seconds: Option<u64>,
    },
    /// Bulk-fetch and cache historical minute candles for every constituent
    /// of the configured seed universe (regardless of today's liquidity
    /// screen) — data for backtesting/research, not just the live cap.
    Backfill {
        /// How many months of minute history to fetch, ending `to` (today
        /// by default).
        #[arg(long, default_value_t = 6)]
        months: i64,
        /// Seed list to backfill: nifty50 | nifty100 | nifty250. Defaults
        /// to config/default.toml's [universe] seed.
        #[arg(long)]
        index: Option<String>,
        /// YYYY-MM-DD. Defaults to today. Set this to extend coverage
        /// *backward* from an already-cached start date without re-fetching
        /// the range you already have (e.g. `--months 60 --to 2026-03-26`
        /// to fill in everything before an existing 6-month cache).
        #[arg(long)]
        to: Option<String>,
        /// YYYY-MM-DD start date, overriding `--months`. Use this to repair a
        /// known gap: `--from 2026-09-30 --to 2026-09-30` refetches exactly
        /// one session instead of a whole month across every instrument.
        #[arg(long)]
        from: Option<String>,
        /// Comma-separated tradingsymbols to restrict this run to (e.g.
        /// for retrying just the instruments that failed a prior run,
        /// instead of re-fetching the whole index and wasting API calls
        /// on symbols already cached). Must still be members of `--index`'s
        /// seed list.
        #[arg(long)]
        symbols: Option<String>,
        /// Comma-separated NSE *index* tradingsymbols to backfill instead of
        /// equities, e.g. `--indices "NIFTY 50,INDIA VIX"`. Indices live in
        /// a different instrument segment from cash equities, so they are
        /// invisible to the seed-list path above; this is how market-context
        /// series (index level, volatility) get cached for research that
        /// conditions a stock's trades on what the wider market was doing.
        #[arg(long)]
        indices: Option<String>,
    },
    /// Standalone live market-data collector (config's [collector]
    /// section) — records minute candles for a broad watch list (Nifty
    /// 250 + India VIX + near-month NIFTY/BANKNIFTY futures by default)
    /// every trading day, independent of paper trading entirely: no
    /// strategies, no signals, no trades, just data.
    Collect {
        /// Testing/validation only: stop after this many seconds instead of
        /// running forever. Omit for real unattended operation.
        #[arg(long)]
        max_seconds: Option<u64>,
    },
    /// Live snapshot scan: gap % vs yesterday's close (and today's range so
    /// far) across the liquidity-screened universe, right now. This is the
    /// "which stocks are actually moving today" signal that a static
    /// liquidity/ATR screen can't give you — meant to be run right after
    /// open or any time during the session for a fresh read.
    Scan {
        /// How many top movers (by |gap %|) to print.
        #[arg(long, default_value_t = 15)]
        top: usize,
    },
    /// Research mode: hold a custom exit/risk rule fixed and test each
    /// entry strategy in isolation against every instrument we have enough
    /// cached history for, ranked by win rate rather than average R — for
    /// "I'd rather trade rarely and be right most of the time" tuning.
    /// Results are persisted as normal labeled runs (queryable later via
    /// `report`/`compare`) plus a ranked markdown summary file.
    Research {
        /// An instrument needs at least this many cached 1-minute bars to
        /// be included (default ~= a full 6-month history).
        #[arg(long, default_value_t = 40_000)]
        min_bars: i64,
        /// Hard stop, fraction of entry price (e.g. 0.005 = 0.5% underlying
        /// move = 2% capital loss at 4x MIS margin).
        #[arg(long, default_value_t = 0.005)]
        risk_pct: f64,
        /// Multiple of risk_pct the position must reach before the
        /// give-back trail arms (e.g. 3.0 = 1.5% underlying at risk_pct=0.005).
        #[arg(long, default_value_t = 3.0)]
        min_reward_multiple: f64,
        /// Give-back trail distance from peak, same convention as risk_pct
        /// (e.g. 0.005 = 0.5%, one full risk unit behind the peak).
        #[arg(long, default_value_t = 0.005)]
        trail_param: f64,
        /// Only include (strategy, instrument) rows with at least this many
        /// closed trades in the results table — fewer than this is not
        /// enough sample to call a win rate meaningful.
        #[arg(long, default_value_t = 5)]
        min_trades: i64,
        #[arg(long, default_value = "high-winrate")]
        label_prefix: String,
        /// Only replay the strategy variant with this exact name (as shown
        /// in the results table, e.g. "strong_rally") instead of the whole
        /// batch — much faster when iterating on one variant at a time.
        #[arg(long)]
        only: Option<String>,
        /// Comma-separated tradingsymbols to restrict the replay universe
        /// to (e.g. for validating a specific per-stock parameter combo
        /// against just that stock, instead of replaying the full universe
        /// every time).
        #[arg(long)]
        symbols: Option<String>,
    },
    /// Real-money live trading for the VWAP-slope short book
    /// (see `research/tune30/README.md`) — a separate command family from
    /// `run` (the generic paper daemon) since this trades real capital
    /// through real broker orders, single-position-at-a-time, with its own
    /// pre-flight checks and dry-run/live gate.
    LiveTrade {
        #[command(subcommand)]
        action: LiveTradeAction,
    },
}

#[derive(Subcommand)]
enum LiveTradeAction {
    /// Sends one test alert through every configured channel and pings the
    /// dead-man's switch once, so the wiring is proven before it is needed.
    /// Touches nothing else.
    TestAlert {
        /// Also send one realistic example of every alert the engine can
        /// raise, so the formatting and your phone's notification settings
        /// are checked against the real thing rather than a placeholder.
        #[arg(long, default_value_t = false)]
        samples: bool,
    },
    /// Read-only pre-flight: confirms this Kite API key/token can see
    /// trading-account data (margins) and that the order-margin calculator
    /// accepts a sample order shape — *before* any code path that could
    /// place a real order exists in the loop. Places nothing.
    Check,
    /// Runs the configured book for the rest of today's session.
    /// Dry-run (real strategy/risk logic + real live prices, simulated
    /// fills, nothing sent to the broker) unless `--live` is passed —
    /// there is no flag or default that sends a real order by accident.
    Run {
        #[arg(long, default_value_t = false)]
        live: bool,
        /// Rupee capital to allocate per position (used to size quantity
        /// against the broker's own margin calculator for each stock).
        #[arg(long, default_value_t = 100_000.0)]
        capital: f64,
        /// Testing/validation only: stop after this many seconds instead
        /// of running to today's session close.
        #[arg(long)]
        max_seconds: Option<u64>,
    },
    /// Dry-run validation only: fabricates one signal (bypassing real
    /// strategy detection) for a chosen stock and feeds it through the
    /// exact same entry pipeline — margin check, order placement, fill
    /// tracking — then simulates an adverse price move to exercise the
    /// stop-loss's market-order fallback and confirms the exit resolves
    /// and gets logged. Always uses a `SimulatedBroker`; there is no way
    /// to point this at a real broker.
    TestSignal {
        #[arg(long)]
        symbol: String,
        #[arg(long, default_value = "short")]
        direction: String,
    },
    /// Sets the kill-switch: a running `run` stops taking new entries
    /// (still manages any already-open position to a clean exit) until
    /// `resume` is called. Safe to run from another terminal at any time.
    Halt,
    /// Clears the kill-switch.
    Resume,
}

const LIVE_HALT_SENTINEL: &str = "data/live_halt";

#[derive(Subcommand)]
enum BacktestAction {
    /// Screen the curated universe, fetch historical minute candles
    /// (reusing anything already cached in storage), and replay the
    /// enabled strategies through PaperEngine.
    Run {
        /// YYYY-MM-DD. Defaults to 30 days before `to`.
        #[arg(long)]
        from: Option<String>,
        /// YYYY-MM-DD. Defaults to today.
        #[arg(long)]
        to: Option<String>,
        /// Human-readable name for this run (e.g. "orb-relvol-2x") — makes
        /// it findable later without memorizing a numeric run id.
        #[arg(long)]
        label: Option<String>,
        /// Restrict to specific tradingsymbols (repeatable, e.g. `--symbol
        /// ADANIENSOL --symbol ADANIPOWER`) instead of the liquidity-screened
        /// universe — for checking a stock the screen wouldn't normally
        /// include, or isolating one/two names for a focused read.
        #[arg(long = "symbol")]
        symbols: Vec<String>,
    },
}

#[derive(Subcommand)]
enum UniverseAction {
    /// Fetch a constituent list from NSE and cache it. Defaults to
    /// config/default.toml's [universe] seed; pass --index to refresh a
    /// different one (e.g. nifty250 for the collector's watch list).
    SeedRefresh {
        #[arg(long)]
        index: Option<String>,
    },
    /// Screen the (cached) [universe] seed list by liquidity/volatility and print a ranked table.
    Screen,
}

#[derive(Subcommand)]
enum MarketAction {
    /// Fetch the NSE cash-equity instrument list and print a count + sample.
    Instruments,
    /// Fetch a live quote for a tradingsymbol, e.g. INFY.
    Quote { tradingsymbol: String },
    /// Subscribe to live ticks for a tradingsymbol and print them for a while.
    Ticks {
        tradingsymbol: String,
        /// How many seconds to listen before exiting.
        #[arg(long, default_value_t = 30)]
        seconds: u64,
    },
    /// Fetch historical candles for a tradingsymbol.
    History {
        tradingsymbol: String,
        /// YYYY-MM-DD. Defaults to 5 days ago.
        #[arg(long)]
        from: Option<String>,
        /// YYYY-MM-DD. Defaults to today.
        #[arg(long)]
        to: Option<String>,
        /// "minute" or "day".
        #[arg(long, default_value = "minute")]
        interval: String,
    },
}

#[derive(Subcommand)]
enum AuthAction {
    /// Run the full headless login flow and confirm an access token comes back.
    Test,
}

#[derive(Subcommand)]
enum CalendarAction {
    /// Is today a trading day, and what are today's session times?
    Today,
    /// Fetch the current NSE holiday list and refresh the local cache.
    Refresh,
}

fn today_ist() -> chrono::NaiveDate {
    chrono::Local::now()
        .with_timezone(&chrono_tz::Asia::Kolkata)
        .date_naive()
}

async fn kite_rest_client() -> anyhow::Result<kite_client::KiteRestClient> {
    use core_types::LoginProvider;
    let provider = kite_auth::KiteLoginProvider::load_default(std::path::Path::new(
        "secrets/credentials.toml",
    ))?;
    let api_key = provider.api_key().to_string();
    let token = provider.fresh_access_token().await?;
    kite_client::KiteRestClient::new(api_key, token.0)
}

async fn find_instrument(
    rest: &kite_client::KiteRestClient,
    tradingsymbol: &str,
) -> anyhow::Result<core_types::Instrument> {
    let instruments = kite_client::instruments::fetch_nse_equities(rest).await?;
    instruments
        .into_iter()
        .find(|i| i.tradingsymbol.eq_ignore_ascii_case(tradingsymbol))
        .ok_or_else(|| anyhow::anyhow!("{tradingsymbol} not found in NSE cash-equity instruments"))
}

fn parse_ymd(s: &str) -> anyhow::Result<chrono::NaiveDate> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|e| anyhow::anyhow!("expected YYYY-MM-DD, got {s:?}: {e}"))
}

fn ist_bounds(from_date: chrono::NaiveDate, to_date: chrono::NaiveDate) -> (chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>) {
    let from_dt = core_types::time::IST
        .from_local_datetime(&from_date.and_hms_opt(0, 0, 0).unwrap())
        .single()
        .unwrap()
        .with_timezone(&chrono::Utc);
    let to_dt = core_types::time::IST
        .from_local_datetime(&to_date.and_hms_opt(23, 59, 59).unwrap())
        .single()
        .unwrap()
        .with_timezone(&chrono::Utc);
    (from_dt, to_dt)
}

/// Builds the enabled strategies with their config-tuned parameters —
/// see `config/default.toml`'s `[strategies.*]` sections. Tuning a
/// strategy means editing that file and re-running, not recompiling.
fn build_strategies() -> anyhow::Result<Vec<Box<dyn core_types::Strategy>>> {
    Ok(strategies::config::load_default()?.build())
}

/// Screens the cached seed list (Nifty 50 or Nifty 100, per
/// `config/default.toml`'s `[universe] seed`), consuming `rest` into the
/// `MarketDataFeed` it hands back alongside the included instruments —
/// callers that also need raw REST access afterward should build a second
/// client rather than trying to reuse this one.
async fn screened_universe(
    rest: kite_client::KiteRestClient,
) -> anyhow::Result<(kite_client::KiteMarketDataFeed, Vec<core_types::Instrument>)> {
    let config = universe::config::load_default()?;
    let seed = universe::seed::load_default(config.seed_index()?).ok_or_else(|| {
        anyhow::anyhow!("no cached {} list — run `universe seed-refresh` first", config.seed)
    })?;
    let all_equities = kite_client::instruments::fetch_nse_equities(&rest).await?;
    let candidates: Vec<_> = all_equities
        .into_iter()
        .filter(|i| seed.contains(&i.tradingsymbol))
        .collect();

    let feed = kite_client::KiteMarketDataFeed::new(rest);
    let results = universe::run_screen(&feed, &candidates, &config).await;
    let included: Vec<_> = results
        .into_iter()
        .filter(|r| r.included)
        .map(|r| r.instrument)
        .collect();
    Ok((feed, included))
}

/// Best-effort loud failure signal — a macOS notification in addition to
/// the ERROR-level log line, per the plan's "must fail loudly, never
/// silently skip a trading day" requirement. Never fatal if it can't be
/// delivered (headless environment, non-macOS, etc).
/// Resolves this machine's public IPv4 and compares it against
/// `config/ip_whitelist.toml`. A missing or unreadable whitelist file is
/// reported as `Unknown` rather than failing the caller: the file is the local
/// mirror of a console setting, and its absence is a configuration gap to
/// shout about, not proof that the address is wrong.
async fn verify_whitelisted_ip() -> live_trading::IpCheck {
    let path = std::path::Path::new(live_trading::ip_whitelist::DEFAULT_PATH);
    match live_trading::IpWhitelist::load(path) {
        Ok(whitelist) => live_trading::check_ip(&whitelist, kite_client::egress::public_ipv4().await),
        Err(e) => live_trading::IpCheck::Unknown(e.to_string()),
    }
}

/// A desktop banner, which is all this used to be.
///
/// Kept because it is free and instant when someone *is* at the machine, but
/// it is no longer the whole story: see `notify_everywhere`, which also
/// reaches a phone. Anything a person needs to act on within minutes should
/// use that instead.
fn notify_loudly(message: &str) {
    let _ = std::process::Command::new("osascript")
        .arg("-e")
        .arg(format!(
            "display notification {:?} with title \"papertrader\"",
            message
        ))
        .status();
}

/// Running tally of the day, built from the alerts as they go out rather than
/// re-read from the database — what gets reported is then exactly what was
/// reported at the time, with no chance of the two disagreeing.
#[derive(Default)]
struct SessionTally {
    entries: u32,
    exits: u32,
    total_r: f64,
    total_pnl: f64,
    problems: u32,
}

impl SessionTally {
    fn record(&mut self, alert: &live_trading::Alert) {
        match alert {
            live_trading::Alert::Entry { .. } => self.entries += 1,
            live_trading::Alert::Exit { r_multiple, pnl_abs, .. } => {
                self.exits += 1;
                self.total_r += r_multiple;
                self.total_pnl += pnl_abs;
            }
            a if a.is_critical() => self.problems += 1,
            _ => {}
        }
    }

    fn summary(&self, flat: bool) -> String {
        let mut s = if self.exits == 0 {
            format!("Session ended — {} trades taken", self.entries)
        } else {
            format!(
                "Session ended — {} trades, {:+.2}R, Rs.{:+.0}",
                self.exits, self.total_r, self.total_pnl
            )
        };
        if self.problems > 0 {
            s.push_str(&format!("\n{} problem(s) flagged during the session", self.problems));
        }
        if !flat {
            s.push_str("\nNOT FLAT — a position may still be open. Check Kite.");
        }
        s
    }
}

/// Drains whatever the engine has decided is worth telling a human about and
/// sends it. Called after every bar and on each housekeeping tick, so an alert
/// queued by a bar that arrived during a quiet stretch still goes out promptly.
async fn flush_alerts(
    alerter: &alerts::Alerter,
    tally: &mut SessionTally,
    alerts_out: Vec<live_trading::Alert>,
) {
    for alert in alerts_out {
        tally.record(&alert);
        let message = alert.message();
        tracing::info!(critical = alert.is_critical(), alert = %message.replace('\n', " | "), "ALERT");
        if alert.is_critical() {
            // Critical alerts also raise a desktop banner: if someone *is* at
            // the machine, the fastest path to them is the screen in front of
            // them, not a phone in another room.
            notify_loudly(&message.replace('\n', " — "));
        }
        alerter.push(&message).await;
    }
}

/// Desktop banner *and* phone push.
///
/// Every alert worth waking someone up for goes through here. A banner alone
/// assumes the person is sitting in front of this Mac, which during a trading
/// session is exactly the assumption that has already failed once.
async fn notify_everywhere(alerter: &alerts::Alerter, message: &str) {
    notify_loudly(message);
    alerter.push(message).await;
}

async fn auth_refresh_with_retries() -> anyhow::Result<(kite_auth::KiteLoginProvider, String)> {
    use core_types::LoginProvider;
    const MAX_ATTEMPTS: u32 = 3;

    let provider =
        kite_auth::KiteLoginProvider::load_default(std::path::Path::new("secrets/credentials.toml"))?;
    let mut last_err = None;
    for attempt in 1..=MAX_ATTEMPTS {
        match provider.fresh_access_token().await {
            Ok(token) => return Ok((provider, token.0)),
            Err(e) => {
                tracing::error!(attempt, error = %e, "Kite auth refresh attempt failed");
                last_err = Some(e);
                if attempt < MAX_ATTEMPTS {
                    tokio::time::sleep(std::time::Duration::from_secs(30 * attempt as u64)).await;
                }
            }
        }
    }
    let err = anyhow::anyhow!(
        "Kite auth refresh failed after {MAX_ATTEMPTS} attempts: {:?}",
        last_err.unwrap()
    );
    notify_loudly(&format!("Auth refresh failed: {err}"));
    Err(err)
}

/// The live daemon loop: sleep until market open (holiday-aware), refresh
/// auth, screen the universe, trade through market close via live ticks
/// aggregated into 1-minute candles, repeat. `max_seconds` bounds runtime
/// for validation — real unattended operation omits it.
async fn run_daemon(max_seconds: Option<u64>) -> anyhow::Result<()> {
    use core_types::{MarketDataFeed, TradingCalendar};

    let deadline = max_seconds.map(|s| tokio::time::Instant::now() + std::time::Duration::from_secs(s));
    let past_deadline = || deadline.is_some_and(|dl| tokio::time::Instant::now() >= dl);

    loop {
        if past_deadline() {
            println!("max_seconds reached, stopping.");
            return Ok(());
        }

        let calendar = market_calendar::NseCalendar::load_default()?;
        let now = chrono::Utc::now();
        let wake = scheduler::next_wake_time(&calendar, now);
        if wake > now {
            let sleep_dur = (wake - now).to_std().unwrap_or_default();
            println!(
                "Sleeping until {} IST...",
                wake.with_timezone(&core_types::time::IST).format("%Y-%m-%d %H:%M")
            );
            if let Some(dl) = deadline {
                if tokio::time::Instant::now() + sleep_dur >= dl {
                    println!("Next wake time is beyond the testing window, stopping.");
                    return Ok(());
                }
            }
            tokio::time::sleep(sleep_dur).await;
        }

        println!("Market session starting — refreshing Kite auth...");
        let (provider, access_token) = auth_refresh_with_retries().await?;
        println!("Auth OK.");

        let rest = kite_client::KiteRestClient::new(provider.api_key().to_string(), access_token)?;
        let (feed, instruments) = screened_universe(rest).await?;
        if instruments.is_empty() {
            anyhow::bail!("universe screen returned zero instruments — check config/default.toml [universe] thresholds");
        }
        println!("Trading {} instruments today.", instruments.len());
        let instrument_by_token: std::collections::HashMap<u64, core_types::Instrument> =
            instruments.iter().map(|i| (i.instrument_token, i.clone())).collect();

        let today = chrono::Utc::now().with_timezone(&core_types::time::IST).date_naive();
        let indicator_engine =
            strategy_core::IndicatorEngine::new(chrono::NaiveTime::from_hms_opt(9, 30, 0).unwrap());
        let runner = strategy_core::StrategyRunner::new(indicator_engine, build_strategies()?);
        let storage = storage::SqliteStorage::open(std::path::Path::new("data/papertrader.db"))?;
        storage.upsert_instruments(&instruments)?;
        let (engine_config, config_snapshot) = paper_engine::config_loader::load_default()?;
        let live_label = format!("live-{today}");
        let mut engine = paper_engine::PaperEngine::new(
            runner,
            Box::new(risk::DefaultExitManager),
            Box::new(storage),
            engine_config,
            core_types::NewRun {
                mode: core_types::RunMode::Live,
                config_snapshot_json: config_snapshot,
                label: Some(live_label.clone()),
                from_date: Some(today),
                to_date: Some(today),
            },
        )?;
        println!("Live run {} started (label: {live_label}).", engine.run_id());

        let mut ticks = feed.subscribe_ticks(&instruments).await?;
        let mut aggregator = paper_engine::CandleAggregator::new();

        let session_close = calendar
            .session_close(today)
            .ok_or_else(|| anyhow::anyhow!("today isn't a trading day after waking up for it?"))?;

        loop {
            let now = chrono::Utc::now();
            if now >= session_close || past_deadline() {
                break;
            }
            let until_close = (session_close - now).to_std().unwrap_or_default();
            tokio::select! {
                _ = tokio::time::sleep(until_close) => break,
                maybe_tick = ticks.next() => {
                    match maybe_tick {
                        Some(t) => {
                            if let Some((token, candle)) = aggregator.ingest(&t) {
                                if let Some(inst) = instrument_by_token.get(&token) {
                                    if let Err(e) = engine.on_bar(inst, &candle) {
                                        tracing::error!(error = %e, "engine.on_bar failed, continuing");
                                    }
                                }
                            }
                        }
                        None => {
                            tracing::error!("tick stream ended unexpectedly");
                            notify_loudly("Live tick stream ended unexpectedly");
                            break;
                        }
                    }
                }
            }
        }

        for (token, candle) in aggregator.flush() {
            if let Some(inst) = instrument_by_token.get(&token) {
                let _ = engine.on_bar(inst, &candle);
            }
        }
        engine.end_run()?;
        println!("Live run ended for {today}. Open positions: {}", engine.open_position_count());

        if past_deadline() {
            return Ok(());
        }
        // Loop back — next_wake_time will compute tomorrow's open.
    }
}

#[derive(Debug, serde::Deserialize)]
struct RawCollectorConfig {
    seed: String,
    include_india_vix: bool,
    include_nifty_futures: bool,
    include_banknifty_futures: bool,
}

#[derive(Debug, serde::Deserialize)]
struct CollectorRootConfig {
    collector: RawCollectorConfig,
}

fn load_collector_config() -> anyhow::Result<RawCollectorConfig> {
    let text = std::fs::read_to_string("config/default.toml")
        .map_err(|e| anyhow::anyhow!("reading config/default.toml: {e}"))?;
    let root: CollectorRootConfig =
        toml::from_str(&text).map_err(|e| anyhow::anyhow!("parsing [collector] section: {e}"))?;
    Ok(root.collector)
}

/// Assembles the collector's watch list: the full (unscreened) seed index
/// plus whichever indices/futures are enabled in config — independent of
/// the [universe] trading screen entirely.
async fn collection_targets(
    rest: &kite_client::KiteRestClient,
    cfg: &RawCollectorConfig,
) -> anyhow::Result<Vec<core_types::Instrument>> {
    let seed_index = universe::SeedIndex::parse(&cfg.seed)?;
    let seed = universe::seed::load_default(seed_index).ok_or_else(|| {
        anyhow::anyhow!("no cached {} list — run `universe seed-refresh --index {}` first", cfg.seed, cfg.seed)
    })?;

    let all_equities = kite_client::instruments::fetch_nse_equities(rest).await?;
    let mut targets: Vec<_> = all_equities.into_iter().filter(|i| seed.contains(&i.tradingsymbol)).collect();
    let matched: std::collections::HashSet<_> = targets.iter().map(|i| i.tradingsymbol.clone()).collect();
    let missing: Vec<_> = seed.iter().filter(|s| !matched.contains(*s)).collect();
    if !missing.is_empty() {
        println!(
            "Warning: {} of {} {} symbols not found in the NSE instrument list: {:?}",
            missing.len(),
            seed.len(),
            cfg.seed,
            missing
        );
    }

    if cfg.include_india_vix {
        let indices = kite_client::instruments::fetch_nse_indices(rest).await?;
        match indices.into_iter().find(|i| i.tradingsymbol == "INDIA VIX") {
            Some(vix) => targets.push(vix),
            None => println!("Warning: INDIA VIX not found in the NSE indices list"),
        }
    }

    if cfg.include_nifty_futures || cfg.include_banknifty_futures {
        let futures = kite_client::instruments::fetch_nfo_futures(rest).await?;
        let today = today_ist();
        if cfg.include_nifty_futures {
            match kite_client::instruments::current_month_contract(&futures, "NIFTY", today) {
                Some(f) => {
                    println!("Current NIFTY futures contract: {}", f.tradingsymbol);
                    targets.push(f);
                }
                None => println!("Warning: no current NIFTY futures contract found"),
            }
        }
        if cfg.include_banknifty_futures {
            match kite_client::instruments::current_month_contract(&futures, "BANKNIFTY", today) {
                Some(f) => {
                    println!("Current BANKNIFTY futures contract: {}", f.tradingsymbol);
                    targets.push(f);
                }
                None => println!("Warning: no current BANKNIFTY futures contract found"),
            }
        }
    }

    Ok(targets)
}

/// Standalone live data collector: records 1-minute candles for a broad
/// watch list every trading day, entirely independent of paper trading —
/// no strategies, no signals, no trades, no PaperEngine at all. Mirrors
/// `run_daemon`'s sleep/auth/EOD scheduling structure (identical concern),
/// diverging only in what happens with each finalized candle.
async fn run_collector(max_seconds: Option<u64>) -> anyhow::Result<()> {
    use core_types::{MarketDataFeed, TradingCalendar};

    let collector_alerter = alerts::Alerter::load_default();

    let deadline = max_seconds.map(|s| tokio::time::Instant::now() + std::time::Duration::from_secs(s));
    let past_deadline = || deadline.is_some_and(|dl| tokio::time::Instant::now() >= dl);

    loop {
        if past_deadline() {
            println!("max_seconds reached, stopping.");
            return Ok(());
        }

        let calendar = market_calendar::NseCalendar::load_default()?;
        let now = chrono::Utc::now();
        let wake = scheduler::next_wake_time(&calendar, now);
        if wake > now {
            let sleep_dur = (wake - now).to_std().unwrap_or_default();
            println!(
                "Sleeping until {} IST...",
                wake.with_timezone(&core_types::time::IST).format("%Y-%m-%d %H:%M")
            );
            if let Some(dl) = deadline {
                if tokio::time::Instant::now() + sleep_dur >= dl {
                    println!("Next wake time is beyond the testing window, stopping.");
                    return Ok(());
                }
            }
            tokio::time::sleep(sleep_dur).await;
        }

        println!("Collection session starting — refreshing Kite auth...");
        let (provider, access_token) = auth_refresh_with_retries().await?;
        println!("Auth OK.");

        let rest = kite_client::KiteRestClient::new(provider.api_key().to_string(), access_token)?;
        let cfg = load_collector_config()?;
        let targets = collection_targets(&rest, &cfg).await?;
        if targets.is_empty() {
            anyhow::bail!("collector target list is empty — check [collector] config and cached seed lists");
        }
        println!("Collecting {} instruments today.", targets.len());

        let storage = storage::SqliteStorage::open(std::path::Path::new("data/papertrader.db"))?;
        storage.upsert_instruments(&targets)?;

        let feed = kite_client::KiteMarketDataFeed::new(rest);
        let mut ticks = feed.subscribe_ticks(&targets).await?;
        let mut aggregator = paper_engine::CandleAggregator::new();

        let today = chrono::Utc::now().with_timezone(&core_types::time::IST).date_naive();
        let session_close = calendar
            .session_close(today)
            .ok_or_else(|| anyhow::anyhow!("today isn't a trading day after waking up for it?"))?;

        let mut candle_count = 0u64;
        loop {
            let now = chrono::Utc::now();
            if now >= session_close || past_deadline() {
                break;
            }
            let until_close = (session_close - now).to_std().unwrap_or_default();
            tokio::select! {
                _ = tokio::time::sleep(until_close) => break,
                maybe_tick = ticks.next() => {
                    match maybe_tick {
                        Some(t) => {
                            if let Some((token, candle)) = aggregator.ingest(&t) {
                                match storage.insert_candles(token, core_types::CandleInterval::OneMinute, &[candle]) {
                                    Ok(()) => candle_count += 1,
                                    Err(e) => tracing::error!(error = %e, "failed to persist candle, continuing"),
                                }
                            }
                        }
                        None => {
                            tracing::error!("tick stream ended unexpectedly");
                            // The collector has died silently before — an
                            // established socket recording nothing for 42
                            // minutes, found by chance rather than by alert.
                            notify_everywhere(&collector_alerter, "Collector tick stream ended unexpectedly").await;
                            break;
                        }
                    }
                }
            }
        }

        for (token, candle) in aggregator.flush() {
            match storage.insert_candles(token, core_types::CandleInterval::OneMinute, &[candle]) {
                Ok(()) => candle_count += 1,
                Err(e) => tracing::error!(error = %e, "failed to persist final candle"),
            }
        }
        println!(
            "Collection ended for {today}. {candle_count} candles recorded across {} instruments.",
            targets.len()
        );

        if past_deadline() {
            return Ok(());
        }
        // Loop back — next_wake_time will compute tomorrow's open.
    }
}

fn open_storage_readonly() -> anyhow::Result<storage::SqliteStorage> {
    storage::SqliteStorage::open(std::path::Path::new("data/papertrader.db"))
}

fn run_label_str(run_id: i64, label: &Option<String>) -> String {
    match label {
        Some(l) => format!("#{run_id} {l}"),
        None => format!("#{run_id}"),
    }
}

/// Default view: per run x per strategy, rolled up across instruments
/// (correctly trade-weighted, not an average of averages — see
/// `strategy_summary`'s doc comment). `--by-instrument` switches to the
/// finer breakdown where stock-specific edges actually show up.
fn print_report(run: Option<i64>, by_instrument: bool) -> anyhow::Result<()> {
    let storage = open_storage_readonly()?;

    if by_instrument {
        let rows = storage.strategy_performance(run)?;
        if rows.is_empty() {
            println!("No closed trades in data/papertrader.db yet (for this filter).");
            return Ok(());
        }
        println!(
            "\n{:<8} {:<20} {:<12} {:<10} {:<12} {:>7} {:>10} {:>8} {:>9}",
            "RUN", "STRATEGY", "WINDOW", "MODE", "SYMBOL", "TRADES", "AVG PNL%", "AVG R", "WIN RATE"
        );
        for r in &rows {
            let symbol = r.tradingsymbol.clone().unwrap_or_else(|| format!("#{}", r.instrument_token));
            println!(
                "{:<8} {:<20} {:<12} {:<10} {:<12} {:>7} {:>10.2} {:>8.2} {:>8.1}%",
                run_label_str(r.run_id, &r.run_label),
                r.strategy_id,
                r.window,
                r.mode,
                symbol,
                r.trades,
                r.avg_pnl_pct,
                r.avg_r,
                r.win_rate * 100.0,
            );
        }
    } else {
        let run_ids = run.map(|id| vec![id]);
        let rows = storage.strategy_summary(run_ids.as_deref())?;
        if rows.is_empty() {
            println!("No closed trades in data/papertrader.db yet (for this filter).");
            return Ok(());
        }
        println!(
            "\n{:<8} {:<20} {:<10} {:>7} {:>10} {:>8} {:>9}",
            "RUN", "STRATEGY", "MODE", "TRADES", "AVG PNL%", "AVG R", "WIN RATE"
        );
        for r in &rows {
            println!(
                "{:<8} {:<20} {:<10} {:>7} {:>10.2} {:>8.2} {:>8.1}%",
                run_label_str(r.run_id, &r.run_label),
                r.strategy_id,
                r.mode,
                r.trades,
                r.avg_pnl_pct,
                r.avg_r,
                r.win_rate * 100.0,
            );
        }
        println!("\n(pass --by-instrument to see the per-symbol breakdown)");
    }
    Ok(())
}

/// Puts the given runs' per-strategy summaries side by side, sorted so the
/// same strategy's different runs sit next to each other — the direct
/// answer to "did tuning that parameter help."
fn print_compare(run_ids: Vec<i64>) -> anyhow::Result<()> {
    let storage = open_storage_readonly()?;
    let filter = if run_ids.is_empty() { None } else { Some(run_ids.as_slice()) };
    let rows = storage.strategy_summary(filter)?;
    if rows.is_empty() {
        println!("No matching closed trades in data/papertrader.db.");
        return Ok(());
    }
    println!(
        "\n{:<20} {:<10} {:<10} {:>7} {:>10} {:>8} {:>9}",
        "STRATEGY", "RUN", "MODE", "TRADES", "AVG PNL%", "AVG R", "WIN RATE"
    );
    let mut last_strategy: Option<String> = None;
    for r in &rows {
        if last_strategy.as_deref() != Some(r.strategy_id.as_str()) {
            if last_strategy.is_some() {
                println!();
            }
            last_strategy = Some(r.strategy_id.clone());
        }
        println!(
            "{:<20} {:<10} {:<10} {:>7} {:>10.2} {:>8.2} {:>8.1}%",
            r.strategy_id,
            run_label_str(r.run_id, &r.run_label),
            r.mode,
            r.trades,
            r.avg_pnl_pct,
            r.avg_r,
            r.win_rate * 100.0,
        );
    }
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Defaults to `info` rather than to nothing.
    //
    // `from_default_env()` with `RUST_LOG` unset builds an *empty* filter,
    // which discards every event at every level — including `error!`. So the
    // ENTRY/EXIT/SIGNAL audit trail, and every failure the code carefully
    // logs instead of panicking on, went nowhere unless someone remembered to
    // export RUST_LOG. Verified on 2026-09-30: zero log lines without it, the
    // full trail with it.
    //
    // `RUST_LOG` still wins when set, so `RUST_LOG=debug` and friends work as
    // before.
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let cli = Cli::parse();
    match cli.command {
        Command::Calendar { action } => match action {
            CalendarAction::Today => {
                let cal = market_calendar::NseCalendar::load_default()?;
                cal.print_today(today_ist());
            }
            CalendarAction::Refresh => {
                let cal = market_calendar::NseCalendar::refresh_default().await?;
                println!("Holiday cache refreshed.");
                cal.print_today(today_ist());
            }
        },
        Command::Auth { action } => match action {
            AuthAction::Test => {
                use core_types::LoginProvider;
                let provider = kite_auth::KiteLoginProvider::load_default(std::path::Path::new(
                    "secrets/credentials.toml",
                ))?;
                let token = provider.fresh_access_token().await?;
                let masked = if token.0.len() > 4 {
                    format!("{}...", &token.0[..4])
                } else {
                    "***".to_string()
                };
                println!("Got a Kite access token: {masked}");
            }
        },
        Command::Market { action } => match action {
            MarketAction::Instruments => {
                let rest = kite_rest_client().await?;
                let instruments = kite_client::instruments::fetch_nse_equities(&rest).await?;
                println!("{} NSE cash-equity instruments", instruments.len());
                for i in instruments.iter().take(5) {
                    println!("  {} (token {})", i.tradingsymbol, i.instrument_token);
                }
                let storage = storage::SqliteStorage::open(std::path::Path::new("data/papertrader.db"))?;
                storage.upsert_instruments(&instruments)?;
                println!("Upserted all {} into data/papertrader.db's instruments table.", instruments.len());
            }
            MarketAction::Quote { tradingsymbol } => {
                use core_types::MarketDataFeed;
                let rest = kite_rest_client().await?;
                let instrument = find_instrument(&rest, &tradingsymbol).await?;
                let feed = kite_client::KiteMarketDataFeed::new(rest);
                let q = feed.quote(&instrument).await?;
                println!(
                    "{}: {} @ {}",
                    tradingsymbol,
                    q.last_price,
                    q.ts.with_timezone(&core_types::time::IST)
                );
            }
            MarketAction::Ticks { tradingsymbol, seconds } => {
                use core_types::MarketDataFeed;
                let rest = kite_rest_client().await?;
                let instrument = find_instrument(&rest, &tradingsymbol).await?;
                let feed = kite_client::KiteMarketDataFeed::new(rest);
                let mut ticks = feed.subscribe_ticks(std::slice::from_ref(&instrument)).await?;
                println!("Listening for {tradingsymbol} ticks for {seconds}s...");
                let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(seconds);
                let mut count = 0u32;
                loop {
                    tokio::select! {
                        _ = tokio::time::sleep_until(deadline) => break,
                        tick = ticks.next() => {
                            match tick {
                                Some(t) => {
                                    count += 1;
                                    println!(
                                        "{} last={:.2} vol={} ts={}",
                                        tradingsymbol, t.last_price, t.volume_traded,
                                        t.ts.with_timezone(&core_types::time::IST)
                                    );
                                }
                                None => break,
                            }
                        }
                    }
                }
                println!("Received {count} ticks.");
            }
            MarketAction::History {
                tradingsymbol,
                from,
                to,
                interval,
            } => {
                use core_types::MarketDataFeed;
                let to_date = to.map(|s| parse_ymd(&s)).transpose()?.unwrap_or_else(today_ist);
                let from_date = from
                    .map(|s| parse_ymd(&s))
                    .transpose()?
                    .unwrap_or_else(|| to_date - chrono::Duration::days(5));
                let interval = match interval.as_str() {
                    "minute" => core_types::CandleInterval::OneMinute,
                    "day" => core_types::CandleInterval::Day,
                    other => anyhow::bail!("unknown interval {other:?}, expected minute|day"),
                };

                let rest = kite_rest_client().await?;
                let instrument = find_instrument(&rest, &tradingsymbol).await?;
                let feed = kite_client::KiteMarketDataFeed::new(rest);
                let from_dt = core_types::time::IST
                    .from_local_datetime(&from_date.and_hms_opt(0, 0, 0).unwrap())
                    .single()
                    .unwrap()
                    .with_timezone(&chrono::Utc);
                let to_dt = core_types::time::IST
                    .from_local_datetime(&to_date.and_hms_opt(23, 59, 59).unwrap())
                    .single()
                    .unwrap()
                    .with_timezone(&chrono::Utc);
                let candles = feed
                    .historical_candles(&instrument, from_dt, to_dt, interval)
                    .await?;
                println!("{} candles for {tradingsymbol} ({from_date}..{to_date})", candles.len());
                if let Some(first) = candles.first() {
                    println!("  first: {first:?}");
                }
                if let Some(last) = candles.last() {
                    println!("  last:  {last:?}");
                }
            }
        },
        Command::Universe { action } => match action {
            UniverseAction::SeedRefresh { index } => {
                let (label, seed_index) = match index {
                    Some(s) => (s.clone(), universe::SeedIndex::parse(&s)?),
                    None => {
                        let config = universe::config::load_default()?;
                        (config.seed.clone(), config.seed_index()?)
                    }
                };
                let symbols = universe::seed::refresh_default(seed_index).await?;
                println!("{label} constituent list refreshed: {} symbols", symbols.len());
            }
            UniverseAction::Screen => {
                let config = universe::config::load_default()?;
                let seed = universe::seed::load_default(config.seed_index()?).ok_or_else(|| {
                    anyhow::anyhow!("no cached {} list — run `universe seed-refresh` first", config.seed)
                })?;

                let rest = kite_rest_client().await?;
                let all_equities = kite_client::instruments::fetch_nse_equities(&rest).await?;
                let candidates: Vec<_> = all_equities
                    .into_iter()
                    .filter(|i| seed.contains(&i.tradingsymbol))
                    .collect();
                println!(
                    "Screening {} of {} Nifty 50 symbols found in the NSE instrument list...",
                    candidates.len(),
                    seed.len()
                );

                let feed = kite_client::KiteMarketDataFeed::new(rest);
                let results = universe::run_screen(&feed, &candidates, &config).await;

                println!(
                    "\n{:<12} {:>6} {:>16} {:>10} {:>14} {:>9}",
                    "SYMBOL", "RANK", "AVG TURNOVER CR", "ATR %", "AVG VOLUME", "INCLUDED"
                );
                for r in &results {
                    println!(
                        "{:<12} {:>6} {:>16.2} {:>10.2} {:>14.0} {:>9}",
                        r.instrument.tradingsymbol,
                        r.rank,
                        r.avg_turnover_cr,
                        r.atr_pct,
                        r.avg_volume,
                        if r.included { "yes" } else { "no" },
                    );
                }
                let included_count = results.iter().filter(|r| r.included).count();
                println!("\n{included_count} of {} symbols passed the screen.", results.len());
            }
        },
        Command::Backtest { action } => match action {
            BacktestAction::Run { from, to, label, symbols } => {
                let to_date = to.map(|s| parse_ymd(&s)).transpose()?.unwrap_or_else(today_ist);
                let from_date = from
                    .map(|s| parse_ymd(&s))
                    .transpose()?
                    .unwrap_or_else(|| to_date - chrono::Duration::days(30));

                let storage = storage::SqliteStorage::open(std::path::Path::new("data/papertrader.db"))?;
                let rest = kite_rest_client().await?;
                let (feed, instruments) = if symbols.is_empty() {
                    screened_universe(rest).await?
                } else {
                    let mut picked = Vec::with_capacity(symbols.len());
                    for s in &symbols {
                        picked.push(find_instrument(&rest, s).await?);
                    }
                    (kite_client::KiteMarketDataFeed::new(rest), picked)
                };
                println!("Backtesting {} instruments from {from_date} to {to_date}...", instruments.len());

                let (engine_config, config_snapshot) = paper_engine::config_loader::load_default()?;
                let run_id = backtest::run_backtest_job(
                    storage,
                    &feed,
                    build_strategies()?,
                    backtest::BacktestJobParams {
                        from: from_date,
                        to: to_date,
                        label: label.clone(),
                        instruments,
                        engine_config,
                        config_snapshot_json: config_snapshot,
                    },
                    |p| println!("  [{}/{}] {}", p.current, p.total, p.message),
                )
                .await?;
                println!("Backtest run {} complete.", run_label_str(run_id, &label));

                print_report(Some(run_id), false)?;
            }
        },
        Command::Report { run, by_instrument } => print_report(run, by_instrument)?,
        Command::Compare { runs } => print_compare(runs)?,
        Command::Run { max_seconds } => run_daemon(max_seconds).await?,
        Command::Backfill { months, index, to, from, symbols, indices } => run_backfill(months, index, to, from, symbols, indices).await?,
        Command::Collect { max_seconds } => run_collector(max_seconds).await?,
        Command::Scan { top } => run_scan(top).await?,
        Command::Research {
            min_bars,
            risk_pct,
            min_reward_multiple,
            trail_param,
            min_trades,
            label_prefix,
            only,
            symbols,
        } => {
            run_research(min_bars, risk_pct, min_reward_multiple, trail_param, min_trades, label_prefix, only, symbols).await?
        }
        Command::Notify { message, title } => run_notify(message, title).await?,
        Command::Shadow { capital, max_seconds } => run_shadow(capital, max_seconds).await?,
        Command::LiveTrade { action } => match action {
            LiveTradeAction::TestAlert { samples } => run_test_alert(samples).await?,
            LiveTradeAction::Check => run_live_trade_check().await?,
            LiveTradeAction::Run { live, capital, max_seconds } => run_live_trade(live, capital, max_seconds).await?,
            LiveTradeAction::TestSignal { symbol, direction } => run_live_trade_test_signal(symbol, direction).await?,
            LiveTradeAction::Halt => {
                std::fs::write(LIVE_HALT_SENTINEL, b"halted by `live-trade halt`\n")?;
                println!("Halt sentinel written to {LIVE_HALT_SENTINEL} — a running `live-trade run` will stop taking new entries within one bar.");
            }
            LiveTradeAction::Resume => {
                if std::fs::remove_file(LIVE_HALT_SENTINEL).is_ok() {
                    println!("Halt sentinel removed — a running `live-trade run` will resume within one bar.");
                } else {
                    println!("No halt sentinel was present.");
                }
            }
        },
    }
    Ok(())
}

/// Ranks the liquidity-screened universe by today's live gap % vs
/// yesterday's close — the most useful "what to watch today" signal right
/// at/after open, before enough intraday volume/momentum has accumulated
/// for the strategies themselves to have an opinion. Two Kite REST clients
/// are built (screening consumes one into a `MarketDataFeed`); token
/// fetching is cheap/cached so this costs nothing extra.
/// Read-only pre-flight for real-money live trading: confirms (a) this API
/// key/token can see trading-account margin data at all, and (b) Kite's own
/// order-margin calculator accepts a realistic sample order for one of the
/// configured book — proving order placement is provisioned on this account
/// *before* any code that could submit a real order exists in the loop.
/// Places nothing; the sample order never reaches `/orders`.
/// Runs the configured book for the remainder of today's session.
/// Dry-run (`live=false`, the default from the CLI) drives real strategy
/// and risk logic against real live prices through a `SimulatedBroker` —
/// nothing ever reaches Kite. `live=true` is the only path that
/// constructs a `KiteBroker` at all.
async fn run_live_trade(live: bool, capital: f64, max_seconds: Option<u64>) -> anyhow::Result<()> {
    use core_types::{LoginProvider, MarketDataFeed, Storage, TradingCalendar};
    use futures::StreamExt;

    let tune30 = live_trading::Tune30Config::load(std::path::Path::new("config/tune30_live.toml"))?;
    println!(
        "{} mode — {} stocks, ~Rs.{capital:.0} capital per position, risk_pct={:.3}%",
        if live { "LIVE (real money)" } else { "DRY-RUN (simulated fills)" },
        tune30.stocks.len(),
        tune30.risk_pct * 100.0,
    );
    if live {
        println!("*** THIS WILL PLACE REAL ORDERS WITH REAL MONEY. Ctrl-C now to abort. ***");
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }

    let calendar = market_calendar::NseCalendar::load_default()?;
    let today = chrono::Utc::now().with_timezone(&core_types::time::IST).date_naive();
    if !calendar.is_trading_day(today) {
        anyhow::bail!("{today} is not a trading day");
    }
    let session_close = calendar
        .session_close(today)
        .ok_or_else(|| anyhow::anyhow!("no session close time for {today}"))?;
    // 15:10, not 15:15. Zerodha stops accepting MIS orders at 15:12 and
    // squares off the position itself — so a 15:15 exit is refused outright
    // and the broker picks the price. Observed live on 2026-09-30; see
    // `live_trading::session` for the full timeline.
    let eod_square_off = chrono::NaiveTime::from_hms_opt(15, 10, 0).unwrap();

    // Limit at 15:10, market at 15:11, backstop at 15:12 — all ahead of the
    // broker cutoff. See `live_trading::session`.
    let loop_end = live_trading::loop_end(today, eod_square_off, session_close)
        .ok_or_else(|| anyhow::anyhow!("ambiguous IST square-off cutoff for {today}"))?;
    let in_ist = |t: core_types::DateTime| t.with_timezone(&core_types::time::IST).format("%H:%M:%S");
    if chrono::Utc::now() >= loop_end {
        anyhow::bail!("it is already past the {} IST square-off cutoff for {today}", in_ist(loop_end));
    }
    println!(
        "Loop ends {} IST — the square-off backstop runs then, ahead of the broker's MIS auto-close.",
        in_ist(loop_end),
    );

    let mut alerter = alerts::Alerter::load_default();
    let mut tally = SessionTally::default();
    println!("Alerting — {}.", alerter.describe());
    if live && !(alerter.has_push() && alerter.has_heartbeat()) {
        // Say it plainly rather than leaving it to be discovered during an
        // outage. Without a push channel nothing reaches a phone; without the
        // dead-man's switch nothing outside this process can tell that it
        // stopped running at all.
        println!("WARNING: alerting is incomplete — {}.", alerter.describe());
        if !alerter.has_push() {
            println!("         No phone channel: alerts stay on this Mac's screen.");
        }
        if !alerter.has_heartbeat() {
            println!("         No dead-man's switch: if this process dies, nothing will tell you.");
        }
        println!("         See {} to finish setting it up.", alerts::DEFAULT_PATH);
    }

    // Before anything else costly: is this machine's egress address still the
    // one Kite will accept orders from? On dynamic broadband it changes on a
    // re-dial, and the whitelist cannot be corrected quickly (one change per
    // calendar week), so discovering it at 09:14 is worth a great deal more
    // than discovering it when the day's first signal is rejected.
    match verify_whitelisted_ip().await {
        live_trading::IpCheck::Match(ip) => println!("Egress IP {ip} matches the Kite whitelist."),
        live_trading::IpCheck::Unknown(why) => {
            // Not knowing the address is not evidence of a mismatch, and an
            // echo service being down must not cost a trading day.
            tracing::warn!(reason = %why, "could not determine egress IP — continuing unverified");
            println!("WARNING: could not determine this machine's egress IP ({why}).");
            println!("         Continuing unverified — if orders start getting rejected, this is why.");
        }
        live_trading::IpCheck::Mismatch { report, .. } => {
            notify_everywhere(&alerter, "Egress IP is not whitelisted at Kite — live session refused to start").await;
            if live {
                anyhow::bail!("{report}");
            }
            // A dry run places nothing, so a mismatch cannot hurt it — but it
            // is still the one warning worth carrying into the day.
            println!("WARNING (dry-run, not fatal): {report}");
        }
    }

    println!("Refreshing Kite auth...");
    let (provider, access_token) = auth_refresh_with_retries().await?;
    let rest = kite_client::KiteRestClient::new(provider.api_key().to_string(), access_token)?;

    let symbols = tune30.symbols();
    let all_equities = kite_client::instruments::fetch_nse_equities(&rest).await?;
    let instruments: Vec<_> = all_equities
        .into_iter()
        .filter(|i| symbols.iter().any(|s: &String| s.eq_ignore_ascii_case(&i.tradingsymbol)))
        .collect();
    if instruments.len() != symbols.len() {
        let found: std::collections::HashSet<_> = instruments.iter().map(|i| i.tradingsymbol.clone()).collect();
        let missing: Vec<_> = symbols.iter().filter(|s| !found.contains(*s)).collect();
        anyhow::bail!("could not resolve every tuned symbol against NSE instruments — missing: {missing:?}");
    }
    println!("Resolved {} instruments.", instruments.len());

    let storage = storage::SqliteStorage::open(std::path::Path::new("data/papertrader.db"))?;
    storage.upsert_instruments(&instruments)?;
    let label = format!("live-trade/{}-{today}", if live { "live" } else { "dryrun" });
    let config_snapshot = serde_json::json!({
        "live": live,
        "capital": capital,
        "risk_pct": tune30.risk_pct,
        "stocks": tune30.stocks.iter().map(|s| serde_json::json!({"symbol": s.symbol, "lookback": s.lookback, "mrm": s.mrm, "trail": s.trail})).collect::<Vec<_>>(),
    })
    .to_string();
    let run_id = storage.start_run(&core_types::NewRun {
        mode: core_types::RunMode::Live,
        config_snapshot_json: config_snapshot,
        label: Some(label.clone()),
        from_date: Some(today),
        to_date: Some(today),
    })?;
    println!("Run {} started (label: {label}).", run_id);
    let storage: Box<dyn core_types::Storage> = Box::new(storage);

    let tuning_by_symbol = tune30.by_symbol();
    let stocks = live_trading::build_stock_runtimes(&instruments, &tuning_by_symbol);
    let clock: std::sync::Arc<dyn live_trading::Clock> = std::sync::Arc::new(live_trading::RealClock::new());

    let instrument_by_token: std::collections::HashMap<u64, core_types::Instrument> =
        instruments.iter().map(|i| (i.instrument_token, i.clone())).collect();
    let feed = kite_client::KiteMarketDataFeed::new(rest);
    let mut ticks = feed.subscribe_ticks(&instruments).await?;
    let mut aggregator = paper_engine::CandleAggregator::new();

    let mut last_reconcile = std::time::Instant::now();
    let reconcile_every = std::time::Duration::from_secs(60);
    // The loop body must run on a timer, not only when a tick arrives.
    // Everything below the `select!` — reconciliation, the halt sentinel, the
    // staleness check — used to be reachable only via the tick arm, so a feed
    // that went quiet without closing its socket froze all of it while a real
    // position stayed open. That exact failure (an established socket
    // recording nothing for 42 minutes) has already happened once here.
    let housekeeping_tick = std::time::Duration::from_secs(5);
    // 25 liquid names during market hours tick many times a second. Silence
    // this long is a dead feed, not a lull.
    let tick_stale_after = std::time::Duration::from_secs(90);
    let mut last_tick_at = std::time::Instant::now();
    let mut stale_alerted = false;
    let deadline = max_seconds.map(|s| tokio::time::Instant::now() + std::time::Duration::from_secs(s));

    // Today's already-elapsed bars, fetched from the historical API so the
    // engine's indicators match what the backtest would have computed.
    //
    // VWAP is session-cumulative, so a process started mid-session computes it
    // from its own start rather than from the 09:15 open. Measured on real
    // sessions that shifts VWAP by only 0.01-0.57%, but the strategy gates on
    // which side of VWAP price sits and that is enough to flip the test — an
    // unseeded engine is not running the strategy that was validated. Fetched
    // once here and replayed through `seed_bar`, which advances indicators
    // only and never touches the position slot.
    let seed_bars: Vec<(u64, core_types::Candle)> = {
        let session_open_ist = core_types::time::IST
            .from_local_datetime(&today.and_hms_opt(9, 15, 0).unwrap())
            .single()
            .ok_or_else(|| anyhow::anyhow!("ambiguous IST session open for {today}"))?
            .with_timezone(&chrono::Utc);
        let mut out = Vec::new();
        for inst in &instruments {
            match feed
                .historical_candles(
                    inst,
                    session_open_ist,
                    chrono::Utc::now(),
                    core_types::CandleInterval::OneMinute,
                )
                .await
            {
                Ok(cs) => {
                    println!("  seed {}: {} bars since 09:15", inst.tradingsymbol, cs.len());
                    out.extend(cs.into_iter().map(|c| (inst.instrument_token, c)));
                }
                // A seed failure degrades signal fidelity for that stock but
                // must not stop the session — better to trade it on a
                // truncated VWAP than to trade nothing at all. Logged loudly
                // so it is visible in the day's audit.
                Err(e) => println!("  seed {}: FAILED ({e}) — will run on a truncated VWAP", inst.tradingsymbol),
            }
        }
        out.sort_by_key(|(_, c)| c.ts);
        println!("Seeding {} historical bars into indicators (no orders placed)...", out.len());
        out
    };

    macro_rules! run_loop {
        ($engine:expr, $on_price:expr) => {{
            for (token, candle) in &seed_bars {
                $engine.seed_bar(*token, candle);
            }
            println!("Indicators seeded from 09:15. Live tick processing starts now.");
            loop {
                let now = chrono::Utc::now();
                if now >= loop_end || deadline.is_some_and(|dl| tokio::time::Instant::now() >= dl) {
                    break;
                }
                let stale = last_tick_at.elapsed() >= tick_stale_after;
                if stale && !stale_alerted {
                    stale_alerted = true;
                    tracing::error!(
                        silent_secs = last_tick_at.elapsed().as_secs(),
                        flat = $engine.is_flat(),
                        "NO TICKS — feed appears dead; halting new entries"
                    );
                    notify_everywhere(&alerter, "Live-trade feed has gone silent — new entries halted, check Kite now").await;
                } else if !stale && stale_alerted {
                    stale_alerted = false;
                    tracing::warn!("ticks resumed after a silent stretch — entries re-enabled");
                }
                // A dead feed means the strategy is blind: its stop is
                // evaluated per bar, and no bars are arriving. Entering a new
                // position on stale state is strictly worse than sitting out,
                // so staleness halts entries exactly as the sentinel does.
                // Any position already open is deliberately left to the
                // reconciliation loop and the end-of-session backstop rather
                // than flattened blind at an unknown price.
                if stale || std::path::Path::new(LIVE_HALT_SENTINEL).exists() {
                    $engine.halt();
                } else {
                    $engine.resume();
                }
                let until_end = (loop_end - now).to_std().unwrap_or_default();
                let until_deadline = deadline.map(|dl| dl.saturating_duration_since(tokio::time::Instant::now()));
                let wait = until_deadline
                    .map(|d| d.min(until_end))
                    .unwrap_or(until_end)
                    .min(housekeeping_tick);
                tokio::select! {
                    // Falls through to the housekeeping below; the loop-top
                    // check owns the decision to stop.
                    _ = tokio::time::sleep(wait) => {}
                    maybe_tick = ticks.next() => {
                        match maybe_tick {
                            Some(t) => {
                                last_tick_at = std::time::Instant::now();
                                if let Some((token, candle)) = aggregator.ingest(&t) {
                                    if let Some(inst) = instrument_by_token.get(&token) {
                                        $on_price(&inst.tradingsymbol, candle.close);
                                        if let Err(e) = $engine.on_bar(token, &candle).await {
                                            tracing::error!(error = %e, "engine.on_bar failed, continuing");
                                        }
                                        flush_alerts(&alerter, &mut tally, $engine.take_alerts()).await;
                                    }
                                }
                            }
                            None => {
                                tracing::error!("tick stream ended unexpectedly");
                                notify_everywhere(&alerter, "Live-trade tick stream ended unexpectedly").await;
                                break;
                            }
                        }
                    }
                }
                // Rate-limited inside the alerter, so this is just "do not
                // forget to ping" rather than more timing state in the loop.
                alerter.heartbeat().await;
                // Catches anything queued by a path other than a bar — most
                // importantly the reconciliation halt just below.
                flush_alerts(&alerter, &mut tally, $engine.take_alerts()).await;
                if last_reconcile.elapsed() >= reconcile_every {
                    last_reconcile = std::time::Instant::now();
                    if let Err(e) = $engine.reconcile().await {
                        tracing::error!(error = %e, "reconciliation check itself failed");
                    }
                }
            }
            for (token, candle) in aggregator.flush() {
                if let Some(inst) = instrument_by_token.get(&token) {
                    $on_price(&inst.tradingsymbol, candle.close);
                    let _ = $engine.on_bar(token, &candle).await;
                }
            }
            flush_alerts(&alerter, &mut tally, $engine.take_alerts()).await;
        }};
    }

    // One message at the top of the day. Its absence is itself informative:
    // if this never arrives, the session did not start.
    alerter
        .push(&format!(
            "Session started — {} | {} stocks | Rs.{capital:.0}/position | entries until 11:30, loop ends {} IST",
            if live { "LIVE" } else { "dry-run" },
            tune30.stocks.len(),
            in_ist(loop_end),
        ))
        .await;

    if live {
        // The broker's REST client gets a token refresher, so an access token
        // invalidated mid-session repairs itself instead of killing the
        // session. This is not a rare edge: merely logging into Kite Web to
        // look at a position invalidates the API token, because Kite shares
        // one session between them. On 2026-09-30 that silently disabled the
        // engine's broker calls at 15:17 with a position still open.
        let refresh_provider = std::sync::Arc::new(kite_auth::KiteLoginProvider::load_default(
            std::path::Path::new("secrets/credentials.toml"),
        )?);
        let rest_for_broker = kite_client::KiteRestClient::new(
            provider.api_key().to_string(),
            provider.fresh_access_token().await?.0,
        )?
        .with_token_refresher(Box::new(move || {
            let p = refresh_provider.clone();
            Box::pin(async move { p.force_new_access_token().await.map(|t| t.0).map_err(Into::into) })
        }));
        let broker = std::sync::Arc::new(kite_client::KiteBroker::new(rest_for_broker));
        let mut engine = live_trading::LiveEngine::new(
            stocks, broker, Box::new(risk::DefaultExitManager), storage, run_id,
            tune30.risk_pct, capital, eod_square_off, clock,
        );
        run_loop!(engine, |_sym: &str, _px: f64| {});
        // The clock-based square-off only fires when a BAR arrives at or after
        // the cutoff. If the held stock goes quiet first, no such bar ever
        // comes and a real position would survive the session, leaving the
        // broker to force it closed on its own terms. So close explicitly.
        let flat = match engine.force_square_off().await {
            Ok(flat) => flat,
            Err(e) => {
                tracing::error!(error = %e, "session-end square-off failed");
                false
            }
        };
        flush_alerts(&alerter, &mut tally, engine.take_alerts()).await;
        if !flat {
            notify_everywhere(&alerter, "LIVE SESSION ENDED WITH AN OPEN POSITION — check Kite now").await;
        }
        alerter.push(&tally.summary(flat)).await;
        // Tells the monitor this was a planned stop, so a clean end of
        // session does not page anyone at 15:18 every day.
        if flat {
            alerter.heartbeat_finish().await;
        }
        println!("Live session ended for {today}. Flat: {}", engine.is_flat());
    } else {
        let broker = std::sync::Arc::new(live_trading::SimulatedBroker::new());
        let broker_for_prices = broker.clone();
        let mut engine = live_trading::LiveEngine::new(
            stocks, broker, Box::new(risk::DefaultExitManager), storage, run_id,
            tune30.risk_pct, capital, eod_square_off, clock,
        );
        run_loop!(engine, |sym: &str, px: f64| broker_for_prices.update_price(sym, px));
        if let Err(e) = engine.force_square_off().await {
            tracing::error!(error = %e, "session-end square-off failed");
        }
        flush_alerts(&alerter, &mut tally, engine.take_alerts()).await;
        alerter.push(&tally.summary(engine.is_flat())).await;
        println!("Dry-run session ended for {today}. Flat: {}", engine.is_flat());
    }
    Ok(())
}

/// Fabricates one signal for `symbol`, feeds it through the real entry
/// pipeline against a `SimulatedBroker`, then forces an adverse price gap
/// (past both the stop's trigger *and* its limit price) to prove the
/// stop-loss's market-order fallback actually resolves the exit — the
/// specific mechanism `ExecutionPolicy` exists for. Never touches a real
/// broker; the only live network calls are read-only (auth, instrument
/// lookup, one quote).
async fn run_live_trade_test_signal(symbol: String, direction: String) -> anyhow::Result<()> {
    use core_types::{Direction, Storage};

    let direction = match direction.to_lowercase().as_str() {
        "short" => Direction::Short,
        "long" => Direction::Long,
        other => anyhow::bail!("--direction must be 'long' or 'short', got {other:?}"),
    };

    let tune30 = live_trading::Tune30Config::load(std::path::Path::new("config/tune30_live.toml"))?;
    let tuning_by_symbol = tune30.by_symbol();
    let Some(_tuning) = tuning_by_symbol.get(&symbol) else {
        anyhow::bail!("{symbol} is not in config/tune30_live.toml");
    };

    println!("=== test-signal: fabricating a {direction:?} signal for {symbol}, SimulatedBroker only ===\n");
    let rest = kite_rest_client().await?;
    let instrument = find_instrument(&rest, &symbol).await?;
    let quote = kite_client::quote::fetch_quote(&rest, &instrument).await?;
    println!("Live quote: {symbol} @ Rs.{:.2}", quote.last_price);

    let storage = storage::SqliteStorage::open(std::path::Path::new("data/papertrader.db"))?;
    storage.upsert_instruments(std::slice::from_ref(&instrument))?;
    let today = chrono::Utc::now().with_timezone(&core_types::time::IST).date_naive();
    let run_id = storage.start_run(&core_types::NewRun {
        mode: core_types::RunMode::Live,
        config_snapshot_json: serde_json::json!({"test_signal": true, "symbol": symbol, "direction": format!("{direction:?}")}).to_string(),
        label: Some(format!("live-trade/test-signal-{symbol}-{today}")),
        from_date: Some(today),
        to_date: Some(today),
    })?;
    let storage: Box<dyn core_types::Storage> = Box::new(storage);

    let stocks = live_trading::build_stock_runtimes(std::slice::from_ref(&instrument), &tuning_by_symbol);
    let broker = std::sync::Arc::new(live_trading::SimulatedBroker::new());
    let clock: std::sync::Arc<dyn live_trading::Clock> = std::sync::Arc::new(live_trading::RealClock::new());
    let eod_square_off = chrono::NaiveTime::from_hms_opt(15, 15, 0).unwrap();
    let mut engine = live_trading::LiveEngine::new(
        stocks,
        broker.clone(),
        Box::new(risk::DefaultExitManager),
        storage,
        run_id,
        tune30.risk_pct,
        100_000.0,
        eod_square_off,
        clock,
    );

    broker.update_price(&symbol, quote.last_price);
    let now = chrono::Utc::now();
    let signal = core_types::SignalIntent {
        strategy_id: "test_signal_injected".to_string(),
        window: core_types::TimeWindow::MidMorning,
        instrument_token: instrument.instrument_token,
        ts: now,
        direction,
        entry_price_hint: quote.last_price,
        rationale: "manually injected via `live-trade test-signal` for dry-run pipeline validation".to_string(),
        params: serde_json::json!({}),
    };
    let entry_candle = core_types::Candle {
        ts: now,
        open: quote.last_price,
        high: quote.last_price,
        low: quote.last_price,
        close: quote.last_price,
        volume: 10_000,
    };

    println!("\n--- injecting signal ---");
    engine.inject_test_signal(instrument.instrument_token, signal, &entry_candle).await?;

    if engine.is_flat() {
        println!("\nEntry did not result in an open position (rejected or margin-skipped — see logs above). Stopping here.");
        return Ok(());
    }

    // Force a gap: move price 1% past where the stop would trigger *and*
    // past its own limit price too, so the plain SL-limit order can't
    // fill and the market-order fallback has to kick in — the exact
    // scenario ExecutionPolicy's fallback exists for.
    let gap_price = match direction {
        Direction::Short => quote.last_price * 1.02,
        Direction::Long => quote.last_price * 0.98,
    };
    println!("\n--- forcing an adverse price gap to Rs.{gap_price:.2} to trigger the stop-loss fallback ---");
    broker.update_price(&symbol, gap_price);
    let adverse_candle = core_types::Candle {
        ts: chrono::Utc::now(),
        open: gap_price,
        high: gap_price,
        low: gap_price,
        close: gap_price,
        volume: 10_000,
    };
    engine.on_bar(instrument.instrument_token, &adverse_candle).await?;

    println!("\nFlat after exit attempt: {}", engine.is_flat());
    println!("\n=== trade summary for run {run_id} ===");
    print_report(Some(run_id), false)?;
    Ok(())
}

/// Proves the alert path end to end. Worth its own command: an alerting
/// channel that is only ever exercised during an emergency is one nobody has
/// ever seen work.
async fn run_test_alert(samples: bool) -> anyhow::Result<()> {
    let alerter = alerts::Alerter::load_default();
    println!("Config: {}", alerter.describe());
    if !alerter.has_push() && !alerter.has_heartbeat() {
        println!("\nNothing configured. Copy config/alerts.example.toml to {} and fill it in.",
                 alerts::DEFAULT_PATH);
        return Ok(());
    }
    let mut failed = false;
    if alerter.has_push() {
        println!("Sending a test push...");
        match alerter
            .try_push("papertrader test alert — if you can read this on your phone, alerting works.")
            .await
        {
            Ok(()) => println!("  DELIVERED — the service accepted it. Check your phone."),
            Err(e) => {
                failed = true;
                println!("  FAILED — {e}");
            }
        }
    }
    if alerter.has_heartbeat() {
        println!("Pinging the dead-man's switch...");
        match alerter.try_heartbeat().await {
            Ok(()) => println!("  PINGED — the check should now show as up."),
            Err(e) => {
                failed = true;
                println!("  FAILED — {e}");
            }
        }
    }
    if samples && alerter.has_push() {
        println!("\nSending one example of each alert type...");
        for alert in sample_alerts() {
            let tag = if alert.is_critical() { "critical" } else { "routine " };
            match alerter.try_push(&format!("[EXAMPLE] {}", alert.message())).await {
                Ok(()) => println!("  {tag}  {}", alert.message().replace('\n', " | ")),
                Err(e) => {
                    failed = true;
                    println!("  FAILED — {e}");
                }
            }
        }
    }
    if failed {
        anyhow::bail!("alerting is not working — fix the above before relying on it");
    }
    Ok(())
}

/// One realistic instance of every alert the engine can raise. Kept exhaustive
/// on purpose: a variant added later without a sample here is one whose
/// wording nobody has ever read before it arrives during an incident.
fn sample_alerts() -> Vec<live_trading::Alert> {
    use core_types::{Direction, ExitReason};
    use live_trading::Alert;
    vec![
        Alert::Entry {
            symbol: "HAL".into(),
            direction: Direction::Short,
            qty: 22,
            price: 4520.50,
            stop: 4554.40,
            target: 4418.80,
        },
        Alert::Exit {
            symbol: "HAL".into(),
            price: 4451.20,
            reason: ExitReason::TrailingStop,
            r_multiple: 2.04,
            pnl_abs: 1524.60,
        },
        Alert::Exit {
            symbol: "ITC".into(),
            price: 413.10,
            reason: ExitReason::StopLoss,
            r_multiple: -1.00,
            pnl_abs: -3375.00,
        },
        Alert::EntryUnfilled { symbol: "DIXON".into(), seconds: 60 },
        Alert::ExitEscalated { symbol: "PNB".into(), bars: 3, remaining: 1180 },
        Alert::OrderRejected {
            symbol: "LICI".into(),
            what: "entry".into(),
            reason: "Insufficient funds for this order".into(),
        },
        Alert::Halted {
            reason: "reconciliation mismatch — engine expects HAL -22, broker shows []".into(),
        },
        Alert::critical("SESSION ENDED HOLDING HAL — square-off filled only 0 of 22. Check Kite now."),
    ]
}


/// Sends an ad-hoc message through whatever alert channel is configured.
///
/// Reuses `Alerter` rather than curling Telegram directly, so this keeps
/// working if the channel changes, and inherits the length-splitting that a
/// piped query result needs.
async fn run_notify(message: Vec<String>, title: String) -> anyhow::Result<()> {
    let body = if message.is_empty() {
        use std::io::Read;
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf)?;
        buf
    } else {
        message.join(" ")
    };
    let body = body.trim();
    if body.is_empty() {
        anyhow::bail!("nothing to send — pass a message as arguments or pipe it on stdin");
    }

    let alerter = alerts::Alerter::load_default();
    if !alerter.has_push() {
        anyhow::bail!(
            "no push channel configured — copy config/alerts.example.toml to {}",
            alerts::DEFAULT_PATH
        );
    }
    let text = if title.is_empty() {
        body.to_string()
    } else {
        format!("[{title}] {body}")
    };
    alerter.try_push(&text).await?;
    println!("Sent ({} chars).", text.chars().count());
    Ok(())
}

/// Paper-trades the live book with the position cap lifted.
///
/// The live engine holds exactly one position at a time, so roughly half of
/// all qualifying signals are discarded for slot contention rather than for
/// anything to do with the setup. That is a deliberate risk choice, but its
/// cost has only ever been estimated from backtests. This runs the *same*
/// strategy over the *same* stocks against the *same* live ticks with the cap
/// removed, so the two can be compared on identical days.
///
/// Deliberately not a mode of `live-trade run`: it has to run on days when no
/// live session does, and a shadow book that only exists alongside live
/// trading would have gaps exactly where they matter.
async fn run_shadow(capital: f64, max_seconds: Option<u64>) -> anyhow::Result<()> {
    use core_types::{MarketDataFeed, TradingCalendar};

    let tune30 = live_trading::Tune30Config::load(std::path::Path::new("config/tune30_live.toml"))?;
    let Some(first) = tune30.stocks.first().cloned() else {
        anyhow::bail!("config/tune30_live.toml lists no stocks");
    };

    // PaperEngine drives every instrument through ONE StrategyRunner, which is
    // only equivalent to the live book's per-stock runners while every stock
    // shares the same parameters. That holds today (the book was deliberately
    // un-tuned per stock after walk-forward showed the tuning was noise), but
    // if it ever stops holding, silently trading all 25 on the first stock's
    // lookback would make the comparison meaningless rather than wrong-looking.
    let divergent: Vec<&str> = tune30
        .stocks
        .iter()
        .filter(|s| s.lookback != first.lookback || s.mrm != first.mrm || s.trail != first.trail)
        .map(|s| s.symbol.as_str())
        .collect();
    if !divergent.is_empty() {
        anyhow::bail!(
            "shadow needs one shared parameter set, but these differ from {}: {divergent:?}\n\
             (per-stock tuning would need one engine per stock — see run_shadow's comment)",
            first.symbol
        );
    }

    let calendar = market_calendar::NseCalendar::load_default()?;
    let today = chrono::Utc::now().with_timezone(&core_types::time::IST).date_naive();
    if !calendar.is_trading_day(today) {
        anyhow::bail!("{today} is not a trading day");
    }
    let session_close = calendar
        .session_close(today)
        .ok_or_else(|| anyhow::anyhow!("no session close time for {today}"))?;
    // Matches live exactly. A shadow book squaring off at a different time
    // from the live book is not the counterfactual it claims to be.
    let eod_square_off = chrono::NaiveTime::from_hms_opt(15, 10, 0).unwrap();
    let loop_end = live_trading::loop_end(today, eod_square_off, session_close)
        .ok_or_else(|| anyhow::anyhow!("ambiguous IST square-off cutoff for {today}"))?;
    let in_ist = |t: core_types::DateTime| t.with_timezone(&core_types::time::IST).format("%H:%M:%S");
    if chrono::Utc::now() >= loop_end {
        anyhow::bail!("it is already past the {} IST cutoff for {today}", in_ist(loop_end));
    }

    let deadline = max_seconds.map(|s| tokio::time::Instant::now() + std::time::Duration::from_secs(s));

    // Launched by a scheduler before the bell, this waits rather than exiting,
    // so a 09:00 cron entry does the right thing without a wrapper script.
    //
    // `--max-seconds` bounds the WHOLE command, this wait included. Sleeping
    // past it would make a 25-second smoke test hang until tomorrow morning,
    // which is exactly what it did the first time this ran.
    if let Some(open) = calendar.session_open(today) {
        let now = chrono::Utc::now();
        if now < open {
            let wait = (open - now).to_std().unwrap_or_default();
            if deadline.is_some_and(|dl| tokio::time::Instant::now() + wait >= dl) {
                println!(
                    "The {} IST open is {} away, beyond --max-seconds. Stopping.",
                    in_ist(open),
                    format_duration(wait)
                );
                return Ok(());
            }
            println!("Waiting {} until the {} IST open...", format_duration(wait), in_ist(open));
            tokio::time::sleep(wait).await;
        }
    }

    println!("SHADOW — {} stocks, no position limit, Rs.{capital:.0}/position", tune30.stocks.len());
    println!("  lookback {} | mrm {} | trail {:.4} | risk {:.3}%",
             first.lookback, first.mrm, first.trail, tune30.risk_pct * 100.0);

    let (provider, access_token) = auth_refresh_with_retries().await?;
    let rest = kite_client::KiteRestClient::new(provider.api_key().to_string(), access_token)?;
    let symbols = tune30.symbols();
    let all_equities = kite_client::instruments::fetch_nse_equities(&rest).await?;
    let instruments: Vec<_> = all_equities
        .into_iter()
        .filter(|i| symbols.iter().any(|s: &String| s.eq_ignore_ascii_case(&i.tradingsymbol)))
        .collect();
    if instruments.len() != symbols.len() {
        let found: std::collections::HashSet<_> =
            instruments.iter().map(|i| i.tradingsymbol.clone()).collect();
        let missing: Vec<_> = symbols.iter().filter(|s| !found.contains(*s)).collect();
        anyhow::bail!("could not resolve every symbol — missing: {missing:?}");
    }

    let storage = storage::SqliteStorage::open(std::path::Path::new("data/papertrader.db"))?;
    storage.upsert_instruments(&instruments)?;

    // Mirrors the live book exactly, except for `max_concurrent_positions`.
    // The daily loss cap is off for the same reason: this is a measurement of
    // the strategy's raw signal set, and a circuit breaker would truncate the
    // very tail being measured.
    let engine_config = paper_engine::EngineConfig {
        risk_pct: tune30.risk_pct,
        min_reward_multiple: first.mrm,
        trail_policy: core_types::TrailPolicy::GiveBackFromPeak(first.trail),
        capital_per_trade: capital,
        max_concurrent_positions: instruments.len(),
        daily_loss_cap_pct: 0.0,
        eod_square_off,
        max_hold_minutes: None,
        exit_atr_period: 14,
    };
    let config_snapshot = serde_json::json!({
        "shadow": true,
        "source_config": "config/tune30_live.toml",
        "capital_per_trade": capital,
        "max_concurrent_positions": instruments.len(),
        "risk_pct": tune30.risk_pct,
        "lookback": first.lookback,
        "mrm": first.mrm,
        "trail": first.trail,
        "stocks": symbols,
    })
    .to_string();

    let strategy = strategies::VwapTrendFollow::new(9, 21, 10, 1.2, false, true, 45)
        .with_vwap_slope(first.lookback, 0.0)
        .with_id("vwap_trend_short_late_vwapslope_shadow".to_string());
    let indicator_engine =
        strategy_core::IndicatorEngine::new(chrono::NaiveTime::from_hms_opt(9, 30, 0).unwrap());
    let runner = strategy_core::StrategyRunner::new(indicator_engine, vec![Box::new(strategy)]);

    let label = format!("shadow-{today}");
    let mut engine = paper_engine::PaperEngine::new(
        runner,
        Box::new(risk::DefaultExitManager),
        Box::new(storage),
        engine_config,
        core_types::NewRun {
            mode: core_types::RunMode::Live,
            config_snapshot_json: config_snapshot,
            label: Some(label.clone()),
            from_date: Some(today),
            to_date: Some(today),
        },
    )?;
    println!("Shadow run {} started (label: {label}).", engine.run_id());

    let instrument_by_token: std::collections::HashMap<u64, core_types::Instrument> =
        instruments.iter().map(|i| (i.instrument_token, i.clone())).collect();
    let feed = kite_client::KiteMarketDataFeed::new(rest);
    let mut ticks = feed.subscribe_ticks(&instruments).await?;
    let mut aggregator = paper_engine::CandleAggregator::new();

    // Same 09:15 replay the live engine does. Without it the two books compute
    // VWAP over different windows and stop being comparable, which is the one
    // thing this command exists to do.
    let session_open_ist = core_types::time::IST
        .from_local_datetime(&today.and_hms_opt(9, 15, 0).unwrap())
        .single()
        .ok_or_else(|| anyhow::anyhow!("ambiguous IST session open for {today}"))?
        .with_timezone(&chrono::Utc);
    let mut seeded = 0usize;
    for inst in &instruments {
        match feed
            .historical_candles(inst, session_open_ist, chrono::Utc::now(),
                                core_types::CandleInterval::OneMinute)
            .await
        {
            Ok(cs) => {
                for c in &cs {
                    engine.seed_bar(inst, c);
                }
                seeded += cs.len();
            }
            Err(e) => println!("  seed {}: FAILED ({e}) — truncated VWAP for this stock", inst.tradingsymbol),
        }
    }
    println!("Seeded {seeded} historical bars. Live ticks from here.");

    let housekeeping = std::time::Duration::from_secs(5);
    let mut last_tick_at = std::time::Instant::now();
    let mut stale_alerted = false;
    let alerter = alerts::Alerter::load_default();
    alerter
        .push(&format!("SHADOW started — {} stocks, no position limit", instruments.len()))
        .await;

    loop {
        let now = chrono::Utc::now();
        if now >= loop_end || deadline.is_some_and(|dl| tokio::time::Instant::now() >= dl) {
            break;
        }
        if last_tick_at.elapsed() >= std::time::Duration::from_secs(90) && !stale_alerted {
            stale_alerted = true;
            tracing::error!("SHADOW: no ticks — feed appears dead");
            alerter.push("SHADOW feed has gone silent").await;
        }
        let until_end = (loop_end - now).to_std().unwrap_or_default();
        let until_deadline = deadline.map(|dl| dl.saturating_duration_since(tokio::time::Instant::now()));
        let wait = until_deadline.map(|d| d.min(until_end)).unwrap_or(until_end).min(housekeeping);
        tokio::select! {
            _ = tokio::time::sleep(wait) => {}
            maybe_tick = ticks.next() => {
                match maybe_tick {
                    Some(t) => {
                        last_tick_at = std::time::Instant::now();
                        stale_alerted = false;
                        if let Some((token, candle)) = aggregator.ingest(&t) {
                            if let Some(inst) = instrument_by_token.get(&token) {
                                if let Err(e) = engine.on_bar(inst, &candle) {
                                    tracing::error!(error = %e, "shadow on_bar failed, continuing");
                                }
                            }
                        }
                    }
                    None => {
                        tracing::error!("SHADOW: tick stream ended unexpectedly");
                        alerter.push("SHADOW tick stream ended unexpectedly").await;
                        break;
                    }
                }
            }
        }
    }

    for (token, candle) in aggregator.flush() {
        if let Some(inst) = instrument_by_token.get(&token) {
            let _ = engine.on_bar(inst, &candle);
        }
    }
    // Squares off anything the 15:15 clock rule missed because that stock's
    // bars stopped arriving first — the same backstop the backtest uses.
    let squared = engine.reset_session()?;
    engine.end_run()?;
    println!("Shadow session ended for {today}. Squared off at the boundary: {squared}.");
    alerter
        .push(&format!("SHADOW ended — run {} recorded for {today}", engine.run_id()))
        .await;
    Ok(())
}

/// Human-readable wait, for the "sleeping until the open" line.
fn format_duration(d: std::time::Duration) -> String {
    let secs = d.as_secs();
    if secs < 60 {
        format!("{secs}s")
    } else if secs < 3600 {
        format!("{}m", secs / 60)
    } else {
        format!("{}h{}m", secs / 3600, (secs % 3600) / 60)
    }
}

async fn run_live_trade_check() -> anyhow::Result<()> {
    println!("=== live-trade check: read-only pre-flight, places no orders ===\n");

    println!("[1/3] Egress IP vs the Kite whitelist...");
    let mut ip_ok = true;
    match verify_whitelisted_ip().await {
        live_trading::IpCheck::Match(ip) => println!("      OK — {ip} is whitelisted."),
        live_trading::IpCheck::Unknown(why) => println!("      UNVERIFIED — {why}"),
        live_trading::IpCheck::Mismatch { report, .. } => {
            ip_ok = false;
            println!("      MISMATCH — {report}");
        }
    }

    let rest = kite_rest_client().await?;
    println!("\n[2/3] Auth OK — fetching equity margins (GET /user/margins/equity)...");
    let margins = kite_client::orders::fetch_equity_margins(&rest).await?;
    let get = |path: &str| margins.get("available").and_then(|a| a.get(path)).and_then(|v| v.as_f64());
    let net = margins.get("net").and_then(|v| v.as_f64());
    match (get("cash"), get("collateral"), net) {
        (Some(cash), Some(collateral), Some(net)) => {
            println!("      cash: Rs.{cash:.2}  collateral: Rs.{collateral:.2}  net available: Rs.{net:.2}");
        }
        _ => println!("      (unexpected shape — raw: {margins})"),
    }

    println!("\n[3/3] Sample order-margin calculation (POST /margins/orders), no order placed...");
    // Sample whatever the live config's first stock happens to be, rather than
    // naming one: the book is re-selected quarterly, so a hardcoded symbol
    // silently breaks this pre-flight the first rebalance that drops it.
    let tune30 = live_trading::Tune30Config::load(std::path::Path::new("config/tune30_live.toml"))?;
    let sample_symbol = tune30
        .stocks
        .first()
        .map(|s| s.symbol.clone())
        .ok_or_else(|| anyhow::anyhow!("config/tune30_live.toml lists no stocks"))?;
    let instrument = find_instrument(&rest, &sample_symbol).await?;
    let quote = kite_client::quote::fetch_quote(&rest, &instrument).await?;
    let sample_price = (quote.last_price * 100.0).round() / 100.0;
    println!("      using {sample_symbol} @ Rs.{sample_price:.2} (live quote), qty=1, MIS, LIMIT, SELL (short-side sample)");

    let broker = kite_client::KiteBroker::new(rest);
    let req = core_types::OrderRequest {
        tradingsymbol: instrument.tradingsymbol.clone(),
        exchange: instrument.exchange,
        side: core_types::OrderSide::Sell,
        quantity: 1,
        product: core_types::Product::Mis,
        order_type: core_types::OrderType::Limit,
        price: Some(sample_price),
        trigger_price: None,
        tag: "livecheck".to_string(),
    };
    let margin_check = core_types::Broker::check_margin(&broker, &req).await?;

    if margin_check.ok {
        println!("      OK — margin calculator accepted the sample order.");
        if let Some(req_margin) = margin_check.required_margin {
            println!("      required margin for 1 share: Rs.{req_margin:.2}");
        }
        // The margin calculator is not IP-gated, so it passes from any
        // address — a green result here says nothing about whether an actual
        // order would be accepted. Only step 1 covers that.
        if ip_ok {
            println!("\n=== PASS: this API key can place live orders. ===");
        } else {
            println!("\n=== FAIL: the API key is fine, but orders will be rejected from this IP. ===");
            println!("    Fix the whitelist (step 1) before starting a live session.");
        }
    } else {
        println!(
            "      REJECTED: {}",
            margin_check.rejection_reason.as_deref().unwrap_or("(no reason given)")
        );
        println!("\n=== FAIL: resolve the rejection above before attempting live trading. ===");
        println!("raw response: {}", margin_check.raw_response_json);
    }
    Ok(())
}

async fn run_scan(top: usize) -> anyhow::Result<()> {
    let rest_for_screen = kite_rest_client().await?;
    let (_feed, instruments) = screened_universe(rest_for_screen).await?;
    println!(
        "Scanning {} liquidity-screened instruments for today's gap/move...",
        instruments.len()
    );

    let rest_for_quotes = kite_rest_client().await?;
    let quotes = kite_client::quote::fetch_scan_quotes(&rest_for_quotes, &instruments).await?;

    let mut rows: Vec<_> = quotes
        .into_iter()
        .filter(|q| q.prev_close > 0.0)
        .collect();
    rows.sort_by(|a, b| b.gap_pct().abs().partial_cmp(&a.gap_pct().abs()).unwrap());

    println!(
        "\n{:<12} {:>10} {:>10} {:>9} {:>10} {:>14}",
        "SYMBOL", "LTP", "PREV CLOSE", "GAP %", "RANGE %", "VOLUME"
    );
    for q in rows.iter().take(top) {
        println!(
            "{:<12} {:>10.2} {:>10.2} {:>+9.2} {:>10.2} {:>14}",
            q.instrument.tradingsymbol,
            q.last_price,
            q.prev_close,
            q.gap_pct(),
            q.range_pct(),
            q.volume,
        );
    }
    let (gainers, losers): (Vec<_>, Vec<_>) = rows.iter().take(top).partition(|q| q.gap_pct() >= 0.0);
    println!(
        "\nTop {}: {} gapped up, {} gapped down (ranked by |gap %|, out of {} screened).",
        top.min(rows.len()),
        gainers.len(),
        losers.len(),
        rows.len()
    );
    Ok(())
}

/// Holds a custom exit/risk rule fixed (a give-back-from-peak trail, per
/// the user's own manual trading rule) and tests each of the three entry
/// strategies *in isolation* against every instrument with enough cached
/// history — deliberately not the liquidity-screened universe, since the
/// goal here is "which specific stocks does this actually work well on,"
/// not "which stocks would we normally pick." `max_concurrent_positions`
/// is set to the full instrument count so one instrument's signal never
/// steals a slot from another's — the position cap that matters for real
/// live trading would otherwise contaminate the per-stock win-rate numbers
/// this research is trying to isolate.
async fn run_research(
    min_bars: i64,
    risk_pct: f64,
    min_reward_multiple: f64,
    trail_param: f64,
    min_trades: i64,
    label_prefix: String,
    only: Option<String>,
    symbols: Option<String>,
) -> anyhow::Result<()> {
    let scratch = open_storage_readonly()?;
    // With --symbols, resolve each requested symbol directly (an indexed
    // per-instrument lookup) instead of the full instruments_with_candle_coverage
    // scan, which aggregates over the *entire* candles table regardless of
    // how few symbols we actually want — on the full multi-year universe
    // that scan alone took minutes, dwarfing the actual single-stock replay.
    let mut instruments = if let Some(s) = &symbols {
        let mut found = Vec::new();
        for sym in s.split(',').map(|s| s.trim()) {
            if let Some(inst) = scratch.instrument_by_symbol(sym)? {
                if let Some((_, _, count)) = scratch.candle_coverage(inst.instrument_token)? {
                    if count >= min_bars {
                        found.push(inst);
                    }
                }
            }
        }
        found
    } else {
        scratch.instruments_with_candle_coverage(core_types::CandleInterval::OneMinute, min_bars)?
    };
    instruments.sort_by(|a, b| a.tradingsymbol.cmp(&b.tradingsymbol));
    // Applied after both branches above (explicit --symbols and the coverage
    // scan alike) so an instrument with known-bad upstream data can't enter
    // the universe by either route, including by being named directly.
    let universe_cfg = universe::config::load_default()?;
    let before = instruments.len();
    instruments.retain(|i| !universe_cfg.is_excluded(&i.tradingsymbol));
    if instruments.len() < before {
        println!(
            "Excluded {} instrument(s) per [universe].exclude: {}",
            before - instruments.len(),
            universe_cfg.exclude.join(", ")
        );
    }
    if instruments.is_empty() {
        anyhow::bail!("no instruments have >= {min_bars} cached 1m bars — run `backfill` first");
    }
    let (min_ts, max_ts, _) = scratch
        .overall_candle_coverage()?
        .ok_or_else(|| anyhow::anyhow!("no candle coverage found"))?;
    let from_date = chrono::DateTime::parse_from_rfc3339(&min_ts)?
        .with_timezone(&core_types::time::IST)
        .date_naive();
    let to_date = chrono::DateTime::parse_from_rfc3339(&max_ts)?
        .with_timezone(&core_types::time::IST)
        .date_naive();
    drop(scratch);

    println!(
        "Research universe: {} instruments with >= {min_bars} cached bars, {from_date}..{to_date}.",
        instruments.len()
    );
    println!(
        "Exit rule under test: risk_pct={:.3}% min_reward_multiple={:.1}x give_back_trail={:.3}% \
         (arms at {:.3}%, initial lock ~{:.3}%)",
        risk_pct * 100.0,
        min_reward_multiple,
        trail_param * 100.0,
        risk_pct * min_reward_multiple * 100.0,
        (risk_pct * min_reward_multiple - trail_param) * 100.0,
    );

    // Deliberately NOT a Kite client. Research replays cached candles, and the
    // universe above is already filtered to instruments that have them — but
    // merely *constructing* a Kite client performs a headless login, and a
    // login rotates the access token that a running live session is holding.
    // Starting a study mid-session used to invalidate the live book's
    // credentials. See CacheOnlyFeed.
    let feed = backtest::CacheOnlyFeed;

    let engine_config_for = || paper_engine::EngineConfig {
        risk_pct,
        min_reward_multiple,
        trail_policy: core_types::TrailPolicy::GiveBackFromPeak(trail_param),
        capital_per_trade: 50_000.0,
        max_concurrent_positions: instruments.len(),
        daily_loss_cap_pct: 0.0, // disabled — would otherwise suppress later same-day signals mid-research
        // 15:10, matching live. 15:15 was the old value and it models an exit
        // that cannot happen: Zerodha refuses MIS orders from 15:12 and
        // squares off itself, so a backtest exiting at 15:15 is filling at a
        // price the live book would never get. Runs recorded before
        // 2026-09-30 used 15:15 and are not strictly comparable with runs
        // after it — for trades that reach the EOD exit, which is most of them.
        eod_square_off: chrono::NaiveTime::from_hms_opt(15, 10, 0).unwrap(),
        max_hold_minutes: None,
        exit_atr_period: 14,
    };
    let config_snapshot = serde_json::json!({
        "research": true,
        "risk_pct": risk_pct,
        "min_reward_multiple": min_reward_multiple,
        "trail_policy": "give_back_from_peak",
        "trail_param": trail_param,
        // Recorded because it is NOT a constant and it moves results by about
        // 0.03 R. Run 110 was produced at 15:15 and run 263 at 15:10, and the
        // two were compared against each other for a whole evening before
        // anyone noticed — the only way to tell them apart afterwards was to
        // histogram their exit timestamps.
        "eod_square_off": engine_config_for().eod_square_off.to_string(),
    })
    .to_string();

    let strategies: Vec<(&str, Box<dyn core_types::Strategy>)> = vec![
        ("orb", Box::new(strategies::OpeningRangeBreakout::default())),
        ("vwap_trend", Box::new(strategies::VwapTrendFollow::default())),
        ("momentum_breakout", Box::new(strategies::MomentumBreakout::default())),
        ("vwap_reversion", Box::new(strategies::VwapReversion::default())),
        ("trend_pullback", Box::new(strategies::TrendPullback::default())),
        // Filtered variant of vwap_trend — short-side only, ignores
        // crossovers in the first 45 minutes of the window — per the
        // direction/timing analysis in research/highwinrate-v2.md.
        (
            "vwap_trend_short_late",
            Box::new(strategies::VwapTrendFollow::new(9, 21, 10, 1.2, false, true, 45)),
        ),
        // Extends the short_late variant to watch the whole session
        // (previously capped at 11:30 AM) instead of just the 09:45-11:30
        // window, with an absolute cutoff at 2 PM for *new* entries —
        // existing positions still exit normally afterward. Same 10:30 AM
        // earliest-entry time as before (was "45 min into a 09:45 window").
        (
            "vwap_trend_short_allday",
            Box::new(
                strategies::VwapTrendFollow::new(9, 21, 10, 1.2, false, true, 0)
                    .with_window(core_types::TimeWindow::FullDay)
                    .with_min_entry_time(chrono::NaiveTime::from_hms_opt(10, 30, 0).unwrap())
                    .with_max_entry_time(chrono::NaiveTime::from_hms_opt(14, 0, 0).unwrap())
                    .with_id("vwap_trend_short_allday"),
            ),
        ),
        // Same, but requiring a stronger volume spike to fire — testing
        // "raise the bar" on relative volume (2x and 3x baseline instead
        // of 1.2x).
        (
            "vwap_trend_short_allday_vol2x",
            Box::new(
                strategies::VwapTrendFollow::new(9, 21, 10, 2.0, false, true, 0)
                    .with_window(core_types::TimeWindow::FullDay)
                    .with_min_entry_time(chrono::NaiveTime::from_hms_opt(10, 30, 0).unwrap())
                    .with_max_entry_time(chrono::NaiveTime::from_hms_opt(14, 0, 0).unwrap())
                    .with_id("vwap_trend_short_allday_vol2x"),
            ),
        ),
        (
            "vwap_trend_short_allday_vol3x",
            Box::new(
                strategies::VwapTrendFollow::new(9, 21, 10, 3.0, false, true, 0)
                    .with_window(core_types::TimeWindow::FullDay)
                    .with_min_entry_time(chrono::NaiveTime::from_hms_opt(10, 30, 0).unwrap())
                    .with_max_entry_time(chrono::NaiveTime::from_hms_opt(14, 0, 0).unwrap())
                    .with_id("vwap_trend_short_allday_vol3x"),
            ),
        ),
        // Back to the morning-only window (short_late's MidMorning,
        // 09:45-11:30, entries after the first 45 min) but with the
        // relative-volume bar raised much further, to 5x baseline —
        // "raise the bar" tested on the window that actually worked,
        // instead of the all-day window which diluted it.
        (
            "vwap_trend_short_late_vol5x",
            Box::new(
                strategies::VwapTrendFollow::new(9, 21, 10, 5.0, false, true, 45)
                    .with_id("vwap_trend_short_late_vol5x"),
            ),
        ),
        // short_late's window/volume filter unchanged, plus a VWAP-slope
        // confirmation: VWAP itself must have moved in the trade's
        // direction over the last 15 minutes, not just have price on one
        // side of a possibly-flat VWAP line.
        (
            "vwap_trend_short_late_vwapslope",
            Box::new(
                strategies::VwapTrendFollow::new(9, 21, 10, 1.2, false, true, 45)
                    .with_vwap_slope(15, 0.0)
                    .with_id("vwap_trend_short_late_vwapslope"),
            ),
        ),
        // The CONTROL. No information in the entry at all: a coin flip in the
        // same 10:30-11:30 window, short side, managed by the same stop,
        // trail and square-off as the live book.
        //
        // Every Avg R this project reports is an average over trades the
        // strategy chose, which is evidence of an edge only if NOT choosing
        // does worse. A short held from mid-morning to 15:10 is exposed to
        // whatever the average stock does over those hours; if that drift is
        // negative, a short-only book shows a positive Avg R while knowing
        // nothing. Until this is measured, the live book's +0.031 net R has
        // no denominator.
        //
        // Three seeds, because one draw of a random control is itself a
        // sample: if the three disagree widely, the comparison is too noisy
        // to conclude from.
        (
            "random_short_s1",
            Box::new(
                strategies::RandomEntry::new(core_types::Direction::Short, 0.004, 45, 1)
                    .with_id("random_short_s1"),
            ),
        ),
        (
            "random_short_s2",
            Box::new(
                strategies::RandomEntry::new(core_types::Direction::Short, 0.004, 45, 2)
                    .with_id("random_short_s2"),
            ),
        ),
        (
            "random_short_s3",
            Box::new(
                strategies::RandomEntry::new(core_types::Direction::Short, 0.004, 45, 3)
                    .with_id("random_short_s3"),
            ),
        ),
        // The CEILING test. Data from the threshold sweep said the floor
        // points the wrong way: partitioning the live config's own trades by
        // realised slope gave a monotone ladder in which nearly-flat VWAP was
        // best (+0.118 R, 47.5% win) and steep falls worst (-0.158 R, 25.5%).
        // These cap how far VWAP may already have moved instead of demanding
        // it move further.
        //
        // The 0.05% level is fitted to that table, so it needs out-of-sample
        // and year-by-year scrutiny before being believed; 0.10% is included
        // as a neighbouring value, since a real effect should not be
        // knife-edge.
        (
            "vwapslope_lb20_ceil005",
            Box::new(
                strategies::VwapTrendFollow::new(9, 21, 10, 1.2, false, true, 45)
                    .with_vwap_slope(20, 0.0)
                    .with_max_vwap_slope(0.0005)
                    .with_id("vwapslope_lb20_ceil005"),
            ),
        ),
        (
            "vwapslope_lb20_ceil010",
            Box::new(
                strategies::VwapTrendFollow::new(9, 21, 10, 1.2, false, true, 45)
                    .with_vwap_slope(20, 0.0)
                    .with_max_vwap_slope(0.001)
                    .with_id("vwapslope_lb20_ceil010"),
            ),
        ),
        // The NO-GATE control: does requiring slope <= 0 beat not looking at
        // the slope at all? The ladder above runs monotonically right up to
        // zero and gives no reason to think it stops there, so the gate's
        // own value is now in question — not just its threshold.
        (
            "vwapslope_lb20_nogate",
            Box::new(
                strategies::VwapTrendFollow::new(9, 21, 10, 1.2, false, true, 45)
                    .with_id("vwapslope_lb20_nogate"),
            ),
        ),
        // Varying the slope THRESHOLD at the live lookback of 20, holding
        // everything else fixed. The live book requires only that VWAP is
        // flat-or-falling (`min_slope_pct = 0.0`); these ask whether
        // demanding a steeper fall selects better trades or merely fewer.
        //
        // The threshold is a fraction of VWAP over the 20-bar window, so
        // 0.001 = "VWAP fell at least 0.1% in the last 20 minutes".
        //
        // Unlike the entry filters rejected earlier, this does not condition
        // on past performance — it tightens a structural signal condition, so
        // it cannot select for luck the way a "only trade recent winners"
        // rule does. It can still fail by cutting sample without improving
        // quality, which is what the sweep is for.
        (
            "vwapslope_lb20_min005",
            Box::new(
                strategies::VwapTrendFollow::new(9, 21, 10, 1.2, false, true, 45)
                    .with_vwap_slope(20, 0.0005)
                    .with_id("vwapslope_lb20_min005"),
            ),
        ),
        (
            "vwapslope_lb20_min010",
            Box::new(
                strategies::VwapTrendFollow::new(9, 21, 10, 1.2, false, true, 45)
                    .with_vwap_slope(20, 0.001)
                    .with_id("vwapslope_lb20_min010"),
            ),
        ),
        (
            "vwapslope_lb20_min020",
            Box::new(
                strategies::VwapTrendFollow::new(9, 21, 10, 1.2, false, true, 45)
                    .with_vwap_slope(20, 0.002)
                    .with_id("vwapslope_lb20_min020"),
            ),
        ),
        (
            "vwapslope_lb20_min035",
            Box::new(
                strategies::VwapTrendFollow::new(9, 21, 10, 1.2, false, true, 45)
                    .with_vwap_slope(20, 0.0035)
                    .with_id("vwapslope_lb20_min035"),
            ),
        ),
        (
            "vwapslope_lb20_min050",
            Box::new(
                strategies::VwapTrendFollow::new(9, 21, 10, 1.2, false, true, 45)
                    .with_vwap_slope(20, 0.005)
                    .with_id("vwapslope_lb20_min050"),
            ),
        ),
        // Same idea, varying the lookback window (holding min_slope_pct at
        // 0.0 — direction only) to see whether a shorter/longer VWAP-slope
        // horizon does better than the 15-bar baseline above.
        (
            "vwap_trend_short_late_vwapslope_lb10",
            Box::new(
                strategies::VwapTrendFollow::new(9, 21, 10, 1.2, false, true, 45)
                    .with_vwap_slope(10, 0.0)
                    .with_id("vwap_trend_short_late_vwapslope_lb10"),
            ),
        ),
        (
            "vwap_trend_short_late_vwapslope_lb20",
            Box::new(
                strategies::VwapTrendFollow::new(9, 21, 10, 1.2, false, true, 45)
                    .with_vwap_slope(20, 0.0)
                    .with_id("vwap_trend_short_late_vwapslope_lb20"),
            ),
        ),
        (
            "vwap_trend_short_late_vwapslope_lb30",
            Box::new(
                strategies::VwapTrendFollow::new(9, 21, 10, 1.2, false, true, 45)
                    .with_vwap_slope(30, 0.0)
                    .with_id("vwap_trend_short_late_vwapslope_lb30"),
            ),
        ),
        // Holding the 15-bar lookback fixed, requiring an actual minimum
        // slope magnitude instead of just "any positive/negative slope
        // counts" — testing whether raising this bar helps here the way it
        // didn't for the raw relative-volume bar.
        // Entry-window start sweep. The live variants use
        // min_time_into_window_minutes=45 against a MidMorning (09:45-11:30)
        // window, i.e. entries only from 10:30 — a one-hour window. These
        // open it progressively earlier to test whether the blocked
        // 09:45-10:30 stretch actually contains a usable edge or just
        // noisier setups that the 45-minute delay was right to exclude.
        (
            "vwap_trend_short_lb30_t0",
            Box::new(
                strategies::VwapTrendFollow::new(9, 21, 10, 1.2, false, true, 0)
                    .with_vwap_slope(30, 0.0)
                    .with_id("vwap_trend_short_lb30_t0"),
            ),
        ),
        // Entry-start sweep, done correctly. The earlier t0/t15/t30 attempt
        // was confounded: with a MidMorning window the strategy only sees
        // bars from 09:45, so EMA(21)/RelativeVolume(10) aren't warm until
        // ~10:06 and the VWAP-slope buffer only starts filling then — the
        // effective start was set by warmup, not by the gate, and every
        // variant produced identical trades. A FullDay window warms
        // everything from 09:15, so min_entry_time is then the only thing
        // controlling the start. max_entry_time is pinned at 11:30 so the
        // window's END matches the live config and only its START varies.
        (
            "vwap_trend_short_ew0945",
            Box::new(
                strategies::VwapTrendFollow::new(9, 21, 10, 1.2, false, true, 0)
                    .with_window(core_types::TimeWindow::FullDay)
                    .with_vwap_slope(20, 0.0)
                    .with_min_entry_time(chrono::NaiveTime::from_hms_opt(9, 45, 0).unwrap())
                    .with_max_entry_time(chrono::NaiveTime::from_hms_opt(11, 30, 0).unwrap())
                    .with_id("vwap_trend_short_ew0945"),
            ),
        ),
        (
            "vwap_trend_short_ew1000",
            Box::new(
                strategies::VwapTrendFollow::new(9, 21, 10, 1.2, false, true, 0)
                    .with_window(core_types::TimeWindow::FullDay)
                    .with_vwap_slope(20, 0.0)
                    .with_min_entry_time(chrono::NaiveTime::from_hms_opt(10, 0, 0).unwrap())
                    .with_max_entry_time(chrono::NaiveTime::from_hms_opt(11, 30, 0).unwrap())
                    .with_id("vwap_trend_short_ew1000"),
            ),
        ),
        (
            "vwap_trend_short_ew1015",
            Box::new(
                strategies::VwapTrendFollow::new(9, 21, 10, 1.2, false, true, 0)
                    .with_window(core_types::TimeWindow::FullDay)
                    .with_vwap_slope(20, 0.0)
                    .with_min_entry_time(chrono::NaiveTime::from_hms_opt(10, 15, 0).unwrap())
                    .with_max_entry_time(chrono::NaiveTime::from_hms_opt(11, 30, 0).unwrap())
                    .with_id("vwap_trend_short_ew1015"),
            ),
        ),
        (
            "vwap_trend_short_ew1030",
            Box::new(
                strategies::VwapTrendFollow::new(9, 21, 10, 1.2, false, true, 0)
                    .with_window(core_types::TimeWindow::FullDay)
                    .with_vwap_slope(20, 0.0)
                    .with_min_entry_time(chrono::NaiveTime::from_hms_opt(10, 30, 0).unwrap())
                    .with_max_entry_time(chrono::NaiveTime::from_hms_opt(11, 30, 0).unwrap())
                    .with_id("vwap_trend_short_ew1030"),
            ),
        ),
        // 2x2 on the entry window's start and end. The allday runs show an
        // inverted-U in Avg R by time of day: 10:30-11:00 is the weakest
        // slot (0.65x breakeven, i.e. below cost) while 11:30-12:00 is the
        // peak (1.21x) and sits just OUTSIDE the live window. These test
        // dropping the weak opening half-hour and/or extending to capture
        // the peak. FullDay window so warmup never sets the start.
        (
            "vwap_trend_short_ew1100_c1130",
            Box::new(
                strategies::VwapTrendFollow::new(9, 21, 10, 1.2, false, true, 0)
                    .with_window(core_types::TimeWindow::FullDay)
                    .with_vwap_slope(20, 0.0)
                    .with_min_entry_time(chrono::NaiveTime::from_hms_opt(11, 0, 0).unwrap())
                    .with_max_entry_time(chrono::NaiveTime::from_hms_opt(11, 30, 0).unwrap())
                    .with_id("vwap_trend_short_ew1100_c1130"),
            ),
        ),
        (
            "vwap_trend_short_ew1030_c1200",
            Box::new(
                strategies::VwapTrendFollow::new(9, 21, 10, 1.2, false, true, 0)
                    .with_window(core_types::TimeWindow::FullDay)
                    .with_vwap_slope(20, 0.0)
                    .with_min_entry_time(chrono::NaiveTime::from_hms_opt(10, 30, 0).unwrap())
                    .with_max_entry_time(chrono::NaiveTime::from_hms_opt(12, 0, 0).unwrap())
                    .with_id("vwap_trend_short_ew1030_c1200"),
            ),
        ),
        (
            "vwap_trend_short_ew1100_c1200",
            Box::new(
                strategies::VwapTrendFollow::new(9, 21, 10, 1.2, false, true, 0)
                    .with_window(core_types::TimeWindow::FullDay)
                    .with_vwap_slope(20, 0.0)
                    .with_min_entry_time(chrono::NaiveTime::from_hms_opt(11, 0, 0).unwrap())
                    .with_max_entry_time(chrono::NaiveTime::from_hms_opt(12, 0, 0).unwrap())
                    .with_id("vwap_trend_short_ew1100_c1200"),
            ),
        ),
        (
            "vwap_trend_short_lb30_t90",
            Box::new(
                strategies::VwapTrendFollow::new(9, 21, 10, 1.2, false, true, 90)
                    .with_vwap_slope(30, 0.0)
                    .with_id("vwap_trend_short_lb30_t90"),
            ),
        ),
        (
            "vwap_trend_short_lb20_t0",
            Box::new(
                strategies::VwapTrendFollow::new(9, 21, 10, 1.2, false, true, 0)
                    .with_vwap_slope(20, 0.0)
                    .with_id("vwap_trend_short_lb20_t0"),
            ),
        ),
        (
            "vwap_trend_short_lb30_t15",
            Box::new(
                strategies::VwapTrendFollow::new(9, 21, 10, 1.2, false, true, 15)
                    .with_vwap_slope(30, 0.0)
                    .with_id("vwap_trend_short_lb30_t15"),
            ),
        ),
        (
            "vwap_trend_short_lb30_t30",
            Box::new(
                strategies::VwapTrendFollow::new(9, 21, 10, 1.2, false, true, 30)
                    .with_vwap_slope(30, 0.0)
                    .with_id("vwap_trend_short_lb30_t30"),
            ),
        ),
        (
            "vwap_trend_short_late_vwapslope_min05",
            Box::new(
                strategies::VwapTrendFollow::new(9, 21, 10, 1.2, false, true, 45)
                    .with_vwap_slope(15, 0.0005)
                    .with_id("vwap_trend_short_late_vwapslope_min05"),
            ),
        ),
        (
            "vwap_trend_short_late_vwapslope_min10",
            Box::new(
                strategies::VwapTrendFollow::new(9, 21, 10, 1.2, false, true, 45)
                    .with_vwap_slope(15, 0.0010)
                    .with_id("vwap_trend_short_late_vwapslope_min10"),
            ),
        ),
        ("strong_rally", Box::new(strategies::StrongRally::default())),
        // Same pattern, looser move threshold (0.1% vs 0.2%) — more
        // qualifying candles, more trades, per the "bigger sample" request.
        (
            "strong_rally_0.1pct",
            Box::new(strategies::StrongRally::new(5, 3, 0.001)),
        ),
        ("ma_alignment", Box::new(strategies::MaAlignment::default())),
        // Three-timeframe variant requested directly: drop the 60m leg,
        // just 45/30/15.
        (
            "ma_alignment_3tier",
            Box::new(strategies::MaAlignment::new(vec![15, 30, 45])),
        ),
        // Requires the 4-tier stack to hold for 2 consecutive bars before
        // entering, instead of firing on the first aligned bar — a direct
        // test of whether that filters out the alignments that immediately
        // collapse (half of all trades' losses come from exactly that).
        (
            "ma_alignment_confirmed",
            Box::new(strategies::MaAlignment::with_confirmation(vec![15, 30, 45, 60], 1)),
        ),
        // Combined: only signals when vwap_trend's EMA-crossover trigger AND
        // ma_alignment's SMA-stack both agree on the same bar.
        ("trend_confluence", Box::new(strategies::TrendConfluence::default())),
    ];

    let strategies: Vec<_> = match &only {
        Some(name) => {
            let filtered: Vec<_> = strategies.into_iter().filter(|(n, _)| n == name).collect();
            if filtered.is_empty() {
                anyhow::bail!("no research strategy variant named {name:?}");
            }
            filtered
        }
        None => strategies,
    };

    let mut run_ids = Vec::with_capacity(strategies.len());
    for (name, strategy) in strategies {
        let storage = storage::SqliteStorage::open(std::path::Path::new("data/papertrader.db"))?;
        let label = format!("{label_prefix}-{name}");
        println!("\n=== {label}: replaying {} instruments ===", instruments.len());
        let run_id = backtest::run_backtest_job(
            storage,
            &feed,
            vec![strategy],
            backtest::BacktestJobParams {
                from: from_date,
                to: to_date,
                label: Some(label.clone()),
                instruments: instruments.clone(),
                engine_config: engine_config_for(),
                config_snapshot_json: config_snapshot.clone(),
            },
            |p| {
                if p.current % 25 == 0 || p.current == p.total {
                    println!("  [{}/{}] {}", p.current, p.total, p.message);
                }
            },
        )
        .await?;
        println!("  -> run #{run_id} ({label})");
        run_ids.push(run_id);
    }

    let storage = open_storage_readonly()?;
    let summary = storage.strategy_summary(Some(&run_ids))?;
    println!("\n{:<20} {:>7} {:>10} {:>8} {:>9}", "STRATEGY", "TRADES", "AVG PNL%", "AVG R", "WIN RATE");
    for s in &summary {
        println!(
            "{:<20} {:>7} {:>10.3} {:>8.3} {:>8.1}%",
            s.strategy_id,
            s.trades,
            s.avg_pnl_pct,
            s.avg_r,
            s.win_rate * 100.0
        );
    }

    let mut rows = Vec::new();
    for &run_id in &run_ids {
        rows.extend(storage.strategy_performance(Some(run_id))?);
    }
    rows.retain(|r| r.trades >= min_trades);
    rows.sort_by(|a, b| {
        b.win_rate
            .partial_cmp(&a.win_rate)
            .unwrap()
            .then(b.avg_r.partial_cmp(&a.avg_r).unwrap())
    });

    println!(
        "\nTop stock x strategy combos by win rate (min {min_trades} trades), out of {} qualifying combos:",
        rows.len()
    );
    println!("{:<12} {:<20} {:>7} {:>10} {:>8} {:>9}", "SYMBOL", "STRATEGY", "TRADES", "AVG PNL%", "AVG R", "WIN RATE");
    for r in rows.iter().take(30) {
        let symbol = r.tradingsymbol.clone().unwrap_or_else(|| format!("#{}", r.instrument_token));
        println!(
            "{:<12} {:<20} {:>7} {:>10.3} {:>8.3} {:>8.1}%",
            symbol,
            r.strategy_id,
            r.trades,
            r.avg_pnl_pct,
            r.avg_r,
            r.win_rate * 100.0
        );
    }

    std::fs::create_dir_all("research")?;
    let report_path = format!("research/{label_prefix}.md");
    let mut out = String::new();
    out.push_str(&format!("# {label_prefix} — exit-rule research\n\n"));
    out.push_str(&format!(
        "Exit rule: risk_pct={:.3}% (hard stop), armed at {:.1}x risk ({:.3}%), \
         give-back trail {:.3}% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).\n\n",
        risk_pct * 100.0,
        min_reward_multiple,
        risk_pct * min_reward_multiple * 100.0,
        trail_param * 100.0,
    ));
    out.push_str(&format!("Universe: {} instruments, {from_date}..{to_date}.\n\n", instruments.len()));
    out.push_str("## Per-strategy rollup\n\n");
    out.push_str("| Strategy | Trades | Avg PnL% | Avg R | Win rate |\n|---|---|---|---|---|\n");
    for s in &summary {
        out.push_str(&format!(
            "| {} | {} | {:.3} | {:.3} | {:.1}% |\n",
            s.strategy_id,
            s.trades,
            s.avg_pnl_pct,
            s.avg_r,
            s.win_rate * 100.0
        ));
    }
    out.push_str(&format!("\n## Top stock x strategy combos by win rate (min {min_trades} trades)\n\n"));
    out.push_str("| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |\n|---|---|---|---|---|---|\n");
    for r in rows.iter().take(100) {
        let symbol = r.tradingsymbol.clone().unwrap_or_else(|| format!("#{}", r.instrument_token));
        out.push_str(&format!(
            "| {} | {} | {} | {:.3} | {:.3} | {:.1}% |\n",
            symbol,
            r.strategy_id,
            r.trades,
            r.avg_pnl_pct,
            r.avg_r,
            r.win_rate * 100.0
        ));
    }
    std::fs::write(&report_path, out)?;
    println!("\nFull ranked results written to {report_path} (also queryable via `report`/`compare` on run ids {run_ids:?}).");

    // Fold the WAL back in before exiting. A research run writes hundreds of
    // thousands of trade rows, and an unattended batch of runs would
    // otherwise leave a WAL larger than the database itself (see
    // SqliteStorage::checkpoint). Never fatal — the results are already
    // committed either way.
    match storage::SqliteStorage::open(std::path::Path::new("data/papertrader.db")) {
        Ok(s) => {
            if let Err(e) = s.checkpoint() {
                println!("note: WAL checkpoint skipped ({e})");
            }
        }
        Err(e) => println!("note: WAL checkpoint skipped ({e})"),
    }

    Ok(())
}

/// Fetches and caches minute candles for every symbol in the configured
/// seed universe (not filtered by the liquidity screen — the full list),
/// for `months` months ending today. Per-symbol failures are logged and
/// skipped rather than aborting the whole batch; re-running is safe
/// (candle rows are upserted by `(instrument_token, interval, ts)`).
#[allow(clippy::too_many_arguments)]
async fn run_backfill(
    months: i64,
    index: Option<String>,
    to: Option<String>,
    from: Option<String>,
    symbols: Option<String>,
    indices: Option<String>,
) -> anyhow::Result<()> {
    let (label, seed_index) = match index {
        Some(s) => (s.clone(), universe::SeedIndex::parse(&s)?),
        None => {
            let config = universe::config::load_default()?;
            (config.seed.clone(), config.seed_index()?)
        }
    };
    let seed = universe::seed::load_default(seed_index).ok_or_else(|| {
        anyhow::anyhow!("no cached {label} list — run `universe seed-refresh --index {label}` first")
    })?;

    let symbol_filter: Option<std::collections::HashSet<String>> = symbols
        .map(|s| s.split(',').map(|sym| sym.trim().to_string()).collect());

    let mut rest = kite_rest_client().await?;
    let all_equities = kite_client::instruments::fetch_nse_equities(&rest).await?;
    // Excluded symbols are dropped here too, not just in research: there is no
    // point spending API calls and disk re-fetching data we've established is
    // bad upstream, and it keeps the cache from silently regrowing.
    let exclude_cfg = universe::config::load_default()?;
    let mut instruments: Vec<_> = if let Some(want) = &indices {
        // Index mode: resolve against the NSE indices segment instead of the
        // equity seed list entirely. Names are matched case-insensitively
        // because the official symbols carry spaces and inconsistent casing
        // ("NIFTY 50", "INDIA VIX").
        let wanted: Vec<String> = want
            .split(',')
            .map(|s| s.trim().to_ascii_uppercase())
            .collect();
        let all = kite_client::instruments::fetch_nse_indices(&rest).await?;
        let found: Vec<_> = all
            .into_iter()
            .filter(|i| wanted.contains(&i.tradingsymbol.to_ascii_uppercase()))
            .collect();
        for w in &wanted {
            if !found
                .iter()
                .any(|i| i.tradingsymbol.to_ascii_uppercase() == *w)
            {
                println!("Warning: index {w:?} not found in the NSE indices list");
            }
        }
        found
    } else {
        all_equities
            .into_iter()
            .filter(|i| seed.contains(&i.tradingsymbol))
            .filter(|i| symbol_filter.as_ref().is_none_or(|f| f.contains(&i.tradingsymbol)))
            .filter(|i| !exclude_cfg.is_excluded(&i.tradingsymbol))
            .collect()
    };
    instruments.sort_by(|a, b| a.tradingsymbol.cmp(&b.tradingsymbol));

    let matched: std::collections::HashSet<_> =
        instruments.iter().map(|i| i.tradingsymbol.clone()).collect();
    // Deliberately excluded symbols aren't "missing" — reporting them as not
    // found in the NSE list would be a false alarm pointing at Kite.
    let missing: Vec<_> = seed
        .iter()
        .filter(|s| !matched.contains(*s) && !exclude_cfg.is_excluded(s))
        .collect();
    if !missing.is_empty() && indices.is_none() {
        println!(
            "Warning: {} of {} seed symbols not found in the NSE instrument list: {:?}",
            missing.len(),
            seed.len(),
            missing
        );
    }

    let to_date = to.map(|s| parse_ymd(&s)).transpose()?.unwrap_or_else(today_ist);
    // `--from` overrides the months window. `--months` is a blunt instrument
    // for filling a known gap: repairing 24 minutes of a single session would
    // otherwise re-fetch a whole month across every instrument, which is
    // hundreds of wasted API calls against a rate-limited endpoint.
    let from_date = match &from {
        Some(s) => {
            let d = parse_ymd(s)?;
            if d > to_date {
                anyhow::bail!("--from {d} is after --to {to_date}");
            }
            d
        }
        None => to_date - chrono::Duration::days(months * 30),
    };
    let (from_dt, to_dt) = ist_bounds(from_date, to_date);

    let storage = storage::SqliteStorage::open(std::path::Path::new("data/papertrader.db"))?;
    storage.upsert_instruments(&instruments)?;

    println!(
        "Backfilling {} instruments, {from_date} to {to_date} (1-minute candles)...",
        instruments.len()
    );
    let total = instruments.len();
    let mut ok_count = 0usize;
    let mut fail_count = 0usize;
    // Kite's historical endpoint appears to rate-limit on sustained request
    // volume within a session (observed as a wall of 502s after ~3,000
    // chunked requests, not a per-request 429) — so beyond the chunk-level
    // pace in `fetch_candles_chunked`, pace between instruments too, and
    // take a longer rest every so often to let any rolling window recover
    // instead of running the whole universe in one continuous burst.
    const INTER_INSTRUMENT_PACE: std::time::Duration = std::time::Duration::from_secs(2);
    const COOLDOWN_EVERY: usize = 20;
    const COOLDOWN_DURATION: std::time::Duration = std::time::Duration::from_secs(90);
    for (idx, inst) in instruments.iter().enumerate() {
        if idx > 0 {
            tokio::time::sleep(INTER_INSTRUMENT_PACE).await;
        }
        if idx > 0 && idx % COOLDOWN_EVERY == 0 {
            println!(
                "... cooling down {}s after {idx} instruments to respect Kite's rate limit ...",
                COOLDOWN_DURATION.as_secs()
            );
            tokio::time::sleep(COOLDOWN_DURATION).await;
        }
        // A single connection-level hiccup (e.g. a brief internet drop)
        // has been observed to permanently poison the rest of a run — the
        // underlying reqwest client's connection pool doesn't self-heal
        // even once the network recovers. So on failure, rebuild the REST
        // client (fresh connection pool, same cached/re-logged-in token)
        // and retry a couple of times with backoff before giving up on
        // this instrument.
        let mut attempt = 0u32;
        let result = loop {
            attempt += 1;
            let r = kite_client::historical::fetch_candles_chunked(
                &rest,
                inst.instrument_token,
                from_dt,
                to_dt,
                core_types::CandleInterval::OneMinute,
            )
            .await;
            match r {
                Ok(candles) => break Ok(candles),
                Err(e) if attempt < 3 => {
                    println!(
                        "[{}/{total}] {}: attempt {attempt} failed ({e}) — rebuilding connection and retrying...",
                        idx + 1,
                        inst.tradingsymbol
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(10 * attempt as u64)).await;
                    if let Ok(fresh) = kite_rest_client().await {
                        rest = fresh;
                    }
                }
                Err(e) => break Err(e),
            }
        };
        match result {
            Ok(candles) => {
                let n = candles.len();
                storage.insert_candles(inst.instrument_token, core_types::CandleInterval::OneMinute, &candles)?;
                println!("[{}/{total}] {}: {n} candles cached", idx + 1, inst.tradingsymbol);
                ok_count += 1;
            }
            Err(e) => {
                println!("[{}/{total}] {}: FAILED — {e}", idx + 1, inst.tradingsymbol);
                fail_count += 1;
            }
        }
    }
    println!("Backfill complete: {ok_count} ok, {fail_count} failed.");

    // Fold the WAL back in. A backfill writes hundreds of thousands of candle
    // rows per instrument, and without this the log grows unbounded: the
    // 2-instrument index backfill alone left a 16GB WAL beside the database
    // and took the disk to 81%. `research` already checkpoints on exit; this
    // is the other heavy writer. Never fatal — rows are committed regardless.
    if let Err(e) = storage.checkpoint() {
        println!("note: WAL checkpoint skipped ({e})");
    }
    Ok(())
}

#[cfg(test)]
mod tally_tests {
    use super::*;
    use live_trading::Alert;

    fn exit(r: f64, pnl: f64) -> Alert {
        Alert::Exit {
            symbol: "X".into(),
            price: 100.0,
            reason: core_types::ExitReason::TrailingStop,
            r_multiple: r,
            pnl_abs: pnl,
        }
    }

    #[test]
    fn a_quiet_day_reports_no_trades_rather_than_a_fake_zero_pnl() {
        let t = SessionTally::default();
        let s = t.summary(true);
        assert!(s.contains("0 trades taken"), "{s:?}");
        // "0.00R" on a day nothing traded reads as a flat result rather than
        // as no result, which are different things.
        assert!(!s.contains("R"), "a no-trade day must not report an R figure: {s:?}");
    }

    #[test]
    fn wins_and_losses_net_out() {
        let mut t = SessionTally::default();
        t.record(&exit(1.5, 5000.0));
        t.record(&exit(-1.0, -3375.0));
        let s = t.summary(true);
        assert!(s.contains("2 trades"), "{s:?}");
        assert!(s.contains("+0.50R"), "{s:?}");
        assert!(s.contains("1625"), "{s:?}");
    }

    #[test]
    fn a_losing_day_shows_its_sign() {
        let mut t = SessionTally::default();
        t.record(&exit(-1.0, -3375.0));
        assert!(t.summary(true).contains("-1.00R"), "{}", t.summary(true));
    }

    /// The two facts that must never be buried: something went wrong, and the
    /// book is not flat.
    #[test]
    fn problems_and_a_non_flat_book_are_both_called_out() {
        let mut t = SessionTally::default();
        t.record(&exit(0.5, 1000.0));
        t.record(&Alert::critical("reconciliation mismatch"));
        let s = t.summary(false);
        assert!(s.contains("1 problem"), "{s:?}");
        assert!(s.contains("NOT FLAT"), "{s:?}");
    }

    #[test]
    fn entries_are_counted_separately_from_exits() {
        let mut t = SessionTally::default();
        t.record(&Alert::Entry {
            symbol: "X".into(),
            direction: core_types::Direction::Short,
            qty: 1,
            price: 1.0,
            stop: 1.0,
            target: 1.0,
        });
        assert_eq!(t.entries, 1);
        assert_eq!(t.exits, 0, "an entry with no exit yet is an open position, not a trade");
    }
}
