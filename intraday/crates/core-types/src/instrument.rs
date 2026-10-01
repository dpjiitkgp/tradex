use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Exchange {
    Nse,
    /// Futures & options segment — index futures (NIFTY/BANKNIFTY) live
    /// here, not on NSE cash.
    Nfo,
}

/// A tradable instrument — equity (NSE), index (NSE, segment INDICES), or
/// futures contract (NFO). `instrument_token` is Kite's numeric id — note
/// it can drift on instrument-master refreshes (more so for futures, which
/// roll monthly), so callers should re-resolve periodically rather than
/// caching it indefinitely.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Instrument {
    pub tradingsymbol: String,
    pub instrument_token: u64,
    pub exchange: Exchange,
    pub lot_size: u32,
    pub tick_size: f64,
}
