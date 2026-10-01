use crate::config::EngineConfig;
use chrono::NaiveTime;
use core_types::TrailPolicy;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct RawRisk {
    risk_pct: f64,
    min_reward_multiple: f64,
    trail_policy: String,
    trail_param: f64,
    eod_square_off: String,
    max_hold_minutes: i64,
    daily_loss_cap_pct: f64,
    exit_atr_period: u32,
    capital_per_trade: f64,
    max_concurrent_positions: usize,
}

#[derive(Debug, Deserialize)]
struct RootConfig {
    risk: RawRisk,
}

/// Public so other crates building an `EngineConfig` from their own parsed
/// input (e.g. the web UI's trigger form) can reuse the same mapping
/// instead of re-deriving it.
pub fn parse_trail_policy(kind: &str, param: f64) -> anyhow::Result<TrailPolicy> {
    match kind {
        "percent_from_peak" => Ok(TrailPolicy::PercentFromPeak(param)),
        "atr_multiple" => Ok(TrailPolicy::AtrMultiple(param)),
        "vwap_cross" => Ok(TrailPolicy::VwapCross),
        "give_back_from_peak" => Ok(TrailPolicy::GiveBackFromPeak(param)),
        other => anyhow::bail!(
            "unknown trail_policy {other:?} — expected percent_from_peak | atr_multiple | vwap_cross | give_back_from_peak"
        ),
    }
}

/// Also returns the raw JSON of the `[risk]` section, for `runs.config_snapshot_json`
/// so a backtest/live run is reproducible from its stored config alone.
pub fn load(path: &Path) -> anyhow::Result<(EngineConfig, String)> {
    let text = std::fs::read_to_string(path).map_err(|e| anyhow::anyhow!("reading {path:?}: {e}"))?;
    let root: RootConfig =
        toml::from_str(&text).map_err(|e| anyhow::anyhow!("parsing [risk] section of {path:?}: {e}"))?;
    let raw = root.risk;

    let config = EngineConfig {
        risk_pct: raw.risk_pct,
        min_reward_multiple: raw.min_reward_multiple,
        trail_policy: parse_trail_policy(&raw.trail_policy, raw.trail_param)?,
        capital_per_trade: raw.capital_per_trade,
        max_concurrent_positions: raw.max_concurrent_positions,
        daily_loss_cap_pct: raw.daily_loss_cap_pct,
        eod_square_off: NaiveTime::parse_from_str(&raw.eod_square_off, "%H:%M")
            .map_err(|e| anyhow::anyhow!("parsing eod_square_off {:?}: {e}", raw.eod_square_off))?,
        max_hold_minutes: (raw.max_hold_minutes > 0).then_some(raw.max_hold_minutes),
        exit_atr_period: raw.exit_atr_period,
    };
    let snapshot_json = serde_json::to_string(&raw)?;
    Ok((config, snapshot_json))
}

pub fn load_default() -> anyhow::Result<(EngineConfig, String)> {
    load(Path::new("config/default.toml"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_the_real_repo_config() {
        // Run from the `intraday/` directory, as the CLI expects.
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../config/default.toml");
        let (config, snapshot) = load(&path).unwrap();
        assert!((config.risk_pct - 0.0025).abs() < 1e-9);
        assert_eq!(config.max_concurrent_positions, 5);
        assert!(matches!(config.trail_policy, TrailPolicy::PercentFromPeak(p) if (p - 0.0015).abs() < 1e-9));
        assert!(!snapshot.is_empty());
    }
}
