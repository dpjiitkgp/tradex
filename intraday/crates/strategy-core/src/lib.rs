//! Shared strategy scaffolding: StrategyContext, the IndicatorEngine, and the time-window registry that routes each bar to the right strategy.

pub mod indicator_engine;
pub mod runner;
pub mod window;

pub use indicator_engine::IndicatorEngine;
pub use runner::StrategyRunner;
pub use window::window_for_time;
