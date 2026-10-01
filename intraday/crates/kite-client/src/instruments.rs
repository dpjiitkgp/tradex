//! Instrument-master sync. Kite serves this as CSV (not JSON), one dump
//! per exchange (`/instruments/NSE`, `/instruments/NFO`, ...), same 12-column
//! schema regardless of exchange. Tokens can drift on instrument-master
//! refreshes (more so for futures, which roll monthly); re-sync
//! periodically rather than caching indefinitely.

use crate::rest::KiteRestClient;
use chrono::NaiveDate;
use core_types::{Exchange, Instrument};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RawRow {
    instrument_token: u64,
    #[allow(dead_code)]
    exchange_token: u64,
    tradingsymbol: String,
    name: String,
    #[allow(dead_code)]
    last_price: f64,
    expiry: String,
    #[allow(dead_code)]
    strike: f64,
    tick_size: f64,
    lot_size: u32,
    instrument_type: String,
    segment: String,
    #[allow(dead_code)]
    exchange: String,
}

async fn fetch_raw(client: &KiteRestClient, exchange_path: &str) -> anyhow::Result<Vec<RawRow>> {
    let csv_text = client.get_text(&format!("/instruments/{exchange_path}")).await?;
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(csv_text.as_bytes());
    reader
        .deserialize::<RawRow>()
        .map(|r| r.map_err(|e| anyhow::anyhow!("parsing {exchange_path} instrument CSV row: {e}")))
        .collect()
}

/// Cash-equity rows only (`instrument_type == "EQ"`, `segment == "NSE"`) —
/// this is the project's core intraday-equity scope.
pub async fn fetch_nse_equities(client: &KiteRestClient) -> anyhow::Result<Vec<Instrument>> {
    let rows = fetch_raw(client, "NSE").await?;
    Ok(rows
        .into_iter()
        .filter(|r| r.instrument_type == "EQ" && r.segment == "NSE")
        .map(|r| Instrument {
            tradingsymbol: r.tradingsymbol,
            instrument_token: r.instrument_token,
            exchange: Exchange::Nse,
            lot_size: r.lot_size,
            tick_size: r.tick_size,
        })
        .collect())
}

/// NSE index rows (India VIX, NIFTY 50, etc.) — same `/instruments/NSE`
/// dump as equities, but `segment == "INDICES"` (their `instrument_type`
/// is, oddly, still `"EQ"` in Kite's data — segment is what actually
/// distinguishes them, confirmed against the live dump).
pub async fn fetch_nse_indices(client: &KiteRestClient) -> anyhow::Result<Vec<Instrument>> {
    let rows = fetch_raw(client, "NSE").await?;
    Ok(rows
        .into_iter()
        .filter(|r| r.segment == "INDICES")
        .map(|r| Instrument {
            tradingsymbol: r.tradingsymbol,
            instrument_token: r.instrument_token,
            exchange: Exchange::Nse,
            lot_size: r.lot_size,
            tick_size: r.tick_size,
        })
        .collect())
}

/// One NFO futures contract, before narrowing down to "the current month's
/// contract" — kept separate from `Instrument` since `name`/`expiry`
/// matter for that selection but not afterward.
pub struct FuturesContract {
    pub instrument: Instrument,
    pub name: String,
    pub expiry: NaiveDate,
}

/// All NFO futures contracts (every underlying, every expiry) —
/// `instrument_type == "FUT"`, `segment == "NFO-FUT"`. Callers narrow to
/// one underlying/expiry via `current_month_contract`.
pub async fn fetch_nfo_futures(client: &KiteRestClient) -> anyhow::Result<Vec<FuturesContract>> {
    let rows = fetch_raw(client, "NFO").await?;
    let mut out = Vec::new();
    for r in rows {
        if r.instrument_type != "FUT" || r.segment != "NFO-FUT" {
            continue;
        }
        let Ok(expiry) = NaiveDate::parse_from_str(&r.expiry, "%Y-%m-%d") else {
            continue; // malformed/blank expiry on a futures row would be unexpected; skip rather than fail the whole fetch
        };
        out.push(FuturesContract {
            instrument: Instrument {
                tradingsymbol: r.tradingsymbol,
                instrument_token: r.instrument_token,
                exchange: Exchange::Nfo,
                lot_size: r.lot_size,
                tick_size: r.tick_size,
            },
            name: r.name,
            expiry,
        });
    }
    Ok(out)
}

/// The near-month contract for `underlying_name` (e.g. "NIFTY",
/// "BANKNIFTY") as of `as_of` — the soonest expiry that hasn't passed yet.
/// Matches exactly on `name` (Kite's clean underlying field), not a
/// tradingsymbol prefix, so e.g. "NIFTY" doesn't accidentally match
/// "NIFTYFPI" futures.
pub fn current_month_contract(
    futures: &[FuturesContract],
    underlying_name: &str,
    as_of: NaiveDate,
) -> Option<Instrument> {
    futures
        .iter()
        .filter(|f| f.name == underlying_name && f.expiry >= as_of)
        .min_by_key(|f| f.expiry)
        .map(|f| f.instrument.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn contract(name: &str, expiry: &str, token: u64) -> FuturesContract {
        FuturesContract {
            instrument: Instrument {
                tradingsymbol: format!("{name}FUT"),
                instrument_token: token,
                exchange: Exchange::Nfo,
                lot_size: 50,
                tick_size: 0.05,
            },
            name: name.to_string(),
            expiry: NaiveDate::parse_from_str(expiry, "%Y-%m-%d").unwrap(),
        }
    }

    #[test]
    fn picks_the_nearest_unexpired_contract() {
        let futures = vec![
            contract("NIFTY", "2026-09-29", 1),
            contract("NIFTY", "2026-10-27", 2),
            contract("BANKNIFTY", "2026-09-29", 3),
        ];
        let as_of = NaiveDate::parse_from_str("2026-09-22", "%Y-%m-%d").unwrap();
        let picked = current_month_contract(&futures, "NIFTY", as_of).unwrap();
        assert_eq!(picked.instrument_token, 1);
    }

    #[test]
    fn skips_a_contract_that_already_expired() {
        let futures = vec![
            contract("NIFTY", "2026-09-20", 1), // already expired relative to as_of
            contract("NIFTY", "2026-10-27", 2),
        ];
        let as_of = NaiveDate::parse_from_str("2026-09-22", "%Y-%m-%d").unwrap();
        let picked = current_month_contract(&futures, "NIFTY", as_of).unwrap();
        assert_eq!(picked.instrument_token, 2);
    }

    #[test]
    fn does_not_match_a_similarly_named_underlying() {
        let futures = vec![contract("NIFTYFPI", "2026-09-29", 1)];
        let as_of = NaiveDate::parse_from_str("2026-09-22", "%Y-%m-%d").unwrap();
        assert!(current_month_contract(&futures, "NIFTY", as_of).is_none());
    }

    #[test]
    fn none_when_no_contract_matches() {
        let futures: Vec<FuturesContract> = vec![];
        let as_of = NaiveDate::parse_from_str("2026-09-22", "%Y-%m-%d").unwrap();
        assert!(current_month_contract(&futures, "NIFTY", as_of).is_none());
    }
}
