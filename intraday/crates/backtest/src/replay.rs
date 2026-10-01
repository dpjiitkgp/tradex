use core_types::time::IST;
use core_types::{Candle, Instrument, NaiveDate};
use paper_engine::PaperEngine;

pub struct ReplayBar {
    pub instrument: Instrument,
    pub candle: Candle,
}

/// Merges each instrument's candle series into one chronologically-ordered
/// stream. `paper-engine`'s portfolio-level concerns (position cap,
/// daily-loss cap, EOD cutoff) depend on seeing bars in true wall-clock
/// order across the whole curated universe, not all of one instrument's day
/// followed by another's.
pub fn merge_chronological(series: Vec<(Instrument, Vec<Candle>)>) -> Vec<ReplayBar> {
    let mut bars: Vec<ReplayBar> = series
        .into_iter()
        .flat_map(|(inst, candles)| {
            candles.into_iter().map(move |c| ReplayBar {
                instrument: inst.clone(),
                candle: c,
            })
        })
        .collect();
    bars.sort_by_key(|b| b.candle.ts);
    bars
}

/// Feeds `bars` through `engine`, calling `reset_session()` whenever the
/// IST calendar date changes so no state (indicators, strategy
/// "signaled today" flags) carries across a session boundary. Ends the run
/// in storage when done.
pub fn run(engine: &mut PaperEngine, bars: &[ReplayBar]) -> anyhow::Result<()> {
    let mut current_date: Option<NaiveDate> = None;
    for b in bars {
        let ist_date = b.candle.ts.with_timezone(&IST).date_naive();
        if current_date != Some(ist_date) {
            // Squares off anything the clock-based EOD rule missed, at the
            // previous day's last price — see PaperEngine::reset_session.
            engine.reset_session()?;
            current_date = Some(ist_date);
        }
        engine.on_bar(&b.instrument, &b.candle)?;
    }
    engine.end_run()?;
    Ok(())
}
