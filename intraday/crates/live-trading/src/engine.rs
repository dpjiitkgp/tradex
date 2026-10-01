//! The single-position state machine: exactly one open trade at a time
//! across the whole book, each stock running its configured
//! (lookback, MRM, trail) strategy config, real order execution via
//! `ExecutionPolicy` instead of `PaperEngine`'s instant simulated fills.
//!
//! Deliberately simple concurrency model: while `Idle`, every incoming bar
//! is checked for a signal on whichever stock it belongs to and the first
//! one to signal takes the slot; while `EntryPending`/`Open`/`ExitPending`,
//! every other stock's bar is a no-op (logged, never queued) until the
//! current position fully resolves. Bars naturally arrive one instrument
//! at a time from the tick-aggregation loop, so no additional locking is
//! needed for this to be race-free — the caller driving `on_bar` one call
//! at a time *is* the synchronization.

use crate::alert::Alert;
use crate::config::StockTuning;
use crate::execution::{ExecutionOutcome, ExecutionPolicy, ExecutionStep, FallbackConfig};
use core_types::time::IST;
use core_types::{
    Broker, BrokerOrderAction, BrokerOrderEvent, Candle, Direction, EntryContext, ExitManager,
    ExitReason, Instrument, MarketSnapshot, NewTrade, OrderRequest, OrderSide, OrderStatus,
    OrderType, PositionPhase, PositionState, Product, Result as CtResult, RunId, SignalIntent,
    Storage, TradeExit, TradeId, TrailPolicy,
};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use strategy_core::StrategyRunner;

pub struct StockRuntime {
    pub instrument: Instrument,
    pub tuning: StockTuning,
    pub runner: StrategyRunner,
}

#[derive(Debug, Clone)]
struct PendingEntry {
    instrument_token: u64,
    tradingsymbol: String,
    broker_order_id: String,
    direction: Direction,
    requested_qty: u32,
    limit_price: f64,
    signal: SignalIntent,
    signal_id: Option<core_types::SignalId>,
    placed_at: core_types::time::DateTime,
}

#[derive(Debug, Clone)]
struct OpenPos {
    instrument_token: u64,
    trade_id: TradeId,
    tradingsymbol: String,
    qty: u32,
    state: PositionState,
}

#[derive(Debug, Clone)]
struct PendingExit {
    instrument_token: u64,
    trade_id: TradeId,
    tradingsymbol: String,
    qty: u32,
    direction: Direction,
    reason: ExitReason,
    entry_price: f64,
    broker_order_id: String,
    /// The limit currently resting at the broker. Kept so a re-price only
    /// sends `modify_order` when the price has actually moved a tick, rather
    /// than once a minute regardless.
    limit_price: f64,
    /// Bars this exit has been waiting. Drives the chase-then-market
    /// escalation in `poll_pending_exit`.
    bars_pending: u32,
}

enum Slot {
    Idle,
    EntryPending(PendingEntry),
    Open(OpenPos),
    ExitPending(PendingExit),
}

/// How long to wait for a plain-limit entry to fill before giving up on
/// that signal entirely (no market-order fallback for entries, per the
/// user's explicit choice — timing out just means the trade is skipped,
/// not chased). Not user-specified beyond "plain limit, no fallback" —
/// this default is a judgment call, logged clearly whenever it fires.
const ENTRY_TIMEOUT: Duration = Duration::from_secs(60);

/// Bars an unfilled exit limit waits before being cancelled and taken at
/// market.
///
/// **One.** The limit gets a single minute; if it has not filled, we cross the
/// spread rather than keep trying.
///
/// This started at three, re-pricing the limit to each new bar's close on the
/// way. That looked prudent and was not, because the two cases it splits into
/// do not overlap:
///
///   * **Price calm** — the first limit fills inside its own bar. The chase
///     never engages, so it saves nothing.
///   * **Price running** — each re-price lands on the previous bar's close,
///     which the market has already left, so it misses again. Three minutes
///     later we take market anyway: same spread, worse mid, and the position
///     was unprotected throughout (neither stop nor trail runs while the slot
///     is `ExitPending`).
///
/// So the chase's benefit exists only in the case where it is unnecessary, and
/// its cost falls entirely on the case that matters. Measured against the
/// alternative: crossing the spread on this book costs on the order of
/// 0.02-0.2% (~0.03-0.27 R), while a stock that has just triggered a stop can
/// travel far more than that in three minutes.
///
/// A non-fill after one minute in a liquid name is not bad luck — it is the
/// information that price is moving away.
///
/// The re-pricing code below is kept and still tested, but is unreachable at
/// this setting; raising the constant re-enables it.
const EXIT_CHASE_BARS: u32 = 1;

/// How far past its trigger a stop-loss limit is placed, as a fraction of the
/// trigger price.
///
/// Both were previously set to the stop price itself, which cannot work: the
/// engine only sends the stop order *after* price has already crossed the
/// level, so the order triggers instantly and becomes a limit at a price the
/// market has left. Every stop-out then burned the full 10-second fallback
/// window and took market anyway.
///
/// Offsetting it means the limit is marketable when it triggers, and caps
/// slippage at ~0.4 R (0.3% against a 0.75% risk unit) instead of leaving it
/// to the market order. The market fallback still backs it up for a real gap.
const STOP_LIMIT_OFFSET: f64 = 0.003;

pub struct LiveEngine<B: Broker> {
    stocks: HashMap<u64, StockRuntime>,
    broker: Arc<B>,
    execution: ExecutionPolicy<B>,
    exit_manager: Box<dyn ExitManager>,
    storage: Box<dyn Storage>,
    run_id: RunId,
    risk_pct: f64,
    capital: f64,
    eod_square_off: chrono::NaiveTime,
    slot: Slot,
    halted: bool,
    /// Alerts the caller has not drained yet. Queued rather than sent: the
    /// trading loop must never block on a notification service, and a queued
    /// alert is a value a test can assert on. See `alert.rs`.
    pending_alerts: Vec<Alert>,
    /// Last bar seen per instrument. A session-end square-off needs a price
    /// and timestamp to close against, and by then no new bar is coming.
    last_seen: HashMap<u64, Candle>,
}

/// Narrows a broker position list to the rows this engine is responsible for.
///
/// Kite's positions endpoint returns every instrument touched during the day,
/// including ones already squared off (`net_quantity == 0`) — a flat row is a
/// history entry, not exposure. The same account may also be used for manual
/// trades in other symbols, which this engine never opened and must not halt
/// the book over.
///
/// Both cases were observed live: a flat `PPLPHARMA` row — a symbol outside
/// the book entirely — tripped a reconciliation mismatch and halted new
/// entries on 2026-09-29.
fn reconcilable_positions(
    all: &[core_types::BrokerPosition],
    ours: &std::collections::HashSet<&str>,
) -> Vec<core_types::BrokerPosition> {
    all.iter()
        .filter(|p| p.net_quantity != 0)
        .filter(|p| ours.contains(p.tradingsymbol.as_str()))
        .cloned()
        .collect()
}

impl<B: Broker> LiveEngine<B> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        stocks: HashMap<u64, StockRuntime>,
        broker: Arc<B>,
        exit_manager: Box<dyn ExitManager>,
        storage: Box<dyn Storage>,
        run_id: RunId,
        risk_pct: f64,
        capital: f64,
        eod_square_off: chrono::NaiveTime,
        clock: Arc<dyn crate::clock::Clock>,
    ) -> Self {
        let execution = ExecutionPolicy::new(broker.clone(), clock);
        Self {
            stocks,
            broker,
            execution,
            exit_manager,
            storage,
            run_id,
            risk_pct,
            capital,
            eod_square_off,
            slot: Slot::Idle,
            halted: false,
            pending_alerts: Vec::new(),
            last_seen: HashMap::new(),
        }
    }

    pub fn halt(&mut self) {
        self.halted = true;
    }

    pub fn resume(&mut self) {
        self.halted = false;
    }

    /// Drains the queued alerts. The caller sends them; the engine only ever
    /// records that something worth telling a human about happened.
    pub fn take_alerts(&mut self) -> Vec<Alert> {
        std::mem::take(&mut self.pending_alerts)
    }

    pub fn is_flat(&self) -> bool {
        matches!(self.slot, Slot::Idle)
    }

    /// Reconciliation check: compares `Broker::positions()` against what
    /// this engine believes is open. Any mismatch is a critical condition
    /// (something placed/filled outside this engine's knowledge, or a bug
    /// in this engine's own bookkeeping) — halts new entries rather than
    /// continuing to trade on top of an unknown discrepancy. Does not
    /// touch any *existing* open position's exit handling, which keeps
    /// running regardless (getting flat is still the priority).
    pub async fn reconcile(&mut self) -> CtResult<bool> {
        let all_positions = self.broker.positions().await?;
        // Two kinds of noise have to come out before comparing, or the
        // engine halts itself over things that are not exposure at all:
        //
        //  * `net_quantity == 0` — Kite's positions endpoint lists every
        //    instrument touched during the day, including ones already
        //    squared off. A flat row is a history entry, not a position.
        //  * symbols outside this book — the same Kite account may be used
        //    for manual trades. Those are not ours to reconcile, and
        //    halting the book because the account holds something we never
        //    opened would stop trading for an entirely unrelated reason.
        //
        // Both were observed live: a flat PPLPHARMA row (a symbol not in
        // the book) halted new entries on 2026-09-29.
        let ours: std::collections::HashSet<&str> = self
            .stocks
            .values()
            .map(|s| s.instrument.tradingsymbol.as_str())
            .collect();
        let broker_positions = reconcilable_positions(&all_positions, &ours);
        let broker_net: HashMap<&str, i64> = broker_positions
            .iter()
            .map(|p| (p.tradingsymbol.as_str(), p.net_quantity))
            .collect();

        let expected: Option<(&str, i64)> = if let Slot::Open(p) = &self.slot {
            let signed = match p.state.direction {
                Direction::Long => p.qty as i64,
                Direction::Short => -(p.qty as i64),
            };
            Some((p.tradingsymbol.as_str(), signed))
        } else {
            None
        };

        let ok = match expected {
            Some((symbol, qty)) => broker_net.get(symbol).copied().unwrap_or(0) == qty,
            None => broker_positions.is_empty() || matches!(self.slot, Slot::EntryPending(_) | Slot::ExitPending(_)),
        };

        if !ok {
            tracing::error!(
                expected = ?expected,
                broker_positions = ?broker_positions,
                "RECONCILIATION MISMATCH — halting new entries until manually resolved"
            );
            // Only on the transition. Reconciliation runs every 60s and the
            // mismatch persists until a human fixes it, so alerting each time
            // would send a message a minute for the rest of the session.
            if !self.halted {
                self.pending_alerts.push(Alert::Halted {
                    reason: format!(
                        "reconciliation mismatch — engine expects {}, broker shows {:?}",
                        match expected {
                            Some((sym, q)) => format!("{sym} {q}"),
                            None => "nothing held".to_string(),
                        },
                        broker_positions
                            .iter()
                            .map(|p| format!("{} {}", p.tradingsymbol, p.net_quantity))
                            .collect::<Vec<_>>(),
                    ),
                });
            }
            self.halted = true;
        }
        Ok(ok)
    }

    /// Feeds one bar for one instrument. No-ops for instruments not in the
    /// configured book (the caller may be watching a broader universe
    /// for other reasons; this engine only ever acts on its own book).
    pub async fn on_bar(&mut self, instrument_token: u64, candle: &Candle) -> CtResult<()> {
        let ist_time = candle.ts.with_timezone(&IST).time();
        self.last_seen.insert(instrument_token, candle.clone());

        match std::mem::replace(&mut self.slot, Slot::Idle) {
            Slot::Idle => {
                self.slot = Slot::Idle;
                if !self.halted {
                    self.try_enter(instrument_token, candle).await?;
                }
            }
            Slot::EntryPending(pending) => {
                if pending.instrument_token == instrument_token {
                    self.poll_pending_entry(pending, candle).await?;
                } else {
                    tracing::debug!(
                        symbol = %self.stocks.get(&instrument_token).map(|s| s.instrument.tradingsymbol.as_str()).unwrap_or("?"),
                        "signal skipped — position slot busy (entry pending on another stock)"
                    );
                    self.slot = Slot::EntryPending(pending);
                }
            }
            Slot::Open(pos) => {
                if pos.instrument_token == instrument_token {
                    self.manage_open(pos, candle, ist_time).await?;
                } else {
                    self.slot = Slot::Open(pos);
                }
            }
            Slot::ExitPending(pending) => {
                if pending.instrument_token == instrument_token {
                    self.poll_pending_exit(pending, candle).await?;
                } else {
                    self.slot = Slot::ExitPending(pending);
                }
            }
        }
        Ok(())
    }

    /// Warms one stock's indicators with an already-elapsed bar, without
    /// trading on it.
    ///
    /// Needed because the engine builds every indicator from the bars it is
    /// fed, and VWAP is *session-cumulative*: a process started at 09:42
    /// computes VWAP over 09:42-onward, not from the 09:15 open, which is
    /// what the backtest uses. Measured on real sessions the gap is only
    /// 0.01-0.57%, but the strategy gates on which side of VWAP price sits,
    /// and that is enough to flip the test — so an unseeded engine is not
    /// running quite the strategy that was validated.
    ///
    /// Signals from replayed bars are deliberately discarded: the bar is in
    /// the past, so acting on it would place an order against a price that
    /// no longer exists. Only indicator state is advanced. The position slot
    /// is never touched, so seeding cannot open, close or interfere with a
    /// trade.
    pub fn seed_bar(&mut self, instrument_token: u64, candle: &Candle) {
        if let Some(stock) = self.stocks.get_mut(&instrument_token) {
            let _ = stock.runner.on_bar(instrument_token, candle);
        }
    }

    async fn try_enter(&mut self, instrument_token: u64, candle: &Candle) -> CtResult<()> {
        // Everything needed from `stock` is copied out up front so its
        // mutable borrow (needed for `runner.on_bar`) doesn't outlive this
        // block — the rest of the function needs `&mut self`/`&self` for
        // broker calls, execution, and storage, which would otherwise
        // conflict with a still-live borrow into `self.stocks`.
        let (signal, tradingsymbol, exchange, tick_size, tuning) = {
            let Some(stock) = self.stocks.get_mut(&instrument_token) else {
                return Ok(());
            };
            let signals = stock.runner.on_bar(instrument_token, candle);
            let Some(signal) = signals.into_iter().next() else {
                return Ok(());
            };
            (
                signal,
                stock.instrument.tradingsymbol.clone(),
                stock.instrument.exchange,
                stock.instrument.tick_size,
                stock.tuning.clone(),
            )
        };
        self.act_on_signal(instrument_token, signal, tradingsymbol, exchange, tick_size, tuning, candle)
            .await
    }

    /// Dry-run validation only: bypasses real strategy signal detection and
    /// feeds a fabricated `SignalIntent` through the *exact* same pipeline
    /// (margin probe, sizing, real margin re-check, order placement, fill
    /// tracking, storage logging) a real strategy-detected signal would go
    /// through — the only difference is where the signal came from. Never
    /// call this with a `KiteBroker`; nothing here stops it from placing a
    /// real order if the underlying `Broker` actually is one, so that
    /// restriction is on the caller (the CLI only ever wires this to a
    /// `SimulatedBroker` engine).
    pub async fn inject_test_signal(&mut self, instrument_token: u64, signal: SignalIntent, candle: &Candle) -> CtResult<()> {
        if !matches!(self.slot, Slot::Idle) {
            return Err(core_types::Error::Broker(
                "inject_test_signal: engine is not idle (a position or pending order is already in flight)".to_string(),
            ));
        }
        let Some(stock) = self.stocks.get(&instrument_token) else {
            return Err(core_types::Error::Broker(format!(
                "inject_test_signal: instrument_token {instrument_token} is not in the tuned book"
            )));
        };
        let (tradingsymbol, exchange, tick_size, tuning) = (
            stock.instrument.tradingsymbol.clone(),
            stock.instrument.exchange,
            stock.instrument.tick_size,
            stock.tuning.clone(),
        );
        self.act_on_signal(instrument_token, signal, tradingsymbol, exchange, tick_size, tuning, candle)
            .await
    }

    #[allow(clippy::too_many_arguments)]
    async fn act_on_signal(
        &mut self,
        instrument_token: u64,
        signal: SignalIntent,
        tradingsymbol: String,
        exchange: core_types::Exchange,
        tick_size: f64,
        tuning: StockTuning,
        candle: &Candle,
    ) -> CtResult<()> {
        let entry_price_hint = signal.entry_price_hint;

        tracing::info!(
            symbol = %tradingsymbol,
            strategy = %signal.strategy_id,
            direction = ?signal.direction,
            entry_price_hint,
            rationale = %signal.rationale,
            "SIGNAL"
        );
        // Recorded regardless of what happens next (skipped for sizing,
        // rejected, or opened) — every signal is a real decision point
        // worth having on record, not just the ones that become trades.
        let signal_id = self.storage.record_signal(&core_types::SignalRecord {
            run_id: self.run_id,
            ts: signal.ts,
            instrument_token,
            strategy_id: signal.strategy_id.clone(),
            window: signal.window,
            direction: signal.direction,
            entry_price_hint: signal.entry_price_hint,
            rationale: signal.rationale.clone(),
            params_json: signal.params.to_string(),
        })?;

        let side = match signal.direction {
            Direction::Long => OrderSide::Buy,
            Direction::Short => OrderSide::Sell,
        };

        // Size against the broker's own margin calculator for this
        // specific stock (not an assumed flat leverage multiple) — a
        // qty=1 probe gives the true per-share margin, then scale to the
        // ~capital budget and re-confirm at the real quantity before ever
        // placing anything.
        let probe_req = OrderRequest {
            tradingsymbol: tradingsymbol.clone(),
            exchange,
            side,
            quantity: 1,
            product: Product::Mis,
            order_type: OrderType::Limit,
            price: Some(round_to_tick(entry_price_hint, tick_size)),
            trigger_price: None,
            tag: format!("probe-{}", self.run_id),
        };
        let probe = self.broker.check_margin(&probe_req).await?;
        let Some(per_share_margin) = probe.required_margin.filter(|m| *m > 0.0) else {
            tracing::warn!(symbol = %tradingsymbol, reason = ?probe.rejection_reason, "skipping signal — could not determine per-share margin");
            return Ok(());
        };
        let qty = (self.capital / per_share_margin).floor() as u32;
        if qty == 0 {
            tracing::warn!(symbol = %tradingsymbol, per_share_margin, capital = self.capital, "skipping signal — capital too small for even 1 share at this margin rate");
            return Ok(());
        }

        let limit_price = round_to_tick(entry_price_hint, tick_size);
        let entry_req = OrderRequest {
            quantity: qty,
            tag: format!("entry-{}", self.run_id),
            ..probe_req
        };
        let margin_check = self.broker.check_margin(&entry_req).await?;
        if !margin_check.ok {
            tracing::warn!(symbol = %tradingsymbol, qty, reason = ?margin_check.rejection_reason, "skipping signal — full-size margin check rejected");
            return Ok(());
        }

        let outcome = self.execution.place_plain(&entry_req).await?;
        self.log_execution_outcome(None, instrument_token, &tradingsymbol, &entry_req, &outcome);
        let result = &outcome.final_result;

        if result.status == OrderStatus::Rejected {
            tracing::warn!(symbol = %tradingsymbol, reason = ?result.rejection_reason, "entry order rejected");
            self.pending_alerts.push(Alert::OrderRejected {
                symbol: tradingsymbol.clone(),
                what: "entry".to_string(),
                reason: result.rejection_reason.clone().unwrap_or_else(|| "no reason given".to_string()),
            });
            return Ok(());
        }

        if result.status == OrderStatus::Complete {
            let (qty, avg_price) = (entry_req.quantity, result.average_price.unwrap_or(limit_price));
            self.open_position_from_fill(tuning, &tradingsymbol, instrument_token, &signal, Some(signal_id), qty, avg_price, candle.ts)?;
        } else {
            self.slot = Slot::EntryPending(PendingEntry {
                instrument_token,
                tradingsymbol,
                broker_order_id: result.broker_order_id.clone(),
                direction: signal.direction,
                requested_qty: entry_req.quantity,
                limit_price,
                signal,
                signal_id: Some(signal_id),
                placed_at: candle.ts,
            });
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn open_position_from_fill(
        &mut self,
        tuning: StockTuning,
        tradingsymbol: &str,
        instrument_token: u64,
        signal: &SignalIntent,
        signal_id: Option<core_types::SignalId>,
        qty: u32,
        entry_price: f64,
        entry_ts: core_types::time::DateTime,
    ) -> CtResult<()> {
        let entry = EntryContext {
            direction: signal.direction,
            entry_price,
            entry_ts,
            risk_pct: self.risk_pct,
            min_reward_multiple: tuning.mrm,
            trail_policy: TrailPolicy::GiveBackFromPeak(tuning.trail),
        };
        let plan = self.exit_manager.on_entry(&entry);
        let capital_allocated = entry_price * qty as f64;

        tracing::info!(
            symbol = %tradingsymbol,
            direction = ?signal.direction,
            qty,
            entry_price,
            hard_stop = plan.hard_stop_price,
            min_target = plan.min_target_price,
            capital_allocated,
            "ENTRY"
        );
        self.pending_alerts.push(Alert::Entry {
            symbol: tradingsymbol.to_string(),
            direction: signal.direction,
            qty,
            price: entry_price,
            stop: plan.hard_stop_price,
            target: plan.min_target_price,
        });

        let trade_id = self.storage.open_trade(&NewTrade {
            run_id: self.run_id,
            signal_id,
            instrument_token,
            strategy_id: signal.strategy_id.clone(),
            window: signal.window,
            mode: core_types::RunMode::Live,
            direction: signal.direction,
            qty,
            capital_allocated,
            entry_ts,
            entry_price,
            hard_stop_price: plan.hard_stop_price,
            min_target_price: plan.min_target_price,
            trail_policy: plan.trail_policy,
        })?;

        self.slot = Slot::Open(OpenPos {
            instrument_token,
            trade_id,
            tradingsymbol: tradingsymbol.to_string(),
            qty,
            state: PositionState {
                direction: signal.direction,
                entry_price,
                current_stop: plan.hard_stop_price,
                peak_favorable_price: entry_price,
                phase: PositionPhase::Armed,
            },
        });
        Ok(())
    }

    async fn poll_pending_entry(&mut self, pending: PendingEntry, candle: &Candle) -> CtResult<()> {
        let side = match pending.direction {
            Direction::Long => OrderSide::Buy,
            Direction::Short => OrderSide::Sell,
        };
        let result = self.broker.order_status(&pending.broker_order_id).await?;
        self.log_order_event(
            None,
            pending.instrument_token,
            &pending.tradingsymbol,
            BrokerOrderAction::StatusPoll,
            side,
            OrderType::Limit,
            &result,
            false,
        );

        if result.status == OrderStatus::Complete {
            let stock = self.stocks.get(&pending.instrument_token).map(stock_tuning);
            let Some(tuning) = stock else { return Ok(()) };
            self.open_position_from_fill(
                tuning,
                &pending.tradingsymbol,
                pending.instrument_token,
                &pending.signal,
                pending.signal_id,
                result.filled_quantity,
                result.average_price.unwrap_or(pending.limit_price),
                candle.ts,
            )?;
            return Ok(());
        }

        let elapsed = (candle.ts - pending.placed_at).num_seconds().max(0) as u64;
        if elapsed >= ENTRY_TIMEOUT.as_secs() {
            self.broker.cancel_order(&pending.broker_order_id).await?;
            if result.filled_quantity > 0 {
                // Partially filled before the cancel caught up — that
                // partial quantity is now a real position on the broker's
                // books regardless of "no fallback for entries"; it must
                // be tracked as Open (with the real partial qty), not
                // silently dropped, or the engine and the broker disagree
                // about what's held from this point on.
                tracing::warn!(
                    symbol = %pending.tradingsymbol,
                    elapsed_secs = elapsed,
                    requested_qty = pending.requested_qty,
                    filled_qty = result.filled_quantity,
                    "entry limit order timed out PARTIALLY filled — cancelled the remainder, tracking the partial fill as an open position"
                );
                let stock = self.stocks.get(&pending.instrument_token).map(stock_tuning);
                if let Some(tuning) = stock {
                    self.open_position_from_fill(
                        tuning,
                        &pending.tradingsymbol,
                        pending.instrument_token,
                        &pending.signal,
                        pending.signal_id,
                        result.filled_quantity,
                        result.average_price.unwrap_or(pending.limit_price),
                        candle.ts,
                    )?;
                }
            } else {
                tracing::warn!(
                    symbol = %pending.tradingsymbol,
                    elapsed_secs = elapsed,
                    "entry limit order timed out unfilled — cancelled, no fallback for entries per design"
                );
                self.pending_alerts.push(Alert::EntryUnfilled {
                    symbol: pending.tradingsymbol.clone(),
                    seconds: elapsed,
                });
                self.slot = Slot::Idle;
            }
        } else {
            self.slot = Slot::EntryPending(pending);
        }
        Ok(())
    }

    /// Closes anything still open at the end of the session, at market.
    ///
    /// The clock-based square-off in `manage_open` only fires when a *bar*
    /// arrives at or after `eod_square_off`. If the held stock goes quiet
    /// before then — thin book, a halt, a tick stream that stalls — no such
    /// bar ever arrives and a real position survives the session. The same
    /// hole in the backtest engine left 9,685 positions open overnight.
    ///
    /// A resting limit is not good enough here: an unfilled EOD exit parks in
    /// `ExitPending` waiting for a next bar that will never come. Being flat
    /// matters more than the last few paise at 15:15, and the broker will
    /// force MIS square-off on its own terms shortly after anyway, so this
    /// goes to market for whatever is still outstanding.
    ///
    /// Returns `true` if the book is flat afterwards. A `false` return is a
    /// critical condition — a real position is open with no session left to
    /// manage it — and the caller must alert loudly rather than exit quietly.
    /// # The slot is taken before any branch runs
    ///
    /// `mem::replace` installs `Idle` up front, so **every** early return in
    /// this function must either book the trade or put the slot back. An error
    /// escaping with the slot still `Idle` means the engine forgets a real
    /// position and `is_flat()` reports true, which suppresses the caller's
    /// "ended with an open position" alert — the exact three-way failure of
    /// 2026-09-30. The `?` operator is therefore unsafe here on anything that
    /// touches the network; match and restore instead.
    pub async fn force_square_off(&mut self) -> CtResult<bool> {
        match std::mem::replace(&mut self.slot, Slot::Idle) {
            Slot::Idle => Ok(true),

            // No fill yet, so no position exists: cancelling the resting entry
            // is the whole job. A partial fill would already have been
            // converted to `Open` by `poll_pending_entry`.
            Slot::EntryPending(p) => {
                let cancelled = self.broker.cancel_order(&p.broker_order_id).await;
                // `cancel_order` returns no OrderResult, so the audit row is
                // built from what was requested plus the cancel's outcome.
                self.log_order_event(
                    None,
                    p.instrument_token,
                    &p.tradingsymbol,
                    BrokerOrderAction::Cancel,
                    match p.direction {
                        Direction::Long => OrderSide::Buy,
                        Direction::Short => OrderSide::Sell,
                    },
                    OrderType::Limit,
                    &core_types::OrderResult {
                        broker_order_id: p.broker_order_id.clone(),
                        status: if cancelled.is_ok() {
                            OrderStatus::Cancelled
                        } else {
                            OrderStatus::Other("cancel failed".into())
                        },
                        filled_quantity: 0,
                        average_price: None,
                        rejection_reason: cancelled.as_ref().err().map(|e| e.to_string()),
                        raw_response_json: String::new(),
                    },
                    false,
                );
                tracing::info!(
                    symbol = %p.tradingsymbol,
                    "session ended with an unfilled entry — cancelled, no position opened"
                );
                Ok(true)
            }

            // Never send a "closing" order without first confirming the
            // position is still there. Zerodha force-closes MIS around 15:20,
            // before this runs at the 15:30 session close, so by now the
            // engine's view may be stale — and a market order against a
            // position that no longer exists does not close anything, it
            // OPENS a fresh one in the opposite direction.
            Slot::Open(pos) => {
                let held = match self.broker.positions().await {
                    Ok(all) => all
                        .iter()
                        .find(|p| p.tradingsymbol == pos.tradingsymbol)
                        .map(|p| p.net_quantity)
                        .unwrap_or(0),
                    Err(e) => {
                        // Cannot verify, so cannot safely trade. Leaving the
                        // slot Open makes `is_flat()` false, which the caller
                        // turns into a loud alert.
                        tracing::error!(
                            error = %e,
                            symbol = %pos.tradingsymbol,
                            "cannot reach the broker to confirm the position — NOT sending a \
                             square-off order blind; check Kite manually"
                        );
                        self.slot = Slot::Open(pos);
                        return Ok(false);
                    }
                };
                if held == 0 {
                    // The broker already flattened it. Book the trade so the
                    // record is complete; the exit price is the last bar we
                    // saw, which is an estimate, not the broker's fill.
                    tracing::warn!(
                        symbol = %pos.tradingsymbol,
                        "broker already squared this off (MIS auto-close) — booking at the \
                         last seen price, which is an ESTIMATE of the real fill"
                    );
                    let (price, ts) = self
                        .last_seen
                        .get(&pos.instrument_token)
                        .map(|c| (c.close, c.ts))
                        .unwrap_or((pos.state.entry_price, chrono::Utc::now()));
                    let outcome = ExecutionOutcome {
                        final_result: core_types::OrderResult {
                            broker_order_id: String::new(),
                            status: OrderStatus::Complete,
                            filled_quantity: pos.qty,
                            average_price: Some(price),
                            rejection_reason: None,
                            raw_response_json: String::new(),
                        },
                        steps: Vec::new(),
                        used_fallback: false,
                    };
                    self.close_trade(&pos, &outcome, ts, ExitReason::EodSquareOff)?;
                    self.slot = Slot::Idle;
                    return Ok(true);
                }
                self.market_out(pos).await
            }

            // An exit is already working. Give it one last status check, and
            // if it has not fully filled, cancel and go to market for the
            // remainder rather than leaving it resting overnight.
            Slot::ExitPending(p) => {
                // NOT `?`. This is the last thing that runs before the process
                // exits, and the slot has already been replaced with `Idle` by
                // the `mem::replace` above — so propagating an error here
                // silently discards the position AND makes `is_flat()` report
                // true, which suppresses the caller's alert.
                //
                // That happened on 2026-09-30: the access token was
                // invalidated mid-session (a Kite Web login does that), this
                // call failed, and a real +Rs.2,657 trade was never booked
                // while the session reported itself flat. Three failures from
                // one `?`.
                let status = match self.broker.order_status(&p.broker_order_id).await {
                    Ok(s) => s,
                    Err(e) => {
                        tracing::error!(
                            error = %e,
                            symbol = %p.tradingsymbol,
                            trade_id = ?p.trade_id,
                            "cannot reach the broker to resolve the pending exit — leaving the \
                             trade OPEN in the record and reporting NOT flat; reconcile by hand"
                        );
                        self.pending_alerts.push(Alert::critical(format!(
                            "SESSION ENDED UNRESOLVED on {} — could not reach the broker to \
                             confirm the exit. Check Kite and reconcile trade {:?} by hand.",
                            p.tradingsymbol, p.trade_id
                        )));
                        // Restore the slot so the book is not silently
                        // forgotten and `is_flat()` tells the truth.
                        self.slot = Slot::ExitPending(p);
                        return Ok(false);
                    }
                };
                let filled = status.filled_quantity;
                if filled >= p.qty {
                    let ts = self
                        .last_seen
                        .get(&p.instrument_token)
                        .map(|c| c.ts)
                        .unwrap_or_else(chrono::Utc::now);
                    let outcome = ExecutionOutcome {
                        final_result: status,
                        steps: Vec::new(),
                        used_fallback: false,
                    };
                    let pos = OpenPos {
                        instrument_token: p.instrument_token,
                        trade_id: p.trade_id,
                        tradingsymbol: p.tradingsymbol.clone(),
                        qty: p.qty,
                        state: self.state_for_pending_exit(&p),
                    };
                    self.close_trade(&pos, &outcome, ts, p.reason)?;
                    return Ok(true);
                }
                let _ = self.broker.cancel_order(&p.broker_order_id).await;
                let pos = OpenPos {
                    instrument_token: p.instrument_token,
                    trade_id: p.trade_id,
                    tradingsymbol: p.tradingsymbol.clone(),
                    qty: p.qty - filled,
                    state: self.state_for_pending_exit(&p),
                };
                self.market_out(pos).await
            }
        }
    }

    /// Sends a plain MARKET order for the whole position and books the result.
    async fn market_out(&mut self, pos: OpenPos) -> CtResult<bool> {
        let stock = self.stocks.get(&pos.instrument_token);
        let side = match pos.state.direction {
            Direction::Long => OrderSide::Sell,
            Direction::Short => OrderSide::Buy,
        };
        let req = OrderRequest {
            tradingsymbol: pos.tradingsymbol.clone(),
            exchange: stock.map(|s| s.instrument.exchange).unwrap_or(core_types::Exchange::Nse),
            side,
            quantity: pos.qty,
            product: Product::Mis,
            order_type: OrderType::Market,
            price: None,
            trigger_price: None,
            tag: format!("eod-{}-{}", self.run_id, pos.trade_id),
        };
        // Matched rather than `?` for the same reason as force_square_off's
        // other branches: the caller has already taken the slot, so an error
        // escaping here loses the position and falsely reports flat.
        let outcome = match self.execution.place_plain(&req).await {
            Ok(o) => o,
            Err(e) => {
                tracing::error!(
                    error = %e,
                    symbol = %pos.tradingsymbol,
                    qty = pos.qty,
                    "market square-off could not be placed — position still OPEN"
                );
                self.pending_alerts.push(Alert::critical(format!(
                    "COULD NOT SQUARE OFF {} ({} held) — order failed: {e}. Check Kite NOW.",
                    pos.tradingsymbol, pos.qty
                )));
                self.slot = Slot::Open(pos);
                return Ok(false);
            }
        };
        self.log_execution_outcome(
            Some(pos.trade_id),
            pos.instrument_token,
            &pos.tradingsymbol,
            &req,
            &outcome,
        );
        let ts = self
            .last_seen
            .get(&pos.instrument_token)
            .map(|c| c.ts)
            .unwrap_or_else(chrono::Utc::now);
        if outcome.fully_filled() {
            self.close_trade(&pos, &outcome, ts, ExitReason::EodSquareOff)?;
            self.slot = Slot::Idle;
            Ok(true)
        } else {
            tracing::error!(
                symbol = %pos.tradingsymbol,
                qty = pos.qty,
                filled = outcome.final_result.filled_quantity,
                "SESSION ENDED WITH AN OPEN POSITION — market square-off did not fully fill"
            );
            self.pending_alerts.push(Alert::critical(format!(
                "SESSION ENDED HOLDING {} — square-off filled only {} of {}. Check Kite now.",
                pos.tradingsymbol, outcome.final_result.filled_quantity, pos.qty
            )));
            self.slot = Slot::Open(pos);
            Ok(false)
        }
    }

    /// Rebuilds the `PositionState` a pending exit came from. Only the fields
    /// `close_trade` reads (direction, entry price) have to be right.
    fn state_for_pending_exit(&self, p: &PendingExit) -> PositionState {
        PositionState {
            direction: p.direction,
            entry_price: p.entry_price,
            current_stop: p.entry_price,
            peak_favorable_price: p.entry_price,
            phase: PositionPhase::Armed,
        }
    }

    async fn manage_open(&mut self, pos: OpenPos, candle: &Candle, ist_time: chrono::NaiveTime) -> CtResult<()> {
        let forced = if ist_time >= self.eod_square_off {
            Some(ExitReason::EodSquareOff)
        } else {
            None
        };

        if let Some(reason) = forced {
            return self.begin_exit(pos, candle, reason).await;
        }

        // vwap/atr stay None: every tuned stock uses GiveBackFromPeak,
        // which (per `risk::exit_manager::on_update`) only ever reads
        // `last_price`/`ts` — vwap only matters for TrailPolicy::VwapCross,
        // atr only for AtrMultiple, neither of which this book uses.
        let market = MarketSnapshot {
            ts: candle.ts,
            last_price: candle.close,
            vwap: None,
            atr: None,
        };
        let (new_state, decision) = self.exit_manager.on_update(pos.state, &self.rebuild_plan(&pos), &market);

        let mut pos = pos;
        if new_state.current_stop != pos.state.current_stop {
            self.storage.record_stop_move(
                pos.trade_id,
                &core_types::StopMove {
                    ts: candle.ts,
                    new_stop_price: new_state.current_stop,
                },
            )?;
        }
        pos.state = new_state;

        if let Some(decision) = decision {
            self.begin_exit(pos, candle, decision.reason).await
        } else {
            self.slot = Slot::Open(pos);
            Ok(())
        }
    }

    /// `PositionState` alone (what's persisted across bars) doesn't carry
    /// the original `ExitPlan` (trail policy / mrm), so it's rebuilt from
    /// this stock's tuning each bar — cheap, and avoids widening `OpenPos`
    /// for a value that never actually changes after entry.
    fn rebuild_plan(&self, pos: &OpenPos) -> core_types::ExitPlan {
        let tuning = self
            .stocks
            .get(&pos.instrument_token)
            .map(stock_tuning)
            .expect("open position must belong to a tuned stock");
        core_types::ExitPlan {
            hard_stop_price: pos.state.current_stop,
            min_target_price: pos.state.entry_price
                * match pos.state.direction {
                    Direction::Long => 1.0 + self.risk_pct * tuning.mrm,
                    Direction::Short => 1.0 - self.risk_pct * tuning.mrm,
                },
            trail_policy: TrailPolicy::GiveBackFromPeak(tuning.trail),
        }
    }

    async fn begin_exit(&mut self, pos: OpenPos, candle: &Candle, reason: ExitReason) -> CtResult<()> {
        let stock = self.stocks.get(&pos.instrument_token);
        let tick_size = stock.map(|s| s.instrument.tick_size).unwrap_or(0.05);
        let exit_side = match pos.state.direction {
            Direction::Long => OrderSide::Sell,
            Direction::Short => OrderSide::Buy,
        };

        let base_req = OrderRequest {
            tradingsymbol: pos.tradingsymbol.clone(),
            exchange: stock.map(|s| s.instrument.exchange).unwrap_or(core_types::Exchange::Nse),
            side: exit_side,
            quantity: pos.qty,
            product: Product::Mis,
            order_type: OrderType::Limit,
            price: None,
            trigger_price: None,
            tag: format!("exit-{}-{}", self.run_id, pos.trade_id),
        };

        let (outcome, limit_price) = if reason == ExitReason::StopLoss {
            let trigger = round_to_tick(pos.state.current_stop, tick_size);
            let limit = round_to_tick(stop_limit_price(pos.state.direction, trigger), tick_size);
            let req = OrderRequest {
                order_type: OrderType::SlLimit,
                price: Some(limit),
                trigger_price: Some(trigger),
                ..base_req.clone()
            };
            let cfg = FallbackConfig::default();
            let outcome = self.execution.place_with_stop_fallback(&req, &cfg).await?;
            self.log_execution_outcome(Some(pos.trade_id), pos.instrument_token, &pos.tradingsymbol, &req, &outcome);
            (outcome, limit)
        } else {
            let limit = round_to_tick(candle.close, tick_size);
            let req = OrderRequest {
                price: Some(limit),
                ..base_req.clone()
            };
            let outcome = self.execution.place_plain(&req).await?;
            self.log_execution_outcome(Some(pos.trade_id), pos.instrument_token, &pos.tradingsymbol, &req, &outcome);
            (outcome, limit)
        };

        if outcome.fully_filled() {
            self.close_trade(&pos, &outcome, candle.ts, reason)?;
            self.slot = Slot::Idle;
        } else {
            self.slot = Slot::ExitPending(PendingExit {
                instrument_token: pos.instrument_token,
                trade_id: pos.trade_id,
                tradingsymbol: pos.tradingsymbol.clone(),
                qty: pos.qty,
                direction: pos.state.direction,
                reason,
                entry_price: pos.state.entry_price,
                broker_order_id: outcome.final_result.broker_order_id.clone(),
                limit_price,
                bars_pending: 0,
            });
        }
        Ok(())
    }

    /// Exits never time out and give up (unlike entries) — we are *always*
    /// obligated to actually close an open position. But they no longer wait
    /// passively either: each bar the resting limit is re-priced to the
    /// current price, and after `EXIT_CHASE_BARS` it is cancelled and taken at
    /// market.
    ///
    /// A stop-loss exit's own fallback already resolves it inside
    /// `ExecutionPolicy`'s timeout, so it reaches this function already
    /// Complete (see `begin_exit`); the chase is what covers trailing and EOD
    /// exits, which have no fallback of their own.
    async fn poll_pending_exit(&mut self, mut pending: PendingExit, candle: &Candle) -> CtResult<()> {
        let exit_side = match pending.direction {
            Direction::Long => OrderSide::Sell,
            Direction::Short => OrderSide::Buy,
        };
        let result = self.broker.order_status(&pending.broker_order_id).await?;
        self.log_order_event(
            Some(pending.trade_id),
            pending.instrument_token,
            &pending.tradingsymbol,
            BrokerOrderAction::StatusPoll,
            exit_side,
            OrderType::Limit,
            &result,
            false,
        );

        if result.status == OrderStatus::Complete {
            self.close_pending_exit(&pending, result.average_price.unwrap_or(candle.close), candle.ts);
            return Ok(());
        }

        pending.bars_pending += 1;
        let remaining = pending.qty.saturating_sub(result.filled_quantity);
        if remaining == 0 {
            // Filled in full while reporting something other than Complete —
            // treat the fill as authoritative rather than chasing a position
            // that is no longer held.
            self.close_pending_exit(&pending, result.average_price.unwrap_or(candle.close), candle.ts);
            return Ok(());
        }

        // An end-of-day exit escalates on the CLOCK, not after three bars.
        // From a 15:10 square-off the bar-count rule would land at 15:13,
        // past Zerodha's 15:12 MIS cutoff — at which point our order is
        // refused outright and the broker exits the position on its own
        // terms. See `session::eod_market_after`.
        let ist_time = candle.ts.with_timezone(&IST).time();
        let eod_overdue = pending.reason == ExitReason::EodSquareOff
            && ist_time >= crate::session::eod_market_after(self.eod_square_off);
        if eod_overdue || pending.bars_pending >= EXIT_CHASE_BARS {
            return self.escalate_exit_to_market(pending, remaining, candle).await;
        }

        let tick_size = self
            .stocks
            .get(&pending.instrument_token)
            .map(|s| s.instrument.tick_size)
            .unwrap_or(0.05);
        let new_limit = round_to_tick(candle.close, tick_size);
        // Only send a modify when the price actually moved — otherwise this
        // fires an API call a minute for an order already at the right price.
        if (new_limit - pending.limit_price).abs() >= tick_size / 2.0 {
            match self
                .broker
                .modify_order(&pending.broker_order_id, Some(new_limit), None, None)
                .await
            {
                Ok(modified) => {
                    self.log_order_event(
                        Some(pending.trade_id),
                        pending.instrument_token,
                        &pending.tradingsymbol,
                        BrokerOrderAction::Modify,
                        exit_side,
                        OrderType::Limit,
                        &modified,
                        false,
                    );
                    tracing::info!(
                        symbol = %pending.tradingsymbol,
                        from = pending.limit_price,
                        to = new_limit,
                        bars_pending = pending.bars_pending,
                        "exit limit re-priced to chase"
                    );
                    pending.limit_price = new_limit;
                    // A re-price can cross the spread and fill immediately;
                    // waiting a bar to notice would be a minute of holding a
                    // position that is already closed.
                    if modified.status == OrderStatus::Complete {
                        let px = modified.average_price.unwrap_or(new_limit);
                        self.close_pending_exit(&pending, px, candle.ts);
                        return Ok(());
                    }
                }
                // A failed re-price is not fatal: the original limit is still
                // resting, and the chase counter still advances, so this exit
                // reaches the market escalation on schedule regardless.
                Err(e) => tracing::warn!(
                    symbol = %pending.tradingsymbol,
                    error = %e,
                    "exit re-price failed — leaving the original limit in place"
                ),
            }
        }
        self.slot = Slot::ExitPending(pending);
        Ok(())
    }

    /// Cancels the chased limit and takes whatever is left at market.
    ///
    /// Reached when price has run away from the exit for `EXIT_CHASE_BARS`
    /// bars. At that point being out matters more than the last few paise:
    /// the alternative is holding an unmanaged position (neither stop nor
    /// trail runs while the slot is `ExitPending`) until the session backstop.
    async fn escalate_exit_to_market(
        &mut self,
        pending: PendingExit,
        remaining: u32,
        candle: &Candle,
    ) -> CtResult<()> {
        let exit_side = match pending.direction {
            Direction::Long => OrderSide::Sell,
            Direction::Short => OrderSide::Buy,
        };
        tracing::warn!(
            symbol = %pending.tradingsymbol,
            reason = ?pending.reason,
            bars_pending = pending.bars_pending,
            remaining,
            "exit limit did not fill while chasing — escalating to market"
        );
        self.pending_alerts.push(Alert::ExitEscalated {
            symbol: pending.tradingsymbol.clone(),
            bars: pending.bars_pending,
            remaining,
        });

        if let Err(e) = self.broker.cancel_order(&pending.broker_order_id).await {
            // Most often this means the order filled in the instant between
            // the poll and the cancel. Placing a market order on top of that
            // would open a fresh position in the opposite direction, so stay
            // pending and let the next bar's poll settle it.
            tracing::warn!(
                symbol = %pending.tradingsymbol,
                error = %e,
                "could not cancel the exit limit — not sending a market order blind"
            );
            self.slot = Slot::ExitPending(pending);
            return Ok(());
        }

        let stock = self.stocks.get(&pending.instrument_token);
        let req = OrderRequest {
            tradingsymbol: pending.tradingsymbol.clone(),
            exchange: stock.map(|s| s.instrument.exchange).unwrap_or(core_types::Exchange::Nse),
            side: exit_side,
            quantity: remaining,
            product: Product::Mis,
            order_type: OrderType::Market,
            price: None,
            trigger_price: None,
            tag: format!("exitmkt-{}-{}", self.run_id, pending.trade_id),
        };
        let outcome = self.execution.place_plain(&req).await?;
        self.log_execution_outcome(
            Some(pending.trade_id),
            pending.instrument_token,
            &pending.tradingsymbol,
            &req,
            &outcome,
        );

        if outcome.fully_filled() {
            let px = outcome.final_result.average_price.unwrap_or(candle.close);
            self.close_pending_exit(&pending, px, candle.ts);
        } else {
            tracing::error!(
                symbol = %pending.tradingsymbol,
                remaining,
                filled = outcome.final_result.filled_quantity,
                "market escalation did not fully fill — still holding"
            );
            self.pending_alerts.push(Alert::critical(format!(
                "{} exit STILL OPEN — market order filled only {} of {}. Check Kite now.",
                pending.tradingsymbol, outcome.final_result.filled_quantity, remaining
            )));
            self.slot = Slot::ExitPending(pending);
        }
        Ok(())
    }

    /// Books a pending exit that has resolved, whatever resolved it — the
    /// original limit, a chased re-price, or the market escalation.
    fn close_pending_exit(&mut self, pending: &PendingExit, exit_price: f64, exit_ts: core_types::time::DateTime) {
        let pnl_per_share = match pending.direction {
            Direction::Long => exit_price - pending.entry_price,
            Direction::Short => pending.entry_price - exit_price,
        };
        let pnl_abs = pnl_per_share * pending.qty as f64;
        let pnl_pct = if pending.entry_price > 0.0 { pnl_per_share / pending.entry_price * 100.0 } else { 0.0 };
        let risk_per_share = pending.entry_price * self.risk_pct;
        let r_multiple = if risk_per_share > 0.0 { pnl_per_share / risk_per_share } else { 0.0 };
        if let Err(e) = self.storage.close_trade(
            pending.trade_id,
            &TradeExit { exit_ts, exit_price, reason: pending.reason, pnl_abs, pnl_pct, r_multiple },
        ) {
            // The position IS closed at the broker; only the record failed.
            // Holding the slot open over a storage error would block every
            // later entry for a trade that no longer exists.
            tracing::error!(trade_id = ?pending.trade_id, error = %e, "failed to record exit — slot released anyway");
        }
        tracing::info!(
            symbol = %pending.tradingsymbol,
            direction = ?pending.direction,
            exit_price,
            reason = ?pending.reason,
            pnl_pct,
            r_multiple,
            "EXIT (resolved from pending)"
        );
        self.pending_alerts.push(Alert::Exit {
            symbol: pending.tradingsymbol.clone(),
            price: exit_price,
            reason: pending.reason,
            r_multiple,
            pnl_abs,
        });
        self.slot = Slot::Idle;
    }

    fn close_trade(&mut self, pos: &OpenPos, outcome: &ExecutionOutcome, exit_ts: core_types::time::DateTime, reason: ExitReason) -> CtResult<()> {
        let exit_price = outcome.final_result.average_price.unwrap_or(pos.state.current_stop);
        let pnl_per_share = match pos.state.direction {
            Direction::Long => exit_price - pos.state.entry_price,
            Direction::Short => pos.state.entry_price - exit_price,
        };
        let pnl_abs = pnl_per_share * pos.qty as f64;
        let pnl_pct = pnl_per_share / pos.state.entry_price * 100.0;
        let risk_per_share = pos.state.entry_price * self.risk_pct;
        let r_multiple = if risk_per_share > 0.0 { pnl_per_share / risk_per_share } else { 0.0 };

        self.storage.close_trade(
            pos.trade_id,
            &TradeExit {
                exit_ts,
                exit_price,
                reason,
                pnl_abs,
                pnl_pct,
                r_multiple,
            },
        )?;

        tracing::info!(
            symbol = %pos.tradingsymbol,
            direction = ?pos.state.direction,
            entry_price = pos.state.entry_price,
            exit_price,
            reason = ?reason,
            pnl_pct,
            r_multiple,
            used_fallback = outcome.used_fallback,
            "EXIT"
        );
        self.pending_alerts.push(Alert::Exit {
            symbol: pos.tradingsymbol.clone(),
            price: exit_price,
            reason,
            r_multiple,
            pnl_abs,
        });
        Ok(())
    }

    fn log_execution_outcome(
        &self,
        trade_id: Option<TradeId>,
        instrument_token: u64,
        tradingsymbol: &str,
        req: &OrderRequest,
        outcome: &ExecutionOutcome,
    ) {
        for timestamped in &outcome.steps {
            let (action, is_fallback, result) = match &timestamped.step {
                ExecutionStep::Placed(r) => (BrokerOrderAction::Place, false, Some(r)),
                ExecutionStep::Polled(r) => (BrokerOrderAction::StatusPoll, false, Some(r)),
                ExecutionStep::Cancelled { .. } => (BrokerOrderAction::Cancel, false, None),
                ExecutionStep::FallbackPlaced(r) => (BrokerOrderAction::Place, true, Some(r)),
            };
            let event = BrokerOrderEvent {
                run_id: self.run_id,
                trade_id,
                instrument_token,
                tradingsymbol: tradingsymbol.to_string(),
                kite_order_id: result.map(|r| r.broker_order_id.clone()),
                action,
                transaction_type: req.side,
                order_type: if is_fallback { OrderType::Market } else { req.order_type },
                requested_price: req.price,
                trigger_price: req.trigger_price,
                quantity: req.quantity,
                status: result.map(|r| r.status.clone()).unwrap_or(OrderStatus::Cancelled),
                filled_quantity: result.map(|r| r.filled_quantity).unwrap_or(0),
                average_price: result.and_then(|r| r.average_price),
                is_fallback_market: is_fallback,
                rejection_reason: result.and_then(|r| r.rejection_reason.clone()),
                raw_response_json: result.map(|r| r.raw_response_json.clone()).unwrap_or_default(),
                placed_at: timestamped.ts,
            };
            if let Err(e) = self.storage.record_broker_order(&event) {
                tracing::error!(error = %e, "failed to record broker_orders audit row — continuing, but this is a logging gap to investigate");
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn log_order_event(
        &self,
        trade_id: Option<TradeId>,
        instrument_token: u64,
        tradingsymbol: &str,
        action: BrokerOrderAction,
        side: OrderSide,
        order_type: OrderType,
        result: &core_types::OrderResult,
        is_fallback: bool,
    ) {
        let event = BrokerOrderEvent {
            run_id: self.run_id,
            trade_id,
            instrument_token,
            tradingsymbol: tradingsymbol.to_string(),
            kite_order_id: Some(result.broker_order_id.clone()),
            action,
            transaction_type: side,
            order_type,
            requested_price: None,
            trigger_price: None,
            quantity: result.filled_quantity,
            status: result.status.clone(),
            filled_quantity: result.filled_quantity,
            average_price: result.average_price,
            is_fallback_market: is_fallback,
            rejection_reason: result.rejection_reason.clone(),
            raw_response_json: result.raw_response_json.clone(),
            placed_at: chrono::Utc::now(),
        };
        if let Err(e) = self.storage.record_broker_order(&event) {
            tracing::error!(error = %e, "failed to record broker_orders audit row");
        }
    }
}

fn stock_tuning(stock: &StockRuntime) -> StockTuning {
    stock.tuning.clone()
}

/// Where a stop-loss order's limit leg sits relative to its trigger.
///
/// Beyond the trigger, in the direction price is travelling: below it when
/// selling into a fall, above it when buying back into a rise. Level with the
/// trigger — which is what this used to be — the order triggers and then rests
/// at a price the market has already left, so it cannot fill.
fn stop_limit_price(direction: Direction, trigger: f64) -> f64 {
    match direction {
        Direction::Long => trigger * (1.0 - STOP_LIMIT_OFFSET),
        Direction::Short => trigger * (1.0 + STOP_LIMIT_OFFSET),
    }
}

fn round_to_tick(price: f64, tick_size: f64) -> f64 {
    if tick_size <= 0.0 {
        return (price * 100.0).round() / 100.0;
    }
    (price / tick_size).round() * tick_size
}

/// Builds one `StrategyRunner` per stock, each with a single
/// `VwapTrendFollow` instance configured for that stock's own tuned
/// VWAP-slope lookback — deliberately *not* one shared runner with several
/// registered strategy variants, which would evaluate every lookback
/// variant against every stock's bars and risk a signal firing from the
/// wrong variant for that stock.
pub fn build_stock_runtimes(
    instruments: &[Instrument],
    tuning_by_symbol: &HashMap<String, StockTuning>,
) -> HashMap<u64, StockRuntime> {
    let mut out = HashMap::new();
    for inst in instruments {
        let Some(tuning) = tuning_by_symbol.get(&inst.tradingsymbol) else {
            continue;
        };
        let strategy = strategies::VwapTrendFollow::new(9, 21, 10, 1.2, false, true, 45)
            .with_vwap_slope(tuning.lookback, 0.0)
            .with_id(format!("vwap_trend_short_late_vwapslope_live_{}", inst.tradingsymbol));
        let engine = strategy_core::IndicatorEngine::new(chrono::NaiveTime::from_hms_opt(9, 30, 0).unwrap());
        let runner = StrategyRunner::new(engine, vec![Box::new(strategy)]);
        out.insert(
            inst.instrument_token,
            StockRuntime {
                instrument: inst.clone(),
                tuning: tuning.clone(),
                runner,
            },
        );
    }
    out
}

#[cfg(test)]
mod engine_tests {
    use super::*;
    use core_types::BrokerPosition;
    use std::collections::HashSet;

    fn pos(sym: &str, qty: i64) -> BrokerPosition {
        BrokerPosition {
            tradingsymbol: sym.to_string(),
            net_quantity: qty,
            average_price: 100.0,
        }
    }

    fn book() -> HashSet<&'static str> {
        ["HAL", "DMART", "APARINDS"].into_iter().collect()
    }

    #[test]
    fn a_flat_row_for_a_symbol_outside_the_book_is_not_a_position() {
        // The exact live failure: Kite returned a squared-off PPLPHARMA row
        // and the engine halted new entries for the rest of the session.
        let all = vec![pos("PPLPHARMA", 0)];
        assert!(reconcilable_positions(&all, &book()).is_empty());
    }

    #[test]
    fn a_flat_row_for_one_of_our_own_symbols_is_also_not_a_position() {
        // Squaring off our own trade leaves a net_quantity 0 row behind for
        // the rest of the day; that must not read as lingering exposure.
        let all = vec![pos("HAL", 0)];
        assert!(reconcilable_positions(&all, &book()).is_empty());
    }

    #[test]
    fn a_manual_trade_in_another_symbol_is_ignored_even_when_live() {
        // Same Kite account may be used by hand. A real open position we
        // never created is not ours to reconcile and must not halt the book.
        let all = vec![pos("RELIANCE", -50)];
        assert!(reconcilable_positions(&all, &book()).is_empty());
    }

    #[test]
    fn a_real_open_position_in_our_book_is_still_reported() {
        // The safety property that must survive the fix.
        let all = vec![pos("HAL", -25)];
        let got = reconcilable_positions(&all, &book());
        assert_eq!(got.len(), 1);
        assert_eq!(got[0].tradingsymbol, "HAL");
        assert_eq!(got[0].net_quantity, -25);
    }

    #[test]
    fn noise_is_dropped_while_a_genuine_position_alongside_it_survives() {
        let all = vec![
            pos("PPLPHARMA", 0),
            pos("RELIANCE", -50),
            pos("DMART", 0),
            pos("APARINDS", -12),
        ];
        let got = reconcilable_positions(&all, &book());
        assert_eq!(got.len(), 1, "only APARINDS is both ours and non-flat");
        assert_eq!(got[0].tradingsymbol, "APARINDS");
    }

    /// A long exits by SELLING into a fall, so its limit must sit BELOW the
    /// trigger to be marketable once triggered.
    #[test]
    fn a_longs_stop_limit_sits_below_its_trigger() {
        let limit = stop_limit_price(Direction::Long, 1000.0);
        assert!(limit < 1000.0, "a sell stop's limit must be below its trigger, got {limit}");
        assert!((limit - 997.0).abs() < 0.01, "expected a 0.3% offset, got {limit}");
    }

    /// A short exits by BUYING into a rise — the mirror. Getting this backwards
    /// would leave every short's stop unfillable while longs looked fine.
    #[test]
    fn a_shorts_stop_limit_sits_above_its_trigger() {
        let limit = stop_limit_price(Direction::Short, 1000.0);
        assert!(limit > 1000.0, "a buy stop's limit must be above its trigger, got {limit}");
        assert!((limit - 1003.0).abs() < 0.01, "expected a 0.3% offset, got {limit}");
    }

    /// The offset caps slippage on a stop-out. Left level with the trigger it
    /// caps nothing and the market fallback carries every stop; opened too wide
    /// it stops being a cap at all.
    #[test]
    fn the_offset_bounds_stop_slippage_to_well_under_one_r() {
        const RISK_PCT: f64 = 0.0075;
        let extra_r = STOP_LIMIT_OFFSET / RISK_PCT;
        assert!(extra_r > 0.0, "a zero offset means the limit can never fill");
        assert!(extra_r < 0.5, "offset is worth {extra_r:.2} R of extra loss — too wide to be a cap");
    }

    #[test]
    fn a_long_position_in_our_book_is_reported_too() {
        // Sign must not be assumed short — direction is checked by the caller.
        let all = vec![pos("DMART", 30)];
        assert_eq!(reconcilable_positions(&all, &book())[0].net_quantity, 30);
    }
}
