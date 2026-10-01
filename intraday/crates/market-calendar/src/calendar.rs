use crate::holidays::{self, HolidayCache};
use crate::nse_http::NseHttpClient;
use chrono::{NaiveDate, TimeZone};
use core_types::time::IST;
use core_types::{DateTime, TradingCalendar};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

const SESSION_OPEN: (u32, u32) = (9, 15);
const SESSION_CLOSE: (u32, u32) = (15, 30);

fn default_cache_path() -> PathBuf {
    PathBuf::from("data/nse_holidays_cache.json")
}

fn default_override_path() -> PathBuf {
    PathBuf::from("config/nse_holidays_override.toml")
}

/// NSE equity trading-day and session-time calendar. Weekends are always
/// non-trading; specific-date holidays come from a locally cached fetch
/// (see `holidays::fetch_from_nse`) plus a manual override file, so the
/// scheduler can answer "is today a trading day" without hitting the
/// network on every wakeup.
pub struct NseCalendar {
    holidays: BTreeSet<NaiveDate>,
    cache_age: Option<chrono::DateTime<chrono::Utc>>,
}

impl NseCalendar {
    pub fn from_holidays(holidays: BTreeSet<NaiveDate>) -> Self {
        Self {
            holidays,
            cache_age: None,
        }
    }

    /// Loads the last-fetched holiday cache plus the manual override, from
    /// the default `data/`/`config/` paths relative to the current working
    /// directory. Missing cache degrades gracefully to weekday-only
    /// trading-day logic (no known holidays) rather than failing — run
    /// `papertrader calendar refresh` to populate it.
    pub fn load_default() -> anyhow::Result<Self> {
        Self::load(&default_cache_path(), &default_override_path())
    }

    pub fn load(cache_path: &Path, override_path: &Path) -> anyhow::Result<Self> {
        let cache = holidays::load_cache(cache_path);
        let over = holidays::load_override(override_path);
        let cache_age = cache.as_ref().map(|c| c.fetched_at);
        let holidays = holidays::resolve(cache.as_ref(), &over);
        Ok(Self {
            holidays,
            cache_age,
        })
    }

    /// Fetches the current NSE holiday list, merges the manual override, and
    /// persists the cache. Call this ahead of time (e.g. weekly, or on
    /// scheduler startup) — not on every trading-day check.
    pub async fn refresh(cache_path: &Path, override_path: &Path) -> anyhow::Result<Self> {
        let client = NseHttpClient::new()?;
        let fetched = holidays::fetch_from_nse(&client).await?;
        let cache = HolidayCache {
            fetched_at: chrono::Utc::now(),
            holidays: fetched,
        };
        holidays::save_cache(cache_path, &cache)?;

        let over = holidays::load_override(override_path);
        let holidays = holidays::resolve(Some(&cache), &over);
        Ok(Self {
            holidays,
            cache_age: Some(cache.fetched_at),
        })
    }

    pub async fn refresh_default() -> anyhow::Result<Self> {
        Self::refresh(&default_cache_path(), &default_override_path()).await
    }

    fn session_bound(&self, date: NaiveDate, (h, m): (u32, u32)) -> Option<DateTime> {
        if !self.is_trading_day(date) {
            return None;
        }
        let naive = date.and_hms_opt(h, m, 0)?;
        let ist = IST.from_local_datetime(&naive).single()?;
        Some(ist.with_timezone(&chrono::Utc))
    }

    pub fn print_today(&self, today: NaiveDate) {
        if self.cache_age.is_none() {
            println!(
                "warning: no holiday cache found — treating every weekday as a trading day.\n\
                 Run `papertrader calendar refresh` first for an accurate answer.\n"
            );
        }
        if self.is_trading_day(today) {
            let open = self.session_bound(today, SESSION_OPEN).unwrap();
            let close = self.session_bound(today, SESSION_CLOSE).unwrap();
            println!(
                "{today} is a trading day. Session: {} - {} IST",
                open.with_timezone(&IST).format("%H:%M"),
                close.with_timezone(&IST).format("%H:%M"),
            );
        } else {
            println!("{today} is NOT a trading day.");
        }
    }
}

impl TradingCalendar for NseCalendar {
    fn is_trading_day(&self, date: NaiveDate) -> bool {
        use chrono::Datelike;
        let weekday = date.weekday();
        if weekday == chrono::Weekday::Sat || weekday == chrono::Weekday::Sun {
            return false;
        }
        !self.holidays.contains(&date)
    }

    fn session_open(&self, date: NaiveDate) -> Option<DateTime> {
        self.session_bound(date, SESSION_OPEN)
    }

    fn session_close(&self, date: NaiveDate) -> Option<DateTime> {
        self.session_bound(date, SESSION_CLOSE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calendar_with(holidays: &[&str]) -> NseCalendar {
        let holidays = holidays
            .iter()
            .map(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap())
            .collect();
        NseCalendar::from_holidays(holidays)
    }

    #[test]
    fn weekends_are_never_trading_days() {
        let cal = calendar_with(&[]);
        // 2026-09-19 is a Saturday, 2026-09-20 a Sunday.
        assert!(!cal.is_trading_day(NaiveDate::from_ymd_opt(2026, 9, 19).unwrap()));
        assert!(!cal.is_trading_day(NaiveDate::from_ymd_opt(2026, 9, 20).unwrap()));
    }

    #[test]
    fn ordinary_weekday_is_a_trading_day() {
        let cal = calendar_with(&[]);
        // 2026-09-22 is a Tuesday.
        assert!(cal.is_trading_day(NaiveDate::from_ymd_opt(2026, 9, 22).unwrap()));
    }

    #[test]
    fn holiday_weekday_is_not_a_trading_day() {
        let cal = calendar_with(&["2026-10-02"]); // Gandhi Jayanti, a Friday
        assert!(!cal.is_trading_day(NaiveDate::from_ymd_opt(2026, 10, 2).unwrap()));
    }

    #[test]
    fn session_bounds_are_none_on_non_trading_days() {
        let cal = calendar_with(&[]);
        let sunday = NaiveDate::from_ymd_opt(2026, 9, 20).unwrap();
        assert!(cal.session_open(sunday).is_none());
        assert!(cal.session_close(sunday).is_none());
    }

    #[test]
    fn session_bounds_are_0915_to_1530_ist_on_a_trading_day() {
        let cal = calendar_with(&[]);
        let tuesday = NaiveDate::from_ymd_opt(2026, 9, 22).unwrap();
        let open = cal.session_open(tuesday).unwrap().with_timezone(&IST);
        let close = cal.session_close(tuesday).unwrap().with_timezone(&IST);
        assert_eq!(open.format("%H:%M").to_string(), "09:15");
        assert_eq!(close.format("%H:%M").to_string(), "15:30");
    }
}
