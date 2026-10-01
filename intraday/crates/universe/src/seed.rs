//! Nifty constituent lists — fetched live from NSE's published index CSVs
//! rather than hardcoded, since membership is rebalanced ~twice a year and
//! a stale hardcoded list would silently drift wrong. Same
//! fetch-then-cache shape as `market_calendar::holidays`, generalized over
//! which index (each gets its own cache file, since they're different
//! lists with different intended uses — Nifty 50 for the tighter live
//! trading universe, Nifty 100 for broader backtest/research data).

use market_calendar::nse_http::NseHttpClient;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeedIndex {
    Nifty50,
    Nifty100,
    /// NSE doesn't publish a plain "Nifty 250" — this is NSE's "NIFTY
    /// LargeMidcap 250" (Nifty 100 + Nifty Midcap 150), confirmed live to
    /// have exactly 250 constituents, which is what "Nifty 250" commonly
    /// refers to in practice.
    Nifty250,
}

impl SeedIndex {
    pub fn parse(s: &str) -> anyhow::Result<Self> {
        match s {
            "nifty50" => Ok(Self::Nifty50),
            "nifty100" => Ok(Self::Nifty100),
            "nifty250" => Ok(Self::Nifty250),
            other => {
                anyhow::bail!("unknown universe seed {other:?}, expected \"nifty50\" | \"nifty100\" | \"nifty250\"")
            }
        }
    }

    fn nse_url(self) -> &'static str {
        match self {
            Self::Nifty50 => "https://archives.nseindia.com/content/indices/ind_nifty50list.csv",
            Self::Nifty100 => "https://archives.nseindia.com/content/indices/ind_nifty100list.csv",
            Self::Nifty250 => "https://archives.nseindia.com/content/indices/ind_niftylargemidcap250list.csv",
        }
    }

    fn cache_path(self) -> PathBuf {
        match self {
            Self::Nifty50 => PathBuf::from("data/nifty50_constituents.json"),
            Self::Nifty100 => PathBuf::from("data/nifty100_constituents.json"),
            Self::Nifty250 => PathBuf::from("data/nifty250_constituents.json"),
        }
    }
}

#[derive(Debug, Deserialize)]
struct RawRow {
    #[serde(rename = "Symbol")]
    symbol: String,
}

#[derive(Debug, Deserialize)]
struct RawRowWithIndustry {
    #[serde(rename = "Symbol")]
    symbol: String,
    #[serde(rename = "Industry")]
    industry: String,
}

/// Same CSV as `fetch_from_nse`, but also capturing the "Industry" column
/// NSE's index constituent lists carry — needed for the web UI's
/// sector-scoped backtesting, not used by the plain symbol-set callers
/// (screening, backfill, collect), which is why this is a separate
/// function rather than changing what `fetch_from_nse` returns.
pub async fn fetch_with_industry_from_nse(
    client: &NseHttpClient,
    index: SeedIndex,
) -> anyhow::Result<Vec<(String, String)>> {
    let url = index.nse_url();
    let csv_text = client.get_text(url).await?;
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(csv_text.as_bytes());

    let mut out = Vec::new();
    for record in reader.deserialize::<RawRowWithIndustry>() {
        let row = record.map_err(|e| anyhow::anyhow!("parsing {index:?} constituent CSV row: {e}"))?;
        out.push((row.symbol, row.industry));
    }
    if out.is_empty() {
        anyhow::bail!("parsed zero symbols from {url} — NSE's CSV shape may have changed");
    }
    Ok(out)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedCache {
    pub fetched_at: chrono::DateTime<chrono::Utc>,
    pub symbols: BTreeSet<String>,
}

pub async fn fetch_from_nse(client: &NseHttpClient, index: SeedIndex) -> anyhow::Result<BTreeSet<String>> {
    let url = index.nse_url();
    let csv_text = client.get_text(url).await?;
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(csv_text.as_bytes());

    let mut symbols = BTreeSet::new();
    for record in reader.deserialize::<RawRow>() {
        let row = record.map_err(|e| anyhow::anyhow!("parsing {index:?} constituent CSV row: {e}"))?;
        symbols.insert(row.symbol);
    }
    if symbols.is_empty() {
        anyhow::bail!("parsed zero symbols from {url} — NSE's CSV shape may have changed");
    }
    Ok(symbols)
}

pub fn load_cache(path: &Path) -> Option<SeedCache> {
    let text = std::fs::read_to_string(path).ok()?;
    match serde_json::from_str(&text) {
        Ok(cache) => Some(cache),
        Err(e) => {
            tracing::warn!(?path, error = %e, "seed cache is unreadable, ignoring");
            None
        }
    }
}

pub fn save_cache(path: &Path, cache: &SeedCache) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, serde_json::to_string_pretty(cache)?)?;
    Ok(())
}

/// Loads the cached constituent list (no network). Use `refresh_default`
/// to populate/update it.
pub fn load_default(index: SeedIndex) -> Option<BTreeSet<String>> {
    load_cache(&index.cache_path()).map(|c| c.symbols)
}

pub async fn refresh_default(index: SeedIndex) -> anyhow::Result<BTreeSet<String>> {
    let client = NseHttpClient::new()?;
    let symbols = fetch_from_nse(&client, index).await?;
    let cache = SeedCache {
        fetched_at: chrono::Utc::now(),
        symbols: symbols.clone(),
    };
    save_cache(&index.cache_path(), &cache)?;
    Ok(symbols)
}
