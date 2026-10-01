//! Integration tests: wire a real `PaperEngine` to a real `StrategyRunner`,
//! `DefaultExitManager`, and an in-memory `SqliteStorage`, and feed a
//! plausible day of bars through it. Unit tests cover each piece
//! (pnl math, exit-manager state machine, strategies) in isolation; these
//! confirm the whole orchestration — sizing, entry, trailing exit, EOD
//! square-off, and the daily-loss circuit breaker — actually works end to
//! end, the same way a backtest or the live daemon will drive it.

use chrono::{NaiveTime, TimeZone, Utc};
use core_types::time::IST;
use core_types::{
    Candle, Direction, Exchange, IndicatorSpec, Instrument, RunMode, SignalIntent, Strategy,
    StrategyContext, StrategyId, TimeWindow, TrailPolicy,
};
use paper_engine::{EngineConfig, PaperEngine};
use risk::DefaultExitManager;
use storage::SqliteStorage;
use strategies::OpeningRangeBreakout;
use strategy_core::{IndicatorEngine, StrategyRunner};

const TOKEN: u64 = 1;

fn instrument() -> Instrument {
    Instrument {
        tradingsymbol: "TEST".to_string(),
        instrument_token: TOKEN,
        exchange: Exchange::Nse,
        lot_size: 1,
        tick_size: 0.05,
    }
}

fn bar(h: u32, m: u32, open: f64, high: f64, low: f64, close: f64, volume: u64) -> Candle {
    Candle {
        ts: IST.with_ymd_and_hms(2026, 9, 22, h, m, 0).unwrap().with_timezone(&Utc),
        open,
        high,
        low,
        close,
        volume,
    }
}

fn engine_with_orb(config: EngineConfig) -> PaperEngine {
    let indicator_engine = IndicatorEngine::new(NaiveTime::from_hms_opt(9, 30, 0).unwrap());
    let strategy = OpeningRangeBreakout::new(3, 1.5);
    let runner = StrategyRunner::new(indicator_engine, vec![Box::new(strategy)]);
    let storage = SqliteStorage::open_in_memory().unwrap();
    PaperEngine::new(
        runner,
        Box::new(DefaultExitManager),
        Box::new(storage),
        config,
        core_types::NewRun {
            mode: RunMode::Backtest,
            config_snapshot_json: "{}".to_string(),
            label: None,
            from_date: None,
            to_date: None,
        },
    )
    .unwrap()
}

fn base_config() -> EngineConfig {
    EngineConfig {
        risk_pct: 0.0025,
        min_reward_multiple: 2.0,
        trail_policy: TrailPolicy::PercentFromPeak(0.01),
        capital_per_trade: 10_000.0,
        max_concurrent_positions: 1,
        daily_loss_cap_pct: 0.0,
        eod_square_off: NaiveTime::from_hms_opt(15, 15, 0).unwrap(),
        max_hold_minutes: None,
        exit_atr_period: 14,
    }
}

#[test]
fn full_trade_lifecycle_from_breakout_entry_to_trailing_exit() {
    let mut engine = engine_with_orb(base_config());
    let inst = instrument();

    // Opening range formation: 9:15-9:29 -> range settles at [100, 98].
    for b in [
        bar(9, 15, 99.0, 100.0, 99.0, 99.5, 500),
        bar(9, 20, 99.5, 99.8, 98.0, 99.0, 500),
        bar(9, 25, 99.0, 99.5, 98.5, 99.2, 500),
    ] {
        engine.on_bar(&inst, &b).unwrap();
        assert_eq!(engine.open_position_count(), 0);
    }

    // Quiet post-formation bars, still inside the range.
    for b in [
        bar(9, 31, 99.2, 99.6, 98.8, 99.3, 500),
        bar(9, 32, 99.3, 99.7, 98.9, 99.4, 500),
    ] {
        engine.on_bar(&inst, &b).unwrap();
        assert_eq!(engine.open_position_count(), 0);
    }

    // Breakout with volume -> opens a Long position at 102.5.
    engine.on_bar(&inst, &bar(9, 33, 99.4, 103.0, 99.4, 102.5, 3000)).unwrap();
    assert_eq!(engine.open_position_count(), 1);

    // Price rises past the min-target floor (102.5 * 1.005 = 103.0125) ->
    // locks the floor and switches to trailing. Still open.
    engine.on_bar(&inst, &bar(9, 40, 102.5, 103.5, 102.5, 103.5, 800)).unwrap();
    assert_eq!(engine.open_position_count(), 1);

    // Price runs up further -> trailing stop ratchets to ~peak * 0.99.
    engine.on_bar(&inst, &bar(10, 0, 103.5, 105.0, 103.5, 105.0, 800)).unwrap();
    assert_eq!(engine.open_position_count(), 1);

    // Price pulls back through the trailing stop (105 * 0.99 = 103.95) ->
    // closes with a winning trade.
    engine.on_bar(&inst, &bar(10, 5, 105.0, 105.0, 103.5, 103.9, 800)).unwrap();
    assert_eq!(engine.open_position_count(), 0);
    assert!(engine.daily_realized_pnl() > 0.0, "expected a winning trade, got {}", engine.daily_realized_pnl());
}

#[test]
fn eod_square_off_forces_exit_regardless_of_price() {
    let mut engine = engine_with_orb(base_config());
    let inst = instrument();

    for b in [
        bar(9, 15, 99.0, 100.0, 99.0, 99.5, 500),
        bar(9, 20, 99.5, 99.8, 98.0, 99.0, 500),
        bar(9, 25, 99.0, 99.5, 98.5, 99.2, 500),
        bar(9, 31, 99.2, 99.6, 98.8, 99.3, 500),
        bar(9, 32, 99.3, 99.7, 98.9, 99.4, 500),
    ] {
        engine.on_bar(&inst, &b).unwrap();
    }
    engine.on_bar(&inst, &bar(9, 33, 99.4, 103.0, 99.4, 102.5, 3000)).unwrap();
    assert_eq!(engine.open_position_count(), 1);

    // Well within profit and nowhere near the trailing stop, but past the
    // configured EOD cutoff (15:15) — must force-close anyway.
    engine.on_bar(&inst, &bar(15, 20, 104.0, 104.5, 103.8, 104.2, 500)).unwrap();
    assert_eq!(engine.open_position_count(), 0);
}

/// A trivial always-signal strategy, used only to isolate the engine's
/// portfolio-level daily-loss circuit breaker from any strategy-specific
/// edge-triggering (ORB only signals once per session by design, which
/// would confound this test).
struct AlwaysLong;
impl Strategy for AlwaysLong {
    fn id(&self) -> StrategyId {
        "always_long".to_string()
    }
    fn window(&self) -> TimeWindow {
        TimeWindow::Opening
    }
    fn required_indicators(&self) -> &[IndicatorSpec] {
        &[]
    }
    fn on_bar(&mut self, ctx: &StrategyContext) -> Option<SignalIntent> {
        Some(SignalIntent {
            strategy_id: self.id(),
            window: self.window(),
            instrument_token: ctx.instrument_token,
            ts: ctx.candle.ts,
            direction: Direction::Long,
            entry_price_hint: ctx.candle.close,
            rationale: "always".to_string(),
            params: serde_json::json!({}),
        })
    }
}

#[test]
fn daily_loss_cap_blocks_new_entries_after_breach() {
    let indicator_engine = IndicatorEngine::new(NaiveTime::from_hms_opt(9, 30, 0).unwrap());
    let runner = StrategyRunner::new(indicator_engine, vec![Box::new(AlwaysLong)]);
    let storage = SqliteStorage::open_in_memory().unwrap();
    let mut config = base_config();
    // allocated_capital = 10_000 * 1 = 10_000; cap = 1% = 100.
    config.daily_loss_cap_pct = 0.01;
    let mut engine = PaperEngine::new(
        runner,
        Box::new(DefaultExitManager),
        Box::new(storage),
        config,
        core_types::NewRun {
            mode: RunMode::Backtest,
            config_snapshot_json: "{}".to_string(),
            label: None,
            from_date: None,
            to_date: None,
        },
    )
    .unwrap();
    let inst = instrument();

    // Enter immediately (AlwaysLong signals on the very first bar).
    engine.on_bar(&inst, &bar(9, 15, 100.0, 100.0, 100.0, 100.0, 100)).unwrap();
    assert_eq!(engine.open_position_count(), 1);

    // Price drops straight through the hard stop (100 * 0.9975 = 99.75) ->
    // closes at a loss of (100-99.7)*qty. qty = 10000/100 = 100, so
    // pnl_abs = (99.7-100)*100 = -30 — not yet past the 100 cap.
    engine.on_bar(&inst, &bar(9, 16, 99.9, 99.9, 99.6, 99.7, 100)).unwrap();
    assert_eq!(engine.open_position_count(), 0);
    assert!(engine.daily_realized_pnl() < 0.0);

    // AlwaysLong will try to re-enter on the very next bar it's asked
    // about — but only if there's no open position, which there isn't.
    // Force enough additional losing round-trips to actually breach the
    // 100-rupee cap, then confirm a further obvious entry is blocked.
    for i in 0..5 {
        let base_min = 20 + i * 2;
        engine
            .on_bar(&inst, &bar(9, base_min, 100.0, 100.0, 100.0, 100.0, 100))
            .unwrap(); // re-enter
        if engine.open_position_count() == 0 {
            break; // cap already hit before this bar's entry was even accepted
        }
        engine
            .on_bar(&inst, &bar(9, base_min + 1, 99.9, 99.9, 99.6, 99.7, 100))
            .unwrap(); // stopped out again
    }

    assert!(engine.daily_realized_pnl() <= -100.0, "expected the loss cap to be breached, got {}", engine.daily_realized_pnl());
    assert_eq!(engine.open_position_count(), 0, "cap breach must prevent the position that would otherwise be open");

    // One more obvious entry opportunity — must be refused now.
    engine.on_bar(&inst, &bar(9, 40, 100.0, 100.0, 100.0, 100.0, 100)).unwrap();
    assert_eq!(engine.open_position_count(), 0, "no new entries should be accepted after the daily loss cap is hit");
}

/// The failure this guards against: an instrument whose bars stop before the
/// EOD cutoff. The clock-based rule in `manage_open_position` never fires, so
/// before the fix the position survived `reset_session` and was then managed
/// against the NEXT session's bars — collecting an overnight gap an intraday
/// book can never take. It accounted for 9,685 trades (1.23%) across the
/// research runs, including one POLICYBZR position held for six sessions.
#[test]
fn a_position_whose_bars_stop_before_the_cutoff_is_squared_off_at_the_day_boundary() {
    let mut engine = engine_with_orb(base_config());
    let inst = instrument();

    for b in [
        bar(9, 15, 99.0, 100.0, 99.0, 99.5, 500),
        bar(9, 20, 99.5, 99.8, 98.0, 99.0, 500),
        bar(9, 25, 99.0, 99.5, 98.5, 99.2, 500),
        bar(9, 31, 99.2, 99.6, 98.8, 99.3, 500),
        bar(9, 32, 99.3, 99.7, 98.9, 99.4, 500),
    ] {
        engine.on_bar(&inst, &b).unwrap();
    }
    engine.on_bar(&inst, &bar(9, 33, 99.4, 103.0, 99.4, 102.5, 3000)).unwrap();
    assert_eq!(engine.open_position_count(), 1, "position should be open");

    // The last bar of the day arrives at 11:00 — hours before the 15:15
    // cutoff — and then the instrument goes quiet. No further bar ever
    // satisfies `ist_time >= eod_square_off`.
    engine.on_bar(&inst, &bar(11, 0, 102.5, 102.8, 102.0, 102.4, 400)).unwrap();
    assert_eq!(engine.open_position_count(), 1, "still open — cutoff never reached");

    // Crossing into the next session must close it.
    let closed = engine.reset_session().unwrap();
    assert_eq!(closed, 1, "reset_session should have squared one position off");
    assert_eq!(engine.open_position_count(), 0, "nothing may carry overnight");
}

/// The normal path must not change: when a bar does arrive after the cutoff,
/// `manage_open_position` closes it and `reset_session` finds nothing to do.
#[test]
fn reset_session_squares_off_nothing_when_the_cutoff_already_fired() {
    let mut engine = engine_with_orb(base_config());
    let inst = instrument();

    for b in [
        bar(9, 15, 99.0, 100.0, 99.0, 99.5, 500),
        bar(9, 20, 99.5, 99.8, 98.0, 99.0, 500),
        bar(9, 25, 99.0, 99.5, 98.5, 99.2, 500),
        bar(9, 31, 99.2, 99.6, 98.8, 99.3, 500),
        bar(9, 32, 99.3, 99.7, 98.9, 99.4, 500),
    ] {
        engine.on_bar(&inst, &b).unwrap();
    }
    engine.on_bar(&inst, &bar(9, 33, 99.4, 103.0, 99.4, 102.5, 3000)).unwrap();
    engine.on_bar(&inst, &bar(15, 20, 104.0, 104.5, 103.8, 104.2, 500)).unwrap();
    assert_eq!(engine.open_position_count(), 0);

    assert_eq!(engine.reset_session().unwrap(), 0, "nothing left to square off");
}

/// With nothing open at all, the boundary is a no-op.
#[test]
fn reset_session_is_a_no_op_with_no_open_positions() {
    let mut engine = engine_with_orb(base_config());
    assert_eq!(engine.reset_session().unwrap(), 0);
    assert_eq!(engine.open_position_count(), 0);
}
