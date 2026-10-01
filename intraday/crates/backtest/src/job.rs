//! The actual "run a backtest" orchestration: resolve candles (cache-first),
//! replay through a fresh `PaperEngine`, persist. Both `papertrader
//! backtest run` and the web UI's trigger endpoint call this exact
//! function, so a result never depends on which one started the run.

use core_types::time::{ist_day_end, ist_day_start};
use core_types::{
    CandleInterval, Instrument, MarketDataFeed, NaiveDate, NewRun, RunId, RunMode, Strategy,
};
use paper_engine::EngineConfig;
use storage::SqliteStorage;

#[derive(Debug, Clone)]
pub struct JobProgress {
    pub current: usize,
    pub total: usize,
    pub message: String,
}

pub struct BacktestJobParams {
    pub from: NaiveDate,
    pub to: NaiveDate,
    pub label: Option<String>,
    pub instruments: Vec<Instrument>,
    pub engine_config: EngineConfig,
    /// Exact config used, captured verbatim on the run for reproducibility
    /// — typically `serde_json::to_string` of whatever produced
    /// `engine_config` plus the enabled strategies' params.
    pub config_snapshot_json: String,
}

/// Runs one backtest end to end: upserts `params.instruments` (so the
/// `report`/UI can resolve symbols later), fetches candles for
/// `[from, to]` — reusing whatever's already cached in `storage`, only
/// hitting `feed` for what's missing — merges them into one
/// chronologically-ordered stream, and replays it through a fresh
/// `PaperEngine` running `strategies`. Returns the new run's id.
pub async fn run_backtest_job(
    storage: SqliteStorage,
    feed: &dyn MarketDataFeed,
    strategies: Vec<Box<dyn Strategy>>,
    params: BacktestJobParams,
    mut on_progress: impl FnMut(JobProgress),
) -> anyhow::Result<RunId> {
    storage.upsert_instruments(&params.instruments)?;

    let from_dt = ist_day_start(params.from);
    let to_dt = ist_day_end(params.to);
    let total = params.instruments.len();

    let mut series = Vec::with_capacity(total);
    for (idx, inst) in params.instruments.iter().enumerate() {
        let cached = storage.query_candles(inst.instrument_token, CandleInterval::OneMinute, from_dt, to_dt)?;
        let candles = if cached.is_empty() {
            let fetched = feed
                .historical_candles(inst, from_dt, to_dt, CandleInterval::OneMinute)
                .await
                .map_err(|e| anyhow::anyhow!("fetching {}: {e}", inst.tradingsymbol))?;
            storage.insert_candles(inst.instrument_token, CandleInterval::OneMinute, &fetched)?;
            on_progress(JobProgress {
                current: idx + 1,
                total,
                message: format!("{}: {} candles (fetched from Kite)", inst.tradingsymbol, fetched.len()),
            });
            fetched
        } else {
            on_progress(JobProgress {
                current: idx + 1,
                total,
                message: format!("{}: {} candles (from cache)", inst.tradingsymbol, cached.len()),
            });
            cached
        };
        series.push((inst.clone(), candles));
    }

    let bars = crate::merge_chronological(series);
    on_progress(JobProgress {
        current: total,
        total,
        message: format!("{} total bars across {total} instruments, replaying...", bars.len()),
    });

    let indicator_engine = strategy_core::IndicatorEngine::new(chrono::NaiveTime::from_hms_opt(9, 30, 0).unwrap());
    let runner = strategy_core::StrategyRunner::new(indicator_engine, strategies);
    let mut engine = paper_engine::PaperEngine::new(
        runner,
        Box::new(risk::DefaultExitManager),
        Box::new(storage),
        params.engine_config,
        NewRun {
            mode: RunMode::Backtest,
            config_snapshot_json: params.config_snapshot_json,
            label: params.label,
            from_date: Some(params.from),
            to_date: Some(params.to),
        },
    )?;
    let run_id = engine.run_id();
    crate::run(&mut engine, &bars)?;
    on_progress(JobProgress {
        current: total,
        total,
        message: "backtest complete".to_string(),
    });
    Ok(run_id)
}
