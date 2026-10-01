/// Exponential moving average, seeded with a simple average of the first
/// `period` closes (standard convention), then updated with the usual
/// `alpha = 2/(period+1)` multiplier.
#[derive(Debug, Clone)]
pub struct Ema {
    period: usize,
    multiplier: f64,
    seed_buffer: Vec<f64>,
    value: Option<f64>,
}

impl Ema {
    pub fn new(period: usize) -> Self {
        assert!(period > 0, "EMA period must be > 0");
        Self {
            period,
            multiplier: 2.0 / (period as f64 + 1.0),
            seed_buffer: Vec::with_capacity(period),
            value: None,
        }
    }

    pub fn update(&mut self, close: f64) -> Option<f64> {
        if let Some(prev) = self.value {
            let v = (close - prev) * self.multiplier + prev;
            self.value = Some(v);
            return Some(v);
        }
        self.seed_buffer.push(close);
        if self.seed_buffer.len() == self.period {
            let seed = self.seed_buffer.iter().sum::<f64>() / self.period as f64;
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

    #[test]
    fn seeds_with_simple_average_of_first_period_values() {
        let mut ema = Ema::new(3);
        assert_eq!(ema.update(1.0), None);
        assert_eq!(ema.update(2.0), None);
        assert_eq!(ema.update(3.0), Some(2.0)); // (1+2+3)/3
    }

    #[test]
    fn applies_multiplier_after_seeding() {
        let mut ema = Ema::new(3); // multiplier = 2/4 = 0.5
        ema.update(1.0);
        ema.update(2.0);
        ema.update(3.0); // seed = 2.0
        let next = ema.update(10.0).unwrap();
        // (10 - 2) * 0.5 + 2 = 6.0
        assert!((next - 6.0).abs() < 1e-9);
    }
}
