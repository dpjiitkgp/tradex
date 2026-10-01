use crate::broker::{OrderSide, OrderStatus, OrderType};
use crate::risk::{ExitReason, StopMove, TrailPolicy};
use crate::signal::{Direction, StrategyId};
use crate::time::{DateTime, NaiveDate, TimeWindow};

pub type RunId = i64;
pub type SignalId = i64;
pub type TradeId = i64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum RunMode {
    Live,
    Backtest,
}

/// Everything needed to record a new run — grouped into one struct (rather
/// than growing `start_run`'s parameter list indefinitely) since this is
/// purely descriptive metadata, not something the engine needs to function.
#[derive(Debug, Clone)]
pub struct NewRun {
    pub mode: RunMode,
    pub config_snapshot_json: String,
    /// Human-readable name (e.g. "orb-relvol-2x") for telling runs apart
    /// while tuning — separate from `config_snapshot_json`, which is
    /// always captured regardless.
    pub label: Option<String>,
    /// The backtest's requested date range. `None`/`None` for live runs,
    /// which don't have one.
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
}

#[derive(Debug, Clone)]
pub struct SignalRecord {
    pub run_id: RunId,
    pub ts: DateTime,
    pub instrument_token: u64,
    pub strategy_id: StrategyId,
    pub window: TimeWindow,
    pub direction: Direction,
    pub entry_price_hint: f64,
    pub rationale: String,
    pub params_json: String,
}

#[derive(Debug, Clone)]
pub struct NewTrade {
    pub run_id: RunId,
    pub signal_id: Option<SignalId>,
    pub instrument_token: u64,
    pub strategy_id: StrategyId,
    pub window: TimeWindow,
    pub mode: RunMode,
    pub direction: Direction,
    pub qty: u32,
    pub capital_allocated: f64,
    pub entry_ts: DateTime,
    pub entry_price: f64,
    pub hard_stop_price: f64,
    pub min_target_price: f64,
    pub trail_policy: TrailPolicy,
}

#[derive(Debug, Clone)]
pub struct TradeExit {
    pub exit_ts: DateTime,
    pub exit_price: f64,
    pub reason: ExitReason,
    pub pnl_abs: f64,
    pub pnl_pct: f64,
    pub r_multiple: f64,
}

/// One action taken against a real broker order — a row per attempt
/// (place/modify/cancel/status_poll), not just the final outcome, so the
/// full audit trail (including a limit-then-market-fallback sequence, or a
/// rejected-then-retried order) is reconstructable afterward. `trade_id` is
/// `None` for an order that never resulted in an open position (e.g.
/// rejected on placement).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrokerOrderAction {
    Place,
    Modify,
    Cancel,
    StatusPoll,
}

#[derive(Debug, Clone)]
pub struct BrokerOrderEvent {
    pub run_id: RunId,
    pub trade_id: Option<TradeId>,
    pub instrument_token: u64,
    pub tradingsymbol: String,
    pub kite_order_id: Option<String>,
    pub action: BrokerOrderAction,
    pub transaction_type: OrderSide,
    pub order_type: OrderType,
    pub requested_price: Option<f64>,
    pub trigger_price: Option<f64>,
    pub quantity: u32,
    pub status: OrderStatus,
    pub filled_quantity: u32,
    pub average_price: Option<f64>,
    pub is_fallback_market: bool,
    pub rejection_reason: Option<String>,
    pub raw_response_json: String,
    pub placed_at: DateTime,
}

/// Persists everything needed for the post-hoc "what actually worked"
/// analysis: runs (config snapshot), signals, trades, and their stop moves.
pub trait Storage: Send + Sync {
    fn start_run(&self, new_run: &NewRun) -> crate::error::Result<RunId>;
    fn end_run(&self, run_id: RunId) -> crate::error::Result<()>;
    fn record_signal(&self, signal: &SignalRecord) -> crate::error::Result<SignalId>;
    fn open_trade(&self, trade: &NewTrade) -> crate::error::Result<TradeId>;
    fn record_stop_move(&self, trade_id: TradeId, mv: &StopMove) -> crate::error::Result<()>;
    fn close_trade(&self, trade_id: TradeId, exit: &TradeExit) -> crate::error::Result<()>;
    /// Appends one broker-order audit row — real-money live trading only,
    /// never called from backtest/paper paths. Returns the row id.
    fn record_broker_order(&self, event: &BrokerOrderEvent) -> crate::error::Result<i64>;
}
