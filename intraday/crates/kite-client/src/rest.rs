//! Thin wrapper around Kite Connect's REST API: auth headers plus the same
//! "keep the error body, don't discard it" pattern used in
//! `kite-auth::login` — Kite's error responses (`{"status":"error",
//! "message":...,"error_type":...}`) are far more useful than a bare status
//! code, especially for permission errors on entitlement-gated endpoints
//! like historical data.

use serde::de::DeserializeOwned;

const BASE_URL: &str = "https://api.kite.trade";

pub struct KiteRestClient {
    http: reqwest::Client,
    api_key: String,
    /// Swappable, because a Kite access token can be invalidated *while a
    /// session is running* and the process must be able to recover in place.
    ///
    /// Kite shares one authentication session between the API and the Kite
    /// Web/mobile apps, so simply logging into Kite Web to look at a position
    /// kills the API token. On 2026-09-30 that happened at 15:17 — the
    /// operator opened Kite to check that a trade had closed — and every
    /// subsequent broker call in the live engine failed. Holding the token by
    /// value meant there was no way to repair it short of restarting.
    ///
    /// `RwLock` rather than `Arc<str>` swap: reads are per-request and
    /// uncontended, writes are once in a blue moon.
    access_token: std::sync::RwLock<String>,
    /// Supplies a fresh token on demand. `None` disables recovery, which is
    /// right for one-shot CLI commands where a failure should just surface.
    refresher: Option<TokenRefresher>,
}

/// Performs a fresh login and returns a new access token.
///
/// A boxed async closure rather than a dependency on `kite-auth`: this crate
/// deliberately knows nothing about credentials or headless login, and the
/// binary that owns both wires them together.
pub type TokenRefresher = Box<
    dyn Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<String>> + Send>>
        + Send
        + Sync,
>;

/// Whether a Kite response body is the "your session is gone" error, as
/// opposed to any other failure.
///
/// Matched on the message because Kite returns it as a 400 `InputException`
/// rather than a 401/403, so the status code alone cannot distinguish it from
/// a malformed order.
pub fn is_auth_failure(body: &str) -> bool {
    body.contains("Invalid `api_key` or `access_token`")
        || body.contains("Invalid api_key or access_token")
        || body.contains("TokenException")
}

impl KiteRestClient {
    pub fn new(api_key: impl Into<String>, access_token: impl Into<String>) -> anyhow::Result<Self> {
        Ok(Self {
            // Bound to the IPv4 wildcard so every request leaves over IPv4.
            //
            // Zerodha gates order placement on a whitelisted IP, and the
            // console accepts a *static* address — in practice an IPv4 one.
            // `api.kite.trade` resolves to both families, so without this the
            // egress family is whatever DNS happens to return first: orders
            // silently fail with "IP (2409:...) is not allowed to place
            // orders for this app" even though the correct IPv4 address is
            // whitelisted. Observed live on 2026-09-29, where the same order
            // was rejected over IPv6 and accepted over IPv4 seconds apart.
            //
            // A dynamic IPv6 address cannot be whitelisted usefully anyway:
            // privacy-extension addresses rotate, and the exchange permits
            // only one whitelist change per calendar week.
            http: reqwest::Client::builder()
                .local_address(std::net::IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED))
                .build()?,
            api_key: api_key.into(),
            access_token: std::sync::RwLock::new(access_token.into()),
            refresher: None,
        })
    }

    /// Enables in-place recovery from an invalidated token. Long-running
    /// processes (the live engine, the collector) should always set this; a
    /// one-shot command should not, so that an auth problem is reported rather
    /// than silently papered over.
    pub fn with_token_refresher(mut self, refresher: TokenRefresher) -> Self {
        self.refresher = Some(refresher);
        self
    }

    /// Re-authenticates and swaps the new token in. Returns `false` when no
    /// refresher is configured, so callers can tell "could not recover" from
    /// "recovered, try again".
    pub async fn refresh_access_token(&self) -> anyhow::Result<bool> {
        let Some(refresher) = &self.refresher else {
            return Ok(false);
        };
        tracing::warn!("Kite rejected the access token — re-authenticating in place");
        let fresh = refresher().await?;
        *self
            .access_token
            .write()
            .map_err(|_| anyhow::anyhow!("access-token lock poisoned"))? = fresh;
        tracing::info!("access token refreshed; retrying the failed request");
        Ok(true)
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    pub fn access_token(&self) -> String {
        self.access_token.read().map(|t| t.clone()).unwrap_or_default()
    }

    fn authorize(&self, req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        req.header(
            "Authorization",
            format!("token {}:{}", self.api_key, self.access_token()),
        )
        .header("X-Kite-Version", "3")
    }


    /// Sends `build()`'s request; on an auth failure, re-authenticates and
    /// sends it once more.
    ///
    /// Every endpoint goes through here rather than each one retrying itself:
    /// the token can die between any two calls, and an engine that recovers on
    /// `order_status` but not on `place_order` is barely better than one that
    /// does not recover at all.
    ///
    /// Exactly one retry. If the fresh token is also rejected, something is
    /// wrong that retrying will not fix, and a loop would hammer the login
    /// endpoint during a live session.
    async fn send_with_auth_retry(
        &self,
        build: impl Fn() -> reqwest::RequestBuilder,
    ) -> anyhow::Result<(reqwest::StatusCode, String)> {
        let resp = self.authorize(build()).send().await?;
        let status = resp.status();
        let body = resp.text().await?;
        if !is_auth_failure(&body) {
            return Ok((status, body));
        }
        if !self.refresh_access_token().await? {
            return Ok((status, body));
        }
        let resp = self.authorize(build()).send().await?;
        let status = resp.status();
        let body = resp.text().await?;
        Ok((status, body))
    }

    pub async fn get_json<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, &str)],
    ) -> anyhow::Result<T> {
        let url = format!("{BASE_URL}{path}");
        let (status, body) = self
            .send_with_auth_retry(|| self.http.get(&url).query(query))
            .await?;
        if !status.is_success() {
            anyhow::bail!("Kite GET {path} failed ({status}): {body}");
        }
        serde_json::from_str(&body)
            .map_err(|e| anyhow::anyhow!("Kite GET {path} returned unexpected body: {e}\nbody: {body}"))
    }

    pub async fn get_text(&self, path: &str) -> anyhow::Result<String> {
        let url = format!("{BASE_URL}{path}");
        let (status, body) = self.send_with_auth_retry(|| self.http.get(&url)).await?;
        if !status.is_success() {
            anyhow::bail!("Kite GET {path} failed ({status}): {body}");
        }
        Ok(body)
    }

    /// JSON-body POST — a handful of Kite endpoints (the margin calculator
    /// among them) take a JSON array/object body instead of the
    /// form-encoding every order-placement endpoint uses. Same
    /// don't-bail-on-non-2xx contract as `post_form`, for the same reason.
    pub async fn post_json_body(
        &self,
        path: &str,
        body: &serde_json::Value,
    ) -> anyhow::Result<(reqwest::StatusCode, String)> {
        let url = format!("{BASE_URL}{path}");
        self.send_with_auth_retry(|| self.http.post(&url).json(body)).await
    }

    /// Form-encoded POST — every Kite order-placement/margin-check endpoint
    /// takes `application/x-www-form-urlencoded`, not JSON. Unlike
    /// `get_json`/`get_text`, this does *not* bail on a non-2xx status: an
    /// order rejection (insufficient margin, RMS block, bad price) is a
    /// meaningful business result carried in the body alongside a 4xx/5xx
    /// status, not a transport failure — the caller needs both the status
    /// and the body to tell "order rejected" apart from "network/auth
    /// broke," so both are handed back uninterpreted.
    pub async fn post_form(
        &self,
        path: &str,
        form: &[(&str, &str)],
    ) -> anyhow::Result<(reqwest::StatusCode, String)> {
        let url = format!("{BASE_URL}{path}");
        self.send_with_auth_retry(|| self.http.post(&url).form(form)).await
    }

    /// Form-encoded PUT — order modification.
    pub async fn put_form(
        &self,
        path: &str,
        form: &[(&str, &str)],
    ) -> anyhow::Result<(reqwest::StatusCode, String)> {
        let url = format!("{BASE_URL}{path}");
        self.send_with_auth_retry(|| self.http.put(&url).form(form)).await
    }

    /// DELETE — order cancellation.
    pub async fn delete(&self, path: &str) -> anyhow::Result<(reqwest::StatusCode, String)> {
        let url = format!("{BASE_URL}{path}");
        self.send_with_auth_retry(|| self.http.delete(&url)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Kite returns "your session is gone" as a 400 `InputException`, not a
    /// 401 — so the status code cannot distinguish it from a bad order. If
    /// this detection breaks, the live engine stops recovering from a token
    /// invalidated by a Kite Web login, which is how 2026-09-30 went wrong.
    #[test]
    fn recognises_the_real_kite_auth_failure_body() {
        // Verbatim from the 2026-09-30 live log.
        let body = r#"{"status":"error","message":"Invalid `api_key` or `access_token`.","data":null,"error_type":"InputException"}"#;
        assert!(is_auth_failure(body));
    }

    #[test]
    fn recognises_a_token_exception() {
        assert!(is_auth_failure(
            r#"{"status":"error","message":"Token is invalid or has expired","error_type":"TokenException"}"#
        ));
    }

    /// Must NOT treat ordinary rejections as auth problems: re-authenticating
    /// and retrying a genuinely bad order would place it twice.
    #[test]
    fn does_not_mistake_business_rejections_for_auth_failures() {
        for body in [
            r#"{"status":"error","message":"Intraday orders (MIS) are allowed only till 3:12 PM. Try placing a CNC order.","error_type":"InputException"}"#,
            r#"{"status":"error","message":"Insufficient funds for this order","error_type":"InputException"}"#,
            r#"{"status":"error","message":"IP (2409:...) is not allowed to place orders for this app","error_type":"InputException"}"#,
            r#"{"status":"success","data":{"order_id":"123"}}"#,
        ] {
            assert!(!is_auth_failure(body), "false positive on: {body}");
        }
    }

    /// Without a refresher there is nothing to recover with, and the caller
    /// must be able to tell that apart from a successful refresh.
    #[tokio::test]
    async fn a_client_without_a_refresher_reports_that_it_cannot_recover() {
        let c = KiteRestClient::new("k", "t").unwrap();
        assert!(!c.refresh_access_token().await.unwrap());
        assert_eq!(c.access_token(), "t");
    }

    #[tokio::test]
    async fn a_refresher_swaps_the_token_in_place() {
        let c = KiteRestClient::new("k", "stale")
            .unwrap()
            .with_token_refresher(Box::new(|| Box::pin(async { Ok("fresh".to_string()) })));
        assert!(c.refresh_access_token().await.unwrap());
        assert_eq!(c.access_token(), "fresh", "the new token must be used by later requests");
    }
}
