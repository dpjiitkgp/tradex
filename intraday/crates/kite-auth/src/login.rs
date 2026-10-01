//! Headless Kite Connect login: password -> TOTP -> walk the `connect/login`
//! redirect chain for `request_token` -> exchange for an access token.
//!
//! This automates the normal browser login flow. It's unofficial (Kite's
//! sanctioned flow expects a human in a browser) and can break without
//! notice if Zerodha adds friction (CAPTCHA, device binding) — callers must
//! treat failures here as loud, not silently-skip-the-day, per the plan.

use crate::credentials::KiteCredentials;
use crate::totp;
use secrecy::ExposeSecret;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use sha2::{Digest, Sha256};

/// Sends a request and parses its JSON body, but — unlike
/// `Response::error_for_status()` — keeps the response body on a non-2xx
/// status so Kite's actual `{"status":"error","message":...}` reaches the
/// caller instead of being discarded.
async fn send_and_parse<T: DeserializeOwned>(
    req: reqwest::RequestBuilder,
    step: &str,
) -> anyhow::Result<T> {
    let resp = req.send().await?;
    let status = resp.status();
    let body = resp.text().await?;
    if !status.is_success() {
        anyhow::bail!("Kite {step} failed ({status}): {body}");
    }
    serde_json::from_str(&body)
        .map_err(|e| anyhow::anyhow!("Kite {step} returned unexpected body: {e}\nbody: {body}"))
}

const LOGIN_URL: &str = "https://kite.zerodha.com/api/login";
const TWOFA_URL: &str = "https://kite.zerodha.com/api/twofa";
const CONNECT_LOGIN_URL: &str = "https://kite.zerodha.com/connect/login";
const TOKEN_URL: &str = "https://api.kite.trade/session/token";
const MAX_REDIRECT_HOPS: u32 = 10;

#[derive(Deserialize)]
struct LoginResponse {
    data: LoginData,
}
#[derive(Deserialize)]
struct LoginData {
    request_id: String,
    /// Kite tells us which 2FA type this specific account/session needs
    /// (e.g. "totp", "sms", "pin") — we echo it back in the twofa step
    /// rather than assuming "totp", since guessing wrong is exactly what
    /// produced the "requested 2FA type is not available" error.
    twofa_type: Option<String>,
}

#[derive(Deserialize)]
struct TokenResponse {
    data: TokenData,
}
#[derive(Deserialize)]
struct TokenData {
    access_token: String,
}

/// Runs the password + TOTP steps and walks the redirect chain, returning
/// the one-time `request_token`.
pub async fn fetch_request_token(creds: &KiteCredentials) -> anyhow::Result<String> {
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    let login_req = client.post(LOGIN_URL).form(&[
        ("user_id", creds.user_id.as_str()),
        ("password", creds.password.expose_secret()),
    ]);
    let login_resp: LoginResponse = send_and_parse(login_req, "login (password step)").await?;
    let twofa_type = login_resp.data.twofa_type.as_deref().unwrap_or("totp");
    tracing::info!(twofa_type, "Kite requested this 2FA type");

    let code = totp::current_code(&creds.totp_secret)?;
    let twofa_req = client.post(TWOFA_URL).form(&[
        ("user_id", creds.user_id.as_str()),
        ("request_id", login_resp.data.request_id.as_str()),
        ("twofa_value", code.as_str()),
        ("twofa_type", twofa_type),
    ]);
    let twofa: serde_json::Value = send_and_parse(twofa_req, "login (TOTP step)").await?;
    if twofa.get("status").and_then(|s| s.as_str()) != Some("success") {
        anyhow::bail!("Kite TOTP step did not report success: {twofa}");
    }

    // Walk connect/login's redirect chain for request_token without needing
    // the app's own redirect_url to be reachable — we only read Location
    // headers, we never actually follow through to that final host.
    let mut url = format!("{CONNECT_LOGIN_URL}?api_key={}&v=3", creds.api_key);
    for _ in 0..MAX_REDIRECT_HOPS {
        let resp = client.get(&url).send().await?;
        let location = resp
            .headers()
            .get(reqwest::header::LOCATION)
            .and_then(|v| v.to_str().ok())
            .map(str::to_string);
        let Some(location) = location else {
            anyhow::bail!(
                "Kite's connect/login did not redirect (status {}) — is the api_key correct?",
                resp.status()
            );
        };
        if let Some(token) = extract_request_token(&location) {
            return Ok(token);
        }
        url = location;
    }
    anyhow::bail!(
        "did not find request_token within {MAX_REDIRECT_HOPS} redirect hops from Kite's login"
    )
}

fn extract_request_token(location: &str) -> Option<String> {
    let url = url::Url::parse(location).ok()?;
    url.query_pairs()
        .find(|(k, _)| k == "request_token")
        .map(|(_, v)| v.into_owned())
}

/// Exchanges a `request_token` (valid once, briefly) for a daily access
/// token, per Kite's `/session/token` checksum contract.
pub async fn exchange_for_access_token(
    creds: &KiteCredentials,
    request_token: &str,
) -> anyhow::Result<String> {
    let checksum = {
        let mut hasher = Sha256::new();
        hasher.update(creds.api_key.as_bytes());
        hasher.update(request_token.as_bytes());
        hasher.update(creds.api_secret.expose_secret().as_bytes());
        hex::encode(hasher.finalize())
    };

    let client = reqwest::Client::new();
    let token_req = client
        .post(TOKEN_URL)
        .header("X-Kite-Version", "3")
        .form(&[
            ("api_key", creds.api_key.as_str()),
            ("request_token", request_token),
            ("checksum", checksum.as_str()),
        ]);
    let resp: TokenResponse = send_and_parse(token_req, "session/token exchange").await?;

    Ok(resp.data.access_token)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_request_token_from_redirect_location() {
        let loc = "https://example.com/callback?action=login&status=success&request_token=abc123";
        assert_eq!(extract_request_token(loc), Some("abc123".to_string()));
    }

    #[test]
    fn no_request_token_in_unrelated_redirect() {
        let loc = "https://kite.zerodha.com/connect/finish?api_key=xyz&sess_id=1";
        assert_eq!(extract_request_token(loc), None);
    }
}
