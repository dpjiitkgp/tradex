//! Sleep-until-market-open daemon loop: holiday no-op, morning auth refresh, EOD shutdown, crash-restart throttling.

pub mod wake_time;

pub use wake_time::next_wake_time;
