//! Moving-average alignment / trend acceleration — a full-session strategy
//! (`TimeWindow::FullDay`, like `strong_rally`) that watches N simple
//! moving averages of different lookback lengths stack in order, shortest
//! to longest, e.g. `avg(60m) < avg(45m) < avg(30m) < avg(15m)` for an
//! accelerating uptrend (each shorter, more recent average sits above the
//! next-longer one — price has been climbing faster and faster), mirrored
//! for a downtrend. A stricter, N-timeframe generalization of a two-EMA
//! crossover (`vwap_trend`) — requiring the *whole stack* to agree, not
//! just two lines, is the bet that it filters out more false starts.
//! Generic over the period list so different tier counts/spacings (e.g.
//! three timeframes instead of four) can be tested without new code.

use core_types::{Direction, IndicatorSpec, SignalIntent, Strategy, StrategyContext, StrategyId, TimeWindow};
use std::collections::HashMap;

pub struct MaAlignment {
    /// Shortest to longest, e.g. `[15, 30, 45, 60]`. At least 2 required.
    periods: Vec<u32>,
    /// How many *extra* consecutive bars the alignment must hold before
    /// entering, beyond the bar it first appears on. 0 = enter immediately
    /// on the first aligned bar (the original behavior). >0 trades a later,
    /// worse-priced entry for filtering out alignments that immediately
    /// collapse — aimed directly at the "half of all trades get stopped
    /// out before the signal proves itself" pattern found in backtesting.
    confirmation_bars: u32,
    required: Vec<IndicatorSpec>,
    /// Keyed by instrument_token: (current aligned direction if any,
    /// consecutive bars it's held, whether we've already fired for this
    /// streak) — a signal fires once per streak, on the bar the required
    /// confirmation length is first reached.
    streaks: HashMap<u64, (Option<Direction>, u32, bool)>,
}

impl MaAlignment {
    pub fn new(periods: Vec<u32>) -> Self {
        Self::with_confirmation(periods, 0)
    }

    pub fn with_confirmation(periods: Vec<u32>, confirmation_bars: u32) -> Self {
        assert!(periods.len() >= 2, "MaAlignment needs at least 2 periods to compare");
        Self {
            required: periods.iter().map(|&p| IndicatorSpec::Sma(p)).collect(),
            periods,
            confirmation_bars,
            streaks: HashMap::new(),
        }
    }
}

impl Default for MaAlignment {
    fn default() -> Self {
        Self::new(vec![15, 30, 45, 60])
    }
}

impl Strategy for MaAlignment {
    fn id(&self) -> StrategyId {
        "ma_alignment".to_string()
    }

    fn window(&self) -> TimeWindow {
        TimeWindow::FullDay
    }

    fn required_indicators(&self) -> &[IndicatorSpec] {
        &self.required
    }

    fn reset_session(&mut self) {
        self.streaks.clear();
    }

    fn on_bar(&mut self, ctx: &StrategyContext) -> Option<SignalIntent> {
        // smas[0] = shortest period's SMA ... smas[n-1] = longest period's.
        let smas: Vec<f64> = self
            .periods
            .iter()
            .map(|&p| ctx.indicators.get(&IndicatorSpec::Sma(p).key()).copied())
            .collect::<Option<_>>()?;

        // Long (accelerating up): each shorter-period average sits above
        // the next-longer one, i.e. strictly decreasing as we walk from
        // shortest to longest. Short is the mirror image.
        let long_aligned = smas.windows(2).all(|w| w[0] > w[1]);
        let short_aligned = smas.windows(2).all(|w| w[0] < w[1]);
        let current = if long_aligned {
            Some(Direction::Long)
        } else if short_aligned {
            Some(Direction::Short)
        } else {
            None
        };

        let (prev_dir, prev_count, prev_fired) =
            self.streaks.get(&ctx.instrument_token).copied().unwrap_or((None, 0, false));
        let (count, fired) = if current == prev_dir && current.is_some() {
            (prev_count + 1, prev_fired)
        } else {
            (if current.is_some() { 1 } else { 0 }, false)
        };
        self.streaks.insert(ctx.instrument_token, (current, count, fired));

        let direction = current?;
        if fired || count < self.confirmation_bars + 1 {
            return None; // already fired this streak, or hasn't held long enough yet
        }
        self.streaks.insert(ctx.instrument_token, (current, count, true));

        // Periods are stored shortest-first; report longest-first so the
        // rationale reads naturally ("long-period average, then
        // progressively shorter, each one higher than the last").
        let arrow = if direction == Direction::Long { "<" } else { ">" };
        let stack_longest_first = self
            .periods
            .iter()
            .zip(smas.iter())
            .rev()
            .map(|(p, v)| format!("avg({p}m)={v:.2}"))
            .collect::<Vec<_>>()
            .join(&format!(" {arrow} "));

        Some(SignalIntent {
            strategy_id: self.id(),
            window: self.window(),
            instrument_token: ctx.instrument_token,
            ts: ctx.candle.ts,
            direction,
            entry_price_hint: ctx.candle.close,
            rationale: format!(
                "SMA stack aligned {}: {stack_longest_first}",
                if direction == Direction::Long { "up" } else { "down" },
            ),
            params: serde_json::json!({
                "periods": self.periods,
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

    fn ctx(periods: &[u32], smas: &[f64], close: f64) -> StrategyContext {
        let mut indicators = HashMap::new();
        for (&p, &v) in periods.iter().zip(smas.iter()) {
            indicators.insert(IndicatorSpec::Sma(p).key(), v);
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
                volume: 1_000,
            },
            indicators,
            opening_range_high: None,
            opening_range_low: None,
        }
    }

    const P4: [u32; 4] = [15, 30, 45, 60];

    #[test]
    fn no_signal_when_not_aligned() {
        let mut s = MaAlignment::default();
        assert!(s.on_bar(&ctx(&P4, &[101.0, 102.0, 103.0, 100.0], 101.0)).is_none());
    }

    #[test]
    fn full_uptrend_stack_fires_long() {
        let mut s = MaAlignment::default();
        let sig = s.on_bar(&ctx(&P4, &[104.0, 103.0, 102.0, 101.0], 104.0)).unwrap();
        assert_eq!(sig.direction, Direction::Long);
        assert!((sig.entry_price_hint - 104.0).abs() < 1e-9);
    }

    #[test]
    fn full_downtrend_stack_fires_short() {
        let mut s = MaAlignment::default();
        let sig = s.on_bar(&ctx(&P4, &[96.0, 97.0, 98.0, 99.0], 96.0)).unwrap();
        assert_eq!(sig.direction, Direction::Short);
    }

    #[test]
    fn partial_stack_does_not_fire() {
        let mut s = MaAlignment::default();
        assert!(s.on_bar(&ctx(&P4, &[102.0, 101.0, 99.0, 100.0], 102.0)).is_none());
    }

    #[test]
    fn only_fires_once_while_alignment_holds() {
        let mut s = MaAlignment::default();
        assert!(s.on_bar(&ctx(&P4, &[104.0, 103.0, 102.0, 101.0], 104.0)).is_some());
        assert!(s.on_bar(&ctx(&P4, &[105.0, 103.5, 102.2, 101.1], 105.0)).is_none());
    }

    #[test]
    fn fires_again_after_alignment_breaks_and_reforms() {
        let mut s = MaAlignment::default();
        assert!(s.on_bar(&ctx(&P4, &[104.0, 103.0, 102.0, 101.0], 104.0)).is_some());
        assert!(s.on_bar(&ctx(&P4, &[101.0, 102.0, 102.0, 101.0], 101.0)).is_none());
        assert!(s.on_bar(&ctx(&P4, &[106.0, 104.0, 102.0, 101.0], 106.0)).is_some());
    }

    #[test]
    fn flipping_directly_from_long_to_short_alignment_fires_short() {
        let mut s = MaAlignment::default();
        assert!(s.on_bar(&ctx(&P4, &[104.0, 103.0, 102.0, 101.0], 104.0)).is_some());
        let sig = s.on_bar(&ctx(&P4, &[96.0, 97.0, 98.0, 99.0], 96.0)).unwrap();
        assert_eq!(sig.direction, Direction::Short);
    }

    #[test]
    fn confirmation_bars_delays_entry_until_alignment_holds() {
        let mut s = MaAlignment::with_confirmation(P4.to_vec(), 1); // needs 2 consecutive aligned bars
        assert!(s.on_bar(&ctx(&P4, &[104.0, 103.0, 102.0, 101.0], 104.0)).is_none()); // 1st aligned bar — not enough yet
        let sig = s.on_bar(&ctx(&P4, &[105.0, 103.5, 102.2, 101.1], 105.0)).unwrap(); // 2nd — fires now
        assert_eq!(sig.direction, Direction::Long);
        assert!((sig.entry_price_hint - 105.0).abs() < 1e-9); // entry price is the confirmation bar's, not the first
    }

    #[test]
    fn confirmation_bars_resets_if_alignment_breaks_before_confirming() {
        let mut s = MaAlignment::with_confirmation(P4.to_vec(), 1);
        assert!(s.on_bar(&ctx(&P4, &[104.0, 103.0, 102.0, 101.0], 104.0)).is_none()); // 1st aligned bar
        assert!(s.on_bar(&ctx(&P4, &[101.0, 102.0, 102.0, 101.0], 101.0)).is_none()); // breaks before confirming
        assert!(s.on_bar(&ctx(&P4, &[104.0, 103.0, 102.0, 101.0], 104.0)).is_none()); // 1st aligned bar again, not 2nd
        let sig = s.on_bar(&ctx(&P4, &[105.0, 103.5, 102.2, 101.1], 105.0)).unwrap(); // now confirmed
        assert_eq!(sig.direction, Direction::Long);
    }

    #[test]
    fn missing_indicator_returns_none_gracefully() {
        let mut s = MaAlignment::default();
        let mut c = ctx(&P4, &[104.0, 103.0, 102.0, 101.0], 104.0);
        c.indicators.remove(&IndicatorSpec::Sma(60).key());
        assert!(s.on_bar(&c).is_none());
    }

    #[test]
    fn three_period_variant_works_the_same_way() {
        let mut s = MaAlignment::new(vec![15, 30, 45]);
        let p3 = [15u32, 30, 45];
        assert!(s.on_bar(&ctx(&p3, &[102.0, 101.0, 100.0], 102.0)).is_some());
    }

    #[test]
    #[should_panic(expected = "at least 2 periods")]
    fn fewer_than_two_periods_panics() {
        MaAlignment::new(vec![15]);
    }
}
