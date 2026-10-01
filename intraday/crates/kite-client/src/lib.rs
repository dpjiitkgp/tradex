//! Kite Connect REST (instruments, historical candles) and WebSocket ticker client - the live/historical MarketDataFeed implementation.

pub mod egress;
pub mod feed;
pub mod historical;
pub mod instruments;
pub mod orders;
pub mod quote;
pub mod rest;
pub mod ticker;
pub mod ticker_decode;

pub use feed::KiteMarketDataFeed;
pub use orders::KiteBroker;
pub use rest::KiteRestClient;
