//! NSE trading-day and session-time logic - forward-looking (unlike the Python sibling's reactive approach), since the scheduler must know before 09:15 whether to run at all.

pub mod calendar;
pub mod holidays;
pub mod nse_http;

pub use calendar::NseCalendar;
