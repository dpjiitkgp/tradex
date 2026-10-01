---
name: intraday-edge-research
description: Test whether a change to the papertrader intraday short book actually improves it — entry filters, timing windows, parameters, stock selection. Use when asked to improve win rate or Avg R, analyse why trades lose, or evaluate any strategy/config change before it reaches the live config.
---

# Intraday edge research

Every question here reduces to one: **does this change raise Avg R per trade
taken, on data that was not used to invent the change?** Almost every
plausible-looking improvement fails that test. This skill exists because the
default way of looking for edges — slice the trades, find the bad bucket,
filter it — reliably produces confident nonsense on this dataset.

## The three numbers that decide everything

| number | value | meaning |
|---|---|---|
| **breakeven** | **0.061 R** | Rs206 round-trip cost / 0.75% risk unit. A bucket at +0.03 R is a *losing* bucket. |
| **slot opportunity cost** | **~0.24 R** | The book average. The engine holds ONE position at a time, so a trade you skip is replaced by a typical trade, not by nothing. |
| **per-trade noise** | **sigma ~1.4 R** | Standard error = 1.4/sqrt(n). At n=25 that is +/-0.28 — larger than any effect worth chasing. |

Optimise **Avg R per trade taken**. Win% is not a goal; it can fall while Avg R
rises. Total R per stock is not a goal either — removing mediocre-but-positive
trades lowers a stock's total R while still being correct, because the freed
slot goes to a better trade elsewhere.

## Protocol

**1. State the hypothesis before looking at results.** "Entries after a >1% gap
down are worse" is testable. "Find what's wrong with HAL" produces whatever
the noise happens to favour that day.

**2. Pool for power; never search per-stock.** One stock is ~400 trades, so
buckets are 20-50 and everything looks dramatic. The 15-stock book is ~4,800
trades. Discover on the pool, then check a surviving rule is consistent
per-stock — not the reverse.

**3. Validate out of sample.** Split by date (`--split-date`). A rule found on
the whole sample and tested on part of it is still contaminated, so treat a
weak held-out result as fatal rather than encouraging.

**4. Judge a filter by what it REMOVES, out of sample.** This is the step that
catches self-deception. What a filter keeps always looks better by
construction. What it removes is the question:

- removed trades average **below 0.061 R** -> the filter pays
- removed trades average **near or above ~0.24 R** -> the filter is
  *harmful*; it frees the slot for something worse

A real case: three context filters on HAL removed trades averaging **-0.116 R**
in-sample and **+0.307 R** out-of-sample. A complete sign flip. In-sample they
looked like +5.4pp win rate; they were noise.

**5. Break any surviving effect down BY YEAR.** A single date split is not
enough here. An effect can pass an in-sample/held-out comparison purely because
one strong year landed on the right side of the cut. If the yearly sign
alternates, it is noise however clean the aggregate looked.

The case that established this: narrowing the entry window to 11:00-11:29
scored 0.2278 vs 0.2077 overall, and *better still* held out (0.2606 vs
0.1922, win 57.4%) — it even survived controlling for stock composition. Year
by year the difference ran +0.003, -0.316, +0.229, +0.001, -0.134, **+0.289**,
-0.030, +0.068. The entire effect was 2024, sitting at the start of the
held-out window.

**6. Verify the experiment actually varied what you think.** If variants return
identical or near-identical results, the parameter probably never took effect.
Falsify with an absurd value (e.g. a 90-minute entry delay) and confirm the
output moves. See the warmup trap below — an entire 4-variant sweep once
returned byte-identical trades.

## The warmup trap

`VwapTrendFollow` only receives bars inside its `window()`. With the default
`MidMorning` window it sees nothing before 09:45, so EMA(21) and
RelativeVolume(10) are not warm until ~10:06 — and `vwap_history` is only
pushed *after* those indicator lookups in `on_bar`, so the slope buffer starts
filling then too. Effective earliest entry is therefore roughly
`10:06 + slope_lookback`, which for lookback 30 is ~10:37 — later than the
45-minute gate, so the gate never binds.

Consequences:
- The live book's 10:30 start is mostly an artefact of warmup, not a decision.
- **Slope lookback partly controls the effective start time**, so per-stock
  lookback tuning was partly tuning "how late this stock starts trading".
- To vary entry time cleanly, use `.with_window(TimeWindow::FullDay)` so
  everything warms from 09:15, then control the start with
  `.with_min_entry_time()` and pin `.with_max_entry_time()` so only the start
  moves.

## Tools

```bash
# 1. Generate a stock's trades at a specific config
cargo run -q -p papertrader -- research \
  --only <strategy_id> --symbols HAL --min-bars 40000 \
  --risk-pct 0.0075 --min-reward-multiple 3.0 --trail-param 0.0025 \
  --label-prefix stockanalysis/HAL

# 2. Slice them by market context, with held-out validation
python3 research/stock-analysis/analyze_stock.py \
  --symbol HAL --run <run_id> --split-date 2024-01-01
```

`analyze_stock.py` buckets by entry time, day of week, overnight gap, previous
day move, 5-session trend, extension from open, realised range, position in
day range, and volume burst — all computable at entry time, because anything
the live engine could not evaluate before placing the order is useless. It
prints per-bucket Avg R against breakeven, then a filter-impact table, then the
held-out comparison. Add new candidate rules to `CANDIDATES` in that file.

New strategy variants are registered in the `strategies` vec inside
`run_research` (cli/src/main.rs). Add a variant, `cargo build`, then use its
name with `--only`.

## Data

`data/papertrader.db` (~29GB). Market context beyond the stock itself:
`NIFTY 50` and `INDIA VIX` minute candles, fetched with
`backfill --indices "NIFTY 50,INDIA VIX"` (indices live in a different Kite
segment from equities, so the seed-list path cannot see them).

**PIIND is excluded** via `[universe].exclude` — Kite serves cross-contaminated
bars for it and re-fetching reproduces them. Raw SQL against `candles`/`trades`
still sees it, so add `instrument_token <> 6191105` to ad-hoc queries.

Research runs checkpoint the WAL on exit. If you write to the DB some other
way during a long batch, checkpoint manually — an uncheckpointed WAL once grew
to 29GB beside a 28GB database and nearly filled the disk.

## Reporting

State the held-out number first, then the in-sample one, and say plainly which
is which. If a change does not clear breakeven out of sample, say it failed —
a negative result that stops a bad change reaching live capital is worth more
than an encouraging in-sample table.
