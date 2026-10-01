//! Domain types and trait contracts (MarketDataFeed, Strategy, ExitManager, Storage, TradingCalendar, LoginProvider) that every other crate implements or depends on.

pub mod broker;
pub mod context;
pub mod error;
pub mod indicator;
pub mod instrument;
pub mod market;
pub mod risk;
pub mod signal;
pub mod storage;
pub mod time;
pub mod traits;

pub use broker::{
    Broker, BrokerPosition, MarginCheckResult, OrderRequest, OrderResult, OrderSide, OrderStatus,
    OrderType, Product,
};
pub use context::StrategyContext;
pub use error::{Error, Result};
pub use indicator::IndicatorSpec;
pub use instrument::{Exchange, Instrument};
pub use market::{Candle, CandleInterval, Quote, Tick};
pub use risk::{
    EntryContext, ExitDecision, ExitPlan, ExitReason, MarketSnapshot, PositionPhase,
    PositionState, StopMove, TrailPolicy,
};
pub use signal::{Direction, SignalIntent, StrategyId};
pub use storage::{
    BrokerOrderAction, BrokerOrderEvent, NewRun, NewTrade, RunId, RunMode, SignalId, SignalRecord,
    Storage, TradeExit, TradeId,
};
pub use time::{NaiveDate, TimeWindow, DateTime, IST};
pub use traits::{AccessToken, ExitManager, LoginProvider, MarketDataFeed, Strategy, TradingCalendar};
