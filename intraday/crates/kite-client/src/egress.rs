//! What public IPv4 address our requests leave from — the one Zerodha checks
//! against the whitelist before allowing an order.
//!
//! Zerodha gates order placement on a whitelisted IP, and the console takes a
//! static IPv4 address. On dynamic home broadband that address changes
//! whenever the line re-dials, and the first symptom is an order rejected
//! mid-session with "IP (...) is not allowed to place orders for this app" —
//! by which point a signal has been missed and the whitelist cannot be
//! corrected quickly, because Kite permits only one change per calendar week.
//!
//! So the address is resolved at startup instead, over the same IPv4-only
//! binding `KiteRestClient` uses. Using the default client here would defeat
//! the point: `api.kite.trade` resolves to both families, and a lookup that
//! happened to go out over IPv6 would report an address Kite never sees.

use std::net::Ipv4Addr;
use std::time::Duration;

/// Plain-text echo services, tried in order. Several, because this runs on the
/// critical path of starting a live session: one of them being down must not
/// be the reason a trading day does not start.
const ECHO_SERVICES: &[&str] = &[
    "https://api.ipify.org",
    "https://ipv4.icanhazip.com",
    "https://checkip.amazonaws.com",
];

const TIMEOUT: Duration = Duration::from_secs(5);

/// Resolves this machine's public IPv4 address, or explains why it could not.
///
/// The error is deliberately not fatal to callers: failing to *learn* the
/// address is not evidence of a mismatch, and refusing to trade because an
/// unrelated echo service is unreachable would be its own outage.
pub async fn public_ipv4() -> anyhow::Result<Ipv4Addr> {
    let http = reqwest::Client::builder()
        .local_address(std::net::IpAddr::V4(Ipv4Addr::UNSPECIFIED))
        .timeout(TIMEOUT)
        .build()?;

    let mut failures = Vec::new();
    for url in ECHO_SERVICES {
        match fetch_ipv4(&http, url).await {
            Ok(ip) => return Ok(ip),
            Err(e) => failures.push(format!("{url}: {e}")),
        }
    }
    anyhow::bail!("every IP echo service failed — {}", failures.join("; "))
}

async fn fetch_ipv4(http: &reqwest::Client, url: &str) -> anyhow::Result<Ipv4Addr> {
    let body = http.get(url).send().await?.error_for_status()?.text().await?;
    parse_ipv4(&body)
}

/// These services answer with a bare address and, in at least one case, a
/// trailing newline — so the body is trimmed rather than parsed as-is.
fn parse_ipv4(body: &str) -> anyhow::Result<Ipv4Addr> {
    let trimmed = body.trim();
    trimmed
        .parse::<Ipv4Addr>()
        .map_err(|e| anyhow::anyhow!("{trimmed:?} is not an IPv4 address: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_bare_address() {
        assert_eq!(parse_ipv4("203.0.113.134").unwrap(), Ipv4Addr::new(203, 0, 113, 134));
    }

    #[test]
    fn tolerates_the_trailing_newline_icanhazip_sends() {
        assert_eq!(parse_ipv4("203.0.113.134\n").unwrap(), Ipv4Addr::new(203, 0, 113, 134));
    }

    /// An IPv6 answer means the request left over the wrong family despite the
    /// v4 binding — reporting it as "our IP" would compare an address Kite
    /// never sees against the whitelist, so it has to be an error.
    #[test]
    fn rejects_an_ipv6_answer_rather_than_reporting_it() {
        assert!(parse_ipv4("2409:4080:1e9f:8d1c::1").is_err());
    }

    #[test]
    fn rejects_an_html_error_page() {
        assert!(parse_ipv4("<html><body>503</body></html>").is_err());
    }
}
