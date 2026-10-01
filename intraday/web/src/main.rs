mod dto;
mod handlers;
mod kite;
mod sectors;
mod state;

use axum::routing::{get, post};
use axum::Router;
use state::{AppState, JobRegistry, SectorSyncState};
use std::path::PathBuf;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let state = AppState {
        db_path: PathBuf::from("data/papertrader.db"),
        config_path: PathBuf::from("config/default.toml"),
        credentials_path: PathBuf::from("secrets/credentials.toml"),
        jobs: JobRegistry::default(),
        sector_sync: SectorSyncState::default(),
    };

    let api = Router::new()
        .route("/runs", get(handlers::list_runs))
        .route("/performance", get(handlers::performance))
        .route("/runs/:id/summary", get(handlers::run_summary))
        .route("/runs/:id/detail", get(handlers::run_detail))
        .route("/compare", get(handlers::compare))
        .route("/compare/stock", get(handlers::compare_stock))
        .route("/universe/sectors", get(handlers::universe_sectors))
        .route("/universe/stocks", get(handlers::universe_stocks))
        .route("/coverage", get(handlers::coverage))
        .route("/config/defaults", get(handlers::config_defaults))
        .route("/backtest", post(handlers::trigger_backtest))
        .route("/backtest/:job_id", get(handlers::job_status));

    let app = Router::new()
        .nest("/api", api)
        .fallback_service(ServeDir::new("web/static"))
        .with_state(state);

    // Local-only: this tool has no auth of its own, and a trigger request
    // can spend real Kite API quota — it must never be reachable from
    // anything but this machine.
    let listener = tokio::net::TcpListener::bind("127.0.0.1:4310").await?;
    tracing::info!("papertrader-web listening on http://127.0.0.1:4310");
    axum::serve(listener, app).await?;
    Ok(())
}
