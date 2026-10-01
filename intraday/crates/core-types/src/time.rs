use chrono::{NaiveTime, TimeZone};

/// IST is used for all session/window-boundary logic and display; `Tz` types
/// don't have generic Serde support the way `Utc` does, so every stored or
/// serialized timestamp (`DateTime` below) is UTC — convert with
/// `.with_timezone(&IST)` wherever IST wall-clock time is needed.
pub type Tz = chrono_tz::Tz;
pub const IST: Tz = chrono_tz::Asia::Kolkata;

pub type DateTime = chrono::DateTime<chrono::Utc>;
pub type NaiveDate = chrono::NaiveDate;

/// The UTC instant of 00:00:00 IST on `date` — the start of that IST
/// calendar day, for building a `[from, to]` query range.
pub fn ist_day_start(date: NaiveDate) -> DateTime {
    IST.from_local_datetime(&date.and_hms_opt(0, 0, 0).unwrap())
        .single()
        .unwrap()
        .with_timezone(&chrono::Utc)
}

/// The UTC instant of 23:59:59 IST on `date` — the end of that IST
/// calendar day.
pub fn ist_day_end(date: NaiveDate) -> DateTime {
    IST.from_local_datetime(&date.and_hms_opt(23, 59, 59).unwrap())
        .single()
        .unwrap()
        .with_timezone(&chrono::Utc)
}

/// The three time-of-day windows strategies are scoped to. Boundaries are the
/// defaults from the plan; concrete start/end still comes from config so they
/// can be tuned without a code change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum TimeWindow {
    /// 09:15-09:45 IST: high-volatility opening window (opening-range breakout).
    Opening,
    /// 09:45-11:30 IST: trend / volume-confirmation window.
    MidMorning,
    /// 11:30 IST-close: momentum / breakout watch for the rest of the day.
    Afternoon,
    /// The whole session, 09:15-15:30 IST. `window_for_time` never returns
    /// this — it only ever classifies a bar into Opening/MidMorning/
    /// Afternoon — but a `Strategy` can declare this as its own window to
    /// mean "call me on every bar regardless of which of the three the bar
    /// falls in" (see `strategy_core::StrategyRunner`), for patterns like a
    /// multi-candle rally that shouldn't be cut off at a window boundary.
    FullDay,
}

impl TimeWindow {
    pub fn default_bounds(self) -> (NaiveTime, NaiveTime) {
        match self {
            TimeWindow::Opening => (
                NaiveTime::from_hms_opt(9, 15, 0).unwrap(),
                NaiveTime::from_hms_opt(9, 45, 0).unwrap(),
            ),
            TimeWindow::MidMorning => (
                NaiveTime::from_hms_opt(9, 45, 0).unwrap(),
                NaiveTime::from_hms_opt(11, 30, 0).unwrap(),
            ),
            TimeWindow::Afternoon => (
                NaiveTime::from_hms_opt(11, 30, 0).unwrap(),
                NaiveTime::from_hms_opt(15, 30, 0).unwrap(),
            ),
            TimeWindow::FullDay => (
                NaiveTime::from_hms_opt(9, 15, 0).unwrap(),
                NaiveTime::from_hms_opt(15, 30, 0).unwrap(),
            ),
        }
    }

    pub fn covers(self, t: NaiveTime) -> bool {
        let (start, end) = self.default_bounds();
        t >= start && t < end
    }
}
