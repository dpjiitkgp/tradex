//! EMA pullback continuation — the 11:30-close window strategy's higher-
//! win-rate alternative to `momentum_breakout`. Rather than timing a fresh
//! breakout (a bet on a *new* move starting, inherently lower-probability),
//! this only trades an *already-established* trend (fast EMA sustained on
//! one side of slow EMA), waiting for price to pull back to the fast EMA
//! and then resume — the classic "buy the dip in an uptrend" entry, which
//! typically wins more often than breakout entries at the cost of a
//! smaller average winner.

use core_types::{Direction, IndicatorSpec, SignalIntent, Strategy, StrategyContext, StrategyId, TimeWindow};
use std::collections::HashMap;

pub struct TrendPullback {
    fast_period: u32,
    slow_period: u32,
    relative_volume_period: u32,
    min_relative_volume: f64,
    required: Vec<IndicatorSpec>,
    /// Keyed by instrument_token — see the same note in `orb.rs`. Tracks
    /// whether price is currently touching/through the fast EMA against
    /// the established trend, so a signal fires at most once per pullback.
    in_pullback: HashMap<u64, bool>,
    signaled_this_pullback: HashMap<u64, bool>,
}

impl TrendPullback {
    pub fn new(fast_period: u32, slow_period: u32, relative_volume_period: u32, min_relative_volume: f64) -> Self {
        Self {
            fast_period,
            slow_period,
            relative_volume_period,
            min_relative_volume,
            required: vec![
                IndicatorSpec::Ema(fast_period),
                IndicatorSpec::Ema(slow_period),
                IndicatorSpec::RelativeVolume(relative_volume_period),
            ],
            in_pullback: HashMap::new(),
            signaled_this_pullback: HashMap::new(),
        }
    }
}

impl Default for TrendPullback {
    fn default() -> Self {
        Self::new(9, 21, 10, 1.2)
    }
}

impl Strategy for TrendPullback {
    fn id(&self) -> StrategyId {
        "trend_pullback".to_string()
    }

    fn window(&self) -> TimeWindow {
        TimeWindow::Afternoon
    }

    fn required_indicators(&self) -> &[IndicatorSpec] {
        &self.required
    }

    fn reset_session(&mut self) {
        self.in_pullback.clear();
        self.signaled_this_pullback.clear();
    }

    fn on_bar(&mut self, ctx: &StrategyContext) -> Option<SignalIntent> {
        let fast = *ctx.indicators.get(&IndicatorSpec::Ema(self.fast_period).key())?;
        let slow = *ctx.indicators.get(&IndicatorSpec::Ema(self.slow_period).key())?;
        let rel_vol = *ctx
            .indicators
            .get(&IndicatorSpec::RelativeVolume(self.relative_volume_period).key())?;

        let trend = if fast > slow {
            Direction::Long
        } else if fast < slow {
            Direction::Short
        } else {
            return None; // exactly flat — no established trend to trade with
        };

        let touching = match trend {
            Direction::Long => ctx.candle.low <= fast,
            Direction::Short => ctx.candle.high >= fast,
        };

        if !touching {
            self.in_pullback.insert(ctx.instrument_token, false);
            self.signaled_this_pullback.insert(ctx.instrument_token, false);
            return None;
        }
        self.in_pullback.insert(ctx.instrument_token, true);
        if *self.signaled_this_pullback.get(&ctx.instrument_token).unwrap_or(&false) {
            return None; // already fired for this pullback
        }
        if rel_vol < self.min_relative_volume {
            return None;
        }

        // Confirmation: resumed in the trend direction, closing back on
        // the correct side of the fast EMA — a continuation bar, not just
        // a touch.
        let confirmed = match trend {
            Direction::Long => ctx.candle.close > ctx.candle.open && ctx.candle.close > fast,
            Direction::Short => ctx.candle.close < ctx.candle.open && ctx.candle.close < fast,
        };
        if !confirmed {
            return None; // still testing the EMA, wait for a clean resumption bar
        }

        self.signaled_this_pullback.insert(ctx.instrument_token, true);
        Some(SignalIntent {
            strategy_id: self.id(),
            window: self.window(),
            instrument_token: ctx.instrument_token,
            ts: ctx.candle.ts,
            direction: trend,
            entry_price_hint: ctx.candle.close,
            rationale: format!(
                "pullback to EMA{} ({:.2}) resumed {} with {rel_vol:.2}x relative volume, EMA{}/{} trend intact",
                self.fast_period,
                fast,
                if trend == Direction::Long { "up" } else { "down" },
                self.fast_period,
                self.slow_period,
            ),
            params: serde_json::json!({
                "fast_ema": fast,
                "slow_ema": slow,
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

    fn ctx(open: f64, high: f64, low: f64, close: f64, fast: f64, slow: f64, rel_vol: f64, token: u64) -> StrategyContext {
        let mut indicators = HashMap::new();
        indicators.insert(IndicatorSpec::Ema(9).key(), fast);
        indicators.insert(IndicatorSpec::Ema(21).key(), slow);
        indicators.insert(IndicatorSpec::RelativeVolume(10).key(), rel_vol);
        StrategyContext {
            instrument_token: token,
            window: TimeWindow::Afternoon,
            candle: Candle {
                ts: IST.with_ymd_and_hms(2026, 9, 24, 13, 0, 0).unwrap().with_timezone(&Utc),
                open,
                high,
                low,
                close,
                volume: 10_000,
            },
            indicators,
            opening_range_high: None,
            opening_range_low: None,
        }
    }

    #[test]
    fn no_trend_no_signal() {
        let mut s = TrendPullback::default();
        assert!(s.on_bar(&ctx(100.0, 100.5, 99.5, 100.2, 100.0, 100.0, 2.0, 1)).is_none());
    }

    #[test]
    fn uptrend_not_touching_fast_ema_no_signal() {
        let mut s = TrendPullback::default();
        // fast(105) > slow(100) -> uptrend; low(104) stays above fast(105)? no, 104<105 actually touches.
        // Use a clearly-above-fast low to avoid touching.
        assert!(s.on_bar(&ctx(107.0, 108.0, 106.0, 107.5, 105.0, 100.0, 2.0, 1)).is_none());
    }

    #[test]
    fn uptrend_pullback_confirmed_fires_long() {
        let mut s = TrendPullback::default();
        // fast=105, slow=100 -> uptrend. low(104.5) touches through fast. close(105.5) > open(104.6) and > fast -> confirmed.
        let sig = s.on_bar(&ctx(104.6, 105.6, 104.5, 105.5, 105.0, 100.0, 2.0, 1)).unwrap();
        assert_eq!(sig.direction, Direction::Long);
    }

    #[test]
    fn uptrend_pullback_still_below_fast_does_not_fire_yet() {
        let mut s = TrendPullback::default();
        // Touches fast but closes below it -> not confirmed yet.
        assert!(s.on_bar(&ctx(104.0, 105.2, 103.8, 104.5, 105.0, 100.0, 2.0, 1)).is_none());
    }

    #[test]
    fn weak_volume_on_confirmation_bar_is_rejected() {
        let mut s = TrendPullback::default();
        assert!(s.on_bar(&ctx(104.6, 105.6, 104.5, 105.5, 105.0, 100.0, 0.5, 1)).is_none());
    }

    #[test]
    fn only_fires_once_per_pullback_episode() {
        let mut s = TrendPullback::default();
        assert!(s.on_bar(&ctx(104.6, 105.6, 104.5, 105.5, 105.0, 100.0, 2.0, 1)).is_some());
        // Still touching (another dip within the same episode) -> no second signal.
        assert!(s.on_bar(&ctx(104.7, 105.7, 104.6, 105.6, 105.0, 100.0, 2.0, 1)).is_none());
    }

    #[test]
    fn downtrend_pullback_confirmed_fires_short() {
        let mut s = TrendPullback::default();
        // fast=95, slow=100 -> downtrend. high(95.5) touches through fast. close(94.5) < open(95.4) and < fast -> confirmed.
        let sig = s.on_bar(&ctx(95.4, 95.5, 94.4, 94.5, 95.0, 100.0, 2.0, 1)).unwrap();
        assert_eq!(sig.direction, Direction::Short);
    }
}
