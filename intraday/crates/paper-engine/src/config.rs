use chrono::NaiveTime;
use core_types::TrailPolicy;

/// Portfolio-level knobs — deliberately not inside any `Strategy`, per the
/// plan ("max concurrent positions, capital-per-trade / 5x MIS sizing,
/// daily-loss circuit breaker... live in paper-engine, not strategies").
#[derive(Debug, Clone)]
pub struct EngineConfig {
    /// Hard stop-loss as a fraction of entry price (0.0025 = 0.25%).
    pub risk_pct: f64,
    /// Minimum reward:risk floor before trailing kicks in (e.g. 2.0).
    pub min_reward_multiple: f64,
    pub trail_policy: TrailPolicy,
    /// Rupees allocated per position.
    pub capital_per_trade: f64,
    pub max_concurrent_positions: usize,
    /// Stop taking new entries for the day once realized loss reaches this
    /// fraction of (capital_per_trade * max_concurrent_positions). 0.0 disables.
    pub daily_loss_cap_pct: f64,
    /// Mandatory square-off cutoff, IST wall-clock time.
    pub eod_square_off: NaiveTime,
    /// Hard cap on how long a position can stay open. `None` = no cap.
    pub max_hold_minutes: Option<i64>,
    /// ATR period used for exit-side monitoring (independent of whatever
    /// ATR period, if any, a strategy requested for entries).
    pub exit_atr_period: u32,
}

impl EngineConfig {
    pub fn allocated_capital(&self) -> f64 {
        self.capital_per_trade * self.max_concurrent_positions as f64
    }

    pub fn daily_loss_cap(&self) -> Option<f64> {
        (self.daily_loss_cap_pct > 0.0).then(|| self.daily_loss_cap_pct * self.allocated_capital())
    }
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            risk_pct: 0.0025,
            min_reward_multiple: 2.0,
            trail_policy: TrailPolicy::PercentFromPeak(0.0015),
            capital_per_trade: 50_000.0,
            max_concurrent_positions: 5,
            daily_loss_cap_pct: 0.03,
            eod_square_off: NaiveTime::from_hms_opt(15, 15, 0).unwrap(),
            max_hold_minutes: None,
            exit_atr_period: 14,
        }
    }
}
