//! VWAP mean-reversion — the mirror image of `vwap_trend`. Where
//! `vwap_trend` follows an EMA crossover in the direction price already
//! broke, this fades a price that has stretched unusually far from VWAP,
//! betting on a snap back rather than continuation. Mean-reversion setups
//! like this typically trade a lower average win size for a materially
//! higher win rate than breakout/trend entries — the explicit goal this
//! strategy was added for (see the "high win rate over frequency" research
//! request).

use core_types::{Direction, IndicatorSpec, SignalIntent, Strategy, StrategyContext, StrategyId, TimeWindow};
use std::collections::HashMap;

pub struct VwapReversion {
    atr_period: u32,
    deviation_atr_multiple: f64,
    required: Vec<IndicatorSpec>,
    /// Keyed by instrument_token — see the same note in `orb.rs`. Tracks
    /// whether we're currently inside a "stretched" episode (price more
    /// than `deviation_atr_multiple` ATRs from VWAP) so a signal fires at
    /// most once per episode, on the first reversal-shaped bar.
    in_episode: HashMap<u64, bool>,
    signaled_this_episode: HashMap<u64, bool>,
}

impl VwapReversion {
    pub fn new(atr_period: u32, deviation_atr_multiple: f64) -> Self {
        Self {
            atr_period,
            deviation_atr_multiple,
            required: vec![IndicatorSpec::Vwap, IndicatorSpec::Atr(atr_period)],
            in_episode: HashMap::new(),
            signaled_this_episode: HashMap::new(),
        }
    }
}

impl Default for VwapReversion {
    fn default() -> Self {
        Self::new(14, 2.0)
    }
}

impl Strategy for VwapReversion {
    fn id(&self) -> StrategyId {
        "vwap_reversion".to_string()
    }

    fn window(&self) -> TimeWindow {
        TimeWindow::MidMorning
    }

    fn required_indicators(&self) -> &[IndicatorSpec] {
        &self.required
    }

    fn reset_session(&mut self) {
        self.in_episode.clear();
        self.signaled_this_episode.clear();
    }

    fn on_bar(&mut self, ctx: &StrategyContext) -> Option<SignalIntent> {
        let vwap = *ctx.indicators.get(&IndicatorSpec::Vwap.key())?;
        let atr = *ctx.indicators.get(&IndicatorSpec::Atr(self.atr_period).key())?;
        if atr <= 0.0 {
            return None;
        }

        let deviation = ctx.candle.close - vwap;
        let stretched = deviation.abs() / atr >= self.deviation_atr_multiple;

        if !stretched {
            self.in_episode.insert(ctx.instrument_token, false);
            self.signaled_this_episode.insert(ctx.instrument_token, false);
            return None;
        }
        self.in_episode.insert(ctx.instrument_token, true);
        if *self.signaled_this_episode.get(&ctx.instrument_token).unwrap_or(&false) {
            return None; // already fired for this stretch episode
        }

        // Reversal confirmation: the bar's own candle shape opposing the
        // stretch direction — a red candle while stretched above VWAP (the
        // push up is already fading), or a green candle while stretched
        // below (the push down is already fading).
        let is_red = ctx.candle.close < ctx.candle.open;
        let is_green = ctx.candle.close > ctx.candle.open;
        let direction = if deviation > 0.0 && is_red {
            Direction::Short
        } else if deviation < 0.0 && is_green {
            Direction::Long
        } else {
            return None; // still stretched, no reversal bar yet — keep watching
        };

        self.signaled_this_episode.insert(ctx.instrument_token, true);
        Some(SignalIntent {
            strategy_id: self.id(),
            window: self.window(),
            instrument_token: ctx.instrument_token,
            ts: ctx.candle.ts,
            direction,
            entry_price_hint: ctx.candle.close,
            rationale: format!(
                "price {:.2} stretched {:.2}x ATR({}) from VWAP {vwap:.2}, reversal candle confirmed",
                ctx.candle.close,
                deviation.abs() / atr,
                self.atr_period,
            ),
            params: serde_json::json!({
                "vwap": vwap,
                "atr": atr,
                "deviation_atr_multiple": deviation.abs() / atr,
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

    fn ctx(close: f64, open: f64, vwap: f64, atr: f64, token: u64) -> StrategyContext {
        let mut indicators = HashMap::new();
        indicators.insert(IndicatorSpec::Vwap.key(), vwap);
        indicators.insert(IndicatorSpec::Atr(14).key(), atr);
        StrategyContext {
            instrument_token: token,
            window: TimeWindow::MidMorning,
            candle: Candle {
                ts: IST.with_ymd_and_hms(2026, 9, 24, 10, 0, 0).unwrap().with_timezone(&Utc),
                open,
                high: open.max(close),
                low: open.min(close),
                close,
                volume: 10_000,
            },
            indicators,
            opening_range_high: None,
            opening_range_low: None,
        }
    }

    #[test]
    fn no_signal_while_not_stretched() {
        let mut s = VwapReversion::new(14, 2.0);
        assert!(s.on_bar(&ctx(101.0, 100.5, 100.0, 1.0, 1)).is_none());
    }

    #[test]
    fn stretched_above_vwap_but_still_green_does_not_fire() {
        let mut s = VwapReversion::new(14, 2.0);
        // deviation = 3.0, atr = 1.0 -> 3x stretched, but candle is green (close>open) -> no reversal yet
        assert!(s.on_bar(&ctx(103.0, 102.5, 100.0, 1.0, 1)).is_none());
    }

    #[test]
    fn stretched_above_vwap_with_red_candle_fires_short() {
        let mut s = VwapReversion::new(14, 2.0);
        let sig = s.on_bar(&ctx(103.0, 103.5, 100.0, 1.0, 1)).unwrap();
        assert_eq!(sig.direction, Direction::Short);
    }

    #[test]
    fn stretched_below_vwap_with_green_candle_fires_long() {
        let mut s = VwapReversion::new(14, 2.0);
        let sig = s.on_bar(&ctx(97.0, 96.5, 100.0, 1.0, 1)).unwrap();
        assert_eq!(sig.direction, Direction::Long);
    }

    #[test]
    fn only_fires_once_per_stretch_episode() {
        let mut s = VwapReversion::new(14, 2.0);
        assert!(s.on_bar(&ctx(103.0, 103.5, 100.0, 1.0, 1)).is_some());
        // Still stretched and still red -> no second signal this episode.
        assert!(s.on_bar(&ctx(103.2, 103.6, 100.0, 1.0, 1)).is_none());
    }

    #[test]
    fn a_new_episode_after_returning_to_normal_can_fire_again() {
        let mut s = VwapReversion::new(14, 2.0);
        assert!(s.on_bar(&ctx(103.0, 103.5, 100.0, 1.0, 1)).is_some());
        // Back inside normal range -> episode ends.
        assert!(s.on_bar(&ctx(100.2, 100.1, 100.0, 1.0, 1)).is_none());
        // New stretch episode -> can fire again.
        assert!(s.on_bar(&ctx(103.0, 103.5, 100.0, 1.0, 1)).is_some());
    }

    #[test]
    fn per_instrument_state_does_not_cross_contaminate() {
        let mut s = VwapReversion::new(14, 2.0);
        assert!(s.on_bar(&ctx(103.0, 103.5, 100.0, 1.0, 1)).is_some());
        // A different instrument's first stretched+reversal bar should still fire.
        assert!(s.on_bar(&ctx(103.0, 103.5, 100.0, 1.0, 2)).is_some());
    }
}
