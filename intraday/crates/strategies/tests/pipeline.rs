//! Integration test: wires the real `strategy-core` `IndicatorEngine` +
//! `StrategyRunner` to a real `OpeningRangeBreakout`, feeding a plausible
//! sequence of 1-minute bars through the whole pipeline. Unit tests on each
//! piece pass in isolation; this checks they actually cooperate — the
//! indicator engine warms up the right indicators, the window registry
//! routes bars to the right strategy, and a breakout with volume produces
//! a signal at the point a human would expect one.

use chrono::{NaiveTime, TimeZone, Utc};
use core_types::time::IST;
use core_types::{Candle, Direction};
use strategies::OpeningRangeBreakout;
use strategy_core::{IndicatorEngine, StrategyRunner};

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

#[test]
fn opening_range_breakout_fires_through_the_full_pipeline() {
    let engine = IndicatorEngine::new(NaiveTime::from_hms_opt(9, 30, 0).unwrap());
    let strategy = OpeningRangeBreakout::new(3, 1.5);
    let mut runner = StrategyRunner::new(engine, vec![Box::new(strategy)]);

    const TOKEN: u64 = 12345;

    // Opening range formation: 09:15-09:29, range settles at [100, 98].
    let formation_bars = [
        bar(9, 15, 99.0, 100.0, 99.0, 99.5, 500),
        bar(9, 20, 99.5, 99.8, 98.0, 99.0, 500),
        bar(9, 25, 99.0, 99.5, 98.5, 99.2, 500),
    ];
    for b in &formation_bars {
        let signals = runner.on_bar(TOKEN, b);
        assert!(signals.is_empty(), "no signals expected during range formation");
    }

    // Range is formed (9:30+) but price is still inside [98, 100] — quiet
    // bars, also finishing warming up RelativeVolume(3)'s baseline. No
    // breakout attempted yet, so these must stay silent regardless of the
    // indicator's readiness.
    for b in [
        bar(9, 31, 99.2, 99.6, 98.8, 99.3, 500),
        bar(9, 32, 99.3, 99.7, 98.9, 99.4, 500),
    ] {
        assert!(runner.on_bar(TOKEN, &b).is_empty());
    }

    // Now: price clears the opening-range high (100.0) with strong volume
    // relative to the ~500-share baseline — expect exactly one Long signal
    // from the orb strategy.
    let breakout = bar(9, 33, 99.4, 103.0, 99.4, 102.5, 3000);
    let signals = runner.on_bar(TOKEN, &breakout);
    assert_eq!(signals.len(), 1);
    assert_eq!(signals[0].strategy_id, "orb");
    assert_eq!(signals[0].direction, Direction::Long);
    assert_eq!(signals[0].instrument_token, TOKEN);

    // A bar outside every configured window produces no signals at all,
    // regardless of price action — the window registry gates everything.
    let after_hours = bar(15, 45, 102.5, 110.0, 102.0, 109.0, 5000);
    assert!(runner.on_bar(TOKEN, &after_hours).is_empty());
}

#[test]
fn reset_session_lets_a_new_day_signal_again() {
    let engine = IndicatorEngine::new(NaiveTime::from_hms_opt(9, 30, 0).unwrap());
    let strategy = OpeningRangeBreakout::new(1, 1.0);
    let mut runner = StrategyRunner::new(engine, vec![Box::new(strategy)]);
    const TOKEN: u64 = 1;

    runner.on_bar(TOKEN, &bar(9, 15, 100.0, 100.0, 99.0, 99.5, 500));
    runner.on_bar(TOKEN, &bar(9, 32, 99.5, 99.5, 99.0, 99.2, 500)); // warms up RelativeVolume(1)
    let signals = runner.on_bar(TOKEN, &bar(9, 33, 99.2, 102.0, 99.2, 101.5, 2000));
    assert_eq!(signals.len(), 1);

    // Same day, same setup — orb's internal "already signaled" flag should
    // suppress a second signal without a session reset.
    let again = runner.on_bar(TOKEN, &bar(9, 34, 101.5, 102.0, 101.2, 101.8, 2000));
    assert!(again.is_empty());

    runner.reset_session();

    // New session: opening range must re-form from scratch, so an
    // immediate "breakout" bar with no formation history shouldn't fire —
    // this also confirms reset_session cleared the OpeningRange indicator,
    // not just the strategy's own flag.
    let immediately_after_reset = runner.on_bar(TOKEN, &bar(9, 33, 101.8, 105.0, 101.8, 104.0, 3000));
    assert!(immediately_after_reset.is_empty());
}
