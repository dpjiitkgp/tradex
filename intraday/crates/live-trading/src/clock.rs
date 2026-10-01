//! A `Clock` abstraction so `ExecutionPolicy`'s timeout/poll loop can be
//! unit-tested deterministically (a `FakeClock` that advances instantly on
//! `sleep`) instead of every test actually waiting on real wall-clock time.

use async_trait::async_trait;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

#[async_trait]
pub trait Clock: Send + Sync {
    /// Milliseconds since some arbitrary epoch — only ever compared to
    /// itself, never to wall-clock time.
    fn now_millis(&self) -> u64;
    async fn sleep(&self, d: Duration);
}

/// `now_millis()` must measure elapsed time since a *fixed* reference
/// point, not since "now" — `Instant::now().elapsed()` computed fresh on
/// every call is a bug that looks plausible but always returns ~0 (you're
/// measuring elapsed time since an instant you just created), which made
/// `ExecutionPolicy`'s timeout check (`elapsed >= timeout`) never trip:
/// the stop-loss fallback's poll loop ran forever and the market-order
/// safety net never fired. Caught via `live-trade test-signal` before
/// this ever reached `--live`. `epoch` is fixed at construction so every
/// call measures from the same starting point.
pub struct RealClock {
    epoch: std::time::Instant,
}

impl RealClock {
    pub fn new() -> Self {
        Self {
            epoch: std::time::Instant::now(),
        }
    }
}

impl Default for RealClock {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Clock for RealClock {
    fn now_millis(&self) -> u64 {
        self.epoch.elapsed().as_millis() as u64
    }

    async fn sleep(&self, d: Duration) {
        tokio::time::sleep(d).await;
    }
}

/// Advances instantly on `sleep` (no real waiting) so timeout/poll-loop
/// tests run in microseconds instead of real seconds.
#[derive(Default)]
pub struct FakeClock {
    millis: AtomicU64,
}

impl FakeClock {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl Clock for FakeClock {
    fn now_millis(&self) -> u64 {
        self.millis.load(Ordering::SeqCst)
    }

    async fn sleep(&self, d: Duration) {
        self.millis.fetch_add(d.as_millis() as u64, Ordering::SeqCst);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Regression test for the exact bug that made it to `live-trade
    /// test-signal` before `--live`: `now_millis()` must measure elapsed
    /// time since a fixed point, not since "now" (which is always ~0). If
    /// this fails, `ExecutionPolicy`'s stop-loss fallback timeout silently
    /// never fires again.
    #[tokio::test]
    async fn real_clock_now_millis_actually_advances_over_a_real_wait() {
        let clock = RealClock::new();
        let t0 = clock.now_millis();
        clock.sleep(Duration::from_millis(50)).await;
        let t1 = clock.now_millis();
        assert!(
            t1 >= t0 + 40,
            "now_millis() should reflect real elapsed time (t0={t0}, t1={t1}) — if this is ~0, \
             now_millis() is measuring from a freshly-created instant again, not a fixed epoch"
        );
    }

    #[tokio::test]
    async fn real_clock_two_separate_instances_each_track_their_own_elapsed_time_correctly() {
        // Guards against a fix that accidentally shares a single global
        // epoch in a way that breaks per-instance semantics.
        let a = RealClock::new();
        tokio::time::sleep(Duration::from_millis(30)).await;
        let b = RealClock::new();
        assert!(a.now_millis() >= 25);
        assert!(b.now_millis() < 15, "a freshly constructed clock should start near zero");
    }
}
