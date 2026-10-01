use core_types::{
    Direction, EntryContext, ExitDecision, ExitManager, ExitPlan, ExitReason, MarketSnapshot,
    PositionPhase, PositionState, TrailPolicy,
};

/// The one `ExitManager` this project uses: hard stop while `Armed`, then
/// once the minimum-reward floor is reached, the stop ratchets to that
/// floor and a configurable `TrailPolicy` takes over. See the crate/trait
/// docs for the full state-machine description.
pub struct DefaultExitManager;

impl ExitManager for DefaultExitManager {
    fn on_entry(&self, entry: &EntryContext) -> ExitPlan {
        let (hard_stop_price, min_target_price) = match entry.direction {
            Direction::Long => (
                entry.entry_price * (1.0 - entry.risk_pct),
                entry.entry_price * (1.0 + entry.risk_pct * entry.min_reward_multiple),
            ),
            Direction::Short => (
                entry.entry_price * (1.0 + entry.risk_pct),
                entry.entry_price * (1.0 - entry.risk_pct * entry.min_reward_multiple),
            ),
        };
        ExitPlan {
            hard_stop_price,
            min_target_price,
            trail_policy: entry.trail_policy,
        }
    }

    fn on_update(
        &self,
        mut state: PositionState,
        plan: &ExitPlan,
        market: &MarketSnapshot,
    ) -> (PositionState, Option<ExitDecision>) {
        let more_favorable = match state.direction {
            Direction::Long => market.last_price > state.peak_favorable_price,
            Direction::Short => market.last_price < state.peak_favorable_price,
        };
        if more_favorable {
            state.peak_favorable_price = market.last_price;
        }

        // The current stop is a hard exit regardless of phase — in Armed
        // it's the initial risk stop; in Trailing it's whatever the ratchet
        // has moved it to (starting from the locked-in minimum-target floor).
        let hit_stop = match state.direction {
            Direction::Long => market.last_price <= state.current_stop,
            Direction::Short => market.last_price >= state.current_stop,
        };
        if hit_stop {
            let reason = match state.phase {
                PositionPhase::Armed => ExitReason::StopLoss,
                PositionPhase::Trailing => ExitReason::TrailingStop,
            };
            return (
                state,
                Some(ExitDecision {
                    exit_price: market.last_price,
                    ts: market.ts,
                    reason,
                }),
            );
        }

        if state.phase == PositionPhase::Armed {
            let reached_min_target = match state.direction {
                Direction::Long => market.last_price >= plan.min_target_price,
                Direction::Short => market.last_price <= plan.min_target_price,
            };
            if reached_min_target {
                state.phase = PositionPhase::Trailing;
                // GiveBackFromPeak has no hard-floor guarantee — leave
                // current_stop as the Armed-phase hard stop and let the
                // trailing-candidate computation below set it from the
                // trail fraction alone (which may sit below min_target).
                // Every other policy keeps the existing "floor is
                // inviolable" contract.
                if !matches!(plan.trail_policy, TrailPolicy::GiveBackFromPeak(_)) {
                    state.current_stop = plan.min_target_price;
                }
            }
        }

        if state.phase == PositionPhase::Trailing {
            if let TrailPolicy::VwapCross = plan.trail_policy {
                if let Some(vwap) = market.vwap {
                    let crossed = match state.direction {
                        Direction::Long => market.last_price < vwap,
                        Direction::Short => market.last_price > vwap,
                    };
                    if crossed {
                        return (
                            state,
                            Some(ExitDecision {
                                exit_price: market.last_price,
                                ts: market.ts,
                                reason: ExitReason::VwapCross,
                            }),
                        );
                    }
                }
            } else if let Some(candidate_stop) = trailing_stop_candidate(&state, plan, market) {
                // Stop only ever moves favorably — never loosens.
                state.current_stop = match state.direction {
                    Direction::Long => state.current_stop.max(candidate_stop),
                    Direction::Short => state.current_stop.min(candidate_stop),
                };
            }
        }

        (state, None)
    }
}

fn trailing_stop_candidate(
    state: &PositionState,
    plan: &ExitPlan,
    market: &MarketSnapshot,
) -> Option<f64> {
    match plan.trail_policy {
        TrailPolicy::PercentFromPeak(frac) | TrailPolicy::GiveBackFromPeak(frac) => {
            Some(match state.direction {
                Direction::Long => state.peak_favorable_price * (1.0 - frac),
                Direction::Short => state.peak_favorable_price * (1.0 + frac),
            })
        }
        TrailPolicy::AtrMultiple(mult) => market.atr.map(|atr| match state.direction {
            Direction::Long => state.peak_favorable_price - atr * mult,
            Direction::Short => state.peak_favorable_price + atr * mult,
        }),
        TrailPolicy::VwapCross => None, // handled as an immediate exit above, not a stop level
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn entry(direction: Direction, trail_policy: TrailPolicy) -> EntryContext {
        EntryContext {
            direction,
            entry_price: 100.0,
            entry_ts: Utc::now(),
            risk_pct: 0.0025,
            min_reward_multiple: 2.0,
            trail_policy,
        }
    }

    fn snapshot(last_price: f64, vwap: Option<f64>, atr: Option<f64>) -> MarketSnapshot {
        MarketSnapshot {
            ts: Utc::now(),
            last_price,
            vwap,
            atr,
        }
    }

    #[test]
    fn on_entry_computes_stop_and_min_target_for_long() {
        let mgr = DefaultExitManager;
        let plan = mgr.on_entry(&entry(Direction::Long, TrailPolicy::PercentFromPeak(0.0015)));
        assert!((plan.hard_stop_price - 99.75).abs() < 1e-9);
        assert!((plan.min_target_price - 100.5).abs() < 1e-9);
    }

    #[test]
    fn on_entry_computes_stop_and_min_target_for_short() {
        let mgr = DefaultExitManager;
        let plan = mgr.on_entry(&entry(Direction::Short, TrailPolicy::PercentFromPeak(0.0015)));
        assert!((plan.hard_stop_price - 100.25).abs() < 1e-9);
        assert!((plan.min_target_price - 99.5).abs() < 1e-9);
    }

    fn armed_state(direction: Direction, entry_price: f64, stop: f64) -> PositionState {
        PositionState {
            direction,
            entry_price,
            current_stop: stop,
            peak_favorable_price: entry_price,
            phase: PositionPhase::Armed,
        }
    }

    #[test]
    fn armed_long_hits_hard_stop() {
        let mgr = DefaultExitManager;
        let plan = mgr.on_entry(&entry(Direction::Long, TrailPolicy::PercentFromPeak(0.0015)));
        let state = armed_state(Direction::Long, 100.0, plan.hard_stop_price);
        let (_, decision) = mgr.on_update(state, &plan, &snapshot(99.70, None, None));
        let decision = decision.unwrap();
        assert_eq!(decision.reason, ExitReason::StopLoss);
        assert!((decision.exit_price - 99.70).abs() < 1e-9);
    }

    #[test]
    fn reaching_min_target_locks_stop_and_switches_to_trailing() {
        let mgr = DefaultExitManager;
        let plan = mgr.on_entry(&entry(Direction::Long, TrailPolicy::PercentFromPeak(0.0015)));
        let state = armed_state(Direction::Long, 100.0, plan.hard_stop_price);
        let (state, decision) = mgr.on_update(state, &plan, &snapshot(100.5, None, None));
        assert!(decision.is_none());
        assert_eq!(state.phase, PositionPhase::Trailing);
        assert!((state.current_stop - plan.min_target_price).abs() < 1e-9);
    }

    #[test]
    fn trailing_stop_only_moves_favorably_never_loosens() {
        let mgr = DefaultExitManager;
        let plan = mgr.on_entry(&entry(Direction::Long, TrailPolicy::PercentFromPeak(0.01))); // 1% trail
        let mut state = armed_state(Direction::Long, 100.0, plan.hard_stop_price);
        // Reach min target -> Trailing, stop = 100.5
        let (s, _) = mgr.on_update(state, &plan, &snapshot(100.5, None, None));
        state = s;
        let stop_after_target = state.current_stop;

        // Price rises further -> trailing stop should ratchet up (100.5% peak -> 1% below new peak = 100.5*.99 etc,
        // but must never end up below stop_after_target once it has moved up)
        let (s, _) = mgr.on_update(state, &plan, &snapshot(105.0, None, None));
        state = s;
        assert!(state.current_stop >= stop_after_target);

        // Price dips slightly but stays above the trailing stop -> stop must not loosen back down
        let stop_before_dip = state.current_stop;
        let (s, decision) = mgr.on_update(state, &plan, &snapshot(104.0, None, None));
        state = s;
        assert!(decision.is_none());
        assert_eq!(state.current_stop, stop_before_dip);
    }

    #[test]
    fn trailing_stop_breach_exits_with_trailing_stop_reason() {
        let mgr = DefaultExitManager;
        let plan = mgr.on_entry(&entry(Direction::Long, TrailPolicy::PercentFromPeak(0.01)));
        let state = armed_state(Direction::Long, 100.0, plan.hard_stop_price);
        let (state, _) = mgr.on_update(state, &plan, &snapshot(105.0, None, None)); // Trailing, peak=105, stop~103.95
        let trailing_stop = state.current_stop;
        let (_, decision) = mgr.on_update(state, &plan, &snapshot(trailing_stop - 0.01, None, None));
        let decision = decision.unwrap();
        assert_eq!(decision.reason, ExitReason::TrailingStop);
    }

    #[test]
    fn vwap_cross_policy_exits_immediately_on_cross_not_via_stop_level() {
        let mgr = DefaultExitManager;
        let plan = mgr.on_entry(&entry(Direction::Long, TrailPolicy::VwapCross));
        let state = armed_state(Direction::Long, 100.0, plan.hard_stop_price);
        let (state, _) = mgr.on_update(state, &plan, &snapshot(100.5, Some(99.0), None)); // reach target -> Trailing
        assert_eq!(state.phase, PositionPhase::Trailing);
        // Still above the locked-in floor stop (100.5) — isolates a VWAP
        // cross from a floor breach, which would otherwise also fire here.
        let (_, decision) = mgr.on_update(state, &plan, &snapshot(100.55, Some(100.6), None));
        let decision = decision.unwrap();
        assert_eq!(decision.reason, ExitReason::VwapCross);
    }

    #[test]
    fn give_back_from_peak_locks_below_the_arming_floor_not_at_it() {
        // The user's own rule: risk_pct=0.5% (2% capital at 4x margin),
        // armed at 3R (1.5%), trail 1R (0.5%) behind peak — so crossing the
        // 1.5% arming threshold should lock the stop at ~1% profit, not the
        // full 1.5% a PercentFromPeak floor-lock would guarantee.
        let mgr = DefaultExitManager;
        let entry = EntryContext {
            direction: Direction::Long,
            entry_price: 100.0,
            entry_ts: Utc::now(),
            risk_pct: 0.005,
            min_reward_multiple: 3.0,
            trail_policy: TrailPolicy::GiveBackFromPeak(0.005),
        };
        let plan = mgr.on_entry(&entry);
        assert!((plan.min_target_price - 101.5).abs() < 1e-9); // 3R = +1.5%

        let state = armed_state(Direction::Long, 100.0, plan.hard_stop_price);
        let (state, decision) = mgr.on_update(state, &plan, &snapshot(101.5, None, None));
        assert!(decision.is_none());
        assert_eq!(state.phase, PositionPhase::Trailing);
        // peak(101.5) * (1 - 0.005) ≈ 100.99 -> ~1% profit, well below the
        // 1.5% floor PercentFromPeak would have locked in here.
        assert!((state.current_stop - 100.9925).abs() < 1e-6);
        assert!(state.current_stop < plan.min_target_price);

        // Peak advances to 2% (4R) -> stop follows to ~1.5% (matches the
        // user's second checkpoint).
        let (state, decision) = mgr.on_update(state, &plan, &snapshot(102.0, None, None));
        assert!(decision.is_none());
        assert!((state.current_stop - 101.49).abs() < 1e-6);

        // Peak advances to 2.5% (5R) -> stop follows to ~2% (third checkpoint).
        let (state, _) = mgr.on_update(state, &plan, &snapshot(102.5, None, None));
        assert!((state.current_stop - 101.9875).abs() < 1e-6);
    }

    #[test]
    fn locked_floor_still_protects_a_position_under_vwap_cross_policy() {
        let mgr = DefaultExitManager;
        let plan = mgr.on_entry(&entry(Direction::Long, TrailPolicy::VwapCross));
        let state = armed_state(Direction::Long, 100.0, plan.hard_stop_price);
        let (state, _) = mgr.on_update(state, &plan, &snapshot(100.5, Some(99.0), None)); // reach target -> Trailing
        // Price falls back to the locked floor while VWAP stays supportive
        // (no cross) — the floor itself must still trigger an exit.
        let (_, decision) = mgr.on_update(state, &plan, &snapshot(100.4, Some(99.5), None));
        let decision = decision.unwrap();
        assert_eq!(decision.reason, ExitReason::TrailingStop);
    }
}
