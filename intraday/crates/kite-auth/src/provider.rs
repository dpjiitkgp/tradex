use crate::credentials::KiteCredentials;
use crate::login;
use async_trait::async_trait;
use core_types::time::IST;
use core_types::{AccessToken, LoginProvider};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
struct TokenCache {
    access_token: String,
    minted_at: chrono::DateTime<chrono::Utc>,
    /// The IST trading date this token was minted for — Kite access tokens
    /// are valid until the next day's reset, so any cache hit outside this
    /// date must not be reused.
    ist_date: chrono::NaiveDate,
}

/// How long a cached token is trusted without re-validating. Kite's own
/// docs describe access tokens as valid for the rest of the trading day,
/// but that was NOT what was observed in practice: on 2026-09-25, tokens
/// started failing with TokenException/PermissionException on every
/// endpoint (including plain quotes) after roughly 2-3.5 hours, well
/// within the same IST calendar day. Trusting the cache for the whole day
/// (the original design) risks a live daemon reusing an already-dead
/// token right at the moment it wakes up after a long overnight sleep —
/// exactly the scenario that broke this session's backfill twice. A
/// short, conservative window forces a fresh login far more often than
/// Kite's documented behavior would require, but that's a cheap cost next
/// to a live session silently starting on a dead token.
const CACHE_FRESHNESS: chrono::Duration = chrono::Duration::minutes(60);

/// `LoginProvider` implementation backed by the headless Kite login flow in
/// `login.rs`. Caches the access token for the rest of the IST trading day
/// so a restart doesn't force a fresh login (and doesn't risk hammering
/// Kite's login endpoint).
pub struct KiteLoginProvider {
    creds: KiteCredentials,
    cache_path: PathBuf,
}

impl KiteLoginProvider {
    pub fn new(creds: KiteCredentials, cache_path: PathBuf) -> Self {
        Self { creds, cache_path }
    }

    pub fn load_default(credentials_path: &Path) -> anyhow::Result<Self> {
        let creds = KiteCredentials::load(credentials_path)?;
        Ok(Self::new(creds, PathBuf::from("data/kite_access_token.json")))
    }

    pub fn api_key(&self) -> &str {
        &self.creds.api_key
    }

    fn today_ist() -> chrono::NaiveDate {
        chrono::Utc::now().with_timezone(&IST).date_naive()
    }

    fn cached_for_today(&self) -> Option<String> {
        let text = std::fs::read_to_string(&self.cache_path).ok()?;
        let cache: TokenCache = serde_json::from_str(&text).ok()?;
        let age = chrono::Utc::now() - cache.minted_at;
        if cache.ist_date == Self::today_ist() && age <= CACHE_FRESHNESS {
            Some(cache.access_token)
        } else {
            None
        }
    }

    fn save_cache(&self, access_token: &str) -> anyhow::Result<()> {
        if let Some(parent) = self.cache_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let cache = TokenCache {
            access_token: access_token.to_string(),
            minted_at: chrono::Utc::now(),
            ist_date: Self::today_ist(),
        };
        std::fs::write(&self.cache_path, serde_json::to_string_pretty(&cache)?)?;
        Ok(())
    }
}

impl KiteLoginProvider {
    /// Logs in afresh, ignoring the cache entirely.
    ///
    /// Needed for recovering from an *invalidated* token, as opposed to an
    /// expired one. Kite shares one session between the API and the Kite
    /// Web/mobile apps, so logging into Kite Web kills the API token — and it
    /// kills the copy sitting in our cache file at the same time. Reusing the
    /// cache there would just fail again, so this path must skip it.
    pub async fn force_new_access_token(&self) -> core_types::Result<AccessToken> {
        tracing::warn!("forcing a fresh Kite login (cached token is invalid, not merely stale)");
        let request_token = login::fetch_request_token(&self.creds)
            .await
            .map_err(|e| core_types::Error::Auth(format!("login: {e}")))?;
        let access_token = login::exchange_for_access_token(&self.creds, &request_token)
            .await
            .map_err(|e| core_types::Error::Auth(format!("token exchange: {e}")))?;
        if let Err(e) = self.save_cache(&access_token) {
            tracing::warn!(error = %e, "failed to cache the refreshed Kite access token");
        }
        Ok(AccessToken(access_token))
    }
}

#[async_trait]
impl LoginProvider for KiteLoginProvider {
    async fn fresh_access_token(&self) -> core_types::Result<AccessToken> {
        if let Some(token) = self.cached_for_today() {
            tracing::info!("reusing cached Kite access token for today");
            return Ok(AccessToken(token));
        }

        tracing::info!("no cached token for today — running headless Kite login");
        let request_token = login::fetch_request_token(&self.creds)
            .await
            .map_err(|e| core_types::Error::Auth(format!("login: {e}")))?;
        let access_token = login::exchange_for_access_token(&self.creds, &request_token)
            .await
            .map_err(|e| core_types::Error::Auth(format!("token exchange: {e}")))?;

        if let Err(e) = self.save_cache(&access_token) {
            // Non-fatal: we still have a valid token this run, just won't
            // avoid re-login next process start today.
            tracing::warn!(error = %e, "failed to cache Kite access token");
        }

        Ok(AccessToken(access_token))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_provider(cache_path: PathBuf) -> KiteLoginProvider {
        let creds = KiteCredentials {
            api_key: "test_key".to_string(),
            api_secret: "test_secret".to_string().into(),
            user_id: "TEST".to_string(),
            password: "pw".to_string().into(),
            totp_secret: "secret".to_string().into(),
        };
        KiteLoginProvider::new(creds, cache_path)
    }

    fn write_cache_aged_by(path: &Path, age: chrono::Duration) {
        let cache = TokenCache {
            access_token: "cached_token".to_string(),
            minted_at: chrono::Utc::now() - age,
            ist_date: KiteLoginProvider::today_ist(),
        };
        std::fs::write(path, serde_json::to_string_pretty(&cache).unwrap()).unwrap();
    }

    fn temp_cache_path(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("kite_auth_provider_test_{name}_{}.json", std::process::id()))
    }

    #[test]
    fn a_fresh_cached_token_is_reused() {
        let path = temp_cache_path("fresh");
        write_cache_aged_by(&path, chrono::Duration::minutes(10));
        let provider = dummy_provider(path.clone());
        assert_eq!(provider.cached_for_today(), Some("cached_token".to_string()));
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn a_token_older_than_the_freshness_window_is_not_reused() {
        let path = temp_cache_path("stale");
        write_cache_aged_by(&path, CACHE_FRESHNESS + chrono::Duration::minutes(1));
        let provider = dummy_provider(path.clone());
        assert_eq!(provider.cached_for_today(), None);
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn a_token_from_a_previous_ist_date_is_not_reused_even_if_recently_minted() {
        let path = temp_cache_path("stale_date");
        let cache = TokenCache {
            access_token: "cached_token".to_string(),
            minted_at: chrono::Utc::now(),
            ist_date: KiteLoginProvider::today_ist() - chrono::Duration::days(1),
        };
        std::fs::write(&path, serde_json::to_string_pretty(&cache).unwrap()).unwrap();
        let provider = dummy_provider(path.clone());
        assert_eq!(provider.cached_for_today(), None);
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn no_cache_file_is_a_clean_miss() {
        let path = temp_cache_path("missing");
        let _ = std::fs::remove_file(&path); // ensure it doesn't exist
        let provider = dummy_provider(path);
        assert_eq!(provider.cached_for_today(), None);
    }
}
