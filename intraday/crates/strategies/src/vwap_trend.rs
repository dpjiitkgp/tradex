//! VWAP + EMA-crossover trend following — originally the 09:45-11:30
//! window strategy, now generalized to support running the full session
//! with an absolute entry-time cutoff instead. Signals on an EMA fast/slow
//! crossover, filtered by price being on the same side of VWAP and
//! relative-volume confirmation, biased (not gated entirely) by which way
//! the morning's opening range broke.

use core_types::time::IST;
use core_types::{Direction, IndicatorSpec, SignalIntent, Strategy, StrategyContext, StrategyId, TimeWindow};
use std::collections::HashMap;

pub struct VwapTrendFollow {
    fast_period: u32,
    slow_period: u32,
    relative_volume_period: u32,
    min_relative_volume: f64,
    /// Take long-side (EMA-up) crossovers at all. Backtesting this
    /// strategy in isolation found short signals winning meaningfully more
    /// often than long ones across the whole 6-month/250-stock universe —
    /// these two flags exist so that asymmetry can be tuned/tested/turned
    /// off without touching the crossover logic itself.
    allow_long: bool,
    /// Take short-side (EMA-down) crossovers at all.
    allow_short: bool,
    /// Ignore crossovers in the first N minutes of this strategy's window
    /// (relative to `window`'s own start) — the same backtest found
    /// early-window signals noticeably weaker than late-window ones,
    /// plausibly because VWAP and the EMAs are still settling right after
    /// the volatile opening range. Superseded by `min_entry_time` for new
    /// variants that don't want the ambiguity of "relative to window
    /// start" once `window` itself becomes configurable — kept for
    /// backward compatibility with the already-validated MidMorning variant.
    min_time_into_window_minutes: u32,
    /// Which `Strategy::window()` this instance reports — defaults to
    /// `MidMorning` (the original design). Set to `FullDay` via
    /// `with_window` to let it watch every bar in the session instead of
    /// being confined to one window.
    window: TimeWindow,
    /// Absolute IST cutoffs, independent of `window`/`min_time_into_window_minutes`
    /// — unlike that field, these don't shift if `window` changes.
    min_entry_time: Option<chrono::NaiveTime>,
    max_entry_time: Option<chrono::NaiveTime>,
    /// Reported by `id()` — defaults to `"vwap_trend"`. Override via
    /// `with_id` when running multiple differently-tuned instances in the
    /// same research batch, so results don't collapse into ambiguous rows
    /// under one shared name.
    id: String,
    /// If > 0, additionally requires VWAP itself to have moved in the
    /// trade's direction over this many bars (VWAP now vs VWAP this many
    /// bars ago) — confirms the reference line is actually trending rather
    /// than price merely sitting on one side of a flat VWAP. 0 disables
    /// the check entirely (the original behavior).
    vwap_slope_lookback_bars: u32,
    /// Minimum |VWAP now - VWAP N bars ago| / VWAP N bars ago required in
    /// the trade's direction, as a fraction (e.g. 0.0005 = 0.05%). Only
    /// applies when `vwap_slope_lookback_bars > 0`; 0.0 means "any slope
    /// in the right direction counts, however small".
    min_vwap_slope_pct: f64,
    /// Ceiling on how steeply VWAP may ALREADY have moved, as a fraction —
    /// `Some(0.0005)` rejects a short whose VWAP has fallen more than 0.05%
    /// over the lookback.
    ///
    /// The opposite of `min_vwap_slope_pct`, and added because the data said
    /// the floor points the wrong way. Partitioning 8,500 live-config trades
    /// by realised slope gave a perfectly monotone ladder: the flatter VWAP
    /// was, the better the trade (+0.118 R at nearly-flat down to -0.158 R
    /// below -0.35%, win rate 47.5% down to 25.5%). The edge comes from
    /// catching a turn that has not happened yet; by the time VWAP has
    /// visibly rolled over, the move is spent and the short is into a bounce.
    max_vwap_slope_pct: Option<f64>,
    required: Vec<IndicatorSpec>,
    /// Keyed by instrument_token — see the same note in `orb.rs`.
    prev_fast_above_slow: HashMap<u64, bool>,
    /// Keyed by instrument_token — rolling VWAP history for the slope
    /// check, capped at `vwap_slope_lookback_bars + 1` entries. Cleared
    /// each session since VWAP itself resets daily.
    vwap_history: HashMap<u64, std::collections::VecDeque<f64>>,
}

impl VwapTrendFollow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        fast_period: u32,
        slow_period: u32,
        relative_volume_period: u32,
        min_relative_volume: f64,
        allow_long: bool,
        allow_short: bool,
        min_time_into_window_minutes: u32,
    ) -> Self {
        Self {
            fast_period,
            slow_period,
            relative_volume_period,
            min_relative_volume,
            allow_long,
            allow_short,
            min_time_into_window_minutes,
            window: TimeWindow::MidMorning,
            min_entry_time: None,
            max_entry_time: None,
            id: "vwap_trend".to_string(),
            vwap_slope_lookback_bars: 0,
            min_vwap_slope_pct: 0.0,
            max_vwap_slope_pct: None,
            required: vec![
                IndicatorSpec::Vwap,
                IndicatorSpec::Ema(fast_period),
                IndicatorSpec::Ema(slow_period),
                IndicatorSpec::RelativeVolume(relative_volume_period),
            ],
            prev_fast_above_slow: HashMap::new(),
            vwap_history: HashMap::new(),
        }
    }

    pub fn with_window(mut self, window: TimeWindow) -> Self {
        self.window = window;
        self
    }

    /// No entries before this IST time (absolute, not window-relative).
    pub fn with_min_entry_time(mut self, t: chrono::NaiveTime) -> Self {
        self.min_entry_time = Some(t);
        self
    }

    /// No *new* entries at or after this IST time — existing open
    /// positions are unaffected and still exit normally (stop/trail/EOD
    /// square-off), this only gates fresh signals.
    pub fn with_max_entry_time(mut self, t: chrono::NaiveTime) -> Self {
        self.max_entry_time = Some(t);
        self
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    /// Require VWAP itself to have moved at least `min_slope_pct` in the
    /// trade's direction over the last `lookback_bars` bars (VWAP now vs
    /// VWAP `lookback_bars` bars ago), in addition to the existing
    /// price-vs-VWAP-side check. `lookback_bars == 0` disables the check.
    pub fn with_vwap_slope(mut self, lookback_bars: u32, min_slope_pct: f64) -> Self {
        self.vwap_slope_lookback_bars = lookback_bars;
        self.min_vwap_slope_pct = min_slope_pct;
        self
    }

    /// Caps how far VWAP may already have moved. See `max_vwap_slope_pct`.
    pub fn with_max_vwap_slope(mut self, max_slope_pct: f64) -> Self {
        self.max_vwap_slope_pct = Some(max_slope_pct);
        self
    }
}

impl Default for VwapTrendFollow {
    fn default() -> Self {
        Self::new(9, 21, 10, 1.2, true, true, 0)
    }
}

/// Which way the opening range broke, as a soft directional bias — `None`
/// means no restriction (range not formed, or price still inside it).
fn opening_range_bias(ctx: &StrategyContext) -> Option<Direction> {
    let (high, low) = (ctx.opening_range_high?, ctx.opening_range_low?);
    if ctx.candle.close > high {
        Some(Direction::Long)
    } else if ctx.candle.close < low {
        Some(Direction::Short)
    } else {
        None
    }
}

impl Strategy for VwapTrendFollow {
    fn id(&self) -> StrategyId {
        self.id.clone()
    }

    fn window(&self) -> TimeWindow {
        self.window
    }

    fn required_indicators(&self) -> &[IndicatorSpec] {
        &self.required
    }

    fn reset_session(&mut self) {
        self.prev_fast_above_slow.clear();
        self.vwap_history.clear();
    }

    fn on_bar(&mut self, ctx: &StrategyContext) -> Option<SignalIntent> {
        let vwap = *ctx.indicators.get(&IndicatorSpec::Vwap.key())?;
        let fast = *ctx.indicators.get(&IndicatorSpec::Ema(self.fast_period).key())?;
        let slow = *ctx.indicators.get(&IndicatorSpec::Ema(self.slow_period).key())?;
        let rel_vol = *ctx
            .indicators
            .get(&IndicatorSpec::RelativeVolume(self.relative_volume_period).key())?;

        if self.vwap_slope_lookback_bars > 0 {
            let hist = self.vwap_history.entry(ctx.instrument_token).or_default();
            hist.push_back(vwap);
            let max_len = self.vwap_slope_lookback_bars as usize + 1;
            if hist.len() > max_len {
                hist.pop_front();
            }
        }

        let fast_above_slow = fast > slow;
        let prev = self.prev_fast_above_slow.insert(ctx.instrument_token, fast_above_slow);

        let Some(prev) = prev else { return None }; // need a prior bar to detect a crossover
        if prev == fast_above_slow {
            return None; // no crossover this bar
        }
        if rel_vol < self.min_relative_volume {
            return None;
        }

        let ist_time = ctx.candle.ts.with_timezone(&IST).time();
        if self.min_time_into_window_minutes > 0 {
            let window_start = self.window().default_bounds().0;
            if (ist_time - window_start).num_minutes() < self.min_time_into_window_minutes as i64 {
                return None; // too early in the window — signal quality found weaker here in backtesting
            }
        }
        if let Some(min_t) = self.min_entry_time {
            if ist_time < min_t {
                return None;
            }
        }
        if let Some(max_t) = self.max_entry_time {
            if ist_time >= max_t {
                return None; // past the cutoff — no new entries, existing positions unaffected
            }
        }

        let bias = opening_range_bias(ctx);
        let direction = if fast_above_slow {
            Direction::Long
        } else {
            Direction::Short
        };
        match direction {
            Direction::Long if !self.allow_long => return None,
            Direction::Short if !self.allow_short => return None,
            _ => {}
        }
        if let Some(bias) = bias {
            if bias != direction {
                return None; // crossover fights the morning's established bias — skip
            }
        }
        if (direction == Direction::Long && ctx.candle.close <= vwap)
            || (direction == Direction::Short && ctx.candle.close >= vwap)
        {
            return None; // price isn't even on the crossover's side of VWAP
        }
        // Recorded below as well as gated on. Without it in `params_json`,
        // any later question about the slope threshold needs a full re-run of
        // the backtest instead of a query — which is how this was discovered.
        let mut measured_slope_pct: Option<f64> = None;
        if self.vwap_slope_lookback_bars > 0 {
            let hist = self.vwap_history.get(&ctx.instrument_token)?;
            if hist.len() <= self.vwap_slope_lookback_bars as usize {
                return None; // not enough VWAP history yet to judge slope
            }
            let past_vwap = hist[0];
            let slope_pct = (vwap - past_vwap) / past_vwap;
            measured_slope_pct = Some(slope_pct);
            let slope_ok = match direction {
                Direction::Long => slope_pct >= self.min_vwap_slope_pct,
                Direction::Short => slope_pct <= -self.min_vwap_slope_pct,
            };
            if !slope_ok {
                return None; // VWAP itself isn't trending with the trade
            }
            // Reject a move that has already gone too far. Magnitude, so the
            // same number reads the same way on both sides.
            if let Some(max) = self.max_vwap_slope_pct {
                if slope_pct.abs() > max {
                    return None;
                }
            }
        }

        Some(SignalIntent {
            strategy_id: self.id(),
            window: self.window(),
            instrument_token: ctx.instrument_token,
            ts: ctx.candle.ts,
            direction,
            entry_price_hint: ctx.candle.close,
            rationale: format!(
                "EMA{}/{} crossed {} with price {} VWAP {:.2} and {rel_vol:.2}x relative volume",
                self.fast_period,
                self.slow_period,
                if fast_above_slow { "up" } else { "down" },
                if direction == Direction::Long { "above" } else { "below" },
                vwap,
            ),
            params: serde_json::json!({
                "fast_ema": fast,
                "slow_ema": slow,
                "vwap": vwap,
                "relative_volume": rel_vol,
                "opening_range_bias": bias.map(|d| matches!(d, Direction::Long)),
                // Fraction, signed: -0.002 means VWAP fell 0.2% over the
                // lookback. `null` when the slope gate is disabled.
                "vwap_slope_pct": measured_slope_pct,
                "vwap_slope_lookback_bars": self.vwap_slope_lookback_bars,
            }),
        })
    }
}

#[cfg(test)]
mod ceiling_tests {
    use super::*;
    use chrono::{TimeZone, Utc};
    use core_types::time::IST;
    use core_types::Candle;
    use std::collections::HashMap;

    /// Builds a context with a falling VWAP history so the slope is known.
    fn short_ctx(slope_pct: f64) -> (VwapTrendFollow, StrategyContext) {
        let vwap_now = 100.0;
        let past = vwap_now / (1.0 + slope_pct);
        let mut s = VwapTrendFollow::new(9, 21, 10, 1.2, false, true, 0).with_vwap_slope(2, 0.0);
        // Seed two bars of history so hist[0] is `past`.
        s.vwap_history.entry(1).or_default().extend([past, past]);
        let mut ind = HashMap::new();
        ind.insert(IndicatorSpec::Vwap.key(), vwap_now);
        ind.insert(IndicatorSpec::Ema(9).key(), 99.0);
        ind.insert(IndicatorSpec::Ema(21).key(), 100.0);
        ind.insert(IndicatorSpec::RelativeVolume(10).key(), 2.0);
        let ctx = StrategyContext {
            instrument_token: 1,
            candle: Candle {
                ts: IST.with_ymd_and_hms(2026, 9, 29, 11, 0, 0).unwrap().with_timezone(&Utc),
                open: 99.0, high: 99.0, low: 99.0, close: 99.0, volume: 1000,
            },
            indicators: ind,
            opening_range_high: None,
            opening_range_low: None,
            window: TimeWindow::MidMorning,
        };
        (s, ctx)
    }

    /// A gentle fall is exactly what the ceiling is meant to keep — the band
    /// the data says carries the whole edge.
    #[test]
    fn a_nearly_flat_vwap_passes_the_ceiling() {
        let (mut s, ctx) = short_ctx(-0.0003);
        s = s.with_max_vwap_slope(0.0005);
        // Prime the crossover: EMA9 was above, now below.
        s.prev_fast_above_slow.insert(1, true);
        assert!(s.on_bar(&ctx).is_some(), "a -0.03% slope should pass a 0.05% ceiling");
    }

    /// The case the ceiling exists to reject: the move has already run.
    #[test]
    fn an_already_steep_fall_is_rejected() {
        let (mut s, ctx) = short_ctx(-0.0030);
        s = s.with_max_vwap_slope(0.0005);
        s.prev_fast_above_slow.insert(1, true);
        assert!(s.on_bar(&ctx).is_none(), "a -0.30% slope should fail a 0.05% ceiling");
    }

    /// Without a ceiling the same steep trade is taken — so the test above is
    /// measuring the ceiling and not some other gate.
    #[test]
    fn the_same_steep_trade_is_taken_when_no_ceiling_is_set() {
        let (mut s, ctx) = short_ctx(-0.0030);
        s.prev_fast_above_slow.insert(1, true);
        assert!(s.on_bar(&ctx).is_some(), "with no ceiling this must still signal");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};
    use core_types::time::IST;
    use core_types::Candle;
    use std::collections::HashMap;

    fn ctx_at(hh: u32, mm: u32, vwap: f64, fast: f64, slow: f64, rel_vol: f64, close: f64, bias: Option<Direction>) -> StrategyContext {
        let mut indicators = HashMap::new();
        indicators.insert(IndicatorSpec::Vwap.key(), vwap);
        indicators.insert(IndicatorSpec::Ema(9).key(), fast);
        indicators.insert(IndicatorSpec::Ema(21).key(), slow);
        indicators.insert(IndicatorSpec::RelativeVolume(10).key(), rel_vol);
        let (opening_range_high, opening_range_low) = match bias {
            Some(Direction::Long) => (Some(close - 1.0), Some(close - 5.0)),
            Some(Direction::Short) => (Some(close + 5.0), Some(close + 1.0)),
            None => (None, None),
        };
        StrategyContext {
            instrument_token: 1,
            window: TimeWindow::MidMorning,
            candle: Candle {
                ts: IST.with_ymd_and_hms(2026, 9, 26, hh, mm, 0).unwrap().with_timezone(&Utc),
                open: close,
                high: close,
                low: close,
                close,
                volume: 10_000,
            },
            indicators,
            opening_range_high,
            opening_range_low,
        }
    }

    #[test]
    fn crossover_up_fires_long() {
        let mut s = VwapTrendFollow::default();
        s.on_bar(&ctx_at(9, 46, 99.0, 99.0, 100.0, 2.0, 99.0, None));
        let sig = s.on_bar(&ctx_at(9, 47, 99.0, 101.0, 100.0, 2.0, 101.0, None)).unwrap();
        assert_eq!(sig.direction, Direction::Long);
    }

    #[test]
    fn window_defaults_to_mid_morning() {
        let s = VwapTrendFollow::default();
        assert_eq!(s.window(), TimeWindow::MidMorning);
    }

    #[test]
    fn with_window_switches_to_full_day() {
        let s = VwapTrendFollow::default().with_window(TimeWindow::FullDay);
        assert_eq!(s.window(), TimeWindow::FullDay);
    }

    #[test]
    fn with_id_overrides_reported_id() {
        let s = VwapTrendFollow::default().with_id("vwap_trend_custom");
        assert_eq!(s.id(), "vwap_trend_custom");
    }

    #[test]
    fn default_id_is_unchanged() {
        let s = VwapTrendFollow::default();
        assert_eq!(s.id(), "vwap_trend");
    }

    #[test]
    fn max_entry_time_blocks_a_crossover_at_or_after_the_cutoff() {
        let mut s = VwapTrendFollow::new(9, 21, 10, 1.2, true, true, 0)
            .with_window(TimeWindow::FullDay)
            .with_max_entry_time(chrono::NaiveTime::from_hms_opt(14, 0, 0).unwrap());
        s.on_bar(&ctx_at(13, 59, 99.0, 99.0, 100.0, 2.0, 99.0, None));
        // Crossover happens exactly at 14:00 -> blocked.
        let sig = s.on_bar(&ctx_at(14, 0, 99.0, 101.0, 100.0, 2.0, 101.0, None));
        assert!(sig.is_none());
    }

    #[test]
    fn max_entry_time_allows_a_crossover_just_before_the_cutoff() {
        let mut s = VwapTrendFollow::new(9, 21, 10, 1.2, true, true, 0)
            .with_window(TimeWindow::FullDay)
            .with_max_entry_time(chrono::NaiveTime::from_hms_opt(14, 0, 0).unwrap());
        s.on_bar(&ctx_at(13, 58, 99.0, 99.0, 100.0, 2.0, 99.0, None));
        let sig = s.on_bar(&ctx_at(13, 59, 99.0, 101.0, 100.0, 2.0, 101.0, None));
        assert!(sig.is_some());
    }

    #[test]
    fn min_entry_time_blocks_a_crossover_before_the_cutoff() {
        let mut s = VwapTrendFollow::new(9, 21, 10, 1.2, true, true, 0)
            .with_window(TimeWindow::FullDay)
            .with_min_entry_time(chrono::NaiveTime::from_hms_opt(10, 30, 0).unwrap());
        s.on_bar(&ctx_at(10, 15, 99.0, 99.0, 100.0, 2.0, 99.0, None));
        let sig = s.on_bar(&ctx_at(10, 16, 99.0, 101.0, 100.0, 2.0, 101.0, None));
        assert!(sig.is_none());
    }

    #[test]
    fn min_and_max_entry_time_combine_to_a_window() {
        let mut s = VwapTrendFollow::new(9, 21, 10, 1.2, true, true, 0)
            .with_window(TimeWindow::FullDay)
            .with_min_entry_time(chrono::NaiveTime::from_hms_opt(10, 30, 0).unwrap())
            .with_max_entry_time(chrono::NaiveTime::from_hms_opt(14, 0, 0).unwrap());
        s.on_bar(&ctx_at(11, 0, 99.0, 99.0, 100.0, 2.0, 99.0, None));
        let sig = s.on_bar(&ctx_at(11, 1, 99.0, 101.0, 100.0, 2.0, 101.0, None));
        assert!(sig.is_some());
    }

    #[test]
    fn min_relative_volume_filter_still_applies() {
        let mut s = VwapTrendFollow::default();
        s.on_bar(&ctx_at(9, 46, 99.0, 99.0, 100.0, 2.0, 99.0, None));
        let sig = s.on_bar(&ctx_at(9, 47, 99.0, 101.0, 100.0, 0.5, 101.0, None));
        assert!(sig.is_none());
    }

    #[test]
    fn vwap_side_filter_still_applies() {
        let mut s = VwapTrendFollow::default();
        s.on_bar(&ctx_at(9, 46, 102.0, 99.0, 100.0, 2.0, 99.0, None));
        // Bullish crossover but close (101) still below VWAP (102) -> blocked.
        let sig = s.on_bar(&ctx_at(9, 47, 102.0, 101.0, 100.0, 2.0, 101.0, None));
        assert!(sig.is_none());
    }

    #[test]
    fn allow_long_false_blocks_long_signal() {
        let mut s = VwapTrendFollow::new(9, 21, 10, 1.2, false, true, 0);
        s.on_bar(&ctx_at(9, 46, 99.0, 99.0, 100.0, 2.0, 99.0, None));
        let sig = s.on_bar(&ctx_at(9, 47, 99.0, 101.0, 100.0, 2.0, 101.0, None));
        assert!(sig.is_none());
    }

    #[test]
    fn vwap_slope_blocks_a_crossover_when_vwap_itself_is_flat_and_a_minimum_is_set() {
        // With an explicit min_slope_pct > 0, an exactly-flat VWAP fails it.
        let mut s = VwapTrendFollow::new(9, 21, 10, 1.2, true, true, 0).with_vwap_slope(2, 0.001);
        s.on_bar(&ctx_at(9, 45, 100.0, 99.0, 100.0, 2.0, 99.0, None));
        s.on_bar(&ctx_at(9, 46, 100.0, 99.0, 100.0, 2.0, 99.0, None));
        // Crossover on the 3rd bar, but VWAP still exactly 100.0 -> flat, not "up".
        let sig = s.on_bar(&ctx_at(9, 47, 100.0, 101.0, 100.0, 2.0, 101.0, None));
        assert!(sig.is_none());
    }

    #[test]
    fn vwap_slope_allows_a_crossover_when_vwap_is_trending_the_same_way() {
        let mut s = VwapTrendFollow::new(9, 21, 10, 1.2, true, true, 0).with_vwap_slope(2, 0.0);
        s.on_bar(&ctx_at(9, 45, 98.0, 99.0, 100.0, 2.0, 98.0, None));
        s.on_bar(&ctx_at(9, 46, 99.0, 99.0, 100.0, 2.0, 99.0, None));
        // VWAP rose 98 -> 100 over the lookback, and this is a long crossover.
        let sig = s.on_bar(&ctx_at(9, 47, 100.0, 101.0, 100.0, 2.0, 101.0, None));
        assert!(sig.is_some());
    }

    #[test]
    fn vwap_slope_blocks_a_crossover_against_the_vwap_trend() {
        let mut s = VwapTrendFollow::new(9, 21, 10, 1.2, true, true, 0).with_vwap_slope(2, 0.0);
        // VWAP is falling, but this is a long (up) crossover -> blocked.
        s.on_bar(&ctx_at(9, 45, 102.0, 99.0, 100.0, 2.0, 102.0, None));
        s.on_bar(&ctx_at(9, 46, 101.0, 99.0, 100.0, 2.0, 101.0, None));
        let sig = s.on_bar(&ctx_at(9, 47, 100.0, 101.0, 100.0, 2.0, 101.0, None));
        assert!(sig.is_none());
    }

    #[test]
    fn vwap_slope_requires_enough_history_before_it_will_fire() {
        let mut s = VwapTrendFollow::new(9, 21, 10, 1.2, true, true, 0).with_vwap_slope(5, 0.0);
        // Only 2 bars of history fed, lookback needs 5 -> blocked regardless of direction.
        s.on_bar(&ctx_at(9, 45, 98.0, 99.0, 100.0, 2.0, 98.0, None));
        let sig = s.on_bar(&ctx_at(9, 46, 100.0, 101.0, 100.0, 2.0, 101.0, None));
        assert!(sig.is_none());
    }

    #[test]
    fn vwap_slope_disabled_by_default() {
        // Flat VWAP would fail the slope check if it were on, but lookback_bars=0 means disabled.
        let mut s = VwapTrendFollow::default();
        s.on_bar(&ctx_at(9, 46, 100.0, 99.0, 100.0, 2.0, 99.0, None));
        let sig = s.on_bar(&ctx_at(9, 47, 100.0, 101.0, 100.0, 2.0, 101.0, None));
        assert!(sig.is_some());
    }
}
