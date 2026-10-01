use serde::{Deserialize, Serialize};

/// What a `Strategy` needs the `IndicatorEngine` (in `strategy-core`) to warm
/// up before calling it — lets the engine skip indicators nothing needs.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum IndicatorSpec {
    Vwap,
    Atr(u32),
    Sma(u32),
    Ema(u32),
    RelativeVolume(u32),
    OpeningRange,
}

impl IndicatorSpec {
    /// Stable key `StrategyContext.indicators` is keyed by.
    pub fn key(self) -> String {
        match self {
            IndicatorSpec::Vwap => "vwap".to_string(),
            IndicatorSpec::Atr(n) => format!("atr_{n}"),
            IndicatorSpec::Sma(n) => format!("sma_{n}"),
            IndicatorSpec::Ema(n) => format!("ema_{n}"),
            IndicatorSpec::RelativeVolume(n) => format!("relvol_{n}"),
            IndicatorSpec::OpeningRange => "opening_range".to_string(),
        }
    }
}
