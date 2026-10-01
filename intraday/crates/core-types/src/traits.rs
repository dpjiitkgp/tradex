use crate::context::StrategyContext;
use crate::error::Result;
use crate::indicator::IndicatorSpec;
use crate::instrument::Instrument;
use crate::market::{Candle, CandleInterval, Quote, Tick};
use crate::risk::{EntryContext, ExitDecision, ExitPlan, MarketSnapshot, PositionState};
use crate::signal::{SignalIntent, StrategyId};
use crate::time::{DateTime, NaiveDate, TimeWindow};
use async_trait::async_trait;
use futures::stream::BoxStream;

/// Historical + live price data. Implemented by `kite-client` for real
/// trading and by a `HistoricalReplayFeed` (in `backtest`) for backtests —
/// `paper-engine` runs the identical code path against either.
#[async_trait]
pub trait MarketDataFeed: Send + Sync {
    async fn historical_candles(
        &self,
        instrument: &Instrument,
        from: DateTime,
        to: DateTime,
        interval: CandleInterval,
    ) -> Result<Vec<Candle>>;

    async fn subscribe_ticks(&self, instruments: &[Instrument]) -> Result<BoxStream<'static, Tick>>;

    async fn quote(&self, instrument: &Instrument) -> Result<Quote>;
}

/// One time-windowed trading strategy. Called once per bar for instruments
/// whose current time falls inside `window()`.
pub trait Strategy: Send + Sync {
    fn id(&self) -> StrategyId;
    fn window(&self) -> TimeWindow;
    fn required_indicators(&self) -> &[IndicatorSpec];
    fn on_bar(&mut self, ctx: &StrategyContext) -> Option<SignalIntent>;

    /// Clears any session-scoped state (e.g. "already signaled today").
    /// Default no-op for strategies that don't hold any.
    fn reset_session(&mut self) {}
}

/// The hard-stop / minimum-target-floor / dynamic-trailing state machine
/// shared by every open position, regardless of which strategy opened it.
///
/// `on_update` is a pure state-machine step: given the position's current
/// state and this bar's market snapshot, it returns the *next* state
/// (peak price tracked, stop possibly ratcheted, phase possibly advanced
/// from `Armed` to `Trailing`) plus an exit decision if this bar closes the
/// position. The caller (`paper-engine`) owns persisting the returned state
/// between bars and logging any stop movement.
pub trait ExitManager: Send + Sync {
    fn on_entry(&self, entry: &EntryContext) -> ExitPlan;
    fn on_update(
        &self,
        state: PositionState,
        plan: &ExitPlan,
        market: &MarketSnapshot,
    ) -> (PositionState, Option<ExitDecision>);
}

/// NSE trading-day and session-time logic. Forward-looking: must answer
/// "should the scheduler wake up at all" before the session starts, unlike
/// a reactive after-the-fact holiday check.
pub trait TradingCalendar: Send + Sync {
    fn is_trading_day(&self, date: NaiveDate) -> bool;
    fn session_open(&self, date: NaiveDate) -> Option<DateTime>;
    fn session_close(&self, date: NaiveDate) -> Option<DateTime>;
}

/// A freshly minted Kite Connect access token.
#[derive(Debug, Clone)]
pub struct AccessToken(pub String);

/// Mints a fresh daily broker access token. Isolated behind a trait because
/// the concrete implementation (headless login) is the most fragile,
/// credential-handling piece of the system.
#[async_trait]
pub trait LoginProvider: Send + Sync {
    async fn fresh_access_token(&self) -> Result<AccessToken>;
}
