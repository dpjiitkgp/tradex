//! Populates `instruments.sector` from NSE's Nifty 250 (LargeMidcap 250)
//! constituent list the first time it's needed — everything after that is
//! a local SQLite read, no network involved.

use crate::state::AppState;

pub async fn ensure_synced(state: &AppState) -> anyhow::Result<()> {
    if state.sector_sync.is_done() {
        return Ok(());
    }

    let storage = state.open_storage()?;
    if !storage.distinct_sectors()?.is_empty() {
        state.sector_sync.mark_done();
        return Ok(());
    }

    tracing::info!("no sectors tagged yet — syncing from NSE's Nifty 250 list");
    let index = universe::SeedIndex::Nifty250;
    let seed = match universe::seed::load_default(index) {
        Some(s) => s,
        None => universe::seed::refresh_default(index).await?,
    };

    let nse_client = market_calendar::nse_http::NseHttpClient::new()?;
    let with_industry = universe::seed::fetch_with_industry_from_nse(&nse_client, index).await?;

    let rest = crate::kite::rest_client(&state.credentials_path).await?;
    let all_equities = kite_client::instruments::fetch_nse_equities(&rest).await?;
    let matched: Vec<_> = all_equities.into_iter().filter(|i| seed.contains(&i.tradingsymbol)).collect();

    storage.upsert_instruments(&matched)?;
    storage.upsert_sectors(&with_industry)?;
    tracing::info!(count = matched.len(), "sector sync complete");

    state.sector_sync.mark_done();
    Ok(())
}
