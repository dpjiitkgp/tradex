//! Nifty 50 seed list plus a liquidity/volatility screener that derives the curated intraday stock universe from historical data.

pub mod config;
pub mod screen;
pub mod seed;

pub use config::UniverseConfig;
pub use screen::{screen as run_screen, ScreenResult};
pub use seed::SeedIndex;
