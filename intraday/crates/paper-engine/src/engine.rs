use crate::config::EngineConfig;
use crate::pnl;
use crate::position::OpenPosition;
use core_types::time::IST;
use core_types::{
    Candle, EntryContext, ExitManager, ExitReason, Instrument, MarketSnapshot, NewRun, NewTrade,
    PositionPhase, PositionState, Result, RunId, RunMode, SignalRecord, StopMove, Storage,
    TradeExit,
};
use indicators::{Atr, Vwap};
use std::collections::HashMap;
use strategy_core::StrategyRunner;

/// The shared orchestrator: drives `StrategyRunner` for entries and
/// `ExitManager` for exits against a stream of bars, simulating fills and
/// persisting everything through `Storage`. Identical code path for
/// backtest replay (`backtest`, M8) and the live daemon (later) — only the
/// bar source differs.
pub struct PaperEngine {
    runner: StrategyRunner,
    exit_manager: Box<dyn ExitManager>,
    storage: Box<dyn Storage>,
    config: EngineConfig,
    mode: RunMode,
    run_id: RunId,
    open_positions: HashMap<u64, OpenPosition>,
    exit_indicators: HashMap<u64, (Vwap, Atr)>,
    daily_realized_pnl: f64,
    /// Last bar seen per instrument, so a position can still be squared off
    /// at a real price when no bar arrives at or after `eod_square_off`.
    /// See `reset_session`.
    last_seen: HashMap<u64, (f64, core_types::DateTime)>,
}

impl PaperEngine {
    pub fn new(
        runner: StrategyRunner,
        exit_manager: Box<dyn ExitManager>,
        storage: Box<dyn Storage>,
        config: EngineConfig,
        new_run: NewRun,
    ) -> Result<Self> {
        let mode = new_run.mode;
        let run_id = storage.start_run(&new_run)?;
        Ok(Self {
            runner,
            exit_manager,
            storage,
            config,
            mode,
            run_id,
            open_positions: HashMap::new(),
            exit_indicators: HashMap::new(),
            daily_realized_pnl: 0.0,
            last_seen: HashMap::new(),
        })
    }

    pub fn run_id(&self) -> RunId {
        self.run_id
    }

    pub fn open_position_count(&self) -> usize {
        self.open_positions.len()
    }

    pub fn daily_realized_pnl(&self) -> f64 {
        self.daily_realized_pnl
    }

    pub fn end_run(&self) -> Result<()> {
        self.storage.end_run(self.run_id)
    }

    /// Ends the trading session: squares off anything still open, then clears
    /// per-day state.
    ///
    /// The EOD square-off in `manage_open_position` only fires when a bar
    /// arrives at or after `eod_square_off`. On a thin or halted day an
    /// instrument's bars can stop earlier than that, so the position was
    /// never closed and carried into the next session — it then got managed
    /// against the *following* day's bars, picking up an overnight gap the
    /// live engine would never take. That accounted for 9,685 trades (1.23%)
    /// in an intraday book that should have none.
    ///
    /// So the day boundary, not the clock, is the backstop: whatever is still
    /// open closes at that instrument's last seen price. Returns the number
    /// squared off this way, which callers may log — a non-zero count means
    /// those instruments went quiet before the cutoff.
    pub fn reset_session(&mut self) -> Result<usize> {
        let stragglers: Vec<u64> = self.open_positions.keys().copied().collect();
        let mut closed = 0;
        for token in stragglers {
            match self.last_seen.get(&token).copied() {
                Some((price, ts)) => {
                    self.close_position(token, price, ts, ExitReason::EodSquareOff)?;
                    closed += 1;
                }
                None => {
                    // Cannot happen via on_bar (which records every bar before
                    // opening anything), but dropping the position silently
                    // would corrupt the position count for the next session.
                    tracing::error!(
                        token,
                        "open position with no last-seen bar — dropping it"
                    );
                    self.open_positions.remove(&token);
                }
            }
        }
        if closed > 0 {
            tracing::debug!(closed, "squared off at the session boundary (no bar at cutoff)");
        }
        self.runner.reset_session();
        self.daily_realized_pnl = 0.0;
        self.last_seen.clear();
        Ok(closed)
    }

    fn daily_loss_cap_hit(&self) -> bool {
        match self.config.daily_loss_cap() {
            Some(cap) => self.daily_realized_pnl <= -cap,
            None => false,
        }
    }

    /// Feeds one bar for one instrument through the engine: manages an
    /// existing position if one is open, otherwise considers a new entry
    /// (subject to the position-count cap and the daily-loss circuit
    /// breaker).
    pub fn on_bar(&mut self, instrument: &Instrument, candle: &Candle) -> Result<()> {
        let token = instrument.instrument_token;
        self.last_seen.insert(token, (candle.close, candle.ts));
        let (vwap, atr) = {
            let exit_atr_period = self.config.exit_atr_period as usize;
            let (v, a) = self
                .exit_indicators
                .entry(token)
                .or_insert_with(|| (Vwap::new(), Atr::new(exit_atr_period)));
            (v.update(candle), a.update(candle))
        };

        if self.open_positions.contains_key(&token) {
            self.manage_open_position(token, candle, vwap, atr)?;
        } else if !self.daily_loss_cap_hit()
            && self.open_positions.len() < self.config.max_concurrent_positions
        {
            self.consider_entry(instrument, candle)?;
        }
        Ok(())
    }

    /// Warms indicators with an already-elapsed bar without trading on it.
    ///
    /// VWAP is session-cumulative, so an engine started at 09:42 computes it
    /// over 09:42-onward rather than from the 09:15 open — and the strategy
    /// gates on which side of VWAP price sits, which is enough to flip the
    /// test. A shadow book that seeds differently from the live book is not
    /// the counterfactual it claims to be, so both replay the same history.
    ///
    /// Signals from replayed bars are discarded deliberately: the bar is in
    /// the past, and acting on it would open a position at a price that no
    /// longer exists. Positions are never touched here.
    pub fn seed_bar(&mut self, instrument: &Instrument, candle: &Candle) {
        let token = instrument.instrument_token;
        self.last_seen.insert(token, (candle.close, candle.ts));
        let exit_atr_period = self.config.exit_atr_period as usize;
        let (v, a) = self
            .exit_indicators
            .entry(token)
            .or_insert_with(|| (Vwap::new(), Atr::new(exit_atr_period)));
        v.update(candle);
        a.update(candle);
        let _ = self.runner.on_bar(token, candle);
    }

    fn manage_open_position(
        &mut self,
        token: u64,
        candle: &Candle,
        vwap: Option<f64>,
        atr: Option<f64>,
    ) -> Result<()> {
        let ist_time = candle.ts.with_timezone(&IST).time();

        let forced = {
            let pos = self.open_positions.get(&token).expect("checked by caller");
            if ist_time >= self.config.eod_square_off {
                Some(ExitReason::EodSquareOff)
            } else if let Some(max_hold) = self.config.max_hold_minutes {
                let held_minutes = (candle.ts - pos.entry_ts).num_minutes();
                (held_minutes >= max_hold).then_some(ExitReason::MaxHold)
            } else {
                None
            }
        };
        if let Some(reason) = forced {
            return self.close_position(token, candle.close, candle.ts, reason);
        }

        let Self {
            exit_manager,
            open_positions,
            storage,
            ..
        } = self;
        let pos = open_positions.get_mut(&token).expect("checked by caller");
        let market = MarketSnapshot {
            ts: candle.ts,
            last_price: candle.close,
            vwap,
            atr,
        };
        let prev_stop = pos.state.current_stop;
        let (new_state, decision) = exit_manager.on_update(pos.state, &pos.plan, &market);
        pos.state = new_state;

        if new_state.current_stop != prev_stop {
            storage.record_stop_move(
                pos.trade_id,
                &StopMove {
                    ts: candle.ts,
                    new_stop_price: new_state.current_stop,
                },
            )?;
        }

        if let Some(decision) = decision {
            self.close_position(token, decision.exit_price, decision.ts, decision.reason)?;
        }
        Ok(())
    }

    fn close_position(
        &mut self,
        token: u64,
        exit_price: f64,
        exit_ts: core_types::DateTime,
        reason: ExitReason,
    ) -> Result<()> {
        let pos = self
            .open_positions
            .remove(&token)
            .expect("close_position called without an open position");
        let pnl = pnl::compute(
            pos.state.direction,
            pos.state.entry_price,
            exit_price,
            pos.qty,
            self.config.risk_pct,
        );
        self.storage.close_trade(
            pos.trade_id,
            &TradeExit {
                exit_ts,
                exit_price,
                reason,
                pnl_abs: pnl.pnl_abs,
                pnl_pct: pnl.pnl_pct,
                r_multiple: pnl.r_multiple,
            },
        )?;
        self.daily_realized_pnl += pnl.pnl_abs;
        if self.mode == RunMode::Live {
            tracing::info!(
                symbol = %pos.tradingsymbol,
                direction = ?pos.state.direction,
                entry_price = pos.state.entry_price,
                exit_price,
                reason = ?reason,
                pnl_pct = pnl.pnl_pct,
                r_multiple = pnl.r_multiple,
                "EXIT"
            );
        }
        Ok(())
    }

    fn consider_entry(&mut self, instrument: &Instrument, candle: &Candle) -> Result<()> {
        let token = instrument.instrument_token;
        let signals = self.runner.on_bar(token, candle);
        let Some(signal) = signals.into_iter().next() else {
            return Ok(());
        };

        if self.mode == RunMode::Live {
            tracing::info!(
                symbol = %instrument.tradingsymbol,
                strategy = %signal.strategy_id,
                direction = ?signal.direction,
                entry_price_hint = signal.entry_price_hint,
                rationale = %signal.rationale,
                "SIGNAL"
            );
        }

        let signal_id = self.storage.record_signal(&SignalRecord {
            run_id: self.run_id,
            ts: signal.ts,
            instrument_token: token,
            strategy_id: signal.strategy_id.clone(),
            window: signal.window,
            direction: signal.direction,
            entry_price_hint: signal.entry_price_hint,
            rationale: signal.rationale.clone(),
            params_json: signal.params.to_string(),
        })?;

        let entry_price = signal.entry_price_hint;
        let lot_size = instrument.lot_size.max(1);
        let raw_qty = (self.config.capital_per_trade / entry_price) as u32;
        let qty = (raw_qty / lot_size) * lot_size;
        if qty == 0 {
            tracing::debug!(
                symbol = %instrument.tradingsymbol,
                "skipping signal — capital_per_trade too small for one lot at this price"
            );
            return Ok(());
        }

        let entry = EntryContext {
            direction: signal.direction,
            entry_price,
            entry_ts: signal.ts,
            risk_pct: self.config.risk_pct,
            min_reward_multiple: self.config.min_reward_multiple,
            trail_policy: self.config.trail_policy,
        };
        let plan = self.exit_manager.on_entry(&entry);
        let capital_allocated = entry_price * qty as f64;

        if self.mode == RunMode::Live {
            tracing::info!(
                symbol = %instrument.tradingsymbol,
                direction = ?signal.direction,
                qty,
                entry_price,
                hard_stop = plan.hard_stop_price,
                min_target = plan.min_target_price,
                capital_allocated,
                "ENTRY"
            );
        }

        let trade_id = self.storage.open_trade(&NewTrade {
            run_id: self.run_id,
            signal_id: Some(signal_id),
            instrument_token: token,
            strategy_id: signal.strategy_id.clone(),
            window: signal.window,
            mode: self.mode,
            direction: signal.direction,
            qty,
            capital_allocated,
            entry_ts: signal.ts,
            entry_price,
            hard_stop_price: plan.hard_stop_price,
            min_target_price: plan.min_target_price,
            trail_policy: plan.trail_policy,
        })?;

        self.open_positions.insert(
            token,
            OpenPosition {
                trade_id,
                tradingsymbol: instrument.tradingsymbol.clone(),
                entry_ts: signal.ts,
                qty,
                state: PositionState {
                    direction: signal.direction,
                    entry_price,
                    current_stop: plan.hard_stop_price,
                    peak_favorable_price: entry_price,
                    phase: PositionPhase::Armed,
                },
                plan,
            },
        );
        Ok(())
    }
}
