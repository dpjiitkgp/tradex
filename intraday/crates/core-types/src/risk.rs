use crate::signal::Direction;
use crate::time::DateTime;
use serde::{Deserialize, Serialize};

/// How a position behaves once it has cleared the minimum-target floor.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TrailPolicy {
    /// Stop trails this fraction below (long) / above (short) the peak
    /// favorable price — same convention as `risk_pct` (0.0015 = 0.15%),
    /// not a percentage-point value. The minimum-target floor is locked in
    /// as an inviolable guarantee the instant it's reached (see
    /// `PositionPhase::Trailing`'s docs) — this fraction can only add
    /// *more* protection beyond that floor, never less.
    PercentFromPeak(f64),
    /// Stop trails this multiple of ATR below/above the peak ("chandelier" style).
    AtrMultiple(f64),
    /// Stop is the session VWAP; exit when price crosses back through it.
    VwapCross,
    /// Same peak-trailing math as `PercentFromPeak`, but deliberately
    /// *without* locking the minimum-target price in as a floor on arming —
    /// the stop is simply `peak * (1 -/+ frac)` from the moment the
    /// min-target is first reached, which can sit *below* that target
    /// (e.g. reward floor at 3R, trail fraction equal to 1R, ⇒ stop locks
    /// at 2R the instant it arms, not 3R). Use this when the trader's own
    /// rule is "once I'm up by X, keep my stop Y behind current price" with
    /// no separate hard guarantee at the arming instant — as opposed to
    /// `PercentFromPeak`'s "guarantee at least the floor, trail only adds
    /// more" contract.
    GiveBackFromPeak(f64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PositionPhase {
    /// From entry until the minimum-target floor is reached: only the hard
    /// stop can close the position.
    Armed,
    /// Minimum-target floor reached and locked in as the new stop; from here
    /// the configured `TrailPolicy` governs further stop movement.
    Trailing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExitReason {
    StopLoss,
    TrailingStop,
    VwapCross,
    EodSquareOff,
    MaxHold,
    Manual,
}

/// Config-driven risk parameters for a single entry, resolved into concrete
/// prices by `ExitManager::on_entry`.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct EntryContext {
    pub direction: Direction,
    pub entry_price: f64,
    pub entry_ts: DateTime,
    /// Hard stop-loss as a fraction of entry price, e.g. 0.0025 for 0.25%.
    pub risk_pct: f64,
    /// Minimum reward:risk floor before trailing kicks in, e.g. 2.0.
    pub min_reward_multiple: f64,
    pub trail_policy: TrailPolicy,
}

/// The concrete stop/target prices derived from an `EntryContext`.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ExitPlan {
    pub hard_stop_price: f64,
    pub min_target_price: f64,
    pub trail_policy: TrailPolicy,
}

/// Live state the engine tracks per open position and feeds back into
/// `ExitManager::on_update` each bar.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PositionState {
    pub direction: Direction,
    pub entry_price: f64,
    pub current_stop: f64,
    pub peak_favorable_price: f64,
    pub phase: PositionPhase,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MarketSnapshot {
    pub ts: DateTime,
    pub last_price: f64,
    pub vwap: Option<f64>,
    pub atr: Option<f64>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ExitDecision {
    pub exit_price: f64,
    pub ts: DateTime,
    pub reason: ExitReason,
}

/// A stop-price move to be logged for audit (`trade_stop_events`).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct StopMove {
    pub ts: DateTime,
    pub new_stop_price: f64,
}
