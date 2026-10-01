//! A `MarketDataFeed` that refuses to go to the network.
//!
//! Research replays cached candles — it pre-filters the universe to
//! instruments that already have enough bars, so the fetch fallback in
//! `run_backtest_job` is dead weight in practice. But constructing a real Kite
//! client to satisfy the parameter is not free: building one performs a
//! headless login, and a login rotates the cached access token. A live trading
//! process holds the token it loaded at startup, so starting a research run
//! mid-session invalidates the running book's credentials. That happened on
//! 2026-09-30 and forced a restart of a real-money session.
//!
//! Passing this instead makes research provably offline: it cannot log in,
//! cannot rotate a token, and cannot quietly spend API calls. A genuine cache
//! gap now fails loudly and names the fix, instead of being papered over by a
//! silent fetch in the middle of a study.

use async_trait::async_trait;
use core_types::{
    Candle, CandleInterval, DateTime, Error, Instrument, MarketDataFeed, Quote, Result,
};
use futures::stream::BoxStream;

pub struct CacheOnlyFeed;

#[async_trait]
impl MarketDataFeed for CacheOnlyFeed {
    async fn historical_candles(
        &self,
        instrument: &Instrument,
        from: DateTime,
        to: DateTime,
        _interval: CandleInterval,
    ) -> Result<Vec<Candle>> {
        Err(Error::Feed(format!(
            "{} has no cached candles for {}..{} and this run is cache-only — \
             run `papertrader backfill` first (research must not fetch: a login here would \
             rotate the access token a live session is using)",
            instrument.tradingsymbol,
            from.date_naive(),
            to.date_naive(),
        )))
    }

    async fn subscribe_ticks(&self, _instruments: &[Instrument]) -> Result<BoxStream<'static, core_types::Tick>> {
        Err(Error::Feed("CacheOnlyFeed cannot stream live ticks".to_string()))
    }

    async fn quote(&self, _instrument: &Instrument) -> Result<Quote> {
        Err(Error::Feed("CacheOnlyFeed cannot fetch quotes".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core_types::Exchange;

    fn inst() -> Instrument {
        Instrument {
            tradingsymbol: "TESTCO".to_string(),
            instrument_token: 1,
            exchange: Exchange::Nse,
            lot_size: 1,
            tick_size: 0.05,
        }
    }

    /// The error has to say what to do about it. A cache gap discovered
    /// halfway through a multi-hour study is worth one clear sentence.
    #[tokio::test]
    async fn a_cache_miss_names_the_remedy() {
        let e = CacheOnlyFeed
            .historical_candles(
                &inst(),
                core_types::time::ist_day_start(chrono::NaiveDate::from_ymd_opt(2026, 1, 1).unwrap()),
                core_types::time::ist_day_end(chrono::NaiveDate::from_ymd_opt(2026, 1, 2).unwrap()),
                CandleInterval::OneMinute,
            )
            .await
            .unwrap_err()
            .to_string();
        assert!(e.contains("TESTCO"), "should name the instrument: {e}");
        assert!(e.contains("backfill"), "should name the fix: {e}");
    }

    /// Every path must refuse, not just the one research happens to call —
    /// the point is that this type cannot reach the network at all.
    #[tokio::test]
    async fn the_live_paths_refuse_too() {
        assert!(CacheOnlyFeed.subscribe_ticks(&[inst()]).await.is_err());
        assert!(CacheOnlyFeed.quote(&inst()).await.is_err());
    }
}
