//! Request/response shapes for the JSON API. Strategy params are
//! deliberately one uniform shape across all three strategies (rather than
//! an enum per strategy type) — simpler for the frontend to render as one
//! form, at the cost of a few fields being unused for a given strategy
//! (e.g. `fast_ema` for `orb`), which is a fine trade for a small
//! personal tool.

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ScopeDto {
    /// "all" | "sector" | "symbol"
    pub kind: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyParamsDto {
    pub enabled: bool,
    pub relative_volume_period: u32,
    pub min_relative_volume: f64,
    /// vwap_trend only.
    pub fast_ema: u32,
    /// vwap_trend only.
    pub slow_ema: u32,
    /// momentum_breakout only.
    pub atr_period: u32,
    /// momentum_breakout only.
    pub min_atr_multiple: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategiesOverrideDto {
    pub orb: StrategyParamsDto,
    pub vwap_trend: StrategyParamsDto,
    pub momentum_breakout: StrategyParamsDto,
    /// Not yet on the trigger-backtest form in the UI — API-only for now.
    /// `atr_period`/`min_atr_multiple` carry `atr_period`/`deviation_atr_multiple`.
    pub vwap_reversion: StrategyParamsDto,
    /// Not yet on the trigger-backtest form in the UI — API-only for now.
    pub trend_pullback: StrategyParamsDto,
    /// Not yet on the trigger-backtest form in the UI — API-only for now.
    /// `relative_volume_period` carries `bar_minutes`, `min_relative_volume`
    /// carries `min_move_pct`, `atr_period` carries `min_streak`.
    pub strong_rally: StrategyParamsDto,
    /// Not yet on the trigger-backtest form in the UI — API-only for now.
    /// `relative_volume_period`/`fast_ema`/`slow_ema`/`atr_period` carry
    /// `period_15`/`period_30`/`period_45`/`period_60` respectively.
    pub ma_alignment: StrategyParamsDto,
    /// Not yet on the trigger-backtest form in the UI — API-only for now.
    /// `ma_periods` isn't representable in this shape, so it's fixed at
    /// [15,30,45,60] regardless of what's in `fast_ema`/`slow_ema`/etc.
    pub trend_confluence: StrategyParamsDto,
}

impl StrategiesOverrideDto {
    pub fn from_config(cfg: &strategies::StrategiesConfig) -> Self {
        let is_enabled = |name: &str| cfg.enabled.iter().any(|s| s == name);
        Self {
            orb: StrategyParamsDto {
                enabled: is_enabled("orb"),
                relative_volume_period: cfg.orb.relative_volume_period,
                min_relative_volume: cfg.orb.min_relative_volume,
                fast_ema: 0,
                slow_ema: 0,
                atr_period: 0,
                min_atr_multiple: 0.0,
            },
            vwap_trend: StrategyParamsDto {
                enabled: is_enabled("vwap_trend"),
                relative_volume_period: cfg.vwap_trend.relative_volume_period,
                min_relative_volume: cfg.vwap_trend.min_relative_volume,
                fast_ema: cfg.vwap_trend.fast_ema,
                slow_ema: cfg.vwap_trend.slow_ema,
                atr_period: 0,
                min_atr_multiple: 0.0,
            },
            momentum_breakout: StrategyParamsDto {
                enabled: is_enabled("momentum_breakout"),
                relative_volume_period: cfg.momentum_breakout.relative_volume_period,
                min_relative_volume: cfg.momentum_breakout.min_relative_volume,
                fast_ema: 0,
                slow_ema: 0,
                atr_period: cfg.momentum_breakout.atr_period,
                min_atr_multiple: cfg.momentum_breakout.min_atr_multiple,
            },
            vwap_reversion: StrategyParamsDto {
                enabled: is_enabled("vwap_reversion"),
                relative_volume_period: 0,
                min_relative_volume: 0.0,
                fast_ema: 0,
                slow_ema: 0,
                atr_period: cfg.vwap_reversion.atr_period,
                min_atr_multiple: cfg.vwap_reversion.deviation_atr_multiple,
            },
            trend_pullback: StrategyParamsDto {
                enabled: is_enabled("trend_pullback"),
                relative_volume_period: cfg.trend_pullback.relative_volume_period,
                min_relative_volume: cfg.trend_pullback.min_relative_volume,
                fast_ema: cfg.trend_pullback.fast_ema,
                slow_ema: cfg.trend_pullback.slow_ema,
                atr_period: 0,
                min_atr_multiple: 0.0,
            },
            strong_rally: StrategyParamsDto {
                enabled: is_enabled("strong_rally"),
                relative_volume_period: cfg.strong_rally.bar_minutes as u32,
                min_relative_volume: cfg.strong_rally.min_move_pct,
                fast_ema: 0,
                slow_ema: 0,
                atr_period: cfg.strong_rally.min_streak,
                min_atr_multiple: 0.0,
            },
            ma_alignment: StrategyParamsDto {
                enabled: is_enabled("ma_alignment"),
                relative_volume_period: *cfg.ma_alignment.periods.first().unwrap_or(&15),
                min_relative_volume: 0.0,
                fast_ema: *cfg.ma_alignment.periods.get(1).unwrap_or(&30),
                slow_ema: *cfg.ma_alignment.periods.get(2).unwrap_or(&45),
                atr_period: *cfg.ma_alignment.periods.get(3).unwrap_or(&60),
                min_atr_multiple: 0.0,
            },
            trend_confluence: StrategyParamsDto {
                enabled: is_enabled("trend_confluence"),
                relative_volume_period: cfg.trend_confluence.relative_volume_period,
                min_relative_volume: cfg.trend_confluence.min_relative_volume,
                fast_ema: cfg.trend_confluence.fast_ema,
                slow_ema: cfg.trend_confluence.slow_ema,
                atr_period: 0,
                min_atr_multiple: 0.0,
            },
        }
    }

    pub fn to_strategies_config(&self) -> strategies::StrategiesConfig {
        let mut enabled = Vec::new();
        if self.orb.enabled {
            enabled.push("orb".to_string());
        }
        if self.vwap_trend.enabled {
            enabled.push("vwap_trend".to_string());
        }
        if self.momentum_breakout.enabled {
            enabled.push("momentum_breakout".to_string());
        }
        if self.vwap_reversion.enabled {
            enabled.push("vwap_reversion".to_string());
        }
        if self.trend_pullback.enabled {
            enabled.push("trend_pullback".to_string());
        }
        if self.strong_rally.enabled {
            enabled.push("strong_rally".to_string());
        }
        if self.ma_alignment.enabled {
            enabled.push("ma_alignment".to_string());
        }
        if self.trend_confluence.enabled {
            enabled.push("trend_confluence".to_string());
        }
        strategies::StrategiesConfig {
            enabled,
            orb: strategies::config::OrbConfig {
                relative_volume_period: self.orb.relative_volume_period,
                min_relative_volume: self.orb.min_relative_volume,
            },
            vwap_trend: strategies::config::VwapTrendConfig {
                fast_ema: self.vwap_trend.fast_ema,
                slow_ema: self.vwap_trend.slow_ema,
                relative_volume_period: self.vwap_trend.relative_volume_period,
                min_relative_volume: self.vwap_trend.min_relative_volume,
                // Not yet on the trigger-backtest form — unfiltered until
                // the UI grows fields for these (see VwapTrendFollow's docs).
                allow_long: true,
                allow_short: true,
                min_time_into_window_minutes: 0,
            },
            momentum_breakout: strategies::config::MomentumBreakoutConfig {
                atr_period: self.momentum_breakout.atr_period,
                relative_volume_period: self.momentum_breakout.relative_volume_period,
                min_atr_multiple: self.momentum_breakout.min_atr_multiple,
                min_relative_volume: self.momentum_breakout.min_relative_volume,
            },
            vwap_reversion: strategies::config::VwapReversionConfig {
                atr_period: self.vwap_reversion.atr_period,
                deviation_atr_multiple: self.vwap_reversion.min_atr_multiple,
            },
            trend_pullback: strategies::config::TrendPullbackConfig {
                fast_ema: self.trend_pullback.fast_ema,
                slow_ema: self.trend_pullback.slow_ema,
                relative_volume_period: self.trend_pullback.relative_volume_period,
                min_relative_volume: self.trend_pullback.min_relative_volume,
            },
            strong_rally: strategies::config::StrongRallyConfig {
                bar_minutes: self.strong_rally.relative_volume_period as i64,
                min_streak: self.strong_rally.atr_period,
                min_move_pct: self.strong_rally.min_relative_volume,
            },
            ma_alignment: strategies::config::MaAlignmentConfig {
                periods: vec![
                    self.ma_alignment.relative_volume_period,
                    self.ma_alignment.fast_ema,
                    self.ma_alignment.slow_ema,
                    self.ma_alignment.atr_period,
                ],
            },
            trend_confluence: strategies::config::TrendConfluenceConfig {
                fast_ema: self.trend_confluence.fast_ema,
                slow_ema: self.trend_confluence.slow_ema,
                relative_volume_period: self.trend_confluence.relative_volume_period,
                min_relative_volume: self.trend_confluence.min_relative_volume,
                ma_periods: vec![15, 30, 45, 60],
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskDto {
    pub risk_pct: f64,
    pub min_reward_multiple: f64,
    /// "percent_from_peak" | "atr_multiple" | "vwap_cross"
    pub trail_policy: String,
    pub trail_param: f64,
    pub capital_per_trade: f64,
    pub max_concurrent_positions: usize,
    pub daily_loss_cap_pct: f64,
    /// "HH:MM"
    pub eod_square_off: String,
    /// 0 = no cap.
    pub max_hold_minutes: i64,
    pub exit_atr_period: u32,
}

impl RiskDto {
    pub fn from_config(cfg: &paper_engine::EngineConfig) -> Self {
        let (trail_policy, trail_param) = match cfg.trail_policy {
            core_types::TrailPolicy::PercentFromPeak(p) => ("percent_from_peak".to_string(), p),
            core_types::TrailPolicy::AtrMultiple(p) => ("atr_multiple".to_string(), p),
            core_types::TrailPolicy::VwapCross => ("vwap_cross".to_string(), 0.0),
            core_types::TrailPolicy::GiveBackFromPeak(p) => ("give_back_from_peak".to_string(), p),
        };
        Self {
            risk_pct: cfg.risk_pct,
            min_reward_multiple: cfg.min_reward_multiple,
            trail_policy,
            trail_param,
            capital_per_trade: cfg.capital_per_trade,
            max_concurrent_positions: cfg.max_concurrent_positions,
            daily_loss_cap_pct: cfg.daily_loss_cap_pct,
            eod_square_off: cfg.eod_square_off.format("%H:%M").to_string(),
            max_hold_minutes: cfg.max_hold_minutes.unwrap_or(0),
            exit_atr_period: cfg.exit_atr_period,
        }
    }

    pub fn to_engine_config(&self) -> anyhow::Result<paper_engine::EngineConfig> {
        Ok(paper_engine::EngineConfig {
            risk_pct: self.risk_pct,
            min_reward_multiple: self.min_reward_multiple,
            trail_policy: paper_engine::config_loader::parse_trail_policy(&self.trail_policy, self.trail_param)?,
            capital_per_trade: self.capital_per_trade,
            max_concurrent_positions: self.max_concurrent_positions,
            daily_loss_cap_pct: self.daily_loss_cap_pct,
            eod_square_off: chrono::NaiveTime::parse_from_str(&self.eod_square_off, "%H:%M")
                .map_err(|e| anyhow::anyhow!("parsing eod_square_off {:?}: {e}", self.eod_square_off))?,
            max_hold_minutes: (self.max_hold_minutes > 0).then_some(self.max_hold_minutes),
            exit_atr_period: self.exit_atr_period,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct TriggerRequest {
    pub label: Option<String>,
    /// YYYY-MM-DD
    pub from: String,
    /// YYYY-MM-DD
    pub to: String,
    pub scope: ScopeDto,
    pub strategies: StrategiesOverrideDto,
    pub risk: RiskDto,
}

#[derive(Debug, Serialize)]
pub struct TriggerResponse {
    pub job_id: uuid::Uuid,
    pub instrument_count: usize,
}

#[derive(Debug, Serialize)]
pub struct DefaultsResponse {
    pub strategies: StrategiesOverrideDto,
    pub risk: RiskDto,
}

#[derive(Debug, Serialize)]
pub struct StockDto {
    pub symbol: String,
    pub instrument_token: u64,
    pub sector: Option<String>,
    pub data_from: Option<String>,
    pub data_to: Option<String>,
    pub candle_count: i64,
}

#[derive(Debug, Serialize)]
pub struct SectorDto {
    pub sector: String,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct CoverageDto {
    pub from: Option<String>,
    pub to: Option<String>,
    pub instrument_count: i64,
}

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub error: String,
}
