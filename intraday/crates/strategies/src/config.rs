//! Config-driven strategy construction — lets a strategy's parameters be
//! tuned by editing `config/default.toml` and re-running a backtest,
//! rather than editing Rust source and recompiling. `enabled` also governs
//! which strategies actually get built, so a strategy can be turned off
//! entirely without deleting its config section.

use core_types::Strategy;
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub struct OrbConfig {
    pub relative_volume_period: u32,
    pub min_relative_volume: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VwapTrendConfig {
    pub fast_ema: u32,
    pub slow_ema: u32,
    pub relative_volume_period: u32,
    pub min_relative_volume: f64,
    /// Take long-side crossovers. See `VwapTrendFollow`'s own doc comment
    /// for why this exists — backtesting found a real long/short asymmetry.
    #[serde(default = "default_true")]
    pub allow_long: bool,
    #[serde(default = "default_true")]
    pub allow_short: bool,
    /// Ignore crossovers in the first N minutes of the 09:45 window.
    #[serde(default)]
    pub min_time_into_window_minutes: u32,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Deserialize)]
pub struct MomentumBreakoutConfig {
    pub atr_period: u32,
    pub relative_volume_period: u32,
    pub min_atr_multiple: f64,
    pub min_relative_volume: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VwapReversionConfig {
    pub atr_period: u32,
    pub deviation_atr_multiple: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TrendPullbackConfig {
    pub fast_ema: u32,
    pub slow_ema: u32,
    pub relative_volume_period: u32,
    pub min_relative_volume: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StrongRallyConfig {
    /// Candle size, minutes, for the internal price-action aggregation.
    pub bar_minutes: i64,
    /// Consecutive same-direction qualifying candles required.
    pub min_streak: u32,
    /// Minimum |move| per candle, as a fraction of that candle's open
    /// (0.002 = 0.2%), same convention as risk_pct elsewhere.
    pub min_move_pct: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MaAlignmentConfig {
    /// Shortest to longest, e.g. [15, 30, 45, 60]. At least 2 required.
    pub periods: Vec<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TrendConfluenceConfig {
    pub fast_ema: u32,
    pub slow_ema: u32,
    pub relative_volume_period: u32,
    pub min_relative_volume: f64,
    /// Shortest to longest, e.g. [15, 30, 45, 60]. At least 2 required.
    pub ma_periods: Vec<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StrategiesConfig {
    pub enabled: Vec<String>,
    pub orb: OrbConfig,
    pub vwap_trend: VwapTrendConfig,
    pub momentum_breakout: MomentumBreakoutConfig,
    pub vwap_reversion: VwapReversionConfig,
    pub trend_pullback: TrendPullbackConfig,
    pub strong_rally: StrongRallyConfig,
    pub ma_alignment: MaAlignmentConfig,
    pub trend_confluence: TrendConfluenceConfig,
}

impl StrategiesConfig {
    /// Builds one boxed `Strategy` per name in `enabled`, with that
    /// strategy's tuned parameters — an unrecognized name is logged and
    /// skipped rather than failing the whole build, so a typo in
    /// `enabled` doesn't take the other strategies down with it.
    pub fn build(&self) -> Vec<Box<dyn Strategy>> {
        let mut out: Vec<Box<dyn Strategy>> = Vec::new();
        for name in &self.enabled {
            match name.as_str() {
                "orb" => out.push(Box::new(crate::OpeningRangeBreakout::new(
                    self.orb.relative_volume_period,
                    self.orb.min_relative_volume,
                ))),
                "vwap_trend" => out.push(Box::new(crate::VwapTrendFollow::new(
                    self.vwap_trend.fast_ema,
                    self.vwap_trend.slow_ema,
                    self.vwap_trend.relative_volume_period,
                    self.vwap_trend.min_relative_volume,
                    self.vwap_trend.allow_long,
                    self.vwap_trend.allow_short,
                    self.vwap_trend.min_time_into_window_minutes,
                ))),
                "momentum_breakout" => out.push(Box::new(crate::MomentumBreakout::new(
                    self.momentum_breakout.atr_period,
                    self.momentum_breakout.relative_volume_period,
                    self.momentum_breakout.min_atr_multiple,
                    self.momentum_breakout.min_relative_volume,
                ))),
                "vwap_reversion" => out.push(Box::new(crate::VwapReversion::new(
                    self.vwap_reversion.atr_period,
                    self.vwap_reversion.deviation_atr_multiple,
                ))),
                "trend_pullback" => out.push(Box::new(crate::TrendPullback::new(
                    self.trend_pullback.fast_ema,
                    self.trend_pullback.slow_ema,
                    self.trend_pullback.relative_volume_period,
                    self.trend_pullback.min_relative_volume,
                ))),
                "strong_rally" => out.push(Box::new(crate::StrongRally::new(
                    self.strong_rally.bar_minutes,
                    self.strong_rally.min_streak,
                    self.strong_rally.min_move_pct,
                ))),
                "ma_alignment" => {
                    out.push(Box::new(crate::MaAlignment::new(self.ma_alignment.periods.clone())))
                }
                "trend_confluence" => out.push(Box::new(crate::TrendConfluence::new(
                    self.trend_confluence.fast_ema,
                    self.trend_confluence.slow_ema,
                    self.trend_confluence.relative_volume_period,
                    self.trend_confluence.min_relative_volume,
                    self.trend_confluence.ma_periods.clone(),
                ))),
                other => tracing::warn!(strategy = other, "unknown strategy in [strategies].enabled, skipping"),
            }
        }
        out
    }
}

#[derive(Debug, Deserialize)]
struct RootConfig {
    strategies: StrategiesConfig,
}

pub fn load(path: &Path) -> anyhow::Result<StrategiesConfig> {
    let text = std::fs::read_to_string(path).map_err(|e| anyhow::anyhow!("reading {path:?}: {e}"))?;
    let root: RootConfig =
        toml::from_str(&text).map_err(|e| anyhow::anyhow!("parsing [strategies] section of {path:?}: {e}"))?;
    Ok(root.strategies)
}

pub fn load_default() -> anyhow::Result<StrategiesConfig> {
    load(Path::new("config/default.toml"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_the_real_repo_config_and_builds_all_enabled_strategies() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../config/default.toml");
        let cfg = load(&path).unwrap();
        assert_eq!(cfg.enabled, vec!["orb", "vwap_trend", "momentum_breakout"]);
        let built = cfg.build();
        assert_eq!(built.len(), 3);
        let ids: Vec<_> = built.iter().map(|s| s.id()).collect();
        assert!(ids.contains(&"orb".to_string()));
        assert!(ids.contains(&"vwap_trend".to_string()));
        assert!(ids.contains(&"momentum_breakout".to_string()));
    }

    #[test]
    fn unknown_enabled_name_is_skipped_not_fatal() {
        let cfg = StrategiesConfig {
            enabled: vec!["orb".to_string(), "not_a_real_strategy".to_string()],
            orb: OrbConfig {
                relative_volume_period: 10,
                min_relative_volume: 1.5,
            },
            vwap_trend: VwapTrendConfig {
                fast_ema: 9,
                slow_ema: 21,
                relative_volume_period: 10,
                min_relative_volume: 1.2,
                allow_long: true,
                allow_short: true,
                min_time_into_window_minutes: 0,
            },
            momentum_breakout: MomentumBreakoutConfig {
                atr_period: 14,
                relative_volume_period: 10,
                min_atr_multiple: 1.5,
                min_relative_volume: 2.0,
            },
            vwap_reversion: VwapReversionConfig {
                atr_period: 14,
                deviation_atr_multiple: 2.0,
            },
            trend_pullback: TrendPullbackConfig {
                fast_ema: 9,
                slow_ema: 21,
                relative_volume_period: 10,
                min_relative_volume: 1.2,
            },
            strong_rally: StrongRallyConfig {
                bar_minutes: 5,
                min_streak: 3,
                min_move_pct: 0.002,
            },
            ma_alignment: MaAlignmentConfig {
                periods: vec![15, 30, 45, 60],
            },
            trend_confluence: TrendConfluenceConfig {
                fast_ema: 9,
                slow_ema: 21,
                relative_volume_period: 10,
                min_relative_volume: 1.2,
                ma_periods: vec![15, 30, 45, 60],
            },
        };
        assert_eq!(cfg.build().len(), 1);
    }

    #[test]
    fn new_strategies_are_config_tunable_but_not_enabled_by_default() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../config/default.toml");
        let cfg = load(&path).unwrap();
        assert!(!cfg.enabled.contains(&"vwap_reversion".to_string()));
        assert!(!cfg.enabled.contains(&"trend_pullback".to_string()));
        assert!(cfg.vwap_reversion.deviation_atr_multiple > 0.0);
        assert!(cfg.trend_pullback.slow_ema > cfg.trend_pullback.fast_ema);
    }
}
