//! Liquidity/volatility screener: ranks candidate instruments by average
//! daily turnover and filters on turnover / ATR% / volume floors, using
//! daily candles from a `MarketDataFeed`. This is a coarse day-level pass
//! to pick *which stocks* are worth watching intraday at all — the real
//! (Wilder) ATR and other per-bar indicators strategies use live come from
//! the `indicators` crate, not from here.

use crate::config::UniverseConfig;
use chrono::TimeZone;
use core_types::{CandleInterval, Instrument, MarketDataFeed};

#[derive(Debug, Clone)]
pub struct ScreenResult {
    pub instrument: Instrument,
    pub avg_turnover_cr: f64,
    pub atr_pct: f64,
    pub avg_volume: f64,
    pub included: bool,
    pub rank: usize,
}

/// Screens `candidates` against `feed`, using the last `config.lookback_days`
/// daily candles for each. Instruments a fetch fails for (delisted, no
/// recent trades, etc.) are skipped with a warning rather than failing the
/// whole screen.
pub async fn screen(
    feed: &dyn MarketDataFeed,
    candidates: &[Instrument],
    config: &UniverseConfig,
) -> Vec<ScreenResult> {
    let to_date = chrono::Utc::now().with_timezone(&core_types::time::IST).date_naive();
    // Wide calendar buffer so `lookback_days` trading days are covered even
    // across weekends/holidays.
    let calendar_buffer_days = (config.lookback_days as f64 * 1.6).ceil() as i64 + 5;
    let from_date = to_date - chrono::Duration::days(calendar_buffer_days);
    let from_dt = core_types::time::IST
        .from_local_datetime(&from_date.and_hms_opt(0, 0, 0).unwrap())
        .single()
        .unwrap()
        .with_timezone(&chrono::Utc);
    let to_dt = core_types::time::IST
        .from_local_datetime(&to_date.and_hms_opt(23, 59, 59).unwrap())
        .single()
        .unwrap()
        .with_timezone(&chrono::Utc);

    let mut results = Vec::new();
    for instrument in candidates {
        let candles = match feed
            .historical_candles(instrument, from_dt, to_dt, CandleInterval::Day)
            .await
        {
            Ok(c) => c,
            Err(e) => {
                tracing::warn!(symbol = %instrument.tradingsymbol, error = %e, "skipping — historical fetch failed");
                continue;
            }
        };
        let recent: Vec<_> = candles
            .iter()
            .rev()
            .take(config.lookback_days as usize)
            .collect();
        if recent.is_empty() {
            tracing::warn!(symbol = %instrument.tradingsymbol, "skipping — no daily candles in lookback window");
            continue;
        }

        let n = recent.len() as f64;
        let avg_turnover_cr = recent.iter().map(|c| c.close * c.volume as f64).sum::<f64>() / n / 1e7;
        let atr_pct = recent
            .iter()
            .map(|c| if c.close > 0.0 { (c.high - c.low) / c.close * 100.0 } else { 0.0 })
            .sum::<f64>()
            / n;
        let avg_volume = recent.iter().map(|c| c.volume as f64).sum::<f64>() / n;

        results.push(ScreenResult {
            instrument: instrument.clone(),
            avg_turnover_cr,
            atr_pct,
            avg_volume,
            included: false,
            rank: 0,
        });
    }

    results.sort_by(|a, b| b.avg_turnover_cr.partial_cmp(&a.avg_turnover_cr).unwrap());
    let mut included_so_far = 0usize;
    for (i, r) in results.iter_mut().enumerate() {
        r.rank = i + 1;
        let passes = r.avg_turnover_cr >= config.min_avg_turnover_cr
            && r.atr_pct >= config.min_atr_pct
            && r.avg_volume >= config.min_avg_volume;
        if passes && included_so_far < config.max_symbols {
            r.included = true;
            included_so_far += 1;
        }
    }
    results
}
