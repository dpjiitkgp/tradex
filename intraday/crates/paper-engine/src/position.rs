use core_types::{DateTime, ExitPlan, PositionState, TradeId};

pub(crate) struct OpenPosition {
    pub trade_id: TradeId,
    pub tradingsymbol: String,
    pub entry_ts: DateTime,
    pub qty: u32,
    pub state: PositionState,
    pub plan: ExitPlan,
}
