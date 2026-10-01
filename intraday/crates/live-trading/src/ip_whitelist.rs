//! Checks this machine's egress IP against what is whitelisted at Kite,
//! before a live session starts rather than when the first order is rejected.
//!
//! Kite has no API for reading or setting the whitelist — it is changed by
//! hand at developers.kite.trade under Profile -> IP Whitelist, and only once
//! per calendar week. So `config/ip_whitelist.toml` is a *mirror* of what was
//! entered there, maintained by hand alongside it. That makes the check only
//! as good as the file, which is the point: the file and the console are meant
//! to be updated together, and a check that disagrees with reality is itself
//! the signal that one of them was missed.
//!
//! A mismatch is fatal for a live run. Every order will be rejected, so
//! continuing produces a session that generates signals it cannot act on —
//! strictly worse than not starting, because it looks like it is working.

use serde::Deserialize;
use std::net::Ipv4Addr;
use std::path::Path;

pub const DEFAULT_PATH: &str = "config/ip_whitelist.toml";
pub const CONSOLE_URL: &str = "https://developers.kite.trade (Profile -> IP Whitelist)";

#[derive(Debug, Clone, Deserialize)]
pub struct IpWhitelist {
    /// The addresses currently entered in the Kite console. Kite accepts at
    /// most two.
    pub ips: Vec<Ipv4Addr>,
    /// When the console was last changed, so the once-per-week limit can be
    /// reasoned about before attempting another change.
    #[serde(default)]
    pub last_updated: Option<String>,
}

impl IpWhitelist {
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        let text = std::fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("reading {path:?}: {e}"))?;
        let parsed: Self = toml::from_str(&text)
            .map_err(|e| anyhow::anyhow!("parsing {path:?}: {e}"))?;
        if parsed.ips.is_empty() {
            anyhow::bail!("{path:?} lists no IPs — it must mirror what is entered at {CONSOLE_URL}");
        }
        Ok(parsed)
    }

    pub fn allows(&self, ip: Ipv4Addr) -> bool {
        self.ips.contains(&ip)
    }

    /// What to do about it, in the terms the operator actually needs: which
    /// address to enter, where, and the constraint that governs whether they
    /// can.
    pub fn mismatch_report(&self, actual: Ipv4Addr) -> String {
        let listed = self
            .ips
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        let when = match &self.last_updated {
            Some(d) => format!("\n  whitelist last changed: {d} (Kite allows ONE change per calendar week)"),
            None => String::new(),
        };
        format!(
            "egress IP does not match the Kite whitelist\n  \
             this machine: {actual}\n  \
             whitelisted:  {listed}{when}\n  \
             fix: set {actual} at {CONSOLE_URL}, then update `ips` in {DEFAULT_PATH} to match.\n  \
             Until both agree, every order will be rejected with \"IP ... is not allowed to place orders for this app\"."
        )
    }
}

/// The outcome of the pre-flight, kept separate from what a caller does about
/// it — `run_live_trade` treats `Mismatch` as fatal while `live-trade check`
/// only reports it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IpCheck {
    Match(Ipv4Addr),
    Mismatch { actual: Ipv4Addr, report: String },
    /// The address could not be determined. Not evidence of a mismatch, so
    /// never fatal on its own — refusing to trade because an unrelated echo
    /// service is down would be a self-inflicted outage.
    Unknown(String),
}

pub fn check(whitelist: &IpWhitelist, detected: anyhow::Result<Ipv4Addr>) -> IpCheck {
    match detected {
        Ok(ip) if whitelist.allows(ip) => IpCheck::Match(ip),
        Ok(ip) => IpCheck::Mismatch {
            actual: ip,
            report: whitelist.mismatch_report(ip),
        },
        Err(e) => IpCheck::Unknown(e.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wl(ips: &[&str]) -> IpWhitelist {
        IpWhitelist {
            ips: ips.iter().map(|s| s.parse().unwrap()).collect(),
            last_updated: Some("2026-09-29".to_string()),
        }
    }

    fn ip(s: &str) -> Ipv4Addr {
        s.parse().unwrap()
    }

    #[test]
    fn a_listed_address_passes() {
        assert_eq!(
            check(&wl(&["203.0.113.134"]), Ok(ip("203.0.113.134"))),
            IpCheck::Match(ip("203.0.113.134"))
        );
    }

    /// Kite accepts two addresses; either one is legitimate.
    #[test]
    fn the_second_of_two_listed_addresses_also_passes() {
        assert!(matches!(
            check(&wl(&["203.0.113.134", "198.51.100.22"]), Ok(ip("198.51.100.22"))),
            IpCheck::Match(_)
        ));
    }

    /// The realistic failure: home broadband re-dialled overnight and handed
    /// out a neighbouring address in the same block.
    #[test]
    fn a_re_dialled_address_is_caught_even_when_it_looks_similar() {
        let r = check(&wl(&["203.0.113.134"]), Ok(ip("203.0.113.201")));
        let IpCheck::Mismatch { actual, report } = r else {
            panic!("a different address must not pass");
        };
        assert_eq!(actual, ip("203.0.113.201"));
        // The report has to carry the new address to enter and where to enter
        // it, or it just tells the operator something is wrong at 09:14.
        assert!(report.contains("203.0.113.201"), "report must name the address to whitelist");
        assert!(report.contains("developers.kite.trade"), "report must say where to change it");
        assert!(report.contains(DEFAULT_PATH), "report must say to update the mirror file too");
    }

    /// Failing to learn our address says nothing about whether it matches, so
    /// it must be distinguishable from a real mismatch — the caller blocks the
    /// session on one and not the other.
    #[test]
    fn an_undetectable_address_is_unknown_rather_than_a_mismatch() {
        let r = check(&wl(&["203.0.113.134"]), Err(anyhow::anyhow!("every echo service failed")));
        assert!(matches!(r, IpCheck::Unknown(_)));
    }

    #[test]
    fn an_empty_whitelist_file_is_rejected_at_load() {
        let dir = std::env::temp_dir().join(format!("ipwl-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let p = dir.join("empty.toml");
        std::fs::write(&p, "ips = []\n").unwrap();
        let err = IpWhitelist::load(&p).unwrap_err().to_string();
        assert!(err.contains("lists no IPs"), "got: {err}");
        let _ = std::fs::remove_file(&p);
    }

    #[test]
    fn loads_a_real_file_with_two_addresses() {
        let dir = std::env::temp_dir().join(format!("ipwl2-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let p = dir.join("wl.toml");
        std::fs::write(&p, "ips = [\"1.2.3.4\", \"5.6.7.8\"]\nlast_updated = \"2026-09-29\"\n").unwrap();
        let w = IpWhitelist::load(&p).unwrap();
        assert_eq!(w.ips.len(), 2);
        assert!(w.allows(ip("5.6.7.8")));
        assert!(!w.allows(ip("9.9.9.9")));
        let _ = std::fs::remove_file(&p);
    }
}
