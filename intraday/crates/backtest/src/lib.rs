//! Historical replay of paper-engine over stored candles, plus the reporting queries used to compare strategies before going live.

pub mod cache_only_feed;
pub mod job;
pub mod replay;

pub use cache_only_feed::CacheOnlyFeed;
pub use job::{run_backtest_job, BacktestJobParams, JobProgress};
pub use replay::{merge_chronological, run, ReplayBar};
