//! The shared orchestrator that drives strategies against a MarketDataFeed and simulates fills - identical code path for backtest replay and the live daemon.

pub mod config;
pub mod config_loader;
pub mod engine;
pub mod live_aggregator;
mod pnl;
mod position;

pub use config::EngineConfig;
pub use engine::PaperEngine;
pub use live_aggregator::CandleAggregator;
