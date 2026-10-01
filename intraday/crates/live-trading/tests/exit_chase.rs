//! What happens to an exit order that does not fill.
//!
//! Regression cover for the defect these tests were written against: an exit
//! limit was priced once, from the bar that triggered it, and then never
//! re-priced or escalated. In the case that matters — price continuing in the
//! direction that fired the trailing stop — it could not fill, and because
//! neither the stop nor the trail runs while the slot is `ExitPending`, the
//! position sat unmanaged until the 15:18 session backstop. A trailing exit
//! meant to lock in ~1.5% became a full-day hold closed at market.
//!
//! Everything here runs against `SimulatedBroker` and an in-memory database,
//! so there is no network and no Kite account involved.

use core_types::{
    Broker, Candle, Direction, Exchange, Instrument, NewRun, RunMode, SignalIntent, Storage,
    TimeWindow,
};
use live_trading::{Clock, LiveEngine, RealClock, SimulatedBroker, StockTuning};
use std::collections::HashMap;
use std::sync::Arc;

const TOKEN: u64 = 424242;
const SYMBOL: &str = "TESTCO";
const RISK_PCT: f64 = 0.0075;

fn instrument() -> Instrument {
    Instrument {
        instrument_token: TOKEN,
        tradingsymbol: SYMBOL.to_string(),
        exchange: Exchange::Nse,
        lot_size: 1,
        tick_size: 0.05,
    }
}

fn tuning() -> HashMap<String, StockTuning> {
    HashMap::from([(
        SYMBOL.to_string(),
        StockTuning { symbol: SYMBOL.to_string(), lookback: 20, mrm: 3.0, trail: 0.0075 },
    )])
}

/// Bars start at 10:00 IST.
///
/// The time of day is load-bearing, not decoration. `manage_open` forces an
/// `EodSquareOff` on any bar at or after 15:15 IST, before the exit manager is
/// consulted at all — so a series timestamped outside market hours exercises
/// the square-off path while appearing to test the trail. An earlier version
/// of this file used a bare epoch that landed at 19:43 IST, and every one of
/// its assertions passed with both fixes under test disabled.
fn bar(minute: i64, price: f64) -> Candle {
    use chrono::TimeZone;
    let open = chrono::NaiveDate::from_ymd_opt(2026, 9, 29)
        .unwrap()
        .and_hms_opt(10, 0, 0)
        .unwrap();
    let ts = core_types::time::IST
        .from_local_datetime(&open)
        .unwrap()
        .with_timezone(&chrono::Utc)
        + chrono::Duration::minutes(minute);
    Candle { ts, open: price, high: price, low: price, close: price, volume: 1_000 }
}

fn long_signal(price: f64) -> SignalIntent {
    SignalIntent {
        ts: bar(0, price).ts,
        instrument_token: TOKEN,
        strategy_id: "test".to_string(),
        window: TimeWindow::MidMorning,
        direction: Direction::Long,
        entry_price_hint: price,
        rationale: "exit-chase test".to_string(),
        params: serde_json::json!({}),
    }
}

struct Harness {
    engine: LiveEngine<SimulatedBroker>,
    broker: Arc<SimulatedBroker>,
}

/// Opens a real long position at `entry` through the normal entry path, so the
/// exit machinery under test sees exactly the state a live trade would give
/// it.
async fn open_long_at(entry: f64) -> Harness {
    let mut h = build_engine(entry);
    h.engine.inject_test_signal(TOKEN, long_signal(entry), &bar(0, entry)).await.unwrap();
    assert!(!h.engine.is_flat(), "entry should have opened a position");
    h
}

fn build_engine(price: f64) -> Harness {
    let db = storage::SqliteStorage::open_in_memory().unwrap();
    let inst = instrument();
    db.upsert_instruments(std::slice::from_ref(&inst)).unwrap();
    let run_id = db
        .start_run(&NewRun {
            mode: RunMode::Live,
            config_snapshot_json: "{}".to_string(),
            label: Some("exit-chase-test".to_string()),
            from_date: None,
            to_date: None,
        })
        .unwrap();

    let broker = Arc::new(SimulatedBroker::new());
    broker.update_price(SYMBOL, price);
    let clock: Arc<dyn Clock> = Arc::new(RealClock::new());
    let engine = LiveEngine::new(
        live_trading::build_stock_runtimes(std::slice::from_ref(&inst), &tuning()),
        broker.clone(),
        Box::new(risk::DefaultExitManager),
        Box::new(db),
        run_id,
        RISK_PCT,
        100_000.0,
        chrono::NaiveTime::from_hms_opt(15, 15, 0).unwrap(),
        clock,
    );
    Harness { engine, broker }
}

/// Same, short side.
async fn open_short_at(entry: f64) -> Harness {
    let mut h = build_engine(entry);
    let mut sig = long_signal(entry);
    sig.direction = Direction::Short;
    h.engine.inject_test_signal(TOKEN, sig, &bar(0, entry)).await.unwrap();
    assert!(!h.engine.is_flat(), "entry should have opened a position");
    h
}

/// Feeds one bar while the broker's live price is something *else*.
///
/// The distinction is the whole point. `begin_exit` prices the exit limit from
/// the bar's close, but the order reaches the exchange a moment later, against
/// whatever the market is doing by then. When the two are equal the simulator
/// fills instantly and the pending-exit path is never reached at all — which
/// is exactly how an earlier version of these tests managed to pass with both
/// fixes disabled.
async fn feed(h: &mut Harness, minute: i64, bar_price: f64, broker_price: f64) {
    h.broker.update_price(SYMBOL, broker_price);
    h.engine.on_bar(TOKEN, &bar(minute, bar_price)).await.unwrap();
}

/// Walks price one bar at a time with the broker in step, for the stretches
/// where no exit is in flight.
async fn drive(h: &mut Harness, from_minute: i64, prices: &[f64]) -> i64 {
    let mut minute = from_minute;
    for &p in prices {
        feed(h, minute, p, p).await;
        minute += 1;
    }
    minute
}

/// Runs a long up through the 2.25% arm point and back down onto the 0.75%
/// give-back trail, leaving an exit resting and unfilled because the market
/// has already moved past it.
///
/// Returns the next free bar index. Asserts the position is still open, which
/// is what proves the test is in `ExitPending` rather than having quietly
/// completed at entry.
async fn strand_a_long_exit(h: &mut Harness) -> i64 {
    let m = drive(h, 1, &[1010.0, 1023.0, 1030.0]).await;
    // Bar closes at 1022, tripping the trail (peak 1030 x 0.9925 = 1022.275).
    // The market is already at 1015, so a sell limit at 1022 cannot fill.
    feed(h, m, 1022.0, 1015.0).await;
    assert!(
        !h.engine.is_flat(),
        "setup failed: the exit filled immediately, so the chase path is untested"
    );
    m + 1
}

/// The happy path, kept alongside the failure cases so a fix to the chase
/// cannot quietly break ordinary exits: price comes back to the limit and it
/// fills at the price we asked for, with no chasing and no market order.
#[tokio::test]
async fn an_exit_that_fills_normally_never_chases_or_escalates() {
    let mut h = open_long_at(1000.0).await;
    // Up through the 2.25% arm point, then back to trip the 0.75% give-back.
    drive(&mut h, 1, &[1010.0, 1023.0, 1030.0, 1022.0]).await;
    assert!(h.engine.is_flat(), "the trailing stop should have exited cleanly");
}

/// The defect this file exists for. Price runs away from the exit limit and
/// keeps going; the position must still be closed within a few bars rather
/// than held to the session backstop.
#[tokio::test]
async fn an_exit_that_cannot_fill_escalates_within_the_chase_window() {
    let mut h = open_long_at(1000.0).await;
    let m = strand_a_long_exit(&mut h).await;
    drive(&mut h, m, &[1008.0, 1001.0, 994.0, 988.0, 982.0]).await;

    assert!(
        h.engine.is_flat(),
        "a runaway move must resolve by chasing or escalating, not hold until 15:18"
    );
    let held = h.broker.positions().await.unwrap();
    assert!(
        held.iter().all(|p| p.net_quantity == 0),
        "the broker should show no residual position: {held:?}"
    );
}

/// The resolution must not take longer than the chase window advertises.
/// Before the fix this ran to the end of the session.
#[tokio::test]
async fn the_position_does_not_survive_more_bars_than_the_chase_allows() {
    let mut h = open_long_at(1000.0).await;
    let mut minute = strand_a_long_exit(&mut h).await;

    let mut price = 1008.0;
    let mut bars = 0;
    while !h.engine.is_flat() && bars < 12 {
        feed(&mut h, minute, price, price).await;
        bars += 1;
        minute += 1;
        price -= 7.0;
    }
    assert!(h.engine.is_flat(), "never closed at all — the exit was left resting");
    assert!(
        bars <= 2,
        "took {bars} bars to close; an unfilled exit gets ONE bar before going to market, so \
         anything beyond that means it is being left to rest while unprotected"
    );
}

/// A short mirrors the long: the limit has to chase *upward* and the
/// escalation has to buy back. A sign error here would strand shorts while
/// longs looked fine.
#[tokio::test]
async fn a_short_exit_that_cannot_fill_also_resolves() {
    let mut h = open_short_at(1000.0).await;
    // Down through the 2.25% arm point, then back up onto the trail
    // (trough 970 x 1.0075 = 977.275) with the market already past it.
    let m = drive(&mut h, 1, &[990.0, 977.0, 970.0]).await;
    feed(&mut h, m, 978.0, 985.0).await;
    assert!(!h.engine.is_flat(), "setup failed: the short exit filled immediately");

    drive(&mut h, m + 1, &[992.0, 999.0, 1006.0, 1012.0, 1018.0]).await;
    assert!(h.engine.is_flat(), "a short exit must resolve the same way a long one does");
    assert!(h.broker.positions().await.unwrap().iter().all(|p| p.net_quantity == 0));
}

/// The three-way failure of 2026-09-30, pinned.
///
/// `force_square_off` takes the slot with `mem::replace` before any branch
/// runs. When the broker became unreachable mid-call (a Kite Web login had
/// invalidated the API token), the `?` on `order_status` propagated with the
/// slot already `Idle`. Three things went wrong at once:
///
///   1. a real +Rs.2,657 trade was never booked — `exit_ts` stayed NULL
///   2. `is_flat()` reported true, because the slot had been replaced
///   3. so the caller's "ENDED WITH AN OPEN POSITION" alert never fired
///
/// The session printed `Flat: true` while a position had in fact been closed
/// by the broker and recorded nowhere. Any of the three alone is bad; together
/// they made a real trade invisible.
#[tokio::test]
async fn a_broker_failure_during_square_off_must_not_report_flat() {
    let mut h = open_long_at(1000.0).await;
    // Strand an exit so the slot is ExitPending, which is the state the live
    // book was in when this happened.
    strand_a_long_exit(&mut h).await;
    assert!(!h.engine.is_flat(), "setup: should be holding a pending exit");

    // Make every broker call fail, exactly as an invalidated token does.
    h.broker.fail_all(true);

    let flat = h.engine.force_square_off().await.expect("must not propagate — see the doc comment");
    assert!(
        !flat,
        "force_square_off reported FLAT while the broker was unreachable — the caller uses this \
         to decide whether to raise the alarm, so a false 'flat' silences it"
    );
    assert!(
        !h.engine.is_flat(),
        "the slot was left Idle, so the engine has forgotten a real position"
    );
    let alerts = h.engine.take_alerts();
    assert!(
        alerts.iter().any(|a| a.is_critical()),
        "an unresolvable square-off must raise a critical alert, got {alerts:?}"
    );
}
