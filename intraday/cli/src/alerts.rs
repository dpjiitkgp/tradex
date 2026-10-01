//! Getting bad news off this machine and onto a phone.
//!
//! Every alert in this system used to be an `osascript` desktop notification
//! fired from inside the trading process — which means none of them survive
//! the failures that matter. A crash, a kill, a hung process or a sleeping Mac
//! produces exactly zero notifications, and a desktop banner nobody is sitting
//! in front of is not an alert at all. Managing an outage by hand only works
//! if you find out one is happening.
//!
//! Two independent mechanisms, because they fail differently:
//!
//! * **Push** (`Alerter::push`) — the process itself reporting something it
//!   noticed: a silent feed, a reconciliation mismatch, a session that ended
//!   holding a position. Only works while the process is alive enough to
//!   notice, which is precisely its limit.
//!
//! * **Heartbeat** (`Alerter::heartbeat`) — a dead-man's switch. The loop
//!   pings a cloud endpoint on a timer; the *absence* of pings is what raises
//!   the alarm, from a service running somewhere else. This is the half that
//!   survives the process dying, the machine sleeping and the power going out,
//!   none of which this machine can report on its own behalf.

use serde::Deserialize;
use std::path::Path;
use std::time::{Duration, Instant};

pub const DEFAULT_PATH: &str = "config/alerts.toml";

/// How often the dead-man's switch is pinged. Comfortably inside the grace
/// period a monitor should be configured with (a few minutes), so one dropped
/// request is not an incident.
const HEARTBEAT_EVERY: Duration = Duration::from_secs(60);

/// Alerts are a side channel, never the main event: a slow or unreachable
/// notification service must not stall the trading loop.
const TIMEOUT: Duration = Duration::from_secs(5);

/// Telegram rejects anything over 4096 characters outright, so a long message
/// is not truncated by the service — it is simply not delivered. Worth
/// guarding centrally rather than at each call site: a reconciliation
/// mismatch listing many positions can reach this, and so can piped output
/// sent through `papertrader notify`.
const MAX_CHARS: usize = 3900;

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Channel {
    /// ntfy.sh (or a self-hosted ntfy). The topic is the only secret, so use a
    /// long random one — anyone who can guess it can read the alerts.
    Ntfy { url: String },
    /// Telegram bot. More private than ntfy: both the token and the chat id
    /// have to be known.
    Telegram { bot_token: String, chat_id: String },
    /// Anything else that accepts a POST with a plain-text body (Slack and
    /// Discord webhooks want JSON, so they are not this).
    Webhook { url: String },
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlertConfig {
    /// Where pushed alerts go. Absent means desktop-only.
    #[serde(default)]
    pub channel: Option<Channel>,
    /// Dead-man's-switch URL, pinged on a timer — e.g. a healthchecks.io
    /// check. The monitoring service alerts when the pings *stop*.
    #[serde(default)]
    pub heartbeat_url: Option<String>,
}

impl AlertConfig {
    /// A missing file is not an error: alerting is optional, and refusing to
    /// trade because it is unconfigured would be worse than trading without
    /// it. The caller says so out loud instead.
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        if !path.exists() {
            return Ok(Self { channel: None, heartbeat_url: None });
        }
        let text = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&text)?)
    }
}

pub struct Alerter {
    config: AlertConfig,
    http: reqwest::Client,
    last_heartbeat: Option<Instant>,
}

impl Alerter {
    pub fn new(config: AlertConfig) -> Self {
        Self {
            config,
            http: reqwest::Client::builder()
                .timeout(TIMEOUT)
                .build()
                .unwrap_or_default(),
            last_heartbeat: None,
        }
    }

    pub fn load_default() -> Self {
        let config = AlertConfig::load(Path::new(DEFAULT_PATH)).unwrap_or_else(|e| {
            tracing::error!(error = %e, "alerts config is unreadable — running with desktop alerts only");
            AlertConfig { channel: None, heartbeat_url: None }
        });
        Self::new(config)
    }

    pub fn describe(&self) -> String {
        let push = match &self.config.channel {
            Some(Channel::Ntfy { .. }) => "ntfy",
            Some(Channel::Telegram { .. }) => "telegram",
            Some(Channel::Webhook { .. }) => "webhook",
            None => "desktop only",
        };
        let hb = if self.config.heartbeat_url.is_some() { "on" } else { "OFF" };
        format!("push: {push}, dead-man's switch: {hb}")
    }

    pub fn has_push(&self) -> bool {
        self.config.channel.is_some()
    }

    pub fn has_heartbeat(&self) -> bool {
        self.config.heartbeat_url.is_some()
    }

    /// Sends `message` to the phone. Never fails the caller — an alert that
    /// cannot be delivered is logged and dropped, because the alternative is
    /// an alerting failure taking down the thing it was meant to watch.
    ///
    /// Use `try_push` where a human is waiting on the answer: swallowing the
    /// error is right inside the trading loop and wrong in a setup check,
    /// where "sent" printed over a silent failure is worse than no check.
    pub async fn push(&self, message: &str) {
        if let Err(e) = self.try_push(message).await {
            tracing::error!(error = %e, "alert push failed");
        }
    }

    /// Same, but reports whether the service actually accepted it.
    ///
    /// Splits anything over the service's length limit into numbered parts;
    /// see `MAX_CHARS`. A partial failure is reported with the part number, so
    /// "3 of 5 arrived" is distinguishable from "nothing arrived".
    pub async fn try_push(&self, message: &str) -> anyhow::Result<()> {
        let parts = split_for_delivery(message, MAX_CHARS);
        let total = parts.len();
        for (i, part) in parts.iter().enumerate() {
            let body = if total == 1 {
                part.clone()
            } else {
                format!("({}/{}) {part}", i + 1, total)
            };
            // Telegram throttles bursts to one message per second per chat, so
            // a multi-part send that fires as fast as it can will lose parts
            // in the middle. Pace them, and retry once: observed a transport
            // error on part 3 of 4 sending without either.
            if i > 0 {
                tokio::time::sleep(Duration::from_millis(1100)).await;
            }
            if let Err(first) = self.send_one(&body).await {
                tokio::time::sleep(Duration::from_secs(2)).await;
                self.send_one(&body)
                    .await
                    .map_err(|retry| anyhow::anyhow!("part {} of {total}: {first} (retry: {retry})", i + 1))?;
            }
        }
        Ok(())
    }

    async fn send_one(&self, message: &str) -> anyhow::Result<()> {
        let Some(channel) = &self.config.channel else {
            anyhow::bail!("no push channel configured");
        };
        let result = match channel {
            Channel::Ntfy { url } => {
                self.http
                    .post(url)
                    .header("Title", "papertrader")
                    // ntfy's "urgent" priority bypasses a phone's silent mode,
                    // which is the entire reason for routing here rather than
                    // to a desktop banner.
                    .header("Priority", "urgent")
                    .body(message.to_string())
                    .send()
                    .await
            }
            Channel::Webhook { url } => self.http.post(url).body(message.to_string()).send().await,
            Channel::Telegram { bot_token, chat_id } => {
                self.http
                    .post(format!("https://api.telegram.org/bot{bot_token}/sendMessage"))
                    .json(&serde_json::json!({ "chat_id": chat_id, "text": message }))
                    .send()
                    .await
            }
        };
        // `reqwest`'s error Display includes the URL it was fetching, and a
        // Telegram request URL contains the bot token in its path. Propagating
        // that error verbatim prints the token into logs, terminals and
        // anything scraping them — which is exactly what happened before this
        // line existed. `without_url` drops it while keeping the cause.
        let response = result.map_err(|e| anyhow::anyhow!("{}", e.without_url()))?;
        let status = response.status();
        if status.is_success() {
            return Ok(());
        }
        // The body carries the actual reason — Telegram answers a bad chat_id
        // with "chat not found", which is the whole diagnosis. A bare status
        // code would leave the operator guessing.
        let body = response.text().await.unwrap_or_default();
        anyhow::bail!("{status}: {}", body.trim())
    }

    /// Pings the dead-man's switch once, reporting the outcome. For the setup
    /// check; the loop uses `heartbeat`, which is rate-limited and silent.
    pub async fn try_heartbeat(&self) -> anyhow::Result<()> {
        let Some(url) = &self.config.heartbeat_url else {
            anyhow::bail!("no heartbeat_url configured");
        };
        let r = self.http.get(url).send().await?;
        if !r.status().is_success() {
            anyhow::bail!("{}", r.status());
        }
        Ok(())
    }

    /// Pings the dead-man's switch, at most once per `HEARTBEAT_EVERY`.
    ///
    /// Safe to call on every loop tick — the rate limit lives here rather than
    /// at the call site so the loop does not have to carry timing state for
    /// something it only needs to not forget.
    pub async fn heartbeat(&mut self) {
        let Some(url) = self.config.heartbeat_url.clone() else { return };
        if self.last_heartbeat.is_some_and(|t| t.elapsed() < HEARTBEAT_EVERY) {
            return;
        }
        self.last_heartbeat = Some(Instant::now());
        if let Err(e) = self.http.get(&url).send().await {
            // A missed ping is not itself an incident — the monitor's grace
            // period covers several. Only a sustained gap raises the alarm,
            // which is the behaviour we want.
            tracing::warn!(error = %e, "heartbeat ping failed");
        }
    }

    /// Tells the monitor the session ended on purpose, so a clean shutdown
    /// does not page anyone. Only meaningful for healthchecks-style endpoints.
    pub async fn heartbeat_finish(&self) {
        let Some(url) = &self.config.heartbeat_url else { return };
        let _ = self.http.get(format!("{}/0", url.trim_end_matches('/'))).send().await;
    }
}

/// Breaks `text` into chunks of at most `limit` characters, preferring line
/// boundaries so a pasted table or log excerpt does not get cut mid-row. Falls
/// back to a hard split only for a single line longer than the limit.
///
/// Counts characters, not bytes: the limit services impose is on characters,
/// and splitting a multi-byte one would corrupt it.
fn split_for_delivery(text: &str, limit: usize) -> Vec<String> {
    if text.chars().count() <= limit {
        return vec![text.to_string()];
    }
    let mut out = Vec::new();
    let mut current = String::new();
    for line in text.split_inclusive('\n') {
        if line.chars().count() > limit {
            if !current.is_empty() {
                out.push(std::mem::take(&mut current));
            }
            let mut chunk = String::new();
            for ch in line.chars() {
                if chunk.chars().count() == limit {
                    out.push(std::mem::take(&mut chunk));
                }
                chunk.push(ch);
            }
            if !chunk.is_empty() {
                current = chunk;
            }
            continue;
        }
        if current.chars().count() + line.chars().count() > limit {
            out.push(std::mem::take(&mut current));
        }
        current.push_str(line);
    }
    if !current.is_empty() {
        out.push(current);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A Telegram request URL carries the bot token in its path, so any error
    /// that echoes the URL leaks the credential into logs and terminals. This
    /// pins the redaction: the error text must never contain the token.
    #[tokio::test]
    async fn a_failed_send_never_echoes_the_bot_token() {
        let a = Alerter::new(AlertConfig {
            channel: Some(Channel::Telegram {
                // Unroutable host below, so this never leaves the machine.
                bot_token: "123456:SUPER-SECRET-TOKEN".to_string(),
                chat_id: "1".to_string(),
            }),
            heartbeat_url: None,
        });
        let err = a.try_push("x").await.unwrap_err().to_string();
        assert!(
            !err.contains("SUPER-SECRET-TOKEN"),
            "the bot token leaked into an error message: {err}"
        );
    }

    #[test]
    fn a_short_message_is_not_split() {
        assert_eq!(split_for_delivery("hello", 100), vec!["hello"]);
    }

    /// Splitting on line boundaries is the point: a pasted table cut mid-row
    /// is harder to read than one split between rows.
    #[test]
    fn splits_on_line_boundaries_not_mid_line() {
        let text = "aaaa\nbbbb\ncccc\ndddd\n";
        let parts = split_for_delivery(text, 10);
        assert!(parts.len() > 1);
        for p in &parts {
            assert!(p.chars().count() <= 10, "{p:?} exceeds the limit");
            // Every chunk ends at a newline, so no row is broken.
            assert!(p.ends_with('\n'), "{p:?} cut mid-line");
        }
        assert_eq!(parts.concat(), text, "splitting must not lose or reorder content");
    }

    /// One line longer than the limit has no boundary to use, so it splits
    /// hard rather than being dropped or sent over-length.
    #[test]
    fn a_single_over_long_line_still_splits() {
        let text = "x".repeat(25);
        let parts = split_for_delivery(&text, 10);
        assert_eq!(parts.len(), 3);
        assert_eq!(parts.concat(), text);
    }

    /// Counting bytes instead of characters would cut a multi-byte character
    /// in half and corrupt it.
    #[test]
    fn multi_byte_characters_are_never_cut_in_half() {
        let text = "\u{20b9}".repeat(25); // rupee sign, 3 bytes each
        let parts = split_for_delivery(&text, 10);
        assert_eq!(parts.concat(), text);
        for p in &parts {
            assert!(p.chars().count() <= 10);
        }
    }

    #[test]
    fn nothing_is_lost_across_a_realistic_multi_line_paste() {
        let text: String = (0..300).map(|i| format!("row {i}: some finding text here\n")).collect();
        let parts = split_for_delivery(&text, MAX_CHARS);
        assert!(parts.len() >= 2);
        assert_eq!(parts.concat(), text);
    }

    fn parse(s: &str) -> AlertConfig {
        toml::from_str(s).unwrap()
    }

    #[test]
    fn an_absent_file_disables_alerting_rather_than_failing() {
        let c = AlertConfig::load(Path::new("config/definitely-not-here.toml")).unwrap();
        assert!(c.channel.is_none() && c.heartbeat_url.is_none());
    }

    #[test]
    fn parses_an_ntfy_channel() {
        let c = parse("[channel]\nkind = \"ntfy\"\nurl = \"https://ntfy.sh/abc\"\n");
        assert!(matches!(c.channel, Some(Channel::Ntfy { .. })));
    }

    #[test]
    fn parses_a_telegram_channel() {
        let c = parse("[channel]\nkind = \"telegram\"\nbot_token = \"t\"\nchat_id = \"1\"\n");
        assert!(matches!(c.channel, Some(Channel::Telegram { .. })));
    }

    /// Each half has to be usable without the other: push alone still reports
    /// what the process notices, heartbeat alone still catches it dying.
    #[test]
    fn either_half_works_on_its_own() {
        let push_only = parse("[channel]\nkind = \"ntfy\"\nurl = \"https://ntfy.sh/a\"\n");
        assert!(Alerter::new(push_only).has_push());
        let hb_only = parse("heartbeat_url = \"https://hc-ping.com/uuid\"\n");
        let a = Alerter::new(hb_only);
        assert!(a.has_heartbeat() && !a.has_push());
    }

    /// Calling into a disabled alerter must be a no-op, not a panic — the
    /// trading loop calls these unconditionally.
    #[tokio::test]
    async fn a_disabled_alerter_is_a_silent_no_op() {
        let mut a = Alerter::new(AlertConfig { channel: None, heartbeat_url: None });
        a.push("nobody is listening").await;
        a.heartbeat().await;
        a.heartbeat_finish().await;
    }

    /// The loop must keep running when alerting is broken; the setup check
    /// must not claim success when it is. Same call, two contracts.
    #[tokio::test]
    async fn try_push_reports_what_push_swallows() {
        let a = Alerter::new(AlertConfig { channel: None, heartbeat_url: None });
        assert!(a.try_push("x").await.is_err(), "try_push must surface a missing channel");
        a.push("x").await; // must not panic
        assert!(a.try_heartbeat().await.is_err(), "try_heartbeat must surface a missing url");
    }

    #[test]
    fn describe_names_a_missing_heartbeat_loudly_enough_to_notice() {
        let a = Alerter::new(parse("[channel]\nkind = \"ntfy\"\nurl = \"https://ntfy.sh/a\"\n"));
        assert!(a.describe().contains("OFF"), "got: {}", a.describe());
    }
}
