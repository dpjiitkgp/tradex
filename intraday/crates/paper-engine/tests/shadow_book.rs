//! The behaviour the shadow book depends on: many positions at once, and
//! seeding that warms indicators without trading.
//!
//! The live engine holds exactly one position at a time, so a signal arriving
//! while the slot is busy is discarded — roughly half of them. The shadow book
//! exists to measure what that costs, which only works if lifting
//! `max_concurrent_positions` genuinely lets every signal through. These tests
//! pin that down with several instruments signalling on the same bar, since a
//! cap that silently still applied would make the whole comparison read as
//! "contention costs nothing".

use chrono::{NaiveTime, TimeZone, Utc};
use core_types::time::IST;
use core_types::{Candle, Exchange, Instrument, RunMode, TrailPolicy};
use paper_engine::{EngineConfig, PaperEngine};
use risk::DefaultExitManager;
use storage::SqliteStorage;
use strategies::OpeningRangeBreakout;
use strategy_core::{IndicatorEngine, StrategyRunner};

fn instrument(token: u64, symbol: &str) -> Instrument {
    Instrument {
        tradingsymbol: symbol.to_string(),
        instrument_token: token,
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

fn engine(max_concurrent: usize) -> PaperEngine {
    let runner = StrategyRunner::new(
        IndicatorEngine::new(NaiveTime::from_hms_opt(9, 30, 0).unwrap()),
        vec![Box::new(OpeningRangeBreakout::new(3, 1.5))],
    );
    PaperEngine::new(
        runner,
        Box::new(DefaultExitManager),
        Box::new(SqliteStorage::open_in_memory().unwrap()),
        EngineConfig {
            risk_pct: 0.0075,
            min_reward_multiple: 3.0,
            trail_policy: TrailPolicy::GiveBackFromPeak(0.0075),
            capital_per_trade: 100_000.0,
            max_concurrent_positions: max_concurrent,
            // Off in the shadow book: a circuit breaker would truncate the
            // very tail being measured.
            daily_loss_cap_pct: 0.0,
            eod_square_off: NaiveTime::from_hms_opt(15, 15, 0).unwrap(),
            max_hold_minutes: None,
            exit_atr_period: 14,
        },
        core_types::NewRun {
            mode: RunMode::Backtest,
            config_snapshot_json: "{}".to_string(),
            label: Some("shadow-test".to_string()),
            from_date: None,
            to_date: None,
        },
    )
    .unwrap()
}

/// Drives one instrument from opening-range formation through a breakout that
/// should open a position.
fn drive_to_breakout(e: &mut PaperEngine, inst: &Instrument) {
    for b in [
        bar(9, 15, 99.0, 100.0, 99.0, 99.5, 500),
        bar(9, 20, 99.5, 99.8, 98.0, 99.0, 500),
        bar(9, 25, 99.0, 99.5, 98.5, 99.2, 500),
        bar(9, 30, 99.2, 99.6, 99.0, 99.4, 400),
        bar(9, 35, 99.4, 99.9, 99.2, 99.6, 400),
        // Breakout above the range on heavy volume.
        bar(9, 40, 99.6, 101.5, 99.5, 101.2, 5_000),
    ] {
        e.on_bar(inst, &b).unwrap();
    }
}

#[test]
fn the_live_cap_of_one_discards_every_signal_after_the_first() {
    let mut e = engine(1);
    let a = instrument(1, "AAA");
    let b = instrument(2, "BBB");
    let c = instrument(3, "CCC");
    drive_to_breakout(&mut e, &a);
    drive_to_breakout(&mut e, &b);
    drive_to_breakout(&mut e, &c);
    assert_eq!(
        e.open_position_count(),
        1,
        "this is the live engine's behaviour and the thing the shadow book measures against"
    );
}

/// The claim the shadow book rests on.
#[test]
fn lifting_the_cap_takes_every_signal_instead() {
    let mut e = engine(25);
    let instruments: Vec<_> = (1..=3).map(|i| instrument(i, &format!("S{i}"))).collect();
    for inst in &instruments {
        drive_to_breakout(&mut e, inst);
    }
    assert_eq!(
        e.open_position_count(),
        3,
        "every signalling instrument should hold a position when the cap is lifted"
    );
}

/// Seeding is what keeps the two books comparable — it must advance VWAP and
/// the strategy's own state while never opening anything. If it traded, the
/// shadow book would enter at prices that no longer exist.
#[test]
fn seeding_warms_indicators_without_opening_a_position() {
    let mut e = engine(25);
    let inst = instrument(1, "AAA");
    for b in [
        bar(9, 15, 99.0, 100.0, 99.0, 99.5, 500),
        bar(9, 20, 99.5, 99.8, 98.0, 99.0, 500),
        bar(9, 25, 99.0, 99.5, 98.5, 99.2, 500),
        bar(9, 30, 99.2, 99.6, 99.0, 99.4, 400),
        bar(9, 35, 99.4, 99.9, 99.2, 99.6, 400),
        bar(9, 40, 99.6, 101.5, 99.5, 101.2, 5_000), // would break out
    ] {
        e.seed_bar(&inst, &b);
    }
    assert_eq!(
        e.open_position_count(),
        0,
        "a replayed bar must never open a position — its price is already gone"
    );
}

/// Seeded state has to actually carry forward, or seeding is just a slow no-op
/// and the shadow book runs on a truncated VWAP anyway.
#[test]
fn a_seeded_engine_still_trades_on_the_next_live_bar() {
    let mut e = engine(25);
    let inst = instrument(1, "AAA");
    // Everything except the breakout arrives as history.
    for b in [
        bar(9, 15, 99.0, 100.0, 99.0, 99.5, 500),
        bar(9, 20, 99.5, 99.8, 98.0, 99.0, 500),
        bar(9, 25, 99.0, 99.5, 98.5, 99.2, 500),
        bar(9, 30, 99.2, 99.6, 99.0, 99.4, 400),
        bar(9, 35, 99.4, 99.9, 99.2, 99.6, 400),
    ] {
        e.seed_bar(&inst, &b);
    }
    assert_eq!(e.open_position_count(), 0);

    // The breakout arrives live. It can only fire if the opening range and
    // volume baseline survived the seeding.
    e.on_bar(&inst, &bar(9, 40, 99.6, 101.5, 99.5, 101.2, 5_000)).unwrap();
    assert_eq!(
        e.open_position_count(),
        1,
        "the opening range and volume baseline built during seeding must persist"
    );
}

/// Every open position has to be closed at the session boundary, not just the
/// first — an intraday book that carries anything overnight is not one.
#[test]
fn the_session_boundary_squares_off_all_concurrent_positions() {
    let mut e = engine(25);
    let instruments: Vec<_> = (1..=3).map(|i| instrument(i, &format!("S{i}"))).collect();
    for inst in &instruments {
        drive_to_breakout(&mut e, inst);
    }
    assert_eq!(e.open_position_count(), 3);

    let squared = e.reset_session().unwrap();
    assert_eq!(squared, 3, "all three should square off at the boundary");
    assert_eq!(e.open_position_count(), 0);
}
