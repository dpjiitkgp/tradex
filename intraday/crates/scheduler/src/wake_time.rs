use core_types::time::IST;
use core_types::{DateTime, TradingCalendar};

/// If `now` falls within today's trading session, returns `now` — so a
/// restart mid-day (e.g. after a crash) rejoins immediately instead of
/// waiting for tomorrow. Otherwise returns the next trading day's session
/// open, skipping weekends/holidays via `calendar.is_trading_day`
/// (forward-looking, unlike `tradex/calendar_utils.py`'s reactive
/// after-the-fact miss detection — the scheduler must decide *before*
/// 09:15 whether to wake at all).
pub fn next_wake_time(calendar: &dyn TradingCalendar, now: DateTime) -> DateTime {
    let today = now.with_timezone(&IST).date_naive();
    if let (Some(open), Some(close)) = (calendar.session_open(today), calendar.session_close(today)) {
        if now < open {
            return open;
        }
        if now < close {
            return now; // already mid-session — join immediately
        }
    }

    // Not currently in a trading session (weekend/holiday, or past close) —
    // search forward. NSE has never had anywhere near 14 consecutive
    // non-trading days; that many with no hit means the holiday cache is
    // stale or corrupt, which should fail loudly rather than loop forever.
    let mut d = today.succ_opt().expect("date overflow");
    for _ in 0..14 {
        if let Some(open) = calendar.session_open(d) {
            return open;
        }
        d = d.succ_opt().expect("date overflow");
    }
    panic!(
        "no trading day found within 14 days of {now} — holiday cache is likely stale; \
         run `papertrader calendar refresh`"
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use market_calendar::NseCalendar;
    use std::collections::BTreeSet;

    fn calendar_with(holidays: &[&str]) -> NseCalendar {
        let holidays = holidays
            .iter()
            .map(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap())
            .collect::<BTreeSet<_>>();
        NseCalendar::from_holidays(holidays)
    }

    fn ist(y: i32, mo: u32, d: u32, h: u32, mi: u32) -> DateTime {
        IST.with_ymd_and_hms(y, mo, d, h, mi, 0).unwrap().with_timezone(&chrono::Utc)
    }

    #[test]
    fn before_open_on_a_trading_day_waits_for_today() {
        let cal = calendar_with(&[]);
        let now = ist(2026, 9, 22, 6, 0); // 2026-09-22 is a Tuesday
        let wake = next_wake_time(&cal, now);
        assert_eq!(wake, ist(2026, 9, 22, 9, 15));
    }

    #[test]
    fn mid_session_joins_immediately() {
        let cal = calendar_with(&[]);
        let now = ist(2026, 9, 22, 11, 0);
        assert_eq!(next_wake_time(&cal, now), now);
    }

    #[test]
    fn after_close_skips_to_the_next_trading_day() {
        let cal = calendar_with(&[]);
        let now = ist(2026, 9, 22, 16, 0); // Tuesday, after 15:30 close
        let wake = next_wake_time(&cal, now);
        assert_eq!(wake, ist(2026, 9, 23, 9, 15)); // Wednesday
    }

    #[test]
    fn friday_evening_skips_the_weekend() {
        let cal = calendar_with(&[]);
        // 2026-09-25 is a Friday.
        let now = ist(2026, 9, 25, 16, 0);
        let wake = next_wake_time(&cal, now);
        assert_eq!(wake, ist(2026, 9, 28, 9, 15)); // Monday
    }

    #[test]
    fn a_holiday_is_skipped_too() {
        // Make Wednesday 2026-09-23 a holiday; Tuesday evening should then
        // skip straight to Thursday.
        let cal = calendar_with(&["2026-09-23"]);
        let now = ist(2026, 9, 22, 16, 0);
        let wake = next_wake_time(&cal, now);
        assert_eq!(wake, ist(2026, 9, 24, 9, 15));
    }
}
