//! SQLite schema, migrations, and the Storage implementation persisting runs, signals, trades, and candles for post-hoc analysis.

pub mod convert;
pub mod schema;
pub mod store;

pub use store::SqliteStorage;

/// One row of `v_strategy_performance` — win rate / avg return / avg R by
/// run x strategy x window x instrument x mode. The finest-grained report:
/// this is where "strategy X works better on stock Y" shows up.
#[derive(Debug, Clone, serde::Serialize)]
pub struct StrategyPerformanceRow {
    pub run_id: core_types::RunId,
    pub run_label: Option<String>,
    pub strategy_id: String,
    pub window: String,
    pub mode: String,
    pub instrument_token: u64,
    /// `None` if this instrument was never upserted via `upsert_instruments`.
    pub tradingsymbol: Option<String>,
    pub trades: i64,
    pub avg_pnl_pct: f64,
    pub avg_r: f64,
    pub win_rate: f64,
}

/// One row of the `runs` table, for the UI's runs list.
#[derive(Debug, Clone, serde::Serialize)]
pub struct RunMeta {
    pub id: core_types::RunId,
    pub label: Option<String>,
    pub mode: String,
    pub started_at: String,
    pub ended_at: Option<String>,
    pub from_date: Option<String>,
    pub to_date: Option<String>,
    pub config_snapshot_json: String,
}

/// One row of the coarse per-run x per-strategy rollup (no instrument
/// split) — computed directly from `trades` so averages are correctly
/// weighted by trade, not by instrument. This is the shape `compare` uses.
#[derive(Debug, Clone, serde::Serialize)]
pub struct StrategySummaryRow {
    pub run_id: core_types::RunId,
    pub run_label: Option<String>,
    pub strategy_id: String,
    pub mode: String,
    pub trades: i64,
    pub avg_pnl_pct: f64,
    pub avg_r: f64,
    pub win_rate: f64,
}
