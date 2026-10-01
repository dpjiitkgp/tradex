//! Real-money live trading for the VWAP-slope short book —
//! `SimulatedBroker` (dry-run/testing) and `ExecutionPolicy` (the
//! limit-then-market-fallback stop-loss policy) so far; `LiveEngine` (the
//! single-position state machine driving both against real strategy/risk
//! logic) is next. See `research/tune30/README.md` and the plan this was
//! built from for the full design.

pub mod alert;
pub mod broker_sim;
pub mod clock;
pub mod config;
pub mod engine;
pub mod execution;
pub mod ip_whitelist;
pub mod session;

pub use alert::Alert;
pub use broker_sim::SimulatedBroker;
pub use clock::{Clock, FakeClock, RealClock};
pub use config::{StockTuning, Tune30Config};
pub use engine::{build_stock_runtimes, LiveEngine, StockRuntime};
pub use execution::{ExecutionOutcome, ExecutionPolicy, ExecutionStep, FallbackConfig, TimestampedStep};
pub use ip_whitelist::{check as check_ip, IpCheck, IpWhitelist};
pub use session::loop_end;
