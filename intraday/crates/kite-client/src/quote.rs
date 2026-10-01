use crate::rest::KiteRestClient;
use chrono::TimeZone;
use core_types::{Instrument, Quote};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct QuoteResponse {
    data: HashMap<String, QuoteRow>,
}

#[derive(Debug, Deserialize)]
struct QuoteRow {
    last_price: f64,
    /// e.g. "2026-09-22 15:29:59" — naive IST wall-clock time, no offset.
    timestamp: String,
}

fn symbol_key(instrument: &Instrument) -> String {
    format!("NSE:{}", instrument.tradingsymbol)
}

pub async fn fetch_quote(client: &KiteRestClient, instrument: &Instrument) -> anyhow::Result<Quote> {
    let key = symbol_key(instrument);
    let resp: QuoteResponse = client.get_json("/quote", &[("i", key.as_str())]).await?;
    let row = resp
        .data
        .get(&key)
        .ok_or_else(|| anyhow::anyhow!("Kite /quote response missing {key}"))?;

    let naive = chrono::NaiveDateTime::parse_from_str(&row.timestamp, "%Y-%m-%d %H:%M:%S")
        .map_err(|e| anyhow::anyhow!("unparseable Kite quote timestamp {:?}: {e}", row.timestamp))?;
    let ts = core_types::time::IST
        .from_local_datetime(&naive)
        .single()
        .ok_or_else(|| anyhow::anyhow!("ambiguous/invalid local time in Kite quote timestamp"))?
        .with_timezone(&chrono::Utc);

    Ok(Quote {
        instrument_token: instrument.instrument_token,
        ts,
        last_price: row.last_price,
    })
}

#[derive(Debug, Deserialize)]
struct OhlcBlock {
    open: f64,
    high: f64,
    low: f64,
    /// Kite's convention: this "close" is the *previous trading day's*
    /// close, not today's — it's the reference price for gap/net-change,
    /// not a same-day OHLC field.
    close: f64,
}

#[derive(Debug, Deserialize)]
struct QuoteRowFull {
    last_price: f64,
    volume: u64,
    ohlc: OhlcBlock,
}

#[derive(Debug, Deserialize)]
struct QuoteBatchResponse {
    data: HashMap<String, QuoteRowFull>,
}

/// One instrument's live snapshot for scanning purposes: today's move so
/// far against yesterday's close, today's range so far, and cumulative
/// volume so far (only meaningful relative to other instruments at the
/// same early moment — not comparable to a full-day average right after
/// open).
#[derive(Debug, Clone)]
pub struct ScanQuote {
    pub instrument: Instrument,
    pub last_price: f64,
    pub prev_close: f64,
    pub day_open: f64,
    pub day_high: f64,
    pub day_low: f64,
    pub volume: u64,
}

impl ScanQuote {
    pub fn gap_pct(&self) -> f64 {
        if self.prev_close > 0.0 {
            (self.last_price - self.prev_close) / self.prev_close * 100.0
        } else {
            0.0
        }
    }

    pub fn range_pct(&self) -> f64 {
        if self.prev_close > 0.0 {
            (self.day_high - self.day_low) / self.prev_close * 100.0
        } else {
            0.0
        }
    }
}

/// Batch quote fetch — one Kite request per <=100 instruments (well under
/// any documented limit) instead of one request per instrument, since a
/// scan across dozens/hundreds of stocks would otherwise be dozens/hundreds
/// of round trips.
pub async fn fetch_scan_quotes(client: &KiteRestClient, instruments: &[Instrument]) -> anyhow::Result<Vec<ScanQuote>> {
    let mut out = Vec::with_capacity(instruments.len());
    for chunk in instruments.chunks(100) {
        let keys: Vec<String> = chunk.iter().map(symbol_key).collect();
        let query: Vec<(&str, &str)> = keys.iter().map(|k| ("i", k.as_str())).collect();
        let resp: QuoteBatchResponse = client.get_json("/quote", &query).await?;

        for inst in chunk {
            let key = symbol_key(inst);
            let Some(row) = resp.data.get(&key) else {
                tracing::warn!(symbol = %inst.tradingsymbol, "no quote returned for this instrument, skipping");
                continue;
            };
            out.push(ScanQuote {
                instrument: inst.clone(),
                last_price: row.last_price,
                prev_close: row.ohlc.close,
                day_open: row.ohlc.open,
                day_high: row.ohlc.high,
                day_low: row.ohlc.low,
                volume: row.volume,
            });
        }
    }
    Ok(out)
}
