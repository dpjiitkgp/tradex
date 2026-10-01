use chrono::NaiveTime;
use core_types::TimeWindow;

/// Which of the three time-of-day windows `t` (an IST wall-clock time)
/// falls in, or `None` outside all of them (before 09:15 or after 15:15,
/// using the default bounds — see `TimeWindow::default_bounds`).
pub fn window_for_time(t: NaiveTime) -> Option<TimeWindow> {
    [TimeWindow::Opening, TimeWindow::MidMorning, TimeWindow::Afternoon]
        .into_iter()
        .find(|w| w.covers(t))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_times_to_the_right_window() {
        assert_eq!(
            window_for_time(NaiveTime::from_hms_opt(9, 20, 0).unwrap()),
            Some(TimeWindow::Opening)
        );
        assert_eq!(
            window_for_time(NaiveTime::from_hms_opt(10, 30, 0).unwrap()),
            Some(TimeWindow::MidMorning)
        );
        assert_eq!(
            window_for_time(NaiveTime::from_hms_opt(13, 0, 0).unwrap()),
            Some(TimeWindow::Afternoon)
        );
    }

    #[test]
    fn outside_all_windows_is_none() {
        assert_eq!(window_for_time(NaiveTime::from_hms_opt(8, 0, 0).unwrap()), None);
        assert_eq!(window_for_time(NaiveTime::from_hms_opt(16, 0, 0).unwrap()), None);
    }
}
