use crate::indicator_engine::IndicatorEngine;
use crate::window::window_for_time;
use core_types::time::IST;
use core_types::{Candle, IndicatorSpec, SignalIntent, Strategy, TimeWindow};

/// A strategy declaring `TimeWindow::FullDay` is active on every bar,
/// regardless of which of the three time-of-day windows it falls in —
/// everything else must match exactly.
fn strategy_active_in(strategy_window: TimeWindow, current_window: TimeWindow) -> bool {
    strategy_window == current_window || strategy_window == TimeWindow::FullDay
}

/// Owns the registered strategies and the shared `IndicatorEngine`, and
/// routes each bar to whichever strategy's window covers the bar's IST
/// time — the "window registry" from the plan.
pub struct StrategyRunner {
    engine: IndicatorEngine,
    strategies: Vec<Box<dyn Strategy>>,
}

impl StrategyRunner {
    pub fn new(engine: IndicatorEngine, strategies: Vec<Box<dyn Strategy>>) -> Self {
        Self { engine, strategies }
    }

    pub fn reset_session(&mut self) {
        self.engine.reset_session();
        for s in &mut self.strategies {
            s.reset_session();
        }
    }

    /// Feeds one bar for one instrument through the indicator engine and
    /// every strategy whose window covers this bar's time. Usually returns
    /// 0 or 1 signals, but nothing structurally prevents overlapping
    /// windows producing more than one.
    pub fn on_bar(&mut self, instrument_token: u64, candle: &Candle) -> Vec<SignalIntent> {
        let ist_time = candle.ts.with_timezone(&IST).time();
        let Some(window) = window_for_time(ist_time) else {
            return Vec::new();
        };

        let mut specs: Vec<IndicatorSpec> = Vec::new();
        let mut any_active = false;
        for s in &self.strategies {
            if strategy_active_in(s.window(), window) {
                any_active = true;
                specs.extend_from_slice(s.required_indicators());
            }
        }
        if !any_active {
            return Vec::new();
        }

        self.engine.ensure_registered(instrument_token, &specs);
        let ctx = self.engine.on_bar(instrument_token, window, candle);

        self.strategies
            .iter_mut()
            .filter(|s| strategy_active_in(s.window(), window))
            .filter_map(|s| s.on_bar(&ctx))
            .collect()
    }
}
