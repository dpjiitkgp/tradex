use crate::time::{DateTime, TimeWindow};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    Long,
    Short,
}

/// Identifies a strategy implementation, e.g. "orb", "vwap_trend".
pub type StrategyId = String;

/// A candidate trade a `Strategy` emits for a single bar. The engine decides
/// whether to act on it (sizing, max-concurrent-positions, daily-loss cap —
/// all portfolio concerns live in `paper-engine`, not here).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalIntent {
    pub strategy_id: StrategyId,
    pub window: TimeWindow,
    pub instrument_token: u64,
    pub ts: DateTime,
    pub direction: Direction,
    pub entry_price_hint: f64,
    pub rationale: String,
    pub params: serde_json::Value,
}
