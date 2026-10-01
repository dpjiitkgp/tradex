# Research method

The bar a result has to clear before it reaches the live config, and the
measurement conventions every number in this repo follows.

## The protocol

A candidate has to clear all six. Most stop at 5 or 6.

1. **No look-ahead.** Every indicator uses only bars strictly before the
   decision bar. A daily-timeframe signal on day D fills at day D+1's open.

2. **Net of real costs, for the instrument the trade would actually use.** MIS
   intraday is 0.0446% round trip; CNC delivery is ~5x that; futures ~0.036%.
   A strategy does not get to borrow the cheapest number.

3. **Out-of-sample**, with the selection rule re-applied using only prior data
   at each rebalance — walk-forward, not a single split.

4. **Year by year, not just pooled.** A pooled edge that lives in one year is
   not an edge. This is what the live book's numbers show: pooled +0.0133, but
   +0.0002 from 2021 onward, with the difference sitting in one quarter.

5. **Neighbouring parameters must agree.** A result that works at one stop
   length but not at the two either side of it is a fitted threshold. This is
   the most selective of the six — it is why the 12-month selection window was
   not adopted despite t = +2.93.

6. **Judge a filter by what it removes, out of sample.** A filter that raises
   Avg R by discarding trades has to be shown to discard *bad* ones rather
   than merely to shrink the sample.

## Always compare against random

Every result is reported next to a **random-entry control**
([`crates/strategies/src/random_entry.rs`](../crates/strategies/src/random_entry.rs)):
the same basket, the same stop, trail and square-off, the same time window,
with entries chosen by a seeded PRNG over `(token, timestamp, seed)`.

"The strategy made +0.03 R" answers the wrong question. The question is
whether the *signal* contributed, or whether the stock selection and the exit
management would have produced the same number on a coin flip. Sometimes they
would: in the 6-month book's 2021 and 2025, the control beat the strategy.

So results carry three numbers, not one:

| column | meaning |
|---|---|
| `net R` | strategy, after the 0.061 R breakeven |
| `random` | coin-flip entry on the same basket, same treatment |
| `entry` | `net R − random`: the signal's marginal value |

The control is re-drawn per run, so its value moves by about ±0.01 between
runs of the same configuration.

## Units: R is not comparable across stops

**1 R = `risk_pct x entry_price`.** A run with a 0.50% stop and a run with a
0.75% stop quote different rupee amounts per R, so their Avg R columns do not
compare — and the 140-config grid in [`research/sweeps/`](../research/sweeps/)
varies exactly that. Compare `Avg PnL%` across configurations, or convert to a
percentage of notional.

The breakeven moves with the stop for the same reason: a fixed ~Rs.223 round
trip is 0.061 R against a 0.75% stop and 0.30 R against a 0.15% stop. Tight
stops are more expensive per R, not cheaper.

## Scale of the noise

Per-trade R has sigma ~1.4. At 1,200 trades a year, one year's standard error
is ~0.04 R — larger than any edge measured in this project. Concretely:

* A single year's Avg R says essentially nothing.
* Separating two configurations 0.02 R apart needs ~10,000 trades.
* A stock with ~75 trades in a training window cannot support a choice between
  12 parameter sets, which is why per-stock tuning measured 0.0000 R.

## Comparison pitfalls specific to this setup

Each of these produced a confident wrong number here before it was caught.

**Two runs must differ in exactly one parameter.** An early "beats random"
comparison used a 15:15 strategy run against a 15:10 control, so the
square-off was the difference rather than the signal. Verify the parameter
from the data, not the run label — the square-off time is now confirmed by
querying the EOD exit-time histogram.

**Backtest runs are not nested subsets.** Differencing a filtered run against
a baseline by trade identity gave t = +3.77. Filtering out an early trade
frees the single position slot for a later one, so 4.6–4.7% of each variant's
trades do not exist in the baseline at all. Do not difference two runs by
trade identity unless the slot constraint is identical; the "removed" column
of `slope_threshold_sweep.py` is affected by this.

**Two selection windows need a common start.** A 24-month window cannot begin
testing until it has 24 months of history, so an unmatched comparison against
a 6-month window differs by two years of sample — including the best year —
not by window length. Use
`detail_6m_quarterly.py --first-test YYYY-MM`.

**Part-periods must not be averaged as whole ones.** A by-year table counting a
single quarter as a "year" turned one quarter into a Rs.36,675 headline where
the full-year mean was Rs.4,813. The reports now mark part-years `(1/4 q)` and
exclude them from the mean.

**Do not explain a ranking that is within noise.** A mechanism was offered for
why 12 months was the optimal lookback; under a different refresh rate the
ordering reversed. A plausible causal story makes noise much more convincing
than a bare table.

**Exclusions belong in the loader, not in memory.** `PIIND` (token 6191105)
carries a corrupted pre-2020 series from the broker's API that re-fetching
does not fix. It reached 431 trades of a headline run anyway. `PIIND`,
`NIFTY 50` and `INDIA VIX` are now excluded in code.

## Data caveats

* **Survivorship**: the universe is today's NSE ~250, so recently-listed names
  appear only in later periods and delisted names not at all.
* **Selection bias in a borrowed universe**: the 25-stock book was chosen on
  trailing performance; research reusing it inherits that bias.
* **Daily bars are folded from 1-minute data**, so a day's open is the 09:15
  bar's open. Days with fewer than 60 minute-bars are dropped as part-days or
  halts.
* **Research must never log in to Kite.** A login rotates the access token the
  live session holds and would kill it mid-session.
  [`CacheOnlyFeed`](../crates/backtest/src/cache_only_feed.rs) enforces this by
  refusing all network calls.
