use core_types::Direction;

pub(crate) struct PnlResult {
    pub pnl_abs: f64,
    pub pnl_pct: f64,
    pub r_multiple: f64,
}

/// `risk_pct` is the same fraction used to compute the original hard stop
/// (see `EngineConfig::risk_pct`) — used here as the "1R" unit so
/// `r_multiple` means the same thing across every trade regardless of what
/// its actual stop distance ended up being after trailing.
pub(crate) fn compute(
    direction: Direction,
    entry_price: f64,
    exit_price: f64,
    qty: u32,
    risk_pct: f64,
) -> PnlResult {
    let pnl_per_share = match direction {
        Direction::Long => exit_price - entry_price,
        Direction::Short => entry_price - exit_price,
    };
    let pnl_abs = pnl_per_share * qty as f64;
    let pnl_pct = pnl_per_share / entry_price * 100.0;
    let risk_per_share = entry_price * risk_pct;
    let r_multiple = if risk_per_share > 0.0 {
        pnl_per_share / risk_per_share
    } else {
        0.0
    };
    PnlResult {
        pnl_abs,
        pnl_pct,
        r_multiple,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn long_winner_computes_positive_pnl_and_r() {
        let r = compute(Direction::Long, 100.0, 101.0, 10, 0.0025);
        assert!((r.pnl_abs - 10.0).abs() < 1e-9);
        assert!((r.pnl_pct - 1.0).abs() < 1e-9);
        // risk_per_share = 0.25, pnl_per_share = 1.0 -> r = 4.0
        assert!((r.r_multiple - 4.0).abs() < 1e-9);
    }

    #[test]
    fn short_winner_computes_positive_pnl() {
        let r = compute(Direction::Short, 100.0, 99.0, 10, 0.0025);
        assert!((r.pnl_abs - 10.0).abs() < 1e-9);
        assert!((r.r_multiple - 4.0).abs() < 1e-9);
    }

    #[test]
    fn loser_computes_negative_pnl_and_r() {
        let r = compute(Direction::Long, 100.0, 99.75, 10, 0.0025);
        assert!((r.pnl_abs - (-2.5)).abs() < 1e-9);
        assert!((r.r_multiple - (-1.0)).abs() < 1e-9);
    }
}
