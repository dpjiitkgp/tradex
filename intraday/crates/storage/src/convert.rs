use core_types::{
    BrokerOrderAction, Direction, ExitReason, OrderSide, OrderStatus, OrderType, RunMode,
    TimeWindow,
};

pub fn mode_to_str(mode: RunMode) -> &'static str {
    match mode {
        RunMode::Live => "live",
        RunMode::Backtest => "backtest",
    }
}

pub fn window_to_str(window: TimeWindow) -> &'static str {
    match window {
        TimeWindow::Opening => "opening",
        TimeWindow::MidMorning => "mid_morning",
        TimeWindow::Afternoon => "afternoon",
        TimeWindow::FullDay => "full_day",
    }
}

pub fn direction_to_str(direction: Direction) -> &'static str {
    match direction {
        Direction::Long => "long",
        Direction::Short => "short",
    }
}

pub fn exit_reason_to_str(reason: ExitReason) -> &'static str {
    match reason {
        ExitReason::StopLoss => "stop_loss",
        ExitReason::TrailingStop => "trailing_stop",
        ExitReason::VwapCross => "vwap_cross",
        ExitReason::EodSquareOff => "eod_square_off",
        ExitReason::MaxHold => "max_hold",
        ExitReason::Manual => "manual",
    }
}

pub fn sqlite_err(e: rusqlite::Error) -> core_types::Error {
    core_types::Error::Storage(e.to_string())
}

pub fn broker_order_action_to_str(action: BrokerOrderAction) -> &'static str {
    match action {
        BrokerOrderAction::Place => "place",
        BrokerOrderAction::Modify => "modify",
        BrokerOrderAction::Cancel => "cancel",
        BrokerOrderAction::StatusPoll => "status_poll",
    }
}

pub fn order_side_to_str(side: OrderSide) -> &'static str {
    match side {
        OrderSide::Buy => "BUY",
        OrderSide::Sell => "SELL",
    }
}

pub fn order_type_to_str(t: OrderType) -> &'static str {
    match t {
        OrderType::Market => "MARKET",
        OrderType::Limit => "LIMIT",
        OrderType::SlLimit => "SL",
        OrderType::SlMarket => "SL-M",
    }
}

pub fn order_status_to_str(status: &OrderStatus) -> String {
    match status {
        OrderStatus::Pending => "pending".to_string(),
        OrderStatus::Open => "open".to_string(),
        OrderStatus::Complete => "complete".to_string(),
        OrderStatus::Cancelled => "cancelled".to_string(),
        OrderStatus::Rejected => "rejected".to_string(),
        OrderStatus::Other(s) => format!("other:{s}"),
    }
}
