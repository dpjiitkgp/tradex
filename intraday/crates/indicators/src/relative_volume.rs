use std::collections::VecDeque;

/// Current bar's volume divided by the average of the preceding `period`
/// bars' volume (>1.0 means "more volume than recently typical").
///
/// This is a simple rolling-window baseline, not the same-time-of-day-across-
/// N-days baseline a more sophisticated momentum scanner would eventually
/// want (e.g. "is 9:20am volume today high compared to 9:20am on the last 20
/// days") — that needs per-time-of-day history the engine doesn't track yet.
/// Good enough for the current strategies; revisit if that distinction turns
/// out to matter.
#[derive(Debug, Clone)]
pub struct RelativeVolume {
    period: usize,
    window: VecDeque<f64>,
    sum: f64,
}

impl RelativeVolume {
    pub fn new(period: usize) -> Self {
        assert!(period > 0, "RelativeVolume period must be > 0");
        Self {
            period,
            window: VecDeque::with_capacity(period),
            sum: 0.0,
        }
    }

    pub fn update(&mut self, volume: u64) -> Option<f64> {
        let v = volume as f64;
        let result = (self.window.len() == self.period).then(|| {
            let avg = self.sum / self.period as f64;
            if avg > 0.0 {
                v / avg
            } else {
                0.0
            }
        });

        self.window.push_back(v);
        self.sum += v;
        if self.window.len() > self.period {
            self.sum -= self.window.pop_front().unwrap();
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn none_until_baseline_window_is_full() {
        let mut rv = RelativeVolume::new(2);
        assert_eq!(rv.update(100), None);
        assert_eq!(rv.update(100), None);
    }

    #[test]
    fn compares_current_bar_against_preceding_average() {
        let mut rv = RelativeVolume::new(2);
        rv.update(100);
        rv.update(200); // baseline avg of [100,200] = 150
        let v = rv.update(300).unwrap();
        assert!((v - 300.0 / 150.0).abs() < 1e-9);
    }

    #[test]
    fn zero_baseline_average_does_not_divide_by_zero() {
        let mut rv = RelativeVolume::new(2);
        rv.update(0);
        rv.update(0);
        assert_eq!(rv.update(500), Some(0.0));
    }
}
