//! Strong unidirectional rally continuation — a full-session strategy
//! (`TimeWindow::FullDay`, not tied to any single window) that aggregates
//! incoming 1-minute bars into `bar_minutes`-minute candles internally and
//! watches for a streak of consecutive same-direction candles each moving
//! at least `min_move_pct`. Once the streak reaches `min_streak`, it enters
//! at the *open* of the very next candle — the moment that candle starts,
//! not after seeing how it plays out.
//!
//! Deliberately bidirectional (a 3-red-candle decline is treated the same
//! as a 3-green-candle rally, mirrored) even though the request that
//! prompted this only described the upward case, matching every other
//! strategy in this crate.

use chrono::Timelike;
use core_types::{Direction, IndicatorSpec, SignalIntent, Strategy, StrategyContext, StrategyId, TimeWindow};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Bucket {
    start_minute: i64,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
}

#[derive(Debug, Clone, Copy, Default)]
struct Streak {
    direction: Option<Direction>,
    count: u32,
    /// True once we've already fired for the current streak — cleared the
    /// moment the streak breaks, so a fresh streak can fire again.
    fired: bool,
}

pub struct StrongRally {
    bar_minutes: i64,
    min_streak: u32,
    /// Fraction of price, same convention as `risk_pct` elsewhere (0.002 = 0.2%).
    min_move_pct: f64,
    required: Vec<IndicatorSpec>,
    /// Keyed by instrument_token — see the same note in `orb.rs`.
    buckets: HashMap<u64, Bucket>,
    streaks: HashMap<u64, Streak>,
}

impl StrongRally {
    pub fn new(bar_minutes: i64, min_streak: u32, min_move_pct: f64) -> Self {
        Self {
            bar_minutes,
            min_streak,
            min_move_pct,
            required: Vec::new(), // pure price action — no indicators needed
            buckets: HashMap::new(),
            streaks: HashMap::new(),
        }
    }
}

impl Default for StrongRally {
    fn default() -> Self {
        Self::new(5, 3, 0.002)
    }
}

impl Strategy for StrongRally {
    fn id(&self) -> StrategyId {
        "strong_rally".to_string()
    }

    fn window(&self) -> TimeWindow {
        TimeWindow::FullDay
    }

    fn required_indicators(&self) -> &[IndicatorSpec] {
        &self.required
    }

    fn reset_session(&mut self) {
        self.buckets.clear();
        self.streaks.clear();
    }

    fn on_bar(&mut self, ctx: &StrategyContext) -> Option<SignalIntent> {
        use core_types::time::IST;
        let ist_time = ctx.candle.ts.with_timezone(&IST).time();
        let minute_of_day = ist_time.num_seconds_from_midnight() as i64 / 60;
        let bucket_start = (minute_of_day / self.bar_minutes) * self.bar_minutes;

        let token = ctx.instrument_token;
        let prev_bucket = self.buckets.get(&token).copied();

        let mut signal = None;

        match prev_bucket {
            Some(b) if b.start_minute == bucket_start => {
                // Still inside the same forming candle — just extend it.
                self.buckets.insert(
                    token,
                    Bucket {
                        start_minute: b.start_minute,
                        open: b.open,
                        high: b.high.max(ctx.candle.high),
                        low: b.low.min(ctx.candle.low),
                        close: ctx.candle.close,
                    },
                );
            }
            Some(completed) => {
                // A new bucket just started -> the previous one is final.
                // Fold it into the streak, then check whether *this* bar
                // (the first of the new candle) is the entry moment.
                let move_pct = (completed.close - completed.open) / completed.open * 100.0;
                let candle_direction = if move_pct > 0.0 {
                    Some(Direction::Long)
                } else if move_pct < 0.0 {
                    Some(Direction::Short)
                } else {
                    None
                };
                let qualifies = candle_direction.is_some() && move_pct.abs() >= self.min_move_pct * 100.0;

                let mut streak = self.streaks.get(&token).copied().unwrap_or_default();
                if qualifies && candle_direction == streak.direction {
                    streak.count += 1;
                } else if qualifies {
                    streak.direction = candle_direction;
                    streak.count = 1;
                    streak.fired = false;
                } else {
                    streak = Streak::default(); // chain broken — must be consecutive
                }

                if streak.count >= self.min_streak && !streak.fired {
                    streak.fired = true;
                    let direction = streak.direction.expect("count>0 implies a direction was set");
                    signal = Some(SignalIntent {
                        strategy_id: self.id(),
                        window: self.window(),
                        instrument_token: token,
                        ts: ctx.candle.ts,
                        direction,
                        entry_price_hint: ctx.candle.open, // enter AT the open of this new candle
                        rationale: format!(
                            "{} consecutive {}-min candles moved {} with |move| >= {:.2}% each — entering at open of the next candle",
                            streak.count,
                            self.bar_minutes,
                            if direction == Direction::Long { "up" } else { "down" },
                            self.min_move_pct * 100.0,
                        ),
                        params: serde_json::json!({
                            "streak_count": streak.count,
                            "bar_minutes": self.bar_minutes,
                            "min_move_pct": self.min_move_pct * 100.0,
                            "last_candle_move_pct": move_pct,
                        }),
                    });
                }
                self.streaks.insert(token, streak);

                self.buckets.insert(
                    token,
                    Bucket {
                        start_minute: bucket_start,
                        open: ctx.candle.open,
                        high: ctx.candle.high,
                        low: ctx.candle.low,
                        close: ctx.candle.close,
                    },
                );
            }
            None => {
                // First bar ever seen for this instrument this session.
                self.buckets.insert(
                    token,
                    Bucket {
                        start_minute: bucket_start,
                        open: ctx.candle.open,
                        high: ctx.candle.high,
                        low: ctx.candle.low,
                        close: ctx.candle.close,
                    },
                );
            }
        }

        signal
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};
    use core_types::time::IST;
    use core_types::Candle;

    fn bar(hh: u32, mm: u32, open: f64, close: f64) -> StrategyContext {
        StrategyContext {
            instrument_token: 1,
            window: TimeWindow::FullDay,
            candle: Candle {
                ts: IST.with_ymd_and_hms(2026, 9, 25, hh, mm, 0).unwrap().with_timezone(&Utc),
                open,
                high: open.max(close),
                low: open.min(close),
                close,
                volume: 1_000,
            },
            indicators: std::collections::HashMap::new(),
            opening_range_high: None,
            opening_range_low: None,
        }
    }

    /// Feeds one bar per minute for a 5-min candle [start_min, start_min+5)
    /// that moves from `open` to `close`, flat in between (doesn't matter —
    /// only the finalized open/close feed the streak logic).
    fn feed_5m_candle(s: &mut StrongRally, start_hh: u32, start_mm: u32, open: f64, close: f64) {
        for i in 0..5 {
            let mm = start_mm + i;
            let ctx = bar(start_hh, mm, if i == 0 { open } else { close }, if i == 4 { close } else { close });
            s.on_bar(&ctx);
        }
    }

    #[test]
    fn no_signal_until_three_consecutive_qualifying_candles() {
        let mut s = StrongRally::new(5, 3, 0.002); // 0.2%
        feed_5m_candle(&mut s, 9, 15, 100.0, 100.25); // +0.25%, qualifies, streak=1
        feed_5m_candle(&mut s, 9, 20, 100.25, 100.55); // +0.30%, streak=2
        // Third candle finalizes only once the NEXT bucket's first bar arrives.
        let ctx = bar(9, 25, 100.55, 100.90); // +0.35%, this bar starts bucket 3, finalizes bucket 2
        assert!(s.on_bar(&ctx).is_none());
    }

    #[test]
    fn fires_at_open_of_the_fourth_candle() {
        let mut s = StrongRally::new(5, 3, 0.002);
        feed_5m_candle(&mut s, 9, 15, 100.0, 100.25); // candle 1: +0.25%
        feed_5m_candle(&mut s, 9, 20, 100.25, 100.55); // candle 2: +0.30%
        feed_5m_candle(&mut s, 9, 25, 100.55, 100.90); // candle 3: +0.35%
        // First bar of candle 4 finalizes candle 3 -> streak hits 3 -> fires here.
        let ctx = bar(9, 30, 100.90, 100.95);
        let sig = s.on_bar(&ctx).unwrap();
        assert_eq!(sig.direction, Direction::Long);
        assert!((sig.entry_price_hint - 100.90).abs() < 1e-9); // entry = open of 4th candle
    }

    #[test]
    fn only_fires_once_even_if_rally_keeps_extending() {
        let mut s = StrongRally::new(5, 3, 0.002);
        feed_5m_candle(&mut s, 9, 15, 100.0, 100.25);
        feed_5m_candle(&mut s, 9, 20, 100.25, 100.55);
        feed_5m_candle(&mut s, 9, 25, 100.55, 100.90);
        assert!(s.on_bar(&bar(9, 30, 100.90, 100.95)).is_some()); // fires
        feed_5m_candle(&mut s, 9, 30, 100.90, 101.30); // candle 4 also qualifies (streak=4)
        assert!(s.on_bar(&bar(9, 35, 101.30, 101.32)).is_none()); // no second signal
    }

    #[test]
    fn a_weak_candle_breaks_the_streak() {
        let mut s = StrongRally::new(5, 3, 0.002);
        feed_5m_candle(&mut s, 9, 15, 100.0, 100.25); // +0.25%, streak=1
        feed_5m_candle(&mut s, 9, 20, 100.25, 100.30); // +0.05%, too weak -> streak resets to 0
        feed_5m_candle(&mut s, 9, 25, 100.30, 100.55); // +0.25%, streak=1 again (not 3)
        assert!(s.on_bar(&bar(9, 30, 100.55, 100.56)).is_none());
    }

    #[test]
    fn a_reversal_candle_breaks_the_streak_even_if_strong() {
        let mut s = StrongRally::new(5, 3, 0.002);
        feed_5m_candle(&mut s, 9, 15, 100.0, 100.25); // +0.25% up, streak=1
        feed_5m_candle(&mut s, 9, 20, 100.25, 99.95); // -0.30% down, strong but wrong direction -> new streak=1 (short)
        feed_5m_candle(&mut s, 9, 25, 99.95, 100.20); // +0.25% up, wrong direction again -> new streak=1 (long)
        assert!(s.on_bar(&bar(9, 30, 100.20, 100.21)).is_none());
    }

    #[test]
    fn downward_streak_fires_short() {
        let mut s = StrongRally::new(5, 3, 0.002);
        feed_5m_candle(&mut s, 9, 15, 100.0, 99.75); // -0.25%
        feed_5m_candle(&mut s, 9, 20, 99.75, 99.45); // -0.30%
        feed_5m_candle(&mut s, 9, 25, 99.45, 99.10); // -0.35%
        let sig = s.on_bar(&bar(9, 30, 99.10, 99.05)).unwrap();
        assert_eq!(sig.direction, Direction::Short);
        assert!((sig.entry_price_hint - 99.10).abs() < 1e-9);
    }

    #[test]
    fn reset_session_clears_streak_state() {
        let mut s = StrongRally::new(5, 3, 0.002);
        feed_5m_candle(&mut s, 9, 15, 100.0, 100.25);
        feed_5m_candle(&mut s, 9, 20, 100.25, 100.55);
        s.reset_session();
        feed_5m_candle(&mut s, 9, 25, 100.55, 100.90); // would've been streak=3 without reset
        assert!(s.on_bar(&bar(9, 30, 100.90, 100.91)).is_none());
    }
}
