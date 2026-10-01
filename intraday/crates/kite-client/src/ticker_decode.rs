//! Pure decoding of Kite's v3 WebSocket binary tick envelope — kept
//! separate from any networking so it can be unit-tested against
//! hand-built byte fixtures without a live connection. Layout confirmed
//! against Kite's own docs (kite.trade/docs/connect/v3/websocket):
//!
//! Envelope: `[num_packets: i16 BE] ([packet_len: i16 BE] [packet bytes])*`
//!
//! Packet (all fields i32 BE unless noted), by mode:
//! - LTP (8 bytes): instrument_token, last_price
//! - Quote (44 bytes): + last_qty, avg_price, volume, buy_qty, sell_qty,
//!   open, high, low, close
//! - Full (184 bytes): + last_trade_ts, oi, oi_day_high, oi_day_low,
//!   exchange_ts, then 120 bytes of market depth (not decoded here — we
//!   only need top-of-book price/volume, not depth, for paper trading)
//!
//! Price fields are paise for equities — divide by 100 for rupees.

use core_types::Tick;

fn read_i32_be(bytes: &[u8]) -> i32 {
    i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

/// Decodes every packet in one WebSocket binary frame. Malformed/truncated
/// envelopes decode as many whole packets as possible and stop, rather
/// than panicking — a live feed misbehaving shouldn't crash the daemon.
pub fn decode_packets(data: &[u8]) -> Vec<Tick> {
    if data.len() < 2 {
        return Vec::new();
    }
    let num_packets = i16::from_be_bytes([data[0], data[1]]) as usize;
    let mut ticks = Vec::with_capacity(num_packets);
    let mut offset = 2usize;

    for _ in 0..num_packets {
        if offset + 2 > data.len() {
            break;
        }
        let packet_len = i16::from_be_bytes([data[offset], data[offset + 1]]) as usize;
        offset += 2;
        if offset + packet_len > data.len() {
            break;
        }
        let packet = &data[offset..offset + packet_len];
        offset += packet_len;
        if let Some(tick) = decode_packet(packet) {
            ticks.push(tick);
        }
    }
    ticks
}

fn decode_packet(p: &[u8]) -> Option<Tick> {
    if p.len() < 8 {
        return None; // even LTP mode needs token + price
    }
    let instrument_token = read_i32_be(&p[0..4]) as u64;
    let last_price = read_i32_be(&p[4..8]) as f64 / 100.0;

    // Volume only exists from Quote mode (44 bytes) upward.
    let volume_traded = if p.len() >= 20 {
        read_i32_be(&p[16..20]) as u64
    } else {
        0
    };

    // Exchange timestamp only exists in Full mode (184 bytes); Quote/LTP
    // packets carry no timestamp field, so we stamp receipt time instead —
    // callers should treat Quote/LTP-mode Tick.ts as "received at", not
    // "exchange timestamp".
    let ts = if p.len() >= 64 {
        let epoch_secs = read_i32_be(&p[60..64]) as i64;
        chrono::DateTime::from_timestamp(epoch_secs, 0).unwrap_or_else(chrono::Utc::now)
    } else {
        chrono::Utc::now()
    };

    Some(Tick {
        instrument_token,
        ts,
        last_price,
        volume_traded,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn i32_be(v: i32) -> [u8; 4] {
        v.to_be_bytes()
    }

    fn envelope(packets: &[Vec<u8>]) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&(packets.len() as i16).to_be_bytes());
        for p in packets {
            out.extend_from_slice(&(p.len() as i16).to_be_bytes());
            out.extend_from_slice(p);
        }
        out
    }

    fn ltp_packet(token: i32, last_price_paise: i32) -> Vec<u8> {
        let mut p = Vec::new();
        p.extend_from_slice(&i32_be(token));
        p.extend_from_slice(&i32_be(last_price_paise));
        p
    }

    fn quote_packet(token: i32, last_price_paise: i32, volume: i32) -> Vec<u8> {
        let mut p = Vec::new();
        p.extend_from_slice(&i32_be(token)); // 0-4
        p.extend_from_slice(&i32_be(last_price_paise)); // 4-8
        p.extend_from_slice(&i32_be(10)); // 8-12 last_qty
        p.extend_from_slice(&i32_be(last_price_paise)); // 12-16 avg_price
        p.extend_from_slice(&i32_be(volume)); // 16-20 volume
        p.extend_from_slice(&i32_be(500)); // 20-24 buy_qty
        p.extend_from_slice(&i32_be(600)); // 24-28 sell_qty
        p.extend_from_slice(&i32_be(0)); // 28-32 open
        p.extend_from_slice(&i32_be(0)); // 32-36 high
        p.extend_from_slice(&i32_be(0)); // 36-40 low
        p.extend_from_slice(&i32_be(0)); // 40-44 close
        assert_eq!(p.len(), 44);
        p
    }

    fn full_packet(token: i32, last_price_paise: i32, volume: i32, exchange_ts: i32) -> Vec<u8> {
        let mut p = quote_packet(token, last_price_paise, volume);
        p.extend_from_slice(&i32_be(0)); // 44-48 last_trade_ts
        p.extend_from_slice(&i32_be(0)); // 48-52 oi
        p.extend_from_slice(&i32_be(0)); // 52-56 oi_day_high
        p.extend_from_slice(&i32_be(0)); // 56-60 oi_day_low
        p.extend_from_slice(&i32_be(exchange_ts)); // 60-64
        p.extend(std::iter::repeat(0u8).take(120)); // 64-184 market depth, unused
        assert_eq!(p.len(), 184);
        p
    }

    #[test]
    fn decodes_ltp_mode_price_correctly() {
        let data = envelope(&[ltp_packet(12345, 150050)]);
        let ticks = decode_packets(&data);
        assert_eq!(ticks.len(), 1);
        assert_eq!(ticks[0].instrument_token, 12345);
        assert!((ticks[0].last_price - 1500.50).abs() < 1e-9);
        assert_eq!(ticks[0].volume_traded, 0); // not present in LTP mode
    }

    #[test]
    fn decodes_quote_mode_price_and_volume() {
        let data = envelope(&[quote_packet(999, 250075, 123456)]);
        let ticks = decode_packets(&data);
        assert_eq!(ticks.len(), 1);
        assert_eq!(ticks[0].instrument_token, 999);
        assert!((ticks[0].last_price - 2500.75).abs() < 1e-9);
        assert_eq!(ticks[0].volume_traded, 123456);
    }

    #[test]
    fn decodes_full_mode_with_exchange_timestamp() {
        let exchange_ts = 1_700_000_000i32; // arbitrary valid unix timestamp
        let data = envelope(&[full_packet(42, 100000, 5000, exchange_ts)]);
        let ticks = decode_packets(&data);
        assert_eq!(ticks.len(), 1);
        assert_eq!(ticks[0].instrument_token, 42);
        assert!((ticks[0].last_price - 1000.0).abs() < 1e-9);
        assert_eq!(ticks[0].volume_traded, 5000);
        assert_eq!(ticks[0].ts.timestamp(), exchange_ts as i64);
    }

    #[test]
    fn decodes_multiple_packets_in_one_envelope() {
        let data = envelope(&[
            ltp_packet(1, 100000),
            quote_packet(2, 200000, 999),
            ltp_packet(3, 300000),
        ]);
        let ticks = decode_packets(&data);
        assert_eq!(ticks.len(), 3);
        assert_eq!(ticks[0].instrument_token, 1);
        assert_eq!(ticks[1].instrument_token, 2);
        assert_eq!(ticks[2].instrument_token, 3);
    }

    #[test]
    fn truncated_envelope_decodes_what_it_can_without_panicking() {
        let mut data = envelope(&[ltp_packet(1, 100000), ltp_packet(2, 200000)]);
        data.truncate(data.len() - 3); // cut off partway through the 2nd packet
        let ticks = decode_packets(&data);
        assert_eq!(ticks.len(), 1);
        assert_eq!(ticks[0].instrument_token, 1);
    }

    #[test]
    fn empty_or_tiny_frame_decodes_to_nothing() {
        assert_eq!(decode_packets(&[]).len(), 0);
        assert_eq!(decode_packets(&[0]).len(), 0);
    }
}
