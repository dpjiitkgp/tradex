use chrono::{TimeZone, Timelike};
use core_types::time::IST;
use core_types::{Candle, DateTime, Tick};
use std::collections::HashMap;

/// Kite ticks report `volume_traded` as the **cumulative** volume for the
/// day (confirmed live: repeated ticks carry the same value until the next
/// trade), not a per-tick delta — so a minute bar's volume is the
/// difference between the cumulative volume at the end of the bucket and
/// at its start, not a sum of raw tick volumes.
struct Bucket {
    minute: DateTime,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    baseline_volume: u64,
    last_volume: u64,
}

/// Folds a live tick stream into 1-minute OHLCV candles per instrument —
/// what turns `MarketDataFeed::subscribe_ticks` into something
/// `PaperEngine::on_bar` (which speaks candles, identically to backtest
/// replay) can consume.
///
/// Known simplification: the very first tick of a session seeds both the
/// bucket's baseline volume and its own bar, so that first bar's volume
/// reads as 0 rather than reflecting whatever volume had already traded
/// before the stream connected — negligible for a bar that also needs
/// several bars of `RelativeVolume` history to matter to any strategy
/// anyway.
#[derive(Default)]
pub struct CandleAggregator {
    buckets: HashMap<u64, Bucket>,
}

fn minute_floor(ts: DateTime) -> DateTime {
    let ist = ts.with_timezone(&IST);
    let floored = ist.date_naive().and_hms_opt(ist.time().hour(), ist.time().minute(), 0).unwrap();
    IST.from_local_datetime(&floored).single().unwrap().with_timezone(&chrono::Utc)
}

impl CandleAggregator {
    pub fn new() -> Self {
        Self::default()
    }

    /// Clears all per-instrument bucket state — call at the start of each
    /// trading session (the live daemon's equivalent of `reset_session`).
    pub fn reset(&mut self) {
        self.buckets.clear();
    }

    /// Ingests one tick. Returns `Some((instrument_token, candle))` when
    /// this tick belongs to a new minute, finalizing the previous minute's
    /// bar — callers should feed that candle to `PaperEngine::on_bar`
    /// before this tick's own price has any effect (it starts the next
    /// bucket).
    pub fn ingest(&mut self, tick: &Tick) -> Option<(u64, Candle)> {
        let minute = minute_floor(tick.ts);
        let token = tick.instrument_token;

        match self.buckets.get_mut(&token) {
            Some(bucket) if bucket.minute == minute => {
                bucket.high = bucket.high.max(tick.last_price);
                bucket.low = bucket.low.min(tick.last_price);
                bucket.close = tick.last_price;
                bucket.last_volume = tick.volume_traded;
                None
            }
            Some(bucket) => {
                let finalized = Candle {
                    ts: bucket.minute,
                    open: bucket.open,
                    high: bucket.high,
                    low: bucket.low,
                    close: bucket.close,
                    volume: bucket.last_volume.saturating_sub(bucket.baseline_volume),
                };
                *bucket = Bucket {
                    minute,
                    open: tick.last_price,
                    high: tick.last_price,
                    low: tick.last_price,
                    close: tick.last_price,
                    baseline_volume: bucket.last_volume,
                    last_volume: tick.volume_traded,
                };
                Some((token, finalized))
            }
            None => {
                self.buckets.insert(
                    token,
                    Bucket {
                        minute,
                        open: tick.last_price,
                        high: tick.last_price,
                        low: tick.last_price,
                        close: tick.last_price,
                        baseline_volume: tick.volume_traded,
                        last_volume: tick.volume_traded,
                    },
                );
                None
            }
        }
    }

    /// Force-finalizes every instrument's current (incomplete) bucket —
    /// call at EOD so the last partial minute isn't silently dropped.
    pub fn flush(&mut self) -> Vec<(u64, Candle)> {
        self.buckets
            .drain()
            .map(|(token, b)| {
                (
                    token,
                    Candle {
                        ts: b.minute,
                        open: b.open,
                        high: b.high,
                        low: b.low,
                        close: b.close,
                        volume: b.last_volume.saturating_sub(b.baseline_volume),
                    },
                )
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn tick(token: u64, h: u32, m: u32, s: u32, price: f64, cum_volume: u64) -> Tick {
        Tick {
            instrument_token: token,
            ts: IST
                .with_ymd_and_hms(2026, 9, 22, h, m, s)
                .unwrap()
                .with_timezone(&chrono::Utc),
            last_price: price,
            volume_traded: cum_volume,
        }
    }

    #[test]
    fn first_tick_starts_a_bucket_with_no_finalized_candle() {
        let mut agg = CandleAggregator::new();
        assert!(agg.ingest(&tick(1, 9, 15, 5, 100.0, 1000)).is_none());
    }

    #[test]
    fn ticks_within_the_same_minute_update_the_open_bucket() {
        let mut agg = CandleAggregator::new();
        agg.ingest(&tick(1, 9, 15, 5, 100.0, 1000));
        assert!(agg.ingest(&tick(1, 9, 15, 40, 102.0, 1200)).is_none());
        assert!(agg.ingest(&tick(1, 9, 15, 50, 99.0, 1300)).is_none());
    }

    #[test]
    fn a_new_minute_finalizes_the_previous_bucket_with_ohlc_and_volume_delta() {
        let mut agg = CandleAggregator::new();
        agg.ingest(&tick(1, 9, 15, 5, 100.0, 1000)); // baseline=1000
        agg.ingest(&tick(1, 9, 15, 40, 102.0, 1200)); // high candidate
        agg.ingest(&tick(1, 9, 15, 50, 99.0, 1300)); // low candidate, becomes close, last_volume=1300
        let (token, candle) = agg.ingest(&tick(1, 9, 16, 1, 103.0, 1350)).unwrap();
        assert_eq!(token, 1);
        assert_eq!(candle.open, 100.0);
        assert_eq!(candle.high, 102.0);
        assert_eq!(candle.low, 99.0);
        assert_eq!(candle.close, 99.0);
        assert_eq!(candle.volume, 300); // 1300 - 1000
    }

    #[test]
    fn instruments_are_tracked_independently() {
        let mut agg = CandleAggregator::new();
        agg.ingest(&tick(1, 9, 15, 5, 100.0, 1000));
        agg.ingest(&tick(2, 9, 15, 5, 50.0, 5000));
        let r1 = agg.ingest(&tick(1, 9, 16, 0, 101.0, 1100));
        let r2 = agg.ingest(&tick(2, 9, 16, 0, 51.0, 5200));
        assert_eq!(r1.unwrap().0, 1);
        assert_eq!(r2.unwrap().0, 2);
    }

    #[test]
    fn flush_finalizes_incomplete_buckets_at_eod() {
        let mut agg = CandleAggregator::new();
        agg.ingest(&tick(1, 15, 29, 5, 100.0, 1000));
        agg.ingest(&tick(1, 15, 29, 40, 101.0, 1050));
        let flushed = agg.flush();
        assert_eq!(flushed.len(), 1);
        assert_eq!(flushed[0].1.close, 101.0);
        assert_eq!(flushed[0].1.volume, 50);
        // Buckets are cleared after flush.
        assert!(agg.flush().is_empty());
    }

    #[test]
    fn repeated_identical_cumulative_volume_contributes_zero_extra_volume() {
        // Matches real observed Kite behavior: the same cumulative volume
        // repeats across multiple ticks within a second until the next trade.
        let mut agg = CandleAggregator::new();
        agg.ingest(&tick(1, 9, 15, 5, 100.0, 1000));
        agg.ingest(&tick(1, 9, 15, 6, 100.0, 1000)); // no new volume
        let (_, candle) = agg.ingest(&tick(1, 9, 16, 0, 100.0, 1000)).unwrap();
        assert_eq!(candle.volume, 0);
    }
}
