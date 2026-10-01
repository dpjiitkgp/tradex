//! Historical candle fetch — the `historical_candles` half of
//! `MarketDataFeed`, plus `fetch_candles_chunked` for ranges wider than
//! Kite's minute-interval cap.
//!
//! **Confirmed against the live API**: minute-interval requests are capped
//! at 60 days — `{"status":"error","message":"interval exceeds max limit:
//! 60 days","error_type":"InputException"}` for anything wider, a hard
//! rejection, not a silent truncation. `fetch_candles_chunked` splits a
//! wider range into <=55-day windows (margin under the cap) and paces
//! requests to stay well under Kite's rate limit.

use crate::rest::KiteRestClient;
use core_types::{Candle, CandleInterval, DateTime};
use serde::Deserialize;
use std::time::Duration;

fn kite_interval_str(interval: CandleInterval) -> &'static str {
    match interval {
        CandleInterval::OneMinute => "minute",
        CandleInterval::Day => "day",
    }
}

#[derive(Debug, Deserialize)]
struct HistoricalResponse {
    data: HistoricalData,
}
#[derive(Debug, Deserialize)]
struct HistoricalData {
    candles: Vec<CandleRow>,
}

/// One raw Kite candle: `[timestamp, open, high, low, close, volume]` (a
/// trailing open-interest element can appear when `oi=1` is requested — we
/// don't request it, so this stays a fixed-size tuple-ish row).
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum CandleRow {
    Ohlcv(String, f64, f64, f64, f64, u64),
}

pub async fn fetch_candles(
    client: &KiteRestClient,
    instrument_token: u64,
    from: DateTime,
    to: DateTime,
    interval: CandleInterval,
) -> anyhow::Result<Vec<Candle>> {
    let path = format!("/instruments/historical/{instrument_token}/{}", kite_interval_str(interval));
    let from_ist = from.with_timezone(&core_types::time::IST);
    let to_ist = to.with_timezone(&core_types::time::IST);
    let from_s = from_ist.format("%Y-%m-%d %H:%M:%S").to_string();
    let to_s = to_ist.format("%Y-%m-%d %H:%M:%S").to_string();

    let resp: HistoricalResponse = client
        .get_json(&path, &[("from", from_s.as_str()), ("to", to_s.as_str())])
        .await?;

    resp.data
        .candles
        .into_iter()
        .map(|CandleRow::Ohlcv(ts, open, high, low, close, volume)| {
            let ts = chrono::DateTime::parse_from_str(&ts, "%Y-%m-%dT%H:%M:%S%z")
                .map_err(|e| anyhow::anyhow!("unparseable Kite candle timestamp {ts:?}: {e}"))?
                .with_timezone(&chrono::Utc);
            Ok(Candle {
                ts,
                open,
                high,
                low,
                close,
                volume,
            })
        })
        .collect()
}

const MAX_DAYS_PER_MINUTE_REQUEST: i64 = 55;
const REQUEST_PACE: Duration = Duration::from_millis(600);

/// Splits `[from, to]` into <=55-day chunks (only relevant for
/// `CandleInterval::OneMinute` — `Day` has no such cap, so it goes through
/// as a single request), fetching each in turn with a pacing delay between
/// requests. Returns candles from all chunks concatenated in order.
pub async fn fetch_candles_chunked(
    client: &KiteRestClient,
    instrument_token: u64,
    from: DateTime,
    to: DateTime,
    interval: CandleInterval,
) -> anyhow::Result<Vec<Candle>> {
    if interval == CandleInterval::Day {
        return fetch_candles(client, instrument_token, from, to, interval).await;
    }

    let mut all = Vec::new();
    let mut chunk_start = from;
    let mut first = true;
    while chunk_start < to {
        if !first {
            tokio::time::sleep(REQUEST_PACE).await;
        }
        first = false;

        let chunk_end = (chunk_start + chrono::Duration::days(MAX_DAYS_PER_MINUTE_REQUEST)).min(to);
        let mut candles = fetch_candles(client, instrument_token, chunk_start, chunk_end, interval).await?;
        all.append(&mut candles);
        chunk_start = chunk_end + chrono::Duration::seconds(1);
    }
    Ok(all)
}
