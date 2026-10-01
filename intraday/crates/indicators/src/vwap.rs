use core_types::Candle;

/// Session-cumulative volume-weighted average price, using typical price
/// `(high+low+close)/3`. Caller must call `reset()` at the start of each
/// trading session — this struct has no notion of "a new day" on its own.
#[derive(Debug, Default, Clone, Copy)]
pub struct Vwap {
    cum_pv: f64,
    cum_volume: f64,
}

impl Vwap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn update(&mut self, candle: &Candle) -> Option<f64> {
        let typical = (candle.high + candle.low + candle.close) / 3.0;
        self.cum_pv += typical * candle.volume as f64;
        self.cum_volume += candle.volume as f64;
        (self.cum_volume > 0.0).then_some(self.cum_pv / self.cum_volume)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn candle(high: f64, low: f64, close: f64, volume: u64) -> Candle {
        Candle {
            ts: chrono::Utc::now(),
            open: close,
            high,
            low,
            close,
            volume,
        }
    }

    #[test]
    fn matches_hand_computed_vwap_over_three_bars() {
        let mut vwap = Vwap::new();
        // typical prices: 10, 11, 9 with volumes 100, 200, 100
        assert_eq!(vwap.update(&candle(10.0, 10.0, 10.0, 100)), Some(10.0));
        // cum_pv = 1000 + 11*200 = 3200, cum_vol = 300 -> 10.666...
        let v2 = vwap.update(&candle(11.0, 11.0, 11.0, 200)).unwrap();
        assert!((v2 - 3200.0 / 300.0).abs() < 1e-9);
        // cum_pv = 3200 + 9*100 = 4100, cum_vol = 400 -> 10.25
        let v3 = vwap.update(&candle(9.0, 9.0, 9.0, 100)).unwrap();
        assert!((v3 - 4100.0 / 400.0).abs() < 1e-9);
    }

    #[test]
    fn zero_volume_bar_is_not_a_divide_by_zero() {
        let mut vwap = Vwap::new();
        assert_eq!(vwap.update(&candle(10.0, 10.0, 10.0, 0)), None);
    }

    #[test]
    fn reset_clears_accumulated_state() {
        let mut vwap = Vwap::new();
        vwap.update(&candle(10.0, 10.0, 10.0, 100));
        vwap.reset();
        assert_eq!(vwap.update(&candle(20.0, 20.0, 20.0, 50)), Some(20.0));
    }
}
