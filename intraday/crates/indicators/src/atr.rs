use core_types::Candle;

/// Wilder's Average True Range: seeded with a simple average of the first
/// `period` true ranges, then smoothed as `(prev*(period-1) + tr) / period`.
#[derive(Debug, Clone)]
pub struct Atr {
    period: usize,
    prev_close: Option<f64>,
    tr_buffer: Vec<f64>,
    value: Option<f64>,
}

impl Atr {
    pub fn new(period: usize) -> Self {
        assert!(period > 0, "ATR period must be > 0");
        Self {
            period,
            prev_close: None,
            tr_buffer: Vec::with_capacity(period),
            value: None,
        }
    }

    fn true_range(&self, candle: &Candle) -> f64 {
        match self.prev_close {
            Some(pc) => (candle.high - candle.low)
                .max((candle.high - pc).abs())
                .max((candle.low - pc).abs()),
            None => candle.high - candle.low,
        }
    }

    pub fn update(&mut self, candle: &Candle) -> Option<f64> {
        let tr = self.true_range(candle);
        self.prev_close = Some(candle.close);

        if let Some(prev) = self.value {
            let v = (prev * (self.period as f64 - 1.0) + tr) / self.period as f64;
            self.value = Some(v);
            return Some(v);
        }
        self.tr_buffer.push(tr);
        if self.tr_buffer.len() == self.period {
            let seed = self.tr_buffer.iter().sum::<f64>() / self.period as f64;
            self.value = Some(seed);
            Some(seed)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn candle(high: f64, low: f64, close: f64) -> Candle {
        Candle {
            ts: chrono::Utc::now(),
            open: close,
            high,
            low,
            close,
            volume: 0,
        }
    }

    #[test]
    fn first_bar_true_range_is_just_high_minus_low() {
        let mut atr = Atr::new(1);
        // period 1: seeded immediately from a single true range.
        let v = atr.update(&candle(10.0, 8.0, 9.0)).unwrap();
        assert!((v - 2.0).abs() < 1e-9);
    }

    #[test]
    fn seeds_with_average_of_first_period_true_ranges() {
        let mut atr = Atr::new(2);
        assert_eq!(atr.update(&candle(10.0, 8.0, 9.0)), None); // tr = 2
        // second bar: prev_close=9, tr = max(11-9=2, |11-9|=2, |9-9|=0) = 2
        let v = atr.update(&candle(11.0, 9.0, 10.0)).unwrap();
        assert!((v - 2.0).abs() < 1e-9);
    }

    #[test]
    fn wilder_smoothing_after_seed() {
        let mut atr = Atr::new(2);
        atr.update(&candle(10.0, 8.0, 9.0)); // tr=2
        atr.update(&candle(11.0, 9.0, 10.0)); // tr=2, seed=2.0
        // third bar: prev_close=10, tr = max(15-10=5, |15-10|=5, |10-10|=0) = 5
        // wilder: (2.0*(2-1) + 5) / 2 = 3.5
        let v = atr.update(&candle(15.0, 10.0, 14.0)).unwrap();
        assert!((v - 3.5).abs() < 1e-9);
    }
}
