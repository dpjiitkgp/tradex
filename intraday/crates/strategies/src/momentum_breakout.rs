//! Momentum/breakout watch — the 11:30-close window strategy. Flags a bar
//! whose own move is large relative to the stock's recent volatility (ATR),
//! confirmed by relative volume, edge-triggered so a sustained move signals
//! once rather than every bar.
//!
//! Note: the plan's original sketch described this as "rolling N-bar
//! high/low breaks" — the `indicators` crate (M5) doesn't have a rolling
//! high/low primitive, so this uses an ATR-normalized single-bar move
//! instead, which serves the same purpose (flag an unusual move) with the
//! primitives actually available. Revisit if that distinction matters.

use core_types::{Direction, IndicatorSpec, SignalIntent, Strategy, StrategyContext, StrategyId, TimeWindow};
use std::collections::HashMap;

pub struct MomentumBreakout {
    atr_period: u32,
    relative_volume_period: u32,
    min_atr_multiple: f64,
    min_relative_volume: f64,
    required: Vec<IndicatorSpec>,
    /// Keyed by instrument_token — see the same note in `orb.rs`.
    prev_condition_met: HashMap<u64, bool>,
}

impl MomentumBreakout {
    pub fn new(atr_period: u32, relative_volume_period: u32, min_atr_multiple: f64, min_relative_volume: f64) -> Self {
        Self {
            atr_period,
            relative_volume_period,
            min_atr_multiple,
            min_relative_volume,
            required: vec![
                IndicatorSpec::Atr(atr_period),
                IndicatorSpec::RelativeVolume(relative_volume_period),
            ],
            prev_condition_met: HashMap::new(),
        }
    }
}

impl Default for MomentumBreakout {
    fn default() -> Self {
        Self::new(14, 10, 1.5, 2.0)
    }
}

impl Strategy for MomentumBreakout {
    fn id(&self) -> StrategyId {
        "momentum_breakout".to_string()
    }

    fn window(&self) -> TimeWindow {
        TimeWindow::Afternoon
    }

    fn required_indicators(&self) -> &[IndicatorSpec] {
        &self.required
    }

    fn reset_session(&mut self) {
        self.prev_condition_met.clear();
    }

    fn on_bar(&mut self, ctx: &StrategyContext) -> Option<SignalIntent> {
        let atr = *ctx.indicators.get(&IndicatorSpec::Atr(self.atr_period).key())?;
        let rel_vol = *ctx
            .indicators
            .get(&IndicatorSpec::RelativeVolume(self.relative_volume_period).key())?;

        let bar_move = ctx.candle.close - ctx.candle.open;
        let normalized_move = if atr > 0.0 { bar_move.abs() / atr } else { 0.0 };
        let condition_met = normalized_move >= self.min_atr_multiple && rel_vol >= self.min_relative_volume;

        let was_met = self
            .prev_condition_met
            .insert(ctx.instrument_token, condition_met)
            .unwrap_or(false);
        if !condition_met || was_met {
            return None; // either no unusual move, or it's a continuation of one we already signaled
        }

        let direction = if bar_move > 0.0 { Direction::Long } else { Direction::Short };
        Some(SignalIntent {
            strategy_id: self.id(),
            window: self.window(),
            instrument_token: ctx.instrument_token,
            ts: ctx.candle.ts,
            direction,
            entry_price_hint: ctx.candle.close,
            rationale: format!(
                "bar move {normalized_move:.2}x ATR({}) with {rel_vol:.2}x relative volume",
                self.atr_period
            ),
            params: serde_json::json!({
                "atr": atr,
                "normalized_move": normalized_move,
                "relative_volume": rel_vol,
            }),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};
    use core_types::time::IST;
    use core_types::Candle;
    use std::collections::HashMap;

    fn ctx(open: f64, close: f64, atr: f64, rel_vol: f64) -> StrategyContext {
        let mut indicators = HashMap::new();
        indicators.insert(IndicatorSpec::Atr(14).key(), atr);
        indicators.insert(IndicatorSpec::RelativeVolume(10).key(), rel_vol);
        StrategyContext {
            instrument_token: 1,
            window: TimeWindow::Afternoon,
            candle: Candle {
                ts: IST.with_ymd_and_hms(2026, 9, 22, 13, 0, 0).unwrap().with_timezone(&Utc),
                open,
                high: open.max(close),
                low: open.min(close),
                close,
                volume: 1000,
            },
            indicators,
            opening_range_high: None,
            opening_range_low: None,
        }
    }

    #[test]
    fn large_move_with_volume_signals_long() {
        let mut s = MomentumBreakout::default(); // min_atr_multiple=1.5, min_rel_vol=2.0
        let sig = s.on_bar(&ctx(100.0, 102.0, 1.0, 3.0)).unwrap(); // move=2.0, atr=1.0 -> 2.0x
        assert_eq!(sig.direction, Direction::Long);
    }

    #[test]
    fn downward_move_signals_short() {
        let mut s = MomentumBreakout::default();
        let sig = s.on_bar(&ctx(100.0, 97.0, 1.0, 3.0)).unwrap();
        assert_eq!(sig.direction, Direction::Short);
    }

    #[test]
    fn small_move_does_not_signal() {
        let mut s = MomentumBreakout::default();
        assert!(s.on_bar(&ctx(100.0, 100.5, 1.0, 3.0)).is_none()); // 0.5x ATR
    }

    #[test]
    fn insufficient_volume_does_not_signal() {
        let mut s = MomentumBreakout::default();
        assert!(s.on_bar(&ctx(100.0, 102.0, 1.0, 1.0)).is_none());
    }

    #[test]
    fn sustained_move_signals_only_once_edge_triggered() {
        let mut s = MomentumBreakout::default();
        assert!(s.on_bar(&ctx(100.0, 102.0, 1.0, 3.0)).is_some());
        assert!(s.on_bar(&ctx(102.0, 104.0, 1.0, 3.0)).is_none()); // still elevated, no new edge
    }

    #[test]
    fn condition_dropping_then_recurring_signals_again() {
        let mut s = MomentumBreakout::default();
        assert!(s.on_bar(&ctx(100.0, 102.0, 1.0, 3.0)).is_some());
        assert!(s.on_bar(&ctx(102.0, 102.1, 1.0, 3.0)).is_none()); // condition drops
        assert!(s.on_bar(&ctx(102.1, 104.5, 1.0, 3.0)).is_some()); // new edge
    }
}
