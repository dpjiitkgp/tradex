use crate::dto::*;
use crate::state::AppState;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use std::collections::HashMap;

type ApiResult<T> = Result<Json<T>, (StatusCode, Json<ApiError>)>;

fn err500(e: impl std::fmt::Display) -> (StatusCode, Json<ApiError>) {
    (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError { error: e.to_string() }))
}

fn err400(e: impl std::fmt::Display) -> (StatusCode, Json<ApiError>) {
    (StatusCode::BAD_REQUEST, Json(ApiError { error: e.to_string() }))
}

fn parse_run_ids(csv: &str) -> Vec<i64> {
    csv.split(',')
        .filter_map(|s| s.trim().parse::<i64>().ok())
        .collect()
}

// ---------------------------------------------------------------- runs ---

pub async fn list_runs(State(state): State<AppState>) -> ApiResult<Vec<storage::RunMeta>> {
    let storage = state.open_storage().map_err(err500)?;
    let runs = storage.list_runs().map_err(err500)?;
    Ok(Json(runs))
}

pub async fn run_summary(
    State(state): State<AppState>,
    Path(run_id): Path<i64>,
) -> ApiResult<Vec<storage::StrategySummaryRow>> {
    let storage = state.open_storage().map_err(err500)?;
    let rows = storage.strategy_summary(Some(&[run_id])).map_err(err500)?;
    Ok(Json(rows))
}

/// Every strategy x instrument combination across every run — the raw
/// material for the overview tab's "best combos" widget. Callers that want
/// this scoped to one run should use `/runs/:id/detail` instead.
pub async fn performance(State(state): State<AppState>) -> ApiResult<Vec<storage::StrategyPerformanceRow>> {
    let storage = state.open_storage().map_err(err500)?;
    let rows = storage.strategy_performance(None).map_err(err500)?;
    Ok(Json(rows))
}

pub async fn run_detail(
    State(state): State<AppState>,
    Path(run_id): Path<i64>,
) -> ApiResult<Vec<storage::StrategyPerformanceRow>> {
    let storage = state.open_storage().map_err(err500)?;
    let rows = storage.strategy_performance(Some(run_id)).map_err(err500)?;
    Ok(Json(rows))
}

// ------------------------------------------------------------ compare ---

pub async fn compare(
    State(state): State<AppState>,
    Query(q): Query<HashMap<String, String>>,
) -> ApiResult<Vec<storage::StrategySummaryRow>> {
    let storage = state.open_storage().map_err(err500)?;
    let ids = q.get("runs").map(|s| parse_run_ids(s)).unwrap_or_default();
    let filter = if ids.is_empty() { None } else { Some(ids.as_slice()) };
    let rows = storage.strategy_summary(filter).map_err(err500)?;
    Ok(Json(rows))
}

/// One stock, across the given runs (or every run) — every strategy's
/// result for that symbol side by side. This is the direct answer to
/// "which strategy actually works on this stock."
pub async fn compare_stock(
    State(state): State<AppState>,
    Query(q): Query<HashMap<String, String>>,
) -> ApiResult<Vec<storage::StrategyPerformanceRow>> {
    let symbol = q.get("symbol").cloned().unwrap_or_default();
    if symbol.is_empty() {
        return Err(err400("missing ?symbol="));
    }
    let ids = q.get("runs").map(|s| parse_run_ids(s));

    let storage = state.open_storage().map_err(err500)?;
    let mut rows = storage.strategy_performance(None).map_err(err500)?;
    rows.retain(|r| {
        let symbol_matches = r
            .tradingsymbol
            .as_deref()
            .is_some_and(|s| s.eq_ignore_ascii_case(&symbol));
        let run_matches = ids.as_ref().is_none_or(|ids| ids.contains(&r.run_id));
        symbol_matches && run_matches
    });
    rows.sort_by(|a, b| b.avg_r.partial_cmp(&a.avg_r).unwrap_or(std::cmp::Ordering::Equal));
    Ok(Json(rows))
}

// ----------------------------------------------------------- universe ---

pub async fn universe_sectors(State(state): State<AppState>) -> ApiResult<Vec<SectorDto>> {
    crate::sectors::ensure_synced(&state).await.map_err(err500)?;
    let storage = state.open_storage().map_err(err500)?;
    let sectors = storage
        .distinct_sectors()
        .map_err(err500)?
        .into_iter()
        .map(|(sector, count)| SectorDto { sector, count })
        .collect();
    Ok(Json(sectors))
}

pub async fn universe_stocks(
    State(state): State<AppState>,
    Query(q): Query<HashMap<String, String>>,
) -> ApiResult<Vec<StockDto>> {
    crate::sectors::ensure_synced(&state).await.map_err(err500)?;
    let storage = state.open_storage().map_err(err500)?;
    let sector = q.get("sector").map(|s| s.as_str());
    let tagged = storage.tagged_instruments(sector).map_err(err500)?;

    let mut out = Vec::with_capacity(tagged.len());
    for (inst, sector) in tagged {
        let coverage = storage.candle_coverage(inst.instrument_token).map_err(err500)?;
        let (data_from, data_to, candle_count) = match coverage {
            Some((from, to, count)) => (Some(from), Some(to), count),
            None => (None, None, 0),
        };
        out.push(StockDto {
            symbol: inst.tradingsymbol,
            instrument_token: inst.instrument_token,
            sector,
            data_from,
            data_to,
            candle_count,
        });
    }
    Ok(Json(out))
}

pub async fn coverage(State(state): State<AppState>) -> ApiResult<CoverageDto> {
    let storage = state.open_storage().map_err(err500)?;
    let overall = storage.overall_candle_coverage().map_err(err500)?;
    let dto = match overall {
        Some((from, to, count)) => CoverageDto {
            from: Some(from),
            to: Some(to),
            instrument_count: count,
        },
        None => CoverageDto {
            from: None,
            to: None,
            instrument_count: 0,
        },
    };
    Ok(Json(dto))
}

// ------------------------------------------------------------- config ---

pub async fn config_defaults(State(state): State<AppState>) -> ApiResult<DefaultsResponse> {
    let strategies_cfg = strategies::config::load(&state.config_path).map_err(err500)?;
    let (engine_cfg, _) = paper_engine::config_loader::load(&state.config_path).map_err(err500)?;
    Ok(Json(DefaultsResponse {
        strategies: StrategiesOverrideDto::from_config(&strategies_cfg),
        risk: RiskDto::from_config(&engine_cfg),
    }))
}

// ----------------------------------------------------------- backtest ---

async fn resolve_scope(state: &AppState, scope: &ScopeDto) -> anyhow::Result<Vec<core_types::Instrument>> {
    match scope.kind.as_str() {
        "all" => {
            crate::sectors::ensure_synced(state).await?;
            let storage = state.open_storage()?;
            storage.instruments_by_sector(None)
        }
        "sector" => {
            crate::sectors::ensure_synced(state).await?;
            let sector = scope.value.as_deref().ok_or_else(|| anyhow::anyhow!("scope.value required for sector"))?;
            let storage = state.open_storage()?;
            storage.instruments_by_sector(Some(sector))
        }
        "symbol" => {
            let symbol = scope.value.as_deref().ok_or_else(|| anyhow::anyhow!("scope.value required for symbol"))?;
            let storage = state.open_storage()?;
            match storage.instrument_by_symbol(symbol)? {
                Some(inst) => Ok(vec![inst]),
                None => {
                    // Not tagged/cached yet — resolve directly against
                    // Kite's instrument master rather than failing, so a
                    // stock outside the synced sector set can still be
                    // backtested one-off.
                    let rest = crate::kite::rest_client(&state.credentials_path).await?;
                    let all = kite_client::instruments::fetch_nse_equities(&rest).await?;
                    all.into_iter()
                        .find(|i| i.tradingsymbol.eq_ignore_ascii_case(symbol))
                        .map(|i| vec![i])
                        .ok_or_else(|| anyhow::anyhow!("{symbol} not found in NSE cash-equity instruments"))
                }
            }
        }
        other => anyhow::bail!("unknown scope.kind {other:?}, expected all | sector | symbol"),
    }
}

pub async fn trigger_backtest(
    State(state): State<AppState>,
    Json(req): Json<TriggerRequest>,
) -> ApiResult<TriggerResponse> {
    let from = chrono::NaiveDate::parse_from_str(&req.from, "%Y-%m-%d").map_err(err400)?;
    let to = chrono::NaiveDate::parse_from_str(&req.to, "%Y-%m-%d").map_err(err400)?;
    let engine_config = req.risk.to_engine_config().map_err(err400)?;
    let strategies_config = req.strategies.to_strategies_config();
    let built_strategies = strategies_config.build();
    if built_strategies.is_empty() {
        return Err(err400("no strategies enabled"));
    }

    let instruments = resolve_scope(&state, &req.scope).await.map_err(err400)?;
    if instruments.is_empty() {
        return Err(err400("scope resolved to zero instruments"));
    }
    let instrument_count = instruments.len();

    let config_snapshot_json = serde_json::json!({
        "risk": req.risk,
        "strategies": req.strategies,
        "scope": {"kind": req.scope.kind, "value": req.scope.value},
    })
    .to_string();

    let job_id = state.jobs.insert_running();
    let jobs = state.jobs.clone();
    let db_path = state.db_path.clone();
    let credentials_path = state.credentials_path.clone();
    let label = req.label.clone();

    tokio::spawn(async move {
        let result = async {
            let storage = storage::SqliteStorage::open(&db_path)?;
            let rest = crate::kite::rest_client(&credentials_path).await?;
            let feed = kite_client::KiteMarketDataFeed::new(rest);
            backtest::run_backtest_job(
                storage,
                &feed,
                built_strategies,
                backtest::BacktestJobParams {
                    from,
                    to,
                    label,
                    instruments,
                    engine_config,
                    config_snapshot_json,
                },
                |p| jobs.update_progress(job_id, p),
            )
            .await
        }
        .await;

        match result {
            Ok(run_id) => jobs.finish_ok(job_id, run_id),
            Err(e) => {
                tracing::error!(error = %e, "triggered backtest job failed");
                jobs.finish_err(job_id, e.to_string());
            }
        }
    });

    Ok(Json(TriggerResponse { job_id, instrument_count }))
}

pub async fn job_status(
    State(state): State<AppState>,
    Path(job_id): Path<uuid::Uuid>,
) -> Result<Json<crate::state::JobStatus>, (StatusCode, Json<ApiError>)> {
    match state.jobs.get(job_id) {
        Some(status) => Ok(Json(status)),
        None => Err((StatusCode::NOT_FOUND, Json(ApiError { error: "unknown job id".to_string() }))),
    }
}
