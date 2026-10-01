//! Opening Range Breakout — the 09:15-09:45 high-volatility window strategy.
//! Waits for the opening range to form (via the `OpeningRange` indicator,
//! configured through `strategy_core::IndicatorEngine`), then signals once
//! per session when price closes beyond the range with relative-volume
//! confirmation.

use core_types::{Direction, IndicatorSpec, SignalIntent, Strategy, StrategyContext, StrategyId, TimeWindow};
use std::collections::HashMap;

pub struct OpeningRangeBreakout {
    relative_volume_period: u32,
    min_relative_volume: f64,
    required: Vec<IndicatorSpec>,
    /// Keyed by instrument_token — a single strategy instance is shared
    /// across the whole curated universe (one `StrategyRunner`), so this
    /// cannot be a plain scalar without one instrument's signal wrongly
    /// suppressing every other instrument's for the rest of the session.
    signaled_today: HashMap<u64, bool>,
}

impl OpeningRangeBreakout {
    pub fn new(relative_volume_period: u32, min_relative_volume: f64) -> Self {
        Self {
            relative_volume_period,
            min_relative_volume,
            required: vec![
                IndicatorSpec::OpeningRange,
                IndicatorSpec::RelativeVolume(relative_volume_period),
            ],
            signaled_today: HashMap::new(),
        }
    }
}

impl Default for OpeningRangeBreakout {
    fn default() -> Self {
        Self::new(10, 1.5)
    }
}

impl Strategy for OpeningRangeBreakout {
    fn id(&self) -> StrategyId {
        "orb".to_string()
    }

    fn window(&self) -> TimeWindow {
        TimeWindow::Opening
    }

    fn required_indicators(&self) -> &[IndicatorSpec] {
        &self.required
    }

    fn reset_session(&mut self) {
        self.signaled_today.clear();
    }

    fn on_bar(&mut self, ctx: &StrategyContext) -> Option<SignalIntent> {
        if *self.signaled_today.get(&ctx.instrument_token).unwrap_or(&false) {
            return None;
        }
        let (or_high, or_low) = (ctx.opening_range_high?, ctx.opening_range_low?);
        let rel_vol = *ctx
            .indicators
            .get(&IndicatorSpec::RelativeVolume(self.relative_volume_period).key())?;
        if rel_vol < self.min_relative_volume {
            return None;
        }

        let direction = if ctx.candle.close > or_high {
            Direction::Long
        } else if ctx.candle.close < or_low {
            Direction::Short
        } else {
            return None;
        };

        self.signaled_today.insert(ctx.instrument_token, true);
        Some(SignalIntent {
            strategy_id: self.id(),
            window: self.window(),
            instrument_token: ctx.instrument_token,
            ts: ctx.candle.ts,
            direction,
            entry_price_hint: ctx.candle.close,
            rationale: format!(
                "close {:.2} broke opening range [{or_low:.2}, {or_high:.2}] with {rel_vol:.2}x relative volume"
                , ctx.candle.close
            ),
            params: serde_json::json!({
                "opening_range_high": or_high,
                "opening_range_low": or_low,
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

    fn ctx(close: f64, or_high: f64, or_low: f64, rel_vol: f64) -> StrategyContext {
        let mut indicators = HashMap::new();
        indicators.insert(IndicatorSpec::RelativeVolume(10).key(), rel_vol);
        StrategyContext {
            instrument_token: 1,
            window: TimeWindow::Opening,
            candle: Candle {
                ts: IST.with_ymd_and_hms(2026, 9, 22, 9, 35, 0).unwrap().with_timezone(&Utc),
                open: close,
                high: close,
                low: close,
                close,
                volume: 1000,
            },
            indicators,
            opening_range_high: Some(or_high),
            opening_range_low: Some(or_low),
        }
    }

    #[test]
    fn no_signal_before_range_is_formed() {
        let mut s = OpeningRangeBreakout::default();
        let mut c = ctx(105.0, 102.0, 98.0, 2.0);
        c.opening_range_high = None;
        c.opening_range_low = None;
        assert!(s.on_bar(&c).is_none());
    }

    #[test]
    fn no_signal_without_volume_confirmation() {
        let mut s = OpeningRangeBreakout::default(); // min_relative_volume = 1.5
        let c = ctx(105.0, 102.0, 98.0, 1.0);
        assert!(s.on_bar(&c).is_none());
    }

    #[test]
    fn breaks_above_high_with_volume_signals_long_once() {
        let mut s = OpeningRangeBreakout::default();
        let c = ctx(105.0, 102.0, 98.0, 2.0);
        let sig = s.on_bar(&c).unwrap();
        assert_eq!(sig.direction, Direction::Long);
        // Second bar, still breaking out — must not signal again this session.
        assert!(s.on_bar(&c).is_none());
    }

    #[test]
    fn breaks_below_low_with_volume_signals_short() {
        let mut s = OpeningRangeBreakout::default();
        let c = ctx(95.0, 102.0, 98.0, 2.0);
        let sig = s.on_bar(&c).unwrap();
        assert_eq!(sig.direction, Direction::Short);
    }

    #[test]
    fn reset_session_allows_signaling_again() {
        let mut s = OpeningRangeBreakout::default();
        let c = ctx(105.0, 102.0, 98.0, 2.0);
        assert!(s.on_bar(&c).is_some());
        assert!(s.on_bar(&c).is_none());
        s.reset_session();
        assert!(s.on_bar(&c).is_some());
    }
}
