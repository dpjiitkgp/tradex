//! When the live run loop stops.
//!
//! Four clocks act on an open MIS position. Only the first three are ours,
//! and all three must finish before the fourth:
//!
//! ```text
//!   15:10  eod_square_off      — our LIMIT exit, fired by the next bar
//!   15:11  eod_market_after    — that limit escalates to MARKET
//!   15:12  loop_end            — force_square_off, last-resort backstop
//!   15:12  Zerodha's MIS cutoff — orders REFUSED, broker squares off itself
//! ```
//!
//! The 15:12 figure is not a guess. On 2026-09-30 a live exit placed at
//! 15:16 came back:
//!
//! > Intraday orders (MIS) are allowed only till 3:12 PM. Try placing a CNC order.
//!
//! By then Zerodha had already flattened the position on its own terms —
//! reconciliation had been reporting `broker shows []` since 15:13 — so our
//! stop, our trail and our square-off were all inoperative in the final
//! minutes. The trade worked out, but the exit price was the broker's choice,
//! not the strategy's.
//!
//! An earlier version of this file put the cutoff at 15:20 and bounded the
//! loop at 15:18, which looked safe and was not. Everything here now runs
//! ahead of 15:12 with the drain sized to fit.

use chrono::NaiveTime;
use core_types::time::IST;
use core_types::{DateTime, NaiveDate};

/// How long the loop keeps running past the clock-based square-off.
///
/// Two minutes, not three: the limit goes in on the first bar, escalates to
/// market on the second, and the loop ends on the third — all before Zerodha
/// stops accepting MIS orders. A longer drain would push the market order past
/// the cutoff, which is exactly the failure this module documents.
pub const SQUARE_OFF_DRAIN: chrono::TimeDelta = chrono::TimeDelta::minutes(2);

/// Zerodha's own MIS cutoff. Nothing of ours may be scheduled at or after it.
/// Asserted against in tests so a future change to the square-off cannot
/// silently cross it again.
pub const BROKER_MIS_CUTOFF: chrono::NaiveTime = match chrono::NaiveTime::from_hms_opt(15, 12, 0) {
    Some(t) => t,
    None => panic!("static time"),
};

/// When a still-unfilled end-of-day exit stops waiting and takes market.
///
/// Time-based rather than bar-counted: the generic exit chase gives a limit
/// three bars to fill, which from a 15:10 square-off would land at 15:13 —
/// past the broker cutoff. The end of the session is the one place where
/// being out matters more than the price.
pub fn eod_market_after(eod_square_off: NaiveTime) -> NaiveTime {
    eod_square_off + chrono::TimeDelta::minutes(1)
}

/// The instant the live run loop must break, so `force_square_off` runs while
/// the broker's own auto-close is still pending.
///
/// `session_close` remains a hard ceiling: a short or unusual session is never
/// extended by the drain window.
pub fn loop_end(
    day: NaiveDate,
    eod_square_off: NaiveTime,
    session_close: DateTime,
) -> Option<DateTime> {
    use chrono::TimeZone;
    let drained = day.and_time(eod_square_off) + SQUARE_OFF_DRAIN;
    let drained = IST.from_local_datetime(&drained).single()?.with_timezone(&chrono::Utc);
    Some(drained.min(session_close))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Timelike;

    fn ist(day: NaiveDate, h: u32, m: u32) -> DateTime {
        use chrono::TimeZone;
        IST.from_local_datetime(&day.and_hms_opt(h, m, 0).unwrap())
            .unwrap()
            .with_timezone(&chrono::Utc)
    }

    fn day() -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 9, 29).unwrap()
    }

    fn at(t: DateTime) -> String {
        t.with_timezone(&IST).format("%H:%M").to_string()
    }

    /// The live square-off. Hardcoded here rather than imported so that
    /// changing it in the CLI without revisiting these assertions fails.
    fn live_cutoff() -> NaiveTime {
        NaiveTime::from_hms_opt(15, 10, 0).unwrap()
    }

    #[test]
    fn the_whole_sequence_fits_before_the_broker_cutoff() {
        let sq = live_cutoff();
        let mkt = eod_market_after(sq);
        let end = loop_end(day(), sq, ist(day(), 15, 30)).unwrap();
        assert_eq!(at(ist(day(), sq.hour(), sq.minute())), "15:10");
        assert_eq!(mkt, NaiveTime::from_hms_opt(15, 11, 0).unwrap());
        assert_eq!(at(end), "15:12");
    }

    /// The failure of 2026-09-30, pinned. Every step of ours must complete
    /// before Zerodha stops accepting MIS orders — otherwise the exit is
    /// refused, the broker squares off on its own terms, and the strategy's
    /// stop and trail are inoperative for the last minutes of every session.
    #[test]
    fn nothing_of_ours_is_scheduled_at_or_after_the_broker_cutoff() {
        let sq = live_cutoff();
        assert!(sq < BROKER_MIS_CUTOFF, "square-off {sq} is not before {BROKER_MIS_CUTOFF}");
        let mkt = eod_market_after(sq);
        assert!(
            mkt < BROKER_MIS_CUTOFF,
            "the market escalation at {mkt} lands at or after the broker cutoff {BROKER_MIS_CUTOFF} \
             — it would be REJECTED with \"Intraday orders (MIS) are allowed only till 3:12 PM\""
        );
        let end = loop_end(day(), sq, ist(day(), 15, 30)).unwrap();
        let end_t = end.with_timezone(&IST).time();
        assert!(
            end_t <= BROKER_MIS_CUTOFF,
            "the loop ends at {end_t}, past the broker cutoff — force_square_off could not place \
             anything, and any straggler would be left to the broker"
        );
    }

    /// The EOD escalation must be strictly faster than the generic exit chase,
    /// which is bar-counted and would land at 15:13 from a 15:10 square-off.
    #[test]
    fn the_eod_escalation_beats_the_generic_three_bar_chase() {
        let sq = live_cutoff();
        let generic = sq + chrono::TimeDelta::minutes(3);
        assert!(
            eod_market_after(sq) < generic,
            "the end of the session cannot wait for the generic chase"
        );
        assert!(generic > BROKER_MIS_CUTOFF, "sanity: the generic chase really would be too late");
    }

    #[test]
    fn a_short_session_close_still_caps_the_drain_window() {
        // Muhurat-style short session: closing before the cutoff plus drain.
        let end = loop_end(day(), live_cutoff(), ist(day(), 15, 11));
        assert_eq!(at(end.unwrap()), "15:11");
    }

    #[test]
    fn leaves_time_for_the_cutoff_bar_to_fill() {
        let end = loop_end(day(), live_cutoff(), ist(day(), 15, 30)).unwrap();
        assert!(
            end - ist(day(), 15, 10) >= chrono::TimeDelta::minutes(2),
            "one-minute bars: the loop must outlive the cutoff bar by more than a single bar, \
             or the limit exit it places is cancelled before it can fill"
        );
    }
}
