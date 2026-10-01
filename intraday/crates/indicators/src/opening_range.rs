use chrono::NaiveTime;
use core_types::time::IST;
use core_types::DateTime;

/// Tracks the high/low formed during the opening window (e.g. 09:15-09:30
/// IST), then reports that fixed range for the rest of the day. Caller must
/// call `reset()` at the start of each trading session.
#[derive(Debug, Clone, Copy)]
pub struct OpeningRange {
    formation_end: NaiveTime,
    high: Option<f64>,
    low: Option<f64>,
    formed: bool,
}

impl OpeningRange {
    pub fn new(formation_end: NaiveTime) -> Self {
        Self {
            formation_end,
            high: None,
            low: None,
            formed: false,
        }
    }

    pub fn reset(&mut self) {
        self.high = None;
        self.low = None;
        self.formed = false;
    }

    /// `ts` is the bar's timestamp (UTC) — converted to IST internally to
    /// compare against the formation-window cutoff.
    pub fn update(&mut self, ts: DateTime, high: f64, low: f64) {
        let ist_time = ts.with_timezone(&IST).time();
        if ist_time < self.formation_end {
            self.high = Some(self.high.map_or(high, |h| h.max(high)));
            self.low = Some(self.low.map_or(low, |l| l.min(low)));
        } else {
            self.formed = true;
        }
    }

    /// `Some((high, low))` once the formation window has passed; `None`
    /// while still forming or if no bars fell within the window at all.
    pub fn range(&self) -> Option<(f64, f64)> {
        if self.formed {
            self.high.zip(self.low)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn ist_ts(h: u32, m: u32) -> DateTime {
        IST.with_ymd_and_hms(2026, 9, 22, h, m, 0)
            .unwrap()
            .with_timezone(&chrono::Utc)
    }

    #[test]
    fn range_is_none_while_still_forming() {
        let mut or_ind = OpeningRange::new(NaiveTime::from_hms_opt(9, 30, 0).unwrap());
        or_ind.update(ist_ts(9, 15), 100.0, 98.0);
        or_ind.update(ist_ts(9, 20), 101.0, 99.0);
        assert_eq!(or_ind.range(), None);
    }

    #[test]
    fn range_is_the_high_low_extremes_of_the_formation_window() {
        let mut or_ind = OpeningRange::new(NaiveTime::from_hms_opt(9, 30, 0).unwrap());
        or_ind.update(ist_ts(9, 15), 100.0, 98.0);
        or_ind.update(ist_ts(9, 20), 103.0, 99.5);
        or_ind.update(ist_ts(9, 25), 101.0, 97.0);
        or_ind.update(ist_ts(9, 30), 105.0, 96.0); // this bar is AT the cutoff, not counted
        assert_eq!(or_ind.range(), Some((103.0, 97.0)));
    }

    #[test]
    fn reset_clears_state_for_a_new_session() {
        let mut or_ind = OpeningRange::new(NaiveTime::from_hms_opt(9, 30, 0).unwrap());
        or_ind.update(ist_ts(9, 15), 100.0, 98.0);
        or_ind.update(ist_ts(9, 35), 110.0, 95.0);
        assert_eq!(or_ind.range(), Some((100.0, 98.0)));
        or_ind.reset();
        assert_eq!(or_ind.range(), None);
    }
}
