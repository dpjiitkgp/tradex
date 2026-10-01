use chrono::NaiveTime;
use core_types::{Candle, IndicatorSpec, StrategyContext, TimeWindow};
use indicators::{Atr, Ema, OpeningRange, RelativeVolume, Sma, Vwap};
use std::collections::HashMap;

enum IndicatorState {
    Vwap(Vwap),
    Atr(Atr),
    Sma(Sma),
    Ema(Ema),
    RelativeVolume(RelativeVolume),
    OpeningRange(OpeningRange),
}

impl IndicatorState {
    fn from_spec(spec: IndicatorSpec, opening_range_formation_end: NaiveTime) -> Self {
        match spec {
            IndicatorSpec::Vwap => IndicatorState::Vwap(Vwap::new()),
            IndicatorSpec::Atr(n) => IndicatorState::Atr(Atr::new(n as usize)),
            IndicatorSpec::Sma(n) => IndicatorState::Sma(Sma::new(n as usize)),
            IndicatorSpec::Ema(n) => IndicatorState::Ema(Ema::new(n as usize)),
            IndicatorSpec::RelativeVolume(n) => {
                IndicatorState::RelativeVolume(RelativeVolume::new(n as usize))
            }
            IndicatorSpec::OpeningRange => {
                IndicatorState::OpeningRange(OpeningRange::new(opening_range_formation_end))
            }
        }
    }
}

/// Warms up and updates exactly the indicators any registered `Strategy`
/// actually asked for (via `required_indicators()`), per instrument, and
/// assembles the `StrategyContext` each bar. The identical engine drives
/// both backtest replay and the live daemon — same code path, per the plan.
pub struct IndicatorEngine {
    opening_range_formation_end: NaiveTime,
    per_instrument: HashMap<u64, HashMap<String, (IndicatorSpec, IndicatorState)>>,
}

impl IndicatorEngine {
    pub fn new(opening_range_formation_end: NaiveTime) -> Self {
        Self {
            opening_range_formation_end,
            per_instrument: HashMap::new(),
        }
    }

    /// Ensures indicator state exists for `instrument_token` covering every
    /// spec in `specs` — a no-op for specs already registered.
    pub fn ensure_registered(&mut self, instrument_token: u64, specs: &[IndicatorSpec]) {
        let states = self.per_instrument.entry(instrument_token).or_default();
        for spec in specs {
            states.entry(spec.key()).or_insert_with(|| {
                (*spec, IndicatorState::from_spec(*spec, self.opening_range_formation_end))
            });
        }
    }

    /// Clears all indicator state for every instrument — call at the start
    /// of each trading session so nothing carries over an overnight gap.
    pub fn reset_session(&mut self) {
        for states in self.per_instrument.values_mut() {
            for (spec, state) in states.values_mut() {
                *state = IndicatorState::from_spec(*spec, self.opening_range_formation_end);
            }
        }
    }

    pub fn on_bar(&mut self, instrument_token: u64, window: TimeWindow, candle: &Candle) -> StrategyContext {
        let mut ctx = StrategyContext {
            instrument_token,
            window,
            candle: *candle,
            indicators: HashMap::new(),
            opening_range_high: None,
            opening_range_low: None,
        };

        let Some(states) = self.per_instrument.get_mut(&instrument_token) else {
            return ctx;
        };

        for (key, (_, state)) in states.iter_mut() {
            match state {
                IndicatorState::Vwap(v) => {
                    if let Some(val) = v.update(candle) {
                        ctx.indicators.insert(key.clone(), val);
                    }
                }
                IndicatorState::Atr(a) => {
                    if let Some(val) = a.update(candle) {
                        ctx.indicators.insert(key.clone(), val);
                    }
                }
                IndicatorState::Sma(s) => {
                    if let Some(val) = s.update(candle.close) {
                        ctx.indicators.insert(key.clone(), val);
                    }
                }
                IndicatorState::Ema(e) => {
                    if let Some(val) = e.update(candle.close) {
                        ctx.indicators.insert(key.clone(), val);
                    }
                }
                IndicatorState::RelativeVolume(r) => {
                    if let Some(val) = r.update(candle.volume) {
                        ctx.indicators.insert(key.clone(), val);
                    }
                }
                IndicatorState::OpeningRange(o) => {
                    o.update(candle.ts, candle.high, candle.low);
                    if let Some((h, l)) = o.range() {
                        ctx.opening_range_high = Some(h);
                        ctx.opening_range_low = Some(l);
                    }
                }
            }
        }

        ctx
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};
    use core_types::time::IST;

    fn candle(ts: core_types::DateTime, close: f64, volume: u64) -> Candle {
        Candle {
            ts,
            open: close,
            high: close,
            low: close,
            close,
            volume,
        }
    }

    fn ist_ts(h: u32, m: u32) -> core_types::DateTime {
        IST.with_ymd_and_hms(2026, 9, 22, h, m, 0).unwrap().with_timezone(&Utc)
    }

    #[test]
    fn only_registered_specs_produce_values() {
        let mut engine = IndicatorEngine::new(NaiveTime::from_hms_opt(9, 30, 0).unwrap());
        engine.ensure_registered(1, &[IndicatorSpec::Sma(2)]);
        let ctx = engine.on_bar(1, TimeWindow::MidMorning, &candle(ist_ts(10, 0), 10.0, 100));
        assert!(ctx.indicators.is_empty()); // not warmed up yet, and only sma_2 was registered
        let ctx = engine.on_bar(1, TimeWindow::MidMorning, &candle(ist_ts(10, 1), 20.0, 100));
        assert_eq!(ctx.indicators.get("sma_2"), Some(&15.0));
        assert!(ctx.indicators.get("vwap").is_none()); // never registered
    }

    #[test]
    fn reset_session_clears_accumulated_state() {
        let mut engine = IndicatorEngine::new(NaiveTime::from_hms_opt(9, 30, 0).unwrap());
        engine.ensure_registered(1, &[IndicatorSpec::Vwap]);
        engine.on_bar(1, TimeWindow::Opening, &candle(ist_ts(9, 15), 10.0, 100));
        engine.reset_session();
        let ctx = engine.on_bar(1, TimeWindow::Opening, &candle(ist_ts(9, 15), 50.0, 10));
        assert_eq!(ctx.indicators.get("vwap"), Some(&50.0));
    }

    #[test]
    fn opening_range_populates_context_fields_not_the_indicator_map() {
        let mut engine = IndicatorEngine::new(NaiveTime::from_hms_opt(9, 30, 0).unwrap());
        engine.ensure_registered(1, &[IndicatorSpec::OpeningRange]);
        engine.on_bar(1, TimeWindow::Opening, &candle(ist_ts(9, 15), 100.0, 10));
        let ctx = engine.on_bar(1, TimeWindow::MidMorning, &candle(ist_ts(9, 45), 105.0, 10));
        assert_eq!(ctx.opening_range_high, Some(100.0));
        assert_eq!(ctx.opening_range_low, Some(100.0));
        assert!(ctx.indicators.get("opening_range").is_none());
    }
}
