//! The three time-windowed Strategy implementations: opening-range breakout, VWAP/trend-following, and momentum/breakout scanning.

pub mod config;
pub mod ma_alignment;
pub mod momentum_breakout;
pub mod orb;
pub mod random_entry;
pub mod strong_rally;
pub mod trend_confluence;
pub mod trend_pullback;
pub mod vwap_reversion;
pub mod vwap_trend;

pub use config::StrategiesConfig;
pub use ma_alignment::MaAlignment;
pub use momentum_breakout::MomentumBreakout;
pub use orb::OpeningRangeBreakout;
pub use random_entry::RandomEntry;
pub use strong_rally::StrongRally;
pub use trend_confluence::TrendConfluence;
pub use trend_pullback::TrendPullback;
pub use vwap_reversion::VwapReversion;
pub use vwap_trend::VwapTrendFollow;
