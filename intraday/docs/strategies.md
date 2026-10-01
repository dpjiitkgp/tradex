# Strategies

One strategy is deployed. The rest were tested and their numbers are in
[Other strategies tested](#other-strategies-tested).

Every number here is **net of a 0.061 R breakeven** (the real MIS round trip)
and measured at the **15:10 square-off** the broker actually permits. If you
find a more flattering number elsewhere in this repo, it predates
2026-09-30 and assumes a 15:15 exit that cannot be executed — see
[the correction](#the-1515-correction).

---

## VWAP-slope short (live)

Short-only intraday mean-reversion-against-trend on NSE large/mid caps.
Implemented in [`crates/strategies/src/vwap_trend.rs`](../crates/strategies/src/vwap_trend.rs).

### The hypothesis

A stock that rolls over mid-morning — EMAs crossing down, price below a VWAP
that has itself turned down, on above-average volume — tends to keep sliding
for the next hour or two. The trade is a short held intraday and closed before
the broker's MIS cutoff.

Two refinements the data forced, both worth stating because they are not what
you would guess:

* **Long signals were dropped entirely.** The same crossover logic run long
  lost money across the whole universe. The asymmetry is a flag
  (`allow_long`), not a hardcode, so it stays testable.
* **Flatter VWAP is better, not steeper.** Partitioning 8,500 trades by
  realised VWAP slope gave a monotone ladder: +0.118 R where VWAP was nearly
  flat, down to −0.158 R where it had already fallen more than 0.35% (win rate
  47.5% → 25.5%). The edge is in catching a turn that *has not happened yet*;
  once VWAP has visibly rolled over, the move is spent and the short is
  selling into a bounce. A `max_vwap_slope_pct` ceiling exists for this but is
  **not enabled live** — see [open problems](#open-problems).

### Entry rule

All seven conditions must hold on the same one-minute bar. Parameters are the
live values from [`crates/live-trading/src/engine.rs`](../crates/live-trading/src/engine.rs).

| # | condition | live value |
|---|---|---|
| 1 | EMA(fast) crosses *below* EMA(slow) on this bar | 9 / 21 |
| 2 | Relative volume at least N x the trailing average | 1.2x over 10 bars |
| 3 | Bar time inside the entry window | **10:30–11:30 IST** |
| 4 | Short side enabled; long side disabled | short only |
| 5 | Opening-range break (09:15–09:45) agrees with the signal, when determinable | must be down |
| 6 | Bar close below VWAP | — |
| 7 | VWAP slope over N bars is flat-or-down | 20 bars, threshold 0.0 |

The 10:30 floor is `min_time_into_window_minutes = 45` measured from the
09:45 start of the mid-morning window; 11:30 is that window's end. Indicators
begin accumulating at 09:30. Early-window signals backtested materially worse,
plausibly because VWAP and the EMAs are still settling after the opening range.

Entry is a **plain limit order at the signal bar's close**, with no market
fallback: if it does not fill within 60 seconds the signal is abandoned. A
missed entry is a missed trade, never a chase.

### Exit rule

A two-phase state machine in
[`crates/risk/src/exit_manager.rs`](../crates/risk/src/exit_manager.rs). With
`risk_pct = 0.75%` and `MRM = 3.0`, for a short entered at `P`:

```
                 entry P
                    │
   hard stop ───────┼──────── P x 1.0075        (+0.75%, = -1 R)
   ARMED            │
                    │  price falls
   min target ──────┼──────── P x 0.9775        (-2.25%, = +3 R)
                    │
   TRAILING: stop ratchets to the min-target floor, then follows
             the peak by 0.75%, giving back no more than that
                    │
   square-off ──────┴──────── 15:10 IST, unconditional
```

* **Armed phase**: a single hard stop 0.75% against the position. 1 R is
  defined as this distance, so 1 R = `0.0075 x entry_price`.
* **Arming**: when price reaches 3 R in favour (2.25%), the stop jumps to that
  level — the trade can no longer lose — and the trail takes over.
* **Trailing phase**: `GiveBackFromPeak` — the stop follows the best price
  seen, 0.75% behind it. Note it locks *below* the arming floor, not at it.
* **Square-off at 15:10**: unconditional, and the one parameter that turned
  out to matter more than every other combined.

The 140-configuration grid over (stop x MRM x trail) in
[`research/sweeps/`](../research/sweeps/) found nothing that beats this by
more than noise. MRM 1.5 is significantly *worse* (t = 2.03); MRM 6.0 and a
0.25% trail are within noise.

### Universe selection

The traded book is **25 stocks, re-ranked quarterly**:

1. Rank the NSE ~250 universe by Avg R over a trailing window.
2. Require at least 15 trades in the window and a win rate of at least 48%.
3. Take the top 25.
4. Trade only those for the next quarter, then re-rank.

The live book uses a **24-month** trailing window
([`config/tune30_live.toml`](../config/tune30_live.toml)). Window length turns
out to be noise — see [results](#results).

Two things are deliberately *not* done:

* **No per-stock tuning.** Every stock runs the same (lookback 20, MRM 3.0,
  trail 0.75%). Walk-forward over 482 stock-quarters valued per-stock tuning at
  exactly 0.0000 R (t = 0.00). Each stock has ~75 trades per training window
  against sigma 1.4 R — far too few to choose between 12 configs. The earlier
  "+19% from tuning" was selection noise.
* **No liquidity floor.** A Rs.2M/min traded-value screen costs ~0.02 R up
  front, and its real benefit is position *capacity*, not edge. Deliberately
  deferred until live fills say otherwise. (They are starting to — see
  [open problems](#open-problems).)

### Sizing and costs

| | |
|---|---|
| Margin committed | Rs.1,00,000 |
| Notional at ~5x MIS leverage | ~Rs.5,00,000 |
| 1 R | 0.75% of notional = **Rs.3,750** |
| Round trip (brokerage + STT + exchange + GST + stamp) | ~Rs.223 = **0.061 R** |
| Open positions | **exactly one**, across the whole book |

The single position slot is the binding constraint, and it is why the
optimisation target is **Avg R per trade taken**, not win rate: the scarce
resource is the slot, so what matters is the quality of whatever occupies it.

Cost models for all three instruments (MIS 0.0446%, CNC delivery ~0.225%,
futures ~0.036%) are in [`research/swing/costs.py`](../research/swing/costs.py).

### Results

Unconstrained book (every signal, not the one-slot subset), strategy run 417
against random-entry control run 263, both at 15:10. Full detail:
[6-month window](../research/stock-selection/detail-6m-quarterly.md) ·
[24-month window](../research/stock-selection/detail-24m-quarterly.md).

| selection window | sample | trades | net R | random | entry edge | mean full year |
|---|---|---|---|---|---|---|
| 6 months | 2019Q2–2026Q3 | 8,573 | **+0.0269** | −0.0018 | +0.0287 | Rs.+20,584 |
| 24 months (live) | 2020Q4–2026Q3 | 6,621 | **+0.0133** | +0.0009 | +0.0124 | Rs.+4,813 |

`entry edge` is the strategy minus a coin-flip entry on the *same basket*,
managed by the same stop, trail and square-off. It is the only honest answer to
"did the signal do the work, or the stock picking?"

**The same two windows restricted to 2021Q1 onward**, which drops one
exceptional post-COVID quarter that exists in the 24m series only because that
is the first quarter a 24-month window can trade:

| window | net R | random | entry edge |
|---|---|---|---|
| 6 months | +0.0148 (0.85 SE) | −0.0087 | +0.0235 |
| 24 months (live) | **+0.0002** | −0.0003 | +0.0006 |

**Read that honestly: the live window has returned nothing since 2021.** Its
pooled +0.0133 is one quarter. The 6-month window's +0.0148 is positive but
under one standard error, which is not evidence of an edge either.

Window length itself is noise. Eight windows from 3 to 24 months, matched to a
common start, produce a sawtooth rather than a curve with an optimum: 12m looks
significant at t = +2.93 while its neighbour 9m is the worst of the lot, and 21m
looks significant while *its* neighbours are flat. That is what picking the
maximum of eight correlated series looks like. Full working:
[window-length-matched.md](../research/stock-selection/window-length-matched.md).

### The 15:15 correction

Zerodha refuses MIS orders from **15:12**, so the 15:15 square-off that every
pre-2026-09-30 number in this repo assumed was never executable. Re-measured at
15:10, on the live 24-month book:

| | 15:15 | 15:10 |
|---|---|---|
| net R / trade | +0.0559 | **+0.0133** |
| 1-slot P&L | Rs.+52,401/yr | Rs.+12,446/yr |
| quarters positive | 18/24 | 14/24 |

−0.0426 R/trade, **76% of the measured edge**, t = −3.91, 19 of 24 quarters
worse. Joining the two runs on `(instrument, entry_ts)` shows 80,781 of ~81,200
trades are the *same trade* in both, so this is purely the exit:

* 44% of trades had already stopped or trailed out before 15:10 — unaffected.
* 56% were still open, and those are the winners (+0.66 Avg R vs −0.74 for the
  ones already out). They lost 0.0325 R each.

Worse, the selected book is hurt **2.3x more than the average stock**
(−0.0426 vs −0.0182 universe-wide), because ranking by trailing Avg R
preferentially picks stocks that make their money in the last five minutes —
money the broker will not let us collect. Full working:
[square-off-1515-vs-1510.md](../research/stock-selection/square-off-1515-vs-1510.md).

---

## Other strategies tested

Numbers as measured. Breakeven is 0.061 R throughout, and the random-entry
control is re-drawn per run so its value moves by about ±0.01 between runs.

### Long side

Selected on 2022–2024, measured on 2025–2026.
[`research/long-baseline/analysis/`](../research/long-baseline/analysis/)

| strategy | picked | in-sample Avg R | OOS Avg R | universe OOS | edge | stayed positive |
|---|---|---|---|---|---|---|
| `ma_alignment` | 76 | 0.100 | 0.035 | 0.025 | +0.011 | 50/76 |
| `trend_confluence` | 34 | 0.180 | 0.019 | 0.015 | +0.004 | 17/34 |
| `ma_alignment_confirmed` | 76 | 0.102 | 0.026 | 0.025 | +0.001 | 46/76 |
| `strong_rally` | 25 | 0.176 | 0.022 | 0.021 | +0.001 | 13/25 |

### Swing / multi-day

Daily bars, 10 standard setups. Full sample 2019–2026 vs out-of-sample from
2024-01-01. [`research/swing/`](../research/swing/)

| setup | trades | net R | random | edge | t |
|---|---|---|---|---|---|
| `ma_pullback` | 21,107 | +0.077 | +0.020 | +0.057 | +4.21 |
| `macd_cross` | — | — | — | +0.058 | +3.77 |
| `nr7_breakout` | — | — | — | +0.057 | +5.62 |
| `ema_10_30` | — | — | — | +0.056 | +2.36 |
| `golden_cross` | — | — | — | +0.022 | +0.36 |
| `bollinger_rev` | — | — | — | +0.009 | +0.38 |
| `rsi2_pullback` | — | — | — | +0.005 | +0.41 |
| `donchian20` | — | — | — | +0.000 | +0.01 |
| `volume_breakout` | — | — | — | −0.014 | −0.67 |
| `high_52w` | — | — | — | −0.032 | −1.61 |

The same four best setups, entries from 2024-01-01 only:

| setup | trades | net R | random | edge | t |
|---|---|---|---|---|---|
| `ma_pullback` | 8,349 | −0.036 | +0.020 | −0.056 | −3.09 |
| `macd_cross` | 6,533 | −0.045 | +0.005 | −0.050 | −2.44 |
| `nr7_breakout` | 13,391 | −0.060 | +0.020 | −0.080 | −5.70 |
| `ema_10_30` | 2,765 | −0.100 | −0.008 | −0.093 | −2.95 |

`ma_pullback` year by year (net R, then edge over the control): 2019 −0.023 /
−0.035 · 2020 +0.223 / +0.211 · 2021 +0.161 / +0.149 · 2022 −0.128 / −0.140 ·
2023 +0.285 / +0.273 · 2024 +0.081 / +0.069 · 2025 −0.107 / −0.119 ·
2026 −0.231 / −0.243.

A constraint worth noting for this whole category: multi-day equity shorts are
illegal in India (short delivery → auction plus penalty), so the only lawful
multi-day short is stock futures, at ~0.036% round trip instead of 0.0446% and
on a universe of ~180–220 F&O-eligible names.

### Volume-spike scalping

Edge over a volume-matched, same-hour, same-session control, which rises
monotonically with the spike threshold:

| spike threshold | edge over control |
|---|---|
| 20x trailing 60-bar mean | +0.015 … +0.048 |
| 50x | +0.023 … +0.084 |

Measured against a round-trip cost of 0.0446% of notional, which at the tight
stops a scalp needs is **0.30 R at a 0.15% stop** versus 0.06 R at the book's
0.75%. [`spike_scalp.py`](../research/stock-analysis/spike_scalp.py) ·
[`volume_spikes.py`](../research/stock-analysis/volume_spikes.py)

### Intraday entry filters

14 tested, in [`research/stock-analysis/`](../research/stock-analysis/):
market-trend conditioning, drawdown and gap-down state, expiry-week effects,
sector leadership, minimum VWAP-slope floors, relative-volume thresholds above
1.2x, opening-range width, and a volatility screen.

The volatility result is the largest of them: daily range 1.8–3.3% gives
t = +5.54, and above 4.5% gives −0.267 net R. Selecting the book on volatility
captures about a third of that effect.

### Per-stock parameter tuning

0.0000 R, t = 0.00, over 482 stock-quarters of walk-forward. It changed which
configuration was used 482 times.

---

## Open problems

Honest list of what is known to be wrong or unmodelled.

1. **Market impact is not modelled at all.** Square-root estimate
   `Y x sigma x sqrt(Q/V)` puts it near **0.06 R round trip** at median
   position size — the same order as the entire remaining edge.
2. **Fill rates are optimistic.** The backtest fills 97.9% of entries.
   Classifying 8,333 entry signals by what the next bar's open did to a sell
   limit at the signal bar's close:

   | case | share | outcome |
   |---|---|---|
   | opens above our limit | 21.5% | marketable — fills **better** than modelled |
   | opens exactly at our limit | 66.5% | queue-dependent, unknowable from OHLC |
   | opens below, trades up to it | 9.9% | fills at our price, as modelled |
   | never reaches it | 2.1% | no trade |

   So the true rate is bounded between **31.5%** (every "equal" case misses)
   and 97.9%, and nothing in the data narrows it further. Live experience is at
   the bad end: two failures in two days from two different mechanisms — a
   timing miss (price touched the limit only at the bar's open, before the
   order arrived), and a size miss — 23,832 shares ordered against a
   15,534-share minute, i.e. **153% of the whole minute's volume**.
3. **No liquidity-capped sizing.** The fix for (2) is to cap order size at
   ~20% of a minute's typical volume. R is scale-invariant, so this costs no
   Avg R. Not yet implemented.
4. **The VWAP-slope ceiling is not enabled live**, despite the monotone ladder
   above being one of the cleaner findings in the repo.
5. **No daily loss cap** in the live engine.
6. **Indicators freeze while a position is open** — `runner.on_bar` is only
   reached from `try_enter`, so in `LiveEngine` *no* stock's indicators advance
   while the slot is occupied. The backtest only freezes the one held stock, so
   live and backtest diverge here.
7. **The watchdog checks liveness, not productivity.** It missed a collector
   that was alive but had stopped writing candles, twice.
