//! Real order execution — placing, tracking, and reconciling actual broker
//! orders, as opposed to `ExitManager`/`PaperEngine`'s simulated fills.
//! Isolated behind a trait for the same reason as `MarketDataFeed`: a
//! `SimulatedBroker` (instant/quote-based fills, used for dry-run and tests)
//! and a `KiteBroker` (real Kite Connect order placement) both implement it,
//! and everything above this layer (`LiveEngine`, `ExecutionPolicy`) is
//! written once against the trait.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::error::Result;

/// BUY or SELL, as the broker sees it — distinct from `Direction`
/// (Long/Short), which is the strategy's view. A Long entry is a Buy; a
/// Short entry is a Sell; either exit is the opposite side.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderType {
    Market,
    Limit,
    /// Stop-loss limit: dormant until `trigger_price` is touched, then
    /// becomes a limit order at `price`.
    SlLimit,
    /// Stop-loss market: dormant until `trigger_price` is touched, then
    /// fires as a market order. Kite's native "SL-M" — the primary
    /// alternative to the limit-then-market-fallback policy built in
    /// `ExecutionPolicy`.
    SlMarket,
}

/// Kite's MIS (intraday, leveraged) vs CNC (delivery) vs NRML (F&O
/// carry-forward). Everything in this system is MIS.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Product {
    Mis,
}

#[derive(Debug, Clone)]
pub struct OrderRequest {
    pub tradingsymbol: String,
    pub exchange: crate::instrument::Exchange,
    pub side: OrderSide,
    pub quantity: u32,
    pub product: Product,
    pub order_type: OrderType,
    /// Required for Limit and SlLimit.
    pub price: Option<f64>,
    /// Required for SlLimit and SlMarket.
    pub trigger_price: Option<f64>,
    /// Client-supplied idempotency/reconciliation tag — if a place_order
    /// call's response is lost (network error, timeout), the same tag can
    /// be used to find the order via `Broker::positions`/order-book lookup
    /// rather than risk double-placing.
    pub tag: String,
}

/// Kite's order lifecycle states, collapsed to what callers actually need
/// to branch on. `Other` preserves the raw string for logging/debugging
/// rather than silently losing an unrecognized state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderStatus {
    Pending,
    Open,
    Complete,
    Cancelled,
    Rejected,
    Other(String),
}

#[derive(Debug, Clone)]
pub struct OrderResult {
    pub broker_order_id: String,
    pub status: OrderStatus,
    pub filled_quantity: u32,
    pub average_price: Option<f64>,
    pub rejection_reason: Option<String>,
    /// The full raw response body, kept verbatim for audit/debugging —
    /// never parsed lossily without also keeping this.
    pub raw_response_json: String,
}

#[derive(Debug, Clone)]
pub struct MarginCheckResult {
    pub ok: bool,
    pub required_margin: Option<f64>,
    pub available_margin: Option<f64>,
    /// Set when `ok` is false — the broker's own reason (insufficient
    /// margin, invalid quantity, symbol restriction, etc.), used instead of
    /// guessing at exchange rules (freeze quantity, leverage multiples)
    /// client-side.
    pub rejection_reason: Option<String>,
    pub raw_response_json: String,
}

/// One line of the broker's actual current holdings — for the
/// reconciliation loop, comparing what we *think* is open against what
/// really is.
#[derive(Debug, Clone)]
pub struct BrokerPosition {
    pub tradingsymbol: String,
    /// Signed: positive = net long, negative = net short, zero = flat.
    pub net_quantity: i64,
    pub average_price: f64,
}

#[async_trait]
pub trait Broker: Send + Sync {
    async fn place_order(&self, req: &OrderRequest) -> Result<OrderResult>;

    async fn modify_order(
        &self,
        broker_order_id: &str,
        new_price: Option<f64>,
        new_trigger_price: Option<f64>,
        new_quantity: Option<u32>,
    ) -> Result<OrderResult>;

    async fn cancel_order(&self, broker_order_id: &str) -> Result<()>;

    async fn order_status(&self, broker_order_id: &str) -> Result<OrderResult>;

    /// Asks the broker's own margin calculator whether this order would be
    /// accepted, without placing it — the safety check before any real
    /// order, rather than assuming a flat leverage multiple.
    async fn check_margin(&self, req: &OrderRequest) -> Result<MarginCheckResult>;

    async fn positions(&self) -> Result<Vec<BrokerPosition>>;
}
