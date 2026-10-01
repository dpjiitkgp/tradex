//! Fetching, caching, and manually overriding NSE's equity (CM segment)
//! trading holiday list. Unlike `tradex/calendar_utils.py`'s reactive
//! "record a miss after a failed download" approach, this must be
//! forward-looking: the scheduler needs to know *before* 09:15 whether
//! today is a trading day, so holidays are fetched ahead of time and cached.

use crate::nse_http::NseHttpClient;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::path::Path;

const HOLIDAY_MASTER_URL: &str = "https://www.nseindia.com/api/holiday-master?type=trading";

#[derive(Debug, Serialize, Deserialize)]
pub struct HolidayCache {
    pub fetched_at: chrono::DateTime<chrono::Utc>,
    pub holidays: BTreeSet<NaiveDate>,
}

#[derive(Debug, Default, Deserialize)]
pub struct HolidayOverride {
    #[serde(default)]
    pub add: BTreeSet<NaiveDate>,
    #[serde(default)]
    pub remove: BTreeSet<NaiveDate>,
}

/// Fetches the live NSE equity (CM) holiday list. NSE's WAF/API shape has
/// changed before (see `tradex/fo_bhavcopy.py`'s UDiFF-cutover note) — if
/// this stops parsing, check the current response shape against
/// `HOLIDAY_MASTER_URL` before assuming the trading calendar itself changed.
pub async fn fetch_from_nse(client: &NseHttpClient) -> anyhow::Result<BTreeSet<NaiveDate>> {
    let body = client.get_json(HOLIDAY_MASTER_URL).await?;
    let entries = body
        .get("CM")
        .and_then(|v| v.as_array())
        .ok_or_else(|| anyhow::anyhow!("unexpected NSE holiday-master response shape: {body}"))?;

    let mut dates = BTreeSet::new();
    for entry in entries {
        let raw = entry
            .get("tradingDate")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("holiday entry missing tradingDate: {entry}"))?;
        let date = NaiveDate::parse_from_str(raw, "%d-%b-%Y")
            .map_err(|e| anyhow::anyhow!("unparseable NSE holiday date {raw:?}: {e}"))?;
        dates.insert(date);
    }
    Ok(dates)
}

pub fn load_cache(path: &Path) -> Option<HolidayCache> {
    let text = std::fs::read_to_string(path).ok()?;
    match serde_json::from_str(&text) {
        Ok(cache) => Some(cache),
        Err(e) => {
            tracing::warn!(?path, error = %e, "holiday cache is unreadable, ignoring");
            None
        }
    }
}

pub fn save_cache(path: &Path, cache: &HolidayCache) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, serde_json::to_string_pretty(cache)?)?;
    Ok(())
}

pub fn load_override(path: &Path) -> HolidayOverride {
    match std::fs::read_to_string(path) {
        Ok(text) => toml::from_str(&text).unwrap_or_else(|e| {
            tracing::warn!(?path, error = %e, "holiday override file is invalid, ignoring");
            HolidayOverride::default()
        }),
        Err(_) => HolidayOverride::default(),
    }
}

/// Cached holidays with the manual override applied.
pub fn resolve(cache: Option<&HolidayCache>, over: &HolidayOverride) -> BTreeSet<NaiveDate> {
    let mut set: BTreeSet<NaiveDate> = cache.map(|c| c.holidays.clone()).unwrap_or_default();
    for d in &over.add {
        set.insert(*d);
    }
    for d in &over.remove {
        set.remove(d);
    }
    set
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(s: &str) -> NaiveDate {
        NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap()
    }

    #[test]
    fn override_add_and_remove_apply_on_top_of_cache() {
        let cache = HolidayCache {
            fetched_at: chrono::Utc::now(),
            holidays: BTreeSet::from([date("2026-01-26"), date("2026-08-15")]),
        };
        let over = HolidayOverride {
            add: BTreeSet::from([date("2026-12-25")]),
            remove: BTreeSet::from([date("2026-08-15")]),
        };
        let resolved = resolve(Some(&cache), &over);
        assert!(resolved.contains(&date("2026-01-26")));
        assert!(resolved.contains(&date("2026-12-25")));
        assert!(!resolved.contains(&date("2026-08-15")));
    }

    #[test]
    fn missing_cache_with_no_override_is_empty() {
        let resolved = resolve(None, &HolidayOverride::default());
        assert!(resolved.is_empty());
    }
}
