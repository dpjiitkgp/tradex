//! A `Broker` implementation with no real broker behind it — used for
//! dry-run (real strategy/risk logic + real live prices, but no real order
//! ever reaches Kite) and for testing `ExecutionPolicy` without a network.
//!
//! Fill behavior aims to be realistic enough to actually exercise
//! `ExecutionPolicy`'s timeout/fallback path in dry-run, not just always
//! fill instantly: a LIMIT order fills only when the *current known price*
//! (fed in externally via `update_price` — this broker has no market-data
//! access of its own) is at least as good as the limit price; otherwise it
//! sits `Open` until a later `order_status` poll re-checks against a
//! possibly-updated price. A MARKET order always fills at the current
//! known price.

use async_trait::async_trait;
use core_types::{
    Broker, BrokerPosition, Error, MarginCheckResult, OrderRequest, OrderResult, OrderSide,
    OrderStatus, OrderType, Result,
};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

struct SimOrder {
    request: OrderRequest,
    status: OrderStatus,
    filled_quantity: u32,
    average_price: Option<f64>,
}

pub struct SimulatedBroker {
    next_id: AtomicU64,
    orders: Mutex<HashMap<String, SimOrder>>,
    /// Last known price per tradingsymbol — the dry-run driver (whatever
    /// feeds `LiveEngine` its bars) calls `update_price` once per bar for
    /// the instrument being watched; this broker never fetches prices
    /// itself.
    prices: Mutex<HashMap<String, f64>>,
    positions: Mutex<HashMap<String, BrokerPosition>>,
    /// See `fail_all`.
    fail_all: std::sync::atomic::AtomicBool,
}

impl Default for SimulatedBroker {
    fn default() -> Self {
        Self::new()
    }
}

impl SimulatedBroker {
    /// Makes every call fail, standing in for an invalidated access token or a
    /// dead network. Needed because the most dangerous paths in `LiveEngine`
    /// are the ones that run *when the broker cannot be reached* — those
    /// cannot be exercised at all against a broker that always answers.
    pub fn fail_all(&self, fail: bool) {
        self.fail_all.store(fail, std::sync::atomic::Ordering::SeqCst);
    }

    fn check_reachable(&self) -> Result<()> {
        if self.fail_all.load(std::sync::atomic::Ordering::SeqCst) {
            return Err(Error::Broker(
                "Invalid `api_key` or `access_token`. (simulated)".to_string(),
            ));
        }
        Ok(())
    }

    pub fn new() -> Self {
        Self {
            next_id: AtomicU64::new(1),
            orders: Mutex::new(HashMap::new()),
            prices: Mutex::new(HashMap::new()),
            positions: Mutex::new(HashMap::new()),
            fail_all: std::sync::atomic::AtomicBool::new(false),
        }
    }

    pub fn update_price(&self, tradingsymbol: &str, price: f64) {
        self.prices.lock().unwrap().insert(tradingsymbol.to_string(), price);
    }

    fn current_price(&self, tradingsymbol: &str) -> Option<f64> {
        self.prices.lock().unwrap().get(tradingsymbol).copied()
    }

    /// Would this request fill *right now* against `current_price`, and at
    /// what price? `None` means it stays pending.
    fn marketable_price(req: &OrderRequest, current_price: f64) -> Option<f64> {
        match req.order_type {
            OrderType::Market => Some(current_price),
            OrderType::Limit => {
                let limit = req.price?;
                match req.side {
                    // Willing to pay up to `limit` — fills if the market
                    // is at or below that.
                    OrderSide::Buy if current_price <= limit => Some(current_price.max(limit.min(current_price))),
                    OrderSide::Sell if current_price >= limit => Some(current_price),
                    _ => None,
                }
            }
            OrderType::SlLimit => {
                // Dormant until `trigger_price`, then behaves like a plain
                // limit at `price` — critically, a hard gap *past* the
                // limit too (not just past the trigger) leaves it
                // unfilled, exactly the scenario the market-order fallback
                // in `ExecutionPolicy` exists to cover. Treating a trigger
                // hit as an unconditional fill (as an earlier version of
                // this did) would make that fallback path untestable.
                let trigger = req.trigger_price?;
                let limit = req.price?;
                let triggered = match req.side {
                    OrderSide::Buy => current_price >= trigger,
                    OrderSide::Sell => current_price <= trigger,
                };
                if !triggered {
                    return None;
                }
                match req.side {
                    OrderSide::Buy if current_price <= limit => Some(current_price),
                    OrderSide::Sell if current_price >= limit => Some(current_price),
                    _ => None,
                }
            }
            OrderType::SlMarket => {
                let trigger = req.trigger_price?;
                let triggered = match req.side {
                    OrderSide::Buy => current_price >= trigger,
                    OrderSide::Sell => current_price <= trigger,
                };
                triggered.then_some(current_price)
            }
        }
    }

    fn apply_fill(&self, req: &OrderRequest, fill_price: f64) {
        let mut positions = self.positions.lock().unwrap();
        let signed_qty = match req.side {
            OrderSide::Buy => req.quantity as i64,
            OrderSide::Sell => -(req.quantity as i64),
        };
        let entry = positions.entry(req.tradingsymbol.clone()).or_insert(BrokerPosition {
            tradingsymbol: req.tradingsymbol.clone(),
            net_quantity: 0,
            average_price: fill_price,
        });
        // Simplified average-price update — adequate for dry-run
        // reconciliation exercise; a flat-then-reverse sequence (not
        // expected under the single-position-at-a-time design) would not
        // be modeled with full accounting precision.
        let new_qty = entry.net_quantity + signed_qty;
        if entry.net_quantity == 0 || entry.net_quantity.signum() == signed_qty.signum() {
            let total_cost = entry.average_price * entry.net_quantity.unsigned_abs() as f64
                + fill_price * signed_qty.unsigned_abs() as f64;
            entry.average_price = if new_qty != 0 {
                total_cost / new_qty.unsigned_abs() as f64
            } else {
                0.0
            };
        }
        entry.net_quantity = new_qty;
        if entry.net_quantity == 0 {
            positions.remove(&req.tradingsymbol);
        }
    }
}

#[async_trait]
impl Broker for SimulatedBroker {
    async fn place_order(&self, req: &OrderRequest) -> Result<OrderResult> {
        self.check_reachable()?;
        let id = self.next_id.fetch_add(1, Ordering::SeqCst).to_string();
        let current = self.current_price(&req.tradingsymbol);

        let (status, filled_quantity, average_price) = match current.and_then(|p| Self::marketable_price(req, p)) {
            Some(fill_price) => {
                self.apply_fill(req, fill_price);
                (OrderStatus::Complete, req.quantity, Some(fill_price))
            }
            None if current.is_none() && req.order_type == OrderType::Market => {
                return Ok(OrderResult {
                    broker_order_id: id,
                    status: OrderStatus::Rejected,
                    filled_quantity: 0,
                    average_price: None,
                    rejection_reason: Some(format!(
                        "SimulatedBroker has no price fed in yet for {}",
                        req.tradingsymbol
                    )),
                    raw_response_json: "{}".to_string(),
                });
            }
            None => (OrderStatus::Open, 0, None),
        };

        self.orders.lock().unwrap().insert(
            id.clone(),
            SimOrder {
                request: req.clone(),
                status: status.clone(),
                filled_quantity,
                average_price,
            },
        );

        Ok(OrderResult {
            broker_order_id: id,
            status,
            filled_quantity,
            average_price,
            rejection_reason: None,
            raw_response_json: "{}".to_string(),
        })
    }

    async fn modify_order(
        &self,
        broker_order_id: &str,
        new_price: Option<f64>,
        new_trigger_price: Option<f64>,
        new_quantity: Option<u32>,
    ) -> Result<OrderResult> {
        self.check_reachable()?;
        let mut orders = self.orders.lock().unwrap();
        let order = orders
            .get_mut(broker_order_id)
            .ok_or_else(|| Error::Broker(format!("modify_order: unknown order {broker_order_id}")))?;
        if order.status != OrderStatus::Open {
            return Err(Error::Broker(format!(
                "modify_order: order {broker_order_id} is not modifiable (status {:?})",
                order.status
            )));
        }
        if let Some(p) = new_price {
            order.request.price = Some(p);
        }
        if let Some(t) = new_trigger_price {
            order.request.trigger_price = Some(t);
        }
        if let Some(q) = new_quantity {
            order.request.quantity = q;
        }
        let req = order.request.clone();
        drop(orders);
        // A more aggressive re-price can make it immediately marketable —
        // re-evaluate rather than leave it pending until the next poll.
        self.reevaluate(broker_order_id, &req)
    }

    async fn cancel_order(&self, broker_order_id: &str) -> Result<()> {
        self.check_reachable()?;
        let mut orders = self.orders.lock().unwrap();
        let Some(order) = orders.get_mut(broker_order_id) else {
            return Ok(()); // already gone — tolerate, matches KiteBroker's race handling
        };
        if order.status == OrderStatus::Open {
            order.status = OrderStatus::Cancelled;
        }
        Ok(())
    }

    async fn order_status(&self, broker_order_id: &str) -> Result<OrderResult> {
        self.check_reachable()?;
        let req = {
            let orders = self.orders.lock().unwrap();
            let order = orders
                .get(broker_order_id)
                .ok_or_else(|| Error::Broker(format!("order_status: unknown order {broker_order_id}")))?;
            if order.status != OrderStatus::Open {
                return Ok(OrderResult {
                    broker_order_id: broker_order_id.to_string(),
                    status: order.status.clone(),
                    filled_quantity: order.filled_quantity,
                    average_price: order.average_price,
                    rejection_reason: None,
                    raw_response_json: "{}".to_string(),
                });
            }
            order.request.clone()
        };
        self.reevaluate(broker_order_id, &req)
    }

    async fn check_margin(&self, req: &OrderRequest) -> Result<MarginCheckResult> {
        self.check_reachable()?;
        // Dry-run only — never gates a real trade. A flat, conservative
        // (no-leverage) estimate is enough to exercise the code path.
        let price = req.price.or_else(|| self.current_price(&req.tradingsymbol)).unwrap_or(0.0);
        Ok(MarginCheckResult {
            ok: true,
            required_margin: Some(price * req.quantity as f64),
            available_margin: None,
            rejection_reason: None,
            raw_response_json: "{}".to_string(),
        })
    }

    async fn positions(&self) -> Result<Vec<BrokerPosition>> {
        self.check_reachable()?;
        Ok(self.positions.lock().unwrap().values().cloned().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core_types::{Exchange, Product};

    fn stop_req() -> OrderRequest {
        OrderRequest {
            tradingsymbol: "TESTSTOCK".to_string(),
            exchange: Exchange::Nse,
            side: OrderSide::Sell,
            quantity: 10,
            product: Product::Mis,
            order_type: OrderType::SlLimit,
            price: Some(94.90),
            trigger_price: Some(95.0),
            tag: "test".to_string(),
        }
    }

    #[tokio::test]
    async fn stays_open_until_a_later_poll_finds_it_marketable() {
        let broker = SimulatedBroker::new();
        broker.update_price("TESTSTOCK", 100.0); // above trigger — dormant
        let placed = broker.place_order(&stop_req()).await.unwrap();
        assert_eq!(placed.status, OrderStatus::Open);

        // A poll while still dormant changes nothing.
        let still_open = broker.order_status(&placed.broker_order_id).await.unwrap();
        assert_eq!(still_open.status, OrderStatus::Open);

        // Price moves onto the stop between polls — a later poll must
        // pick that up rather than only checking at placement time.
        broker.update_price("TESTSTOCK", 94.95);
        let filled = broker.order_status(&placed.broker_order_id).await.unwrap();
        assert_eq!(filled.status, OrderStatus::Complete);
        assert_eq!(filled.average_price, Some(94.95));
    }

    #[tokio::test]
    async fn cancel_is_a_noop_once_already_filled() {
        let broker = SimulatedBroker::new();
        broker.update_price("TESTSTOCK", 94.95);
        let placed = broker.place_order(&stop_req()).await.unwrap();
        assert_eq!(placed.status, OrderStatus::Complete);
        assert!(broker.cancel_order(&placed.broker_order_id).await.is_ok());
        let after = broker.order_status(&placed.broker_order_id).await.unwrap();
        assert_eq!(after.status, OrderStatus::Complete); // unchanged, not clobbered to Cancelled
    }

    #[tokio::test]
    async fn positions_reflect_fills_and_net_out_to_flat() {
        let broker = SimulatedBroker::new();
        broker.update_price("TESTSTOCK", 94.95);
        broker.place_order(&stop_req()).await.unwrap(); // sells 10
        let positions = broker.positions().await.unwrap();
        assert_eq!(positions.len(), 1);
        assert_eq!(positions[0].net_quantity, -10);

        let mut buy_back = stop_req();
        buy_back.side = OrderSide::Buy;
        buy_back.order_type = OrderType::Market;
        buy_back.price = None;
        buy_back.trigger_price = None;
        broker.place_order(&buy_back).await.unwrap();
        assert!(broker.positions().await.unwrap().is_empty());
    }
}

impl SimulatedBroker {
    /// Re-checks one still-open order against the latest known price,
    /// filling it (and updating simulated positions) if it's now
    /// marketable. Shared by `modify_order` (a re-price can trigger an
    /// immediate fill) and `order_status` (a poll can discover a fill that
    /// happened since the price last moved).
    fn reevaluate(&self, broker_order_id: &str, req: &OrderRequest) -> Result<OrderResult> {
        let current = self.current_price(&req.tradingsymbol);
        if let Some(fill_price) = current.and_then(|p| Self::marketable_price(req, p)) {
            self.apply_fill(req, fill_price);
            let mut orders = self.orders.lock().unwrap();
            if let Some(order) = orders.get_mut(broker_order_id) {
                order.status = OrderStatus::Complete;
                order.filled_quantity = req.quantity;
                order.average_price = Some(fill_price);
            }
            return Ok(OrderResult {
                broker_order_id: broker_order_id.to_string(),
                status: OrderStatus::Complete,
                filled_quantity: req.quantity,
                average_price: Some(fill_price),
                rejection_reason: None,
                raw_response_json: "{}".to_string(),
            });
        }
        Ok(OrderResult {
            broker_order_id: broker_order_id.to_string(),
            status: OrderStatus::Open,
            filled_quantity: 0,
            average_price: None,
            rejection_reason: None,
            raw_response_json: "{}".to_string(),
        })
    }
}
