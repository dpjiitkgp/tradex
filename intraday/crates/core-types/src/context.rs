use crate::market::Candle;
use crate::time::TimeWindow;
use std::collections::HashMap;

/// Built once per bar by the `IndicatorEngine` (`strategy-core`) and handed
/// to every `Strategy` whose window covers the current time. Plain data —
/// no logic — so it can live in `core-types` alongside the `Strategy` trait
/// that consumes it without a circular dependency on `strategy-core`.
#[derive(Debug, Clone)]
pub struct StrategyContext {
    pub instrument_token: u64,
    pub window: TimeWindow,
    pub candle: Candle,
    /// Keyed by `IndicatorSpec::key()`.
    pub indicators: HashMap<String, f64>,
    pub opening_range_high: Option<f64>,
    pub opening_range_low: Option<f64>,
}
