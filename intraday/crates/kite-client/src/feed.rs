use crate::rest::KiteRestClient;
use crate::{historical, quote, ticker};
use async_trait::async_trait;
use core_types::{Candle, CandleInterval, DateTime, Instrument, MarketDataFeed, Quote, Tick};
use futures::stream::BoxStream;

/// Live/historical `MarketDataFeed` backed by Kite's REST API plus the
/// hand-rolled v3 WebSocket ticker (`ticker.rs`) for live ticks.
pub struct KiteMarketDataFeed {
    rest: KiteRestClient,
}

impl KiteMarketDataFeed {
    pub fn new(rest: KiteRestClient) -> Self {
        Self { rest }
    }
}

#[async_trait]
impl MarketDataFeed for KiteMarketDataFeed {
    async fn historical_candles(
        &self,
        instrument: &Instrument,
        from: DateTime,
        to: DateTime,
        interval: CandleInterval,
    ) -> core_types::Result<Vec<Candle>> {
        historical::fetch_candles(&self.rest, instrument.instrument_token, from, to, interval)
            .await
            .map_err(|e| core_types::Error::Feed(e.to_string()))
    }

    async fn subscribe_ticks(&self, instruments: &[Instrument]) -> core_types::Result<BoxStream<'static, Tick>> {
        Ok(ticker::subscribe_ticks(
            self.rest.api_key().to_string(),
            self.rest.access_token().to_string(),
            instruments,
        ))
    }

    async fn quote(&self, instrument: &Instrument) -> core_types::Result<Quote> {
        quote::fetch_quote(&self.rest, instrument)
            .await
            .map_err(|e| core_types::Error::Feed(e.to_string()))
    }
}
