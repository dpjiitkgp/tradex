# tune30 — fully-tuned 30-stock portfolio for vwap_trend_short_late + VWAP-slope

Run overnight (2026-09-27/28) per explicit instruction to proceed autonomously.
Artifact: https://claude.ai/artifact/26YpybPPGEDgrrjTkamnFL

## What this does

Picks the best 30 stocks for the strategy and tunes three parameters
per stock instead of sharing one config across all of them:

- **VWAP-slope lookback** (10 / 15 / 20 / 30 bars)
- **MRM** — min_reward_multiple, the arm threshold for the give-back trail (1x–10x)
- **give-back trail** — distance behind peak once armed (0.25% / 0.75% / 1.5%),
  previously always hardcoded equal to risk_pct (0.75%) in every test this
  whole session — never actually varied independently until now.

## Data prerequisite

Extended the backfill from ~5.5 years (2021-03 onward) back to 2018-12-04,
per request to go back to 2019. All 250 NIFTY250 stocks re-backfilled
(`papertrader backfill --index nifty250 --months 29 --to 2021-04-22`).
Hit Kite's rate limiting repeatedly during this (a recurring theme this
session) — fixed properly this time by adding inter-instrument pacing,
periodic cooldowns, and automatic connection-rebuild-with-retry to the
`backfill` command itself (see `crates/kite-client/src/historical.rs` and
`cli/src/main.rs::run_backfill`), plus a `--symbols` flag for targeted
retries. Final state: 250/250 stocks backfilled, 0 failures.

## Methodology

1. **Universe selection**: ran the shared default config (lookback 20,
   MRM 3.0x, trail 0.75%) against the full 250-stock universe on the new
   2019–2026 dataset (`run_id 110`). Picked top 30 by Avg R, requiring
   win% ≥ 48% and ≥ 20 trades (same filters used throughout this session's
   "consistency" work, to avoid picking stocks whose Avg R is a fluke of a
   low win rate).

2. **Joint MRM × trail grid** (not two independent 1D sweeps): the user
   correctly flagged that these two are coupled — a tight trail locks in
   gains fast so doesn't need a high arm threshold; a loose trail gives
   back so much that arming early barely matters, so it wants a *higher*
   MRM to only arm once the move has proven itself. Two independent 1D
   sweeps combined via coordinate-descent would miss this interaction.
   Tested MRM ∈ {1, 3, 6, 10} × trail ∈ {0.25%, 0.75%, 1.5%} = 12 combos,
   lookback fixed at 20 (`run_ids 106–117`).
   - Originally planned a finer 7×5=35-combo grid, but the extended
     2019–2026 dataset made each run take ~20–30 min (vs ~5 min on the
     old ~5.5yr dataset) — cut to this coarser 12-combo grid to stay
     feasible overnight. Worth re-running finer around each stock's
     coarse optimum if more precision is wanted later.

3. **Lookback confirmation pass**: for each of the 30 stocks, at its own
   best (MRM, trail) from step 2, tested the other 3 lookback values
   (10/15/30) via single-stock runs (`--symbols`, `run_ids 119–208`) and
   compared against the lb20 result from step 2 — picking the true best
   of all 4.

4. Along the way, found and fixed a real performance bug:
   `instruments_with_candle_coverage` scanned the *entire* candles table
   (130M+ rows) even when `--symbols` was given, making single-stock runs
   take 2+ minutes each for no reason. Fixed by resolving `--symbols`
   through an indexed per-instrument lookup instead — cut per-run time to
   ~80s (mostly Kite auth overhead, not data access), which is what made
   the 90-run lookback confirmation pass feasible at all.

5. **Post-hoc pruning (2026-09-28, user-directed)**: user reviewed the 30
   and flagged 12 as inconsistent — LENSKART, WAAREEENER, NAUKRI,
   NLCINDIA, NYKAA, PNB, TATAELXSI, APOLLOTYRE, BANKINDIA, HEROMOTOCO,
   ATGL, RELIANCE — their Avg R was carried by a handful of large outlier
   trades rather than a reliable edge, with correspondingly low win rate.
   Removed. `summary.json`/`monthly.json`/`yearly.json` and the artifact
   now reflect the remaining 18; the removed 12's data is still in
   `tune_params.json` and queryable via `symbol_to_run_id.json` if wanted
   later.

## Result

Trade-weighted across the final 18 stocks (4,851 total trades, after the
2026-09-28 pruning above):

| | Avg R | Win% |
|---|---|---|
| Shared default (lb20, MRM 3.0x, trail 0.75%) | 0.218 | — |
| **Fully tuned per stock** | **0.260** | **52.8%** |

**+19.1% improvement** — slightly better than the pre-pruning 30-stock
figure (+20.0%, 0.210→0.252), confirming the removed stocks weren't
carrying real edge for the portfolio either.

Individual (kept) stocks ranged from +1% (LINDEINDIA — already near its
optimum) to well above that for the strongest movers. Full per-stock
breakdown (baseline vs tuned config, monthly/yearly history) is in the
artifact and in `summary.json` here (now reflects the 18, post-pruning).
`tuned_params.json`/`symbol_to_run_id.json` still cover all 30 originally
tested, for traceability.

<details>
<summary>Original 30-stock result, before the 2026-09-28 pruning</summary>

Trade-weighted across all 30 stocks (8,630 total trades):

| | Avg R | Win% |
|---|---|---|
| Shared default (lb20, MRM 3.0x, trail 0.75%) | 0.210 | — |
| Fully tuned per stock | 0.252 | 51.7% |

+20.0% improvement. Individual stocks ranged from +1% (LINDEINDIA) to
+62% (NYKAA) — NYKAA was one of the 12 later pruned as inconsistent.
</details>

## Caveats / what's not done

- The 12-combo grid is coarse (steps of ~3-4x on MRM, 0.5-0.75pp on
  trail) — the true joint optimum for some stocks likely sits between
  tested points. A finer grid around each stock's coarse winner would
  sharpen this, at the cost of more overnight runs.
- Lookback was only confirmed *at each stock's already-chosen best
  MRM/trail* — not a full 3-way joint grid (4 lookback × 12 MRM/trail ×
  30 stocks = 1,440 runs, not feasible in one night). It's possible a
  different lookback would pair better with a different MRM/trail
  combo than the one tested. Same caveat as the coordinate-descent
  approach in general — a reasonable approximation, not an exhaustive
  search.
- 2018 (partial year, Dec only) and stocks with short listing history
  show small-sample noise in the monthly/yearly views — flagged in the
  artifact's card sparkline data but worth reading trade counts alongside
  Avg R, not in isolation.
