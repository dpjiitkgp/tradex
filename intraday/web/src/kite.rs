//! Authenticated Kite REST client construction — same shape as the CLI's
//! `kite_rest_client()`. Auth is effectively free to call repeatedly:
//! `KiteLoginProvider` caches the access token for the rest of the IST
//! day, so this only actually talks to Kite once per morning regardless
//! of how many times a handler calls it.

use core_types::LoginProvider;
use std::path::Path;

pub async fn rest_client(credentials_path: &Path) -> anyhow::Result<kite_client::KiteRestClient> {
    let provider = kite_auth::KiteLoginProvider::load_default(credentials_path)?;
    let api_key = provider.api_key().to_string();
    let token = provider.fresh_access_token().await.map_err(|e| anyhow::anyhow!(e))?;
    kite_client::KiteRestClient::new(api_key, token.0)
}
