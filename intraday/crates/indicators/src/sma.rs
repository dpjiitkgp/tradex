use std::collections::VecDeque;

/// Simple moving average over the last `period` closes. Returns `None`
/// until `period` values have been seen.
#[derive(Debug, Clone)]
pub struct Sma {
    period: usize,
    window: VecDeque<f64>,
    sum: f64,
}

impl Sma {
    pub fn new(period: usize) -> Self {
        assert!(period > 0, "SMA period must be > 0");
        Self {
            period,
            window: VecDeque::with_capacity(period),
            sum: 0.0,
        }
    }

    pub fn update(&mut self, close: f64) -> Option<f64> {
        self.window.push_back(close);
        self.sum += close;
        if self.window.len() > self.period {
            self.sum -= self.window.pop_front().unwrap();
        }
        (self.window.len() == self.period).then_some(self.sum / self.period as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn none_until_warmed_up() {
        let mut sma = Sma::new(3);
        assert_eq!(sma.update(1.0), None);
        assert_eq!(sma.update(2.0), None);
        assert_eq!(sma.update(3.0), Some(2.0));
    }

    #[test]
    fn slides_the_window_after_warmup() {
        let mut sma = Sma::new(3);
        sma.update(1.0);
        sma.update(2.0);
        sma.update(3.0);
        assert_eq!(sma.update(6.0), Some((2.0 + 3.0 + 6.0) / 3.0));
    }
}
