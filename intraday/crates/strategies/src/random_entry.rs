//! A strategy with no information in it, for use as a control.
//!
//! Every backtest number this project reports is an average R over trades the
//! strategy chose. That is only evidence of an edge if *not* choosing does
//! worse — and nobody had measured what not choosing does. A short held from
//! mid-morning to 15:10 is exposed to whatever the average stock does over
//! those hours, and if that drift is negative, a short-only book earns a
//! positive Avg R while knowing nothing at all.
//!
//! This fires at random inside the same window as the live strategy, short
//! side only, and is then managed by the same `ExitManager`, the same stop,
//! the same trail and the same square-off. Only the entry differs, so the
//! difference in Avg R is attributable to the entry and nothing else.
//!
//! Signals are a deterministic function of `(instrument_token, bar timestamp,
//! seed)` rather than a sequential RNG. That makes a run reproducible and, more
//! importantly, independent of the order instruments happen to be replayed in
//! — a sequential generator would couple every instrument's entries to the
//! replay order and make two runs incomparable.

use core_types::{
    Direction, IndicatorSpec, SignalIntent, Strategy, StrategyContext, StrategyId, TimeWindow,
};
use core_types::time::IST;

pub struct RandomEntry {
    id: StrategyId,
    window: TimeWindow,
    direction: Direction,
    /// Probability that any one eligible bar produces a signal.
    rate: f64,
    /// Matches the live strategy's `min_time_into_window_minutes` so the
    /// control cannot trade at times the real strategy is forbidden from.
    min_time_into_window_minutes: u32,
    seed: u64,
    required: Vec<IndicatorSpec>,
}

impl RandomEntry {
    pub fn new(direction: Direction, rate: f64, min_time_into_window_minutes: u32, seed: u64) -> Self {
        Self {
            id: format!("random_entry_{}", if direction == Direction::Short { "short" } else { "long" }),
            window: TimeWindow::MidMorning,
            direction,
            rate,
            min_time_into_window_minutes,
            seed,
            // Deliberately none. Requiring an indicator would make the control
            // wait for warmup and so trade a different set of bars than the
            // bars it is meant to sample uniformly.
            required: Vec::new(),
        }
    }

    pub fn with_id(mut self, id: impl Into<StrategyId>) -> Self {
        self.id = id.into();
        self
    }

    /// SplitMix64 over the three inputs — cheap, well-distributed, and no new
    /// dependency. Returns a uniform value in [0, 1).
    fn uniform(&self, token: u64, ts_secs: i64) -> f64 {
        let mut z = token
            .wrapping_mul(0x9E37_79B9_7F4A_7C15)
            .wrapping_add((ts_secs as u64).wrapping_mul(0xBF58_476D_1CE4_E5B9))
            .wrapping_add(self.seed);
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^= z >> 31;
        // Top 53 bits, the usual way to get a double without bias.
        (z >> 11) as f64 / (1u64 << 53) as f64
    }
}

impl Strategy for RandomEntry {
    fn id(&self) -> StrategyId {
        self.id.clone()
    }

    fn window(&self) -> TimeWindow {
        self.window
    }

    fn required_indicators(&self) -> &[IndicatorSpec] {
        &self.required
    }

    fn reset_session(&mut self) {}

    fn on_bar(&mut self, ctx: &StrategyContext) -> Option<SignalIntent> {
        let ist_time = ctx.candle.ts.with_timezone(&IST).time();
        if self.min_time_into_window_minutes > 0 {
            let start = self.window().default_bounds().0;
            if (ist_time - start).num_minutes() < self.min_time_into_window_minutes as i64 {
                return None;
            }
        }
        if self.uniform(ctx.instrument_token, ctx.candle.ts.timestamp()) >= self.rate {
            return None;
        }
        Some(SignalIntent {
            strategy_id: self.id(),
            window: self.window(),
            instrument_token: ctx.instrument_token,
            ts: ctx.candle.ts,
            direction: self.direction,
            entry_price_hint: ctx.candle.close,
            rationale: "random control — no information".to_string(),
            params: serde_json::json!({ "rate": self.rate, "seed": self.seed }),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use core_types::Candle;
    use std::collections::HashMap;

    fn ctx(token: u64, hh: u32, mm: u32) -> StrategyContext {
        StrategyContext {
            instrument_token: token,
            candle: Candle {
                ts: IST.with_ymd_and_hms(2026, 9, 29, hh, mm, 0).unwrap().with_timezone(&chrono::Utc),
                open: 100.0, high: 100.0, low: 100.0, close: 100.0, volume: 1000,
            },
            indicators: HashMap::new(),
            opening_range_high: None,
            opening_range_low: None,
            window: TimeWindow::MidMorning,
        }
    }

    /// The control must respect the same earliest-entry time as the live
    /// strategy, or it is sampling hours the real book cannot trade.
    #[test]
    fn respects_the_same_earliest_entry_time() {
        let mut s = RandomEntry::new(Direction::Short, 1.0, 45, 7);
        assert!(s.on_bar(&ctx(1, 10, 0)).is_none(), "10:00 is before 10:30");
        assert!(s.on_bar(&ctx(1, 10, 29)).is_none(), "10:29 is before 10:30");
        assert!(s.on_bar(&ctx(1, 10, 30)).is_some(), "10:30 is the first eligible bar");
    }

    /// Reproducibility is what makes two runs comparable at all.
    #[test]
    fn the_same_bar_always_gives_the_same_answer() {
        let mut a = RandomEntry::new(Direction::Short, 0.01, 45, 42);
        let mut b = RandomEntry::new(Direction::Short, 0.01, 45, 42);
        for m in 30..60 {
            assert_eq!(a.on_bar(&ctx(999, 10, m)).is_some(), b.on_bar(&ctx(999, 10, m)).is_some());
        }
    }

    /// A different seed must give a different sample, or the "control" is one
    /// fixed arbitrary set of trades rather than a random one.
    #[test]
    fn a_different_seed_samples_differently() {
        let mut a = RandomEntry::new(Direction::Short, 0.5, 45, 1);
        let mut b = RandomEntry::new(Direction::Short, 0.5, 45, 2);
        let fa: Vec<bool> = (30..60).map(|m| a.on_bar(&ctx(5, 10, m)).is_some()).collect();
        let fb: Vec<bool> = (30..60).map(|m| b.on_bar(&ctx(5, 10, m)).is_some()).collect();
        assert_ne!(fa, fb);
    }

    /// The realised rate has to be close to the requested one, or the control
    /// silently trades a different amount than intended and the comparison is
    /// made on mismatched samples.
    #[test]
    fn the_realised_rate_matches_the_requested_one() {
        let mut s = RandomEntry::new(Direction::Short, 0.10, 0, 3);
        let mut hits = 0;
        let n = 20_000;
        for i in 0..n {
            // Spread across instruments and minutes, as a replay would.
            if s.on_bar(&ctx(1000 + (i % 25) as u64, 11, 0)).is_some() {
                hits += 1;
            }
        }
        // Only 25 distinct (token, ts) pairs repeat here, so check the
        // distribution over distinct bars instead.
        let mut distinct = 0;
        for tok in 0..2000u64 {
            if s.on_bar(&ctx(tok, 11, 0)).is_some() {
                distinct += 1;
            }
        }
        assert!((0.06..0.15).contains(&(distinct as f64 / 2000.0)),
                "expected ~10% of bars to fire, got {}/2000", distinct);
        let _ = hits;
        let _ = n;
    }

    #[test]
    fn it_only_ever_signals_the_configured_direction() {
        let mut s = RandomEntry::new(Direction::Short, 1.0, 0, 9);
        let sig = s.on_bar(&ctx(1, 11, 0)).unwrap();
        assert_eq!(sig.direction, Direction::Short);
    }
}
