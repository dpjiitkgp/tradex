//! Wraps a `Broker` and owns the fill-behavior *policy* on top of it —
//! separate from the raw broker wrapper so it's unit-testable against a
//! fake broker and a fake clock, with no network or database involved.
//!
//! Two policies, matching the user's explicit choice: entries and
//! profit-target exits are a **plain limit order**, no fallback — if it
//! doesn't fill, it doesn't fill, and the caller decides what to do (retry
//! next bar, give up). A **stop-loss exit** additionally gets a
//! market-order fallback: a plain limit order can fail to protect against
//! a gap (it just sits unfilled while the loss grows), so if it hasn't
//! filled within a short window, cancel it and fire a market order for
//! whatever quantity remains unfilled — slightly worse fill in a bad gap,
//! but the stop always actually happens.

use crate::clock::Clock;
use core_types::{Broker, Error, OrderRequest, OrderResult, OrderStatus, OrderType, Result};
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct FallbackConfig {
    /// How long to wait for the limit order to fill before falling back to
    /// market.
    pub timeout: Duration,
    /// How often to check the limit order's status while waiting.
    pub poll_interval: Duration,
}

impl Default for FallbackConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(10),
            poll_interval: Duration::from_secs(2),
        }
    }
}

/// One step of what actually happened — the full audit trail, not just the
/// final result. The caller (`LiveEngine`) turns each of these into a
/// `BrokerOrderEvent` row; `ExecutionPolicy` itself never touches storage,
/// so it stays testable without a database.
#[derive(Debug, Clone)]
pub enum ExecutionStep {
    Placed(OrderResult),
    Polled(OrderResult),
    Cancelled { broker_order_id: String },
    FallbackPlaced(OrderResult),
}

/// A step plus *when it actually happened* — captured the instant each
/// broker call returns, not reconstructed later from whenever the caller
/// gets around to logging the whole outcome. Getting this wrong once
/// already made an audit trail claim five polls happened within
/// milliseconds of each other when they were really ~2s apart.
#[derive(Debug, Clone)]
pub struct TimestampedStep {
    pub ts: chrono::DateTime<chrono::Utc>,
    pub step: ExecutionStep,
}

#[derive(Debug, Clone)]
pub struct ExecutionOutcome {
    pub steps: Vec<TimestampedStep>,
    pub final_result: OrderResult,
    pub used_fallback: bool,
}

impl ExecutionOutcome {
    pub fn fully_filled(&self) -> bool {
        self.final_result.status == OrderStatus::Complete
    }
}

pub struct ExecutionPolicy<B: Broker> {
    broker: Arc<B>,
    clock: Arc<dyn Clock>,
}

fn stamp(step: ExecutionStep) -> TimestampedStep {
    TimestampedStep {
        ts: chrono::Utc::now(),
        step,
    }
}

impl<B: Broker> ExecutionPolicy<B> {
    pub fn new(broker: Arc<B>, clock: Arc<dyn Clock>) -> Self {
        Self { broker, clock }
    }

    /// Entries and profit-target exits: place the limit order, return
    /// whatever the broker says immediately. No waiting, no fallback.
    pub async fn place_plain(&self, req: &OrderRequest) -> Result<ExecutionOutcome> {
        let result = self.broker.place_order(req).await?;
        Ok(ExecutionOutcome {
            steps: vec![stamp(ExecutionStep::Placed(result.clone()))],
            final_result: result,
            used_fallback: false,
        })
    }

    /// Stop-loss exits: place an SL (stop-limit — trigger price *and*
    /// limit price, dormant until the trigger, per Kite's native order
    /// type) order, poll until it fills or `cfg.timeout` elapses, then
    /// cancel and fall back to a market order for whatever quantity is
    /// still unfilled. Deliberately *not* a plain `Limit` order: a bare
    /// limit has no trigger, so it would sit active immediately rather
    /// than waiting for price to reach the stop.
    pub async fn place_with_stop_fallback(
        &self,
        req: &OrderRequest,
        cfg: &FallbackConfig,
    ) -> Result<ExecutionOutcome> {
        if req.order_type != OrderType::SlLimit {
            return Err(Error::Broker(
                "place_with_stop_fallback: req must be an SlLimit order (trigger + limit) — a \
                 plain Limit has no trigger and would activate immediately, not wait for the stop"
                    .to_string(),
            ));
        }
        if req.trigger_price.is_none() || req.price.is_none() {
            return Err(Error::Broker(
                "place_with_stop_fallback: SlLimit requires both trigger_price and price".to_string(),
            ));
        }

        let placed = self.broker.place_order(req).await?;
        let mut steps = vec![stamp(ExecutionStep::Placed(placed.clone()))];
        let mut current = placed;

        if current.status == OrderStatus::Rejected {
            // Nothing to wait on — a rejected limit order needs the same
            // fallback treatment as a timed-out one, immediately.
            return self.fall_back_to_market(req, &current, &mut steps).await;
        }

        let start = self.clock.now_millis();
        loop {
            if current.status == OrderStatus::Complete {
                return Ok(ExecutionOutcome {
                    steps,
                    final_result: current,
                    used_fallback: false,
                });
            }
            let elapsed = self.clock.now_millis().saturating_sub(start);
            if elapsed >= cfg.timeout.as_millis() as u64 {
                break;
            }
            self.clock.sleep(cfg.poll_interval).await;
            current = self.broker.order_status(&current.broker_order_id).await?;
            steps.push(stamp(ExecutionStep::Polled(current.clone())));
        }

        // Timed out still not (fully) filled — cancel and fall back.
        if current.status == OrderStatus::Open {
            self.broker.cancel_order(&current.broker_order_id).await?;
            steps.push(stamp(ExecutionStep::Cancelled {
                broker_order_id: current.broker_order_id.clone(),
            }));
        }
        self.fall_back_to_market(req, &current, &mut steps).await
    }

    async fn fall_back_to_market(
        &self,
        original_req: &OrderRequest,
        limit_result: &OrderResult,
        steps: &mut Vec<TimestampedStep>,
    ) -> Result<ExecutionOutcome> {
        let remaining_qty = original_req.quantity.saturating_sub(limit_result.filled_quantity);
        if remaining_qty == 0 {
            // Filled in full right before the cancel raced it through —
            // nothing left to fall back for.
            return Ok(ExecutionOutcome {
                steps: steps.clone(),
                final_result: limit_result.clone(),
                used_fallback: false,
            });
        }

        let market_req = OrderRequest {
            order_type: OrderType::Market,
            price: None,
            trigger_price: None,
            quantity: remaining_qty,
            tag: format!("{}-fallback", original_req.tag),
            ..original_req.clone()
        };
        let fallback_result = self.broker.place_order(&market_req).await?;
        steps.push(stamp(ExecutionStep::FallbackPlaced(fallback_result.clone())));

        // Combine the partial limit fill (if any) with the fallback fill
        // into one logical result — a weighted-average price across both
        // legs, total filled quantity across both.
        let total_filled = limit_result.filled_quantity + fallback_result.filled_quantity;
        let average_price = match (limit_result.average_price, fallback_result.average_price) {
            (Some(lp), Some(fp)) if total_filled > 0 => Some(
                (lp * limit_result.filled_quantity as f64 + fp * fallback_result.filled_quantity as f64)
                    / total_filled as f64,
            ),
            (Some(lp), None) => Some(lp),
            (None, Some(fp)) => Some(fp),
            _ => None,
        };
        let combined = OrderResult {
            broker_order_id: fallback_result.broker_order_id.clone(),
            status: if total_filled == original_req.quantity {
                OrderStatus::Complete
            } else {
                fallback_result.status.clone()
            },
            filled_quantity: total_filled,
            average_price,
            rejection_reason: fallback_result.rejection_reason.clone(),
            raw_response_json: fallback_result.raw_response_json.clone(),
        };
        Ok(ExecutionOutcome {
            steps: steps.clone(),
            final_result: combined,
            used_fallback: true,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::broker_sim::SimulatedBroker;
    use crate::clock::FakeClock;
    use core_types::{Exchange, OrderSide, Product};

    /// A stop-loss exit for a LONG position that entered at 100: stop
    /// trigger at 95, limit price 94.90 (slightly below trigger, the usual
    /// convention for a sell-side SL so it's likely to fill once
    /// triggered, but not guaranteed to — that gap is exactly what the
    /// market-order fallback covers).
    fn sample_stop_req(qty: u32) -> OrderRequest {
        OrderRequest {
            tradingsymbol: "TESTSTOCK".to_string(),
            exchange: Exchange::Nse,
            side: OrderSide::Sell,
            quantity: qty,
            product: Product::Mis,
            order_type: OrderType::SlLimit,
            price: Some(94.90),
            trigger_price: Some(95.0),
            tag: "test".to_string(),
        }
    }

    fn policy(broker: Arc<SimulatedBroker>) -> ExecutionPolicy<SimulatedBroker> {
        ExecutionPolicy::new(broker, Arc::new(FakeClock::new()))
    }

    #[tokio::test]
    async fn does_not_fill_before_the_trigger_is_reached() {
        let broker = Arc::new(SimulatedBroker::new());
        broker.update_price("TESTSTOCK", 100.0); // above the 95 trigger — dormant
        let p = policy(broker);
        let cfg = FallbackConfig {
            timeout: Duration::from_millis(50),
            poll_interval: Duration::from_millis(10),
        };
        let outcome = p.place_with_stop_fallback(&sample_stop_req(10), &cfg).await.unwrap();
        // Never triggers, times out, falls back to market at the
        // still-100.0 price — proving it did NOT fill early at trigger
        // placement time.
        assert!(outcome.used_fallback);
        assert_eq!(outcome.final_result.average_price, Some(100.0));
    }

    #[tokio::test]
    async fn fills_at_the_limit_once_triggered_without_a_gap() {
        let broker = Arc::new(SimulatedBroker::new());
        // Below the 95 trigger (so triggered) and above the 94.90 limit
        // (so also fillable) — the clean, no-gap case.
        broker.update_price("TESTSTOCK", 94.95);
        let p = policy(broker);
        let outcome = p
            .place_with_stop_fallback(&sample_stop_req(10), &FallbackConfig::default())
            .await
            .unwrap();
        assert!(outcome.fully_filled());
        assert!(!outcome.used_fallback);
        assert_eq!(outcome.steps.len(), 1);
        assert_eq!(outcome.final_result.average_price, Some(94.95));
    }

    #[tokio::test]
    async fn gap_through_both_trigger_and_limit_times_out_and_falls_back() {
        let broker = Arc::new(SimulatedBroker::new());
        // A hard gap straight through the limit too (not just the
        // trigger) — the SL-limit order stays unfilled exactly as it
        // would on a real gap, and only the market fallback rescues it.
        broker.update_price("TESTSTOCK", 90.0);
        let p = policy(broker.clone());
        let cfg = FallbackConfig {
            timeout: Duration::from_millis(200),
            poll_interval: Duration::from_millis(50),
        };
        let outcome = p.place_with_stop_fallback(&sample_stop_req(10), &cfg).await.unwrap();
        assert!(outcome.used_fallback);
        assert!(outcome.fully_filled());
        assert_eq!(outcome.final_result.average_price, Some(90.0));
        assert!(matches!(outcome.steps.last().map(|s| &s.step), Some(ExecutionStep::FallbackPlaced(_))));
        assert!(outcome.steps.iter().any(|s| matches!(s.step, ExecutionStep::Cancelled { .. })));
    }

    #[tokio::test]
    async fn rejects_plain_limit_request_missing_trigger() {
        let broker = Arc::new(SimulatedBroker::new());
        let p = policy(broker);
        let mut req = sample_stop_req(10);
        req.order_type = OrderType::Limit; // no trigger semantics — must be rejected
        let cfg = FallbackConfig::default();
        assert!(p.place_with_stop_fallback(&req, &cfg).await.is_err());
    }

    #[tokio::test]
    async fn plain_entry_never_falls_back_even_if_unfilled() {
        let broker = Arc::new(SimulatedBroker::new());
        broker.update_price("TESTSTOCK", 200.0); // unmarketable at a 99 buy limit
        let p = policy(broker);
        let mut req = sample_stop_req(10);
        req.order_type = OrderType::Limit;
        req.side = OrderSide::Buy;
        req.price = Some(99.0);
        req.trigger_price = None;
        let outcome = p.place_plain(&req).await.unwrap();
        assert!(!outcome.fully_filled());
        assert!(!outcome.used_fallback);
        assert_eq!(outcome.steps.len(), 1);
    }
}
