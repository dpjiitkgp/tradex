//! Combined VWAP-crossover + moving-average-alignment confluence — a
//! full-session strategy (`TimeWindow::FullDay`, like `ma_alignment` and
//! `strong_rally`) that only signals when *both* independently-built
//! trend strategies agree on the same bar: `vwap_trend`'s EMA fast/slow
//! crossover (confirmed by price being on the matching side of VWAP and by
//! relative volume) must fire in the same direction that `ma_alignment`'s
//! N-period SMA stack is *currently* aligned in.
//!
//! The crossover supplies the entry *timing* (edge-triggered, same as
//! `vwap_trend`); the SMA stack supplies a broader multi-timeframe trend
//! filter on top of it (a state check, not its own edge-trigger — it just
//! has to currently agree, it doesn't need to have *just* aligned). Both
//! `vwap_trend` and `ma_alignment` individually survived the 5-year
//! validation; this tests whether requiring both together screens out
//! more false starts than either alone.

use core_types::{Direction, IndicatorSpec, SignalIntent, Strategy, StrategyContext, StrategyId, TimeWindow};
use std::collections::HashMap;

pub struct TrendConfluence {
    fast_period: u32,
    slow_period: u32,
    relative_volume_period: u32,
    min_relative_volume: f64,
    /// Shortest to longest, e.g. [15, 30, 45, 60] — same convention as `MaAlignment`.
    ma_periods: Vec<u32>,
    required: Vec<IndicatorSpec>,
    /// Keyed by instrument_token — see the same note in `orb.rs`.
    prev_fast_above_slow: HashMap<u64, bool>,
}

impl TrendConfluence {
    pub fn new(
        fast_period: u32,
        slow_period: u32,
        relative_volume_period: u32,
        min_relative_volume: f64,
        ma_periods: Vec<u32>,
    ) -> Self {
        assert!(ma_periods.len() >= 2, "TrendConfluence needs at least 2 MA periods to compare");
        let mut required = vec![
            IndicatorSpec::Vwap,
            IndicatorSpec::Ema(fast_period),
            IndicatorSpec::Ema(slow_period),
            IndicatorSpec::RelativeVolume(relative_volume_period),
        ];
        required.extend(ma_periods.iter().map(|&p| IndicatorSpec::Sma(p)));
        Self {
            fast_period,
            slow_period,
            relative_volume_period,
            min_relative_volume,
            ma_periods,
            required,
            prev_fast_above_slow: HashMap::new(),
        }
    }
}

impl Default for TrendConfluence {
    fn default() -> Self {
        Self::new(9, 21, 10, 1.2, vec![15, 30, 45, 60])
    }
}

impl Strategy for TrendConfluence {
    fn id(&self) -> StrategyId {
        "trend_confluence".to_string()
    }

    fn window(&self) -> TimeWindow {
        TimeWindow::FullDay
    }

    fn required_indicators(&self) -> &[IndicatorSpec] {
        &self.required
    }

    fn reset_session(&mut self) {
        self.prev_fast_above_slow.clear();
    }

    fn on_bar(&mut self, ctx: &StrategyContext) -> Option<SignalIntent> {
        // --- vwap_trend's own trigger: EMA crossover + VWAP side + volume ---
        let vwap = *ctx.indicators.get(&IndicatorSpec::Vwap.key())?;
        let fast = *ctx.indicators.get(&IndicatorSpec::Ema(self.fast_period).key())?;
        let slow = *ctx.indicators.get(&IndicatorSpec::Ema(self.slow_period).key())?;
        let rel_vol = *ctx
            .indicators
            .get(&IndicatorSpec::RelativeVolume(self.relative_volume_period).key())?;

        let fast_above_slow = fast > slow;
        let prev = self.prev_fast_above_slow.insert(ctx.instrument_token, fast_above_slow);
        let Some(prev) = prev else { return None };
        if prev == fast_above_slow {
            return None; // no crossover this bar
        }
        if rel_vol < self.min_relative_volume {
            return None;
        }

        let direction = if fast_above_slow { Direction::Long } else { Direction::Short };
        if (direction == Direction::Long && ctx.candle.close <= vwap)
            || (direction == Direction::Short && ctx.candle.close >= vwap)
        {
            return None; // price isn't on the crossover's side of VWAP
        }

        // --- ma_alignment's own condition: does the SMA stack agree, right now? ---
        let smas: Vec<f64> = self
            .ma_periods
            .iter()
            .map(|&p| ctx.indicators.get(&IndicatorSpec::Sma(p).key()).copied())
            .collect::<Option<_>>()?;
        let long_aligned = smas.windows(2).all(|w| w[0] > w[1]);
        let short_aligned = smas.windows(2).all(|w| w[0] < w[1]);
        let ma_direction = if long_aligned {
            Some(Direction::Long)
        } else if short_aligned {
            Some(Direction::Short)
        } else {
            None
        };
        if ma_direction != Some(direction) {
            return None; // the crossover fired, but the broader MA stack doesn't agree
        }

        Some(SignalIntent {
            strategy_id: self.id(),
            window: self.window(),
            instrument_token: ctx.instrument_token,
            ts: ctx.candle.ts,
            direction,
            entry_price_hint: ctx.candle.close,
            rationale: format!(
                "EMA{}/{} crossed {} with price {} VWAP {:.2} ({rel_vol:.2}x rel. volume), \
                 AND SMA({:?}) stack independently confirms the same direction",
                self.fast_period,
                self.slow_period,
                if fast_above_slow { "up" } else { "down" },
                if direction == Direction::Long { "above" } else { "below" },
                vwap,
                self.ma_periods,
            ),
            params: serde_json::json!({
                "vwap": vwap, "fast_ema": fast, "slow_ema": slow, "relative_volume": rel_vol,
                "smas": smas,
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

    fn ctx(vwap: f64, fast: f64, slow: f64, rel_vol: f64, ma_smas: &[f64], close: f64) -> StrategyContext {
        let mut indicators = HashMap::new();
        indicators.insert(IndicatorSpec::Vwap.key(), vwap);
        indicators.insert(IndicatorSpec::Ema(9).key(), fast);
        indicators.insert(IndicatorSpec::Ema(21).key(), slow);
        indicators.insert(IndicatorSpec::RelativeVolume(10).key(), rel_vol);
        for (p, v) in [15u32, 30, 45, 60].iter().zip(ma_smas.iter()) {
            indicators.insert(IndicatorSpec::Sma(*p).key(), *v);
        }
        StrategyContext {
            instrument_token: 1,
            window: TimeWindow::FullDay,
            candle: Candle {
                ts: IST.with_ymd_and_hms(2026, 9, 25, 11, 0, 0).unwrap().with_timezone(&Utc),
                open: close,
                high: close,
                low: close,
                close,
                volume: 10_000,
            },
            indicators,
            opening_range_high: None,
            opening_range_low: None,
        }
    }

    #[test]
    fn no_prior_bar_no_signal() {
        let mut s = TrendConfluence::default();
        assert!(s.on_bar(&ctx(100.0, 101.0, 100.0, 2.0, &[104.0, 103.0, 102.0, 101.0], 101.0)).is_none());
    }

    #[test]
    fn crossover_alone_without_ma_agreement_does_not_fire() {
        let mut s = TrendConfluence::default();
        // Bar 1: fast below slow (sets prior state).
        s.on_bar(&ctx(100.0, 99.0, 100.0, 2.0, &[104.0, 103.0, 102.0, 101.0], 99.0));
        // Bar 2: fast crosses above slow (bullish crossover), price above VWAP, good volume —
        // but SMA stack is NOT long-aligned (out of order) -> should not fire.
        let sig = s.on_bar(&ctx(100.0, 101.0, 100.0, 2.0, &[101.0, 102.0, 103.0, 104.0], 101.0));
        assert!(sig.is_none());
    }

    #[test]
    fn crossover_plus_ma_agreement_fires() {
        let mut s = TrendConfluence::default();
        s.on_bar(&ctx(100.0, 99.0, 100.0, 2.0, &[104.0, 103.0, 102.0, 101.0], 99.0));
        // Bullish crossover, price above VWAP, good volume, AND SMA stack long-aligned.
        let sig = s
            .on_bar(&ctx(100.0, 101.0, 100.0, 2.0, &[104.0, 103.0, 102.0, 101.0], 101.0))
            .unwrap();
        assert_eq!(sig.direction, Direction::Long);
    }

    #[test]
    fn weak_volume_still_blocks_even_with_ma_agreement() {
        let mut s = TrendConfluence::default();
        s.on_bar(&ctx(100.0, 99.0, 100.0, 2.0, &[104.0, 103.0, 102.0, 101.0], 99.0));
        let sig = s.on_bar(&ctx(100.0, 101.0, 100.0, 0.5, &[104.0, 103.0, 102.0, 101.0], 101.0));
        assert!(sig.is_none());
    }

    #[test]
    fn wrong_side_of_vwap_still_blocks_even_with_ma_agreement() {
        let mut s = TrendConfluence::default();
        s.on_bar(&ctx(100.0, 99.0, 100.0, 2.0, &[104.0, 103.0, 102.0, 101.0], 99.0));
        // Bullish crossover but price below VWAP (102) -> blocked.
        let sig = s.on_bar(&ctx(102.0, 101.0, 100.0, 2.0, &[104.0, 103.0, 102.0, 101.0], 101.0));
        assert!(sig.is_none());
    }

    #[test]
    fn bearish_crossover_with_short_aligned_stack_fires_short() {
        let mut s = TrendConfluence::default();
        s.on_bar(&ctx(100.0, 100.0, 99.0, 2.0, &[96.0, 97.0, 98.0, 99.0], 100.0));
        let sig = s
            .on_bar(&ctx(100.0, 99.0, 100.0, 2.0, &[96.0, 97.0, 98.0, 99.0], 99.0))
            .unwrap();
        assert_eq!(sig.direction, Direction::Short);
    }
}
