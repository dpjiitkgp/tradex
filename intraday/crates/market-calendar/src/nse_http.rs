//! WAF-cookie-priming HTTP client for nseindia.com — the Rust equivalent of
//! `tradex/nse_client.py`: NSE requires a browser-ish User-Agent and cookies
//! picked up from the landing pages before it will serve API endpoints.

use std::time::Duration;

const UA: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 \
     (KHTML, like Gecko) Chrome/125.0 Safari/537.36";

const MAX_RETRIES: u32 = 4;
const RETRY_BACKOFF_SEC: u64 = 3;

pub struct NseHttpClient {
    client: reqwest::Client,
}

impl NseHttpClient {
    pub fn new() -> anyhow::Result<Self> {
        let client = reqwest::Client::builder()
            .user_agent(UA)
            .cookie_store(true)
            .timeout(Duration::from_secs(30))
            .build()?;
        Ok(Self { client })
    }

    async fn prime(&self) {
        // Best-effort: NSE's WAF hands out session cookies from the landing
        // pages. Not fatal if this fails — some endpoints still respond.
        let _ = self.client.get("https://www.nseindia.com").send().await;
        let _ = self
            .client
            .get("https://www.nseindia.com/all-reports")
            .send()
            .await;
    }

    /// GET `url` and return the body text, retrying transient failures
    /// (timeouts, a stale-cookie 403, 5xx) with re-priming in between.
    pub async fn get_text(&self, url: &str) -> anyhow::Result<String> {
        self.prime().await;

        let mut last_err: Option<anyhow::Error> = None;
        for attempt in 1..=MAX_RETRIES {
            match self.client.get(url).send().await {
                Ok(resp) if resp.status() == reqwest::StatusCode::FORBIDDEN => {
                    self.prime().await;
                    last_err = Some(anyhow::anyhow!("403 from NSE (re-primed)"));
                }
                Ok(resp) if resp.status().is_success() => {
                    return Ok(resp.text().await?);
                }
                Ok(resp) => {
                    last_err = Some(anyhow::anyhow!("NSE returned {}", resp.status()));
                }
                Err(e) => last_err = Some(e.into()),
            }
            if attempt < MAX_RETRIES {
                tokio::time::sleep(Duration::from_secs(RETRY_BACKOFF_SEC * attempt as u64)).await;
            }
        }
        Err(anyhow::anyhow!(
            "failed to fetch {url} after {MAX_RETRIES} tries: {last_err:?}"
        ))
    }

    pub async fn get_json(&self, url: &str) -> anyhow::Result<serde_json::Value> {
        let text = self.get_text(url).await?;
        Ok(serde_json::from_str(&text)?)
    }
}
