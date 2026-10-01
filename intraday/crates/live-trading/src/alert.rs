//! What the engine thinks is worth telling a human about.
//!
//! Alerts are *queued*, not sent. `LiveEngine` pushes values onto a list and
//! the caller drains it with `take_alerts`; nothing here touches the network.
//! Two reasons: the trading loop must never block on a notification service,
//! and an emitted alert is then an ordinary value a test can assert on rather
//! than a side effect it has to intercept.
//!
//! ## What is deliberately NOT here
//!
//! Signals, stop ratchets, status polls and exit re-prices. Each fires many
//! times a session, and a channel that buzzes on every bar gets muted — at
//! which point the one alert that mattered is missed too. Those stay in
//! `tracing` where they can be read after the fact.
//!
//! The rule applied below: alert on anything that changes what you own, or
//! that a human might have to act on. Everything else is a log line.

use core_types::{Direction, ExitReason};

#[derive(Debug, Clone, PartialEq)]
pub enum Alert {
    /// A position was opened. Carries the whole plan, so the message is
    /// enough to reconstruct the trade from a phone without opening Kite.
    Entry {
        symbol: String,
        direction: Direction,
        qty: u32,
        price: f64,
        stop: f64,
        target: f64,
    },
    /// A position was closed. The day's actual news.
    Exit {
        symbol: String,
        price: f64,
        reason: ExitReason,
        r_multiple: f64,
        pnl_abs: f64,
    },
    /// The broker refused an order. Nothing is held, but a rejection that
    /// repeats is a configuration problem worth catching on the first one.
    OrderRejected { symbol: String, what: String, reason: String },
    /// An entry limit never filled and was cancelled. No position, no loss —
    /// but it means a validated signal was not taken, which is worth knowing
    /// when reconciling the day against the backtest.
    EntryUnfilled { symbol: String, seconds: u64 },
    /// An exit chased the price for the full window and gave up. Means price
    /// ran away from the exit, which is the expensive case.
    ExitEscalated { symbol: String, bars: u32, remaining: u32 },
    /// New entries stopped. Either the sentinel file, a silent feed, or a
    /// reconciliation mismatch — the reason says which.
    Halted { reason: String },
    /// Something is wrong that a person has to look at, now.
    Critical { message: String },
}

impl Alert {
    pub fn critical(message: impl Into<String>) -> Self {
        Self::Critical { message: message.into() }
    }

    /// Whether this needs a human *now*, as opposed to being a record of
    /// something that went as designed. Callers may route the two differently;
    /// both still reach the phone.
    pub fn is_critical(&self) -> bool {
        matches!(self, Alert::Critical { .. } | Alert::Halted { .. } | Alert::OrderRejected { .. })
    }

    /// One line, phone-shaped: the verdict first, the detail after. Read on a
    /// lock screen without expanding it.
    pub fn message(&self) -> String {
        match self {
            Alert::Entry { symbol, direction, qty, price, stop, target } => {
                let side = match direction {
                    Direction::Long => "LONG",
                    Direction::Short => "SHORT",
                };
                format!(
                    "{side} {symbol} {qty} @ Rs.{price:.2}\nstop Rs.{stop:.2} | target Rs.{target:.2}"
                )
            }
            Alert::Exit { symbol, price, reason, r_multiple, pnl_abs } => {
                format!(
                    "EXIT {symbol} @ Rs.{price:.2} ({})\n{r_multiple:+.2}R | Rs.{pnl_abs:+.0}",
                    describe_reason(*reason)
                )
            }
            Alert::OrderRejected { symbol, what, reason } => {
                format!("REJECTED: {what} {symbol}\n{reason}")
            }
            Alert::EntryUnfilled { symbol, seconds } => {
                format!("{symbol} entry unfilled after {seconds}s — cancelled, no position taken")
            }
            Alert::ExitEscalated { symbol, bars, remaining } => {
                format!(
                    "{symbol} exit did not fill in {bars} bars — took market for {remaining}\nprice ran away from the exit"
                )
            }
            Alert::Halted { reason } => format!("HALTED — no new entries\n{reason}"),
            Alert::Critical { message } => message.clone(),
        }
    }
}

/// Plain words rather than the enum's spelling — these land on a phone, where
/// `TrailingStop` reads worse than `trailing stop`.
fn describe_reason(reason: ExitReason) -> &'static str {
    match reason {
        ExitReason::StopLoss => "stop loss",
        ExitReason::TrailingStop => "trailing stop",
        ExitReason::EodSquareOff => "end of day",
        ExitReason::MaxHold => "max hold",
        ExitReason::VwapCross => "vwap cross",
        ExitReason::Manual => "manual",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_entry_message_carries_the_whole_plan() {
        let m = Alert::Entry {
            symbol: "HAL".into(),
            direction: Direction::Short,
            qty: 22,
            price: 4520.5,
            stop: 4554.4,
            target: 4418.8,
        }
        .message();
        // Everything needed to check the trade against Kite from a lock
        // screen, without opening the app.
        for part in ["SHORT", "HAL", "22", "4520.50", "4554.40", "4418.80"] {
            assert!(m.contains(part), "{part:?} missing from {m:?}");
        }
    }

    #[test]
    fn an_exit_message_leads_with_the_result() {
        let m = Alert::Exit {
            symbol: "HAL".into(),
            price: 4480.0,
            reason: ExitReason::TrailingStop,
            r_multiple: 0.89,
            pnl_abs: 890.0,
        }
        .message();
        assert!(m.starts_with("EXIT HAL"), "{m:?}");
        assert!(m.contains("trailing stop"), "reason should be plain words: {m:?}");
        assert!(m.contains("+0.89R"), "sign matters at a glance: {m:?}");
    }

    /// A loss must be unmistakable from a negative sign alone.
    #[test]
    fn a_losing_exit_shows_its_sign() {
        let m = Alert::Exit {
            symbol: "ITC".into(),
            price: 410.0,
            reason: ExitReason::StopLoss,
            r_multiple: -1.0,
            pnl_abs: -3375.0,
        }
        .message();
        assert!(m.contains("-1.00R"), "{m:?}");
        assert!(m.contains("-3375"), "{m:?}");
    }

    /// The tiering is the whole design: routine trade records must not be
    /// classed the same as things needing intervention, or the distinction
    /// stops meaning anything.
    #[test]
    fn routine_trade_events_are_not_critical() {
        let entry = Alert::Entry {
            symbol: "X".into(),
            direction: Direction::Long,
            qty: 1,
            price: 1.0,
            stop: 1.0,
            target: 1.0,
        };
        let exit = Alert::Exit {
            symbol: "X".into(),
            price: 1.0,
            reason: ExitReason::TrailingStop,
            r_multiple: 0.0,
            pnl_abs: 0.0,
        };
        assert!(!entry.is_critical());
        assert!(!exit.is_critical());
        assert!(!Alert::EntryUnfilled { symbol: "X".into(), seconds: 60 }.is_critical());
    }

    #[test]
    fn things_needing_a_human_are_critical() {
        assert!(Alert::critical("reconciliation mismatch").is_critical());
        assert!(Alert::Halted { reason: "feed silent".into() }.is_critical());
        assert!(Alert::OrderRejected {
            symbol: "X".into(),
            what: "entry".into(),
            reason: "margin".into()
        }
        .is_critical());
    }

    /// Every exit reason has to render, or a rare one arrives as a debug blob
    /// on the one day it fires.
    #[test]
    fn every_exit_reason_renders_as_plain_words() {
        for r in [
            ExitReason::StopLoss,
            ExitReason::TrailingStop,
            ExitReason::EodSquareOff,
            ExitReason::MaxHold,
            ExitReason::VwapCross,
            ExitReason::Manual,
        ] {
            let d = describe_reason(r);
            assert!(!d.is_empty());
            assert!(d.chars().all(|c| c.is_lowercase() || c == ' '), "{d:?} is not plain words");
        }
    }
}
