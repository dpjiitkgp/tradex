use crate::time::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CandleInterval {
    OneMinute,
    Day,
}

impl CandleInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            CandleInterval::OneMinute => "1m",
            CandleInterval::Day => "day",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Candle {
    pub ts: DateTime,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Tick {
    pub instrument_token: u64,
    pub ts: DateTime,
    pub last_price: f64,
    pub volume_traded: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Quote {
    pub instrument_token: u64,
    pub ts: DateTime,
    pub last_price: f64,
}
