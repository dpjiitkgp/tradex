# Entry window research — 2026-09-29

Question asked: the live book starts entering at 10:30; would starting earlier
(09:45) be better?

**Answer: no, and entry start time is not a meaningful lever in either
direction.** Details below, including two structural findings about the system
that matter more than the original question.

## 1. The first experiment was broken (worth reading before running another)

The obvious test — take the live variant, vary
`min_time_into_window_minutes` over 0/15/30/45, compare — returned
**byte-identical trades for all four variants**: 3,543 trades each, and 3,543
matching `(instrument, entry_ts)` pairs between the extremes.

Cause: `VwapTrendFollow` only receives bars inside its `window()`. With the
default `MidMorning` window it sees nothing before 09:45, so EMA(21) and
RelativeVolume(10) are not warm until ~10:06 — and `vwap_history` is pushed
*after* those indicator lookups in `on_bar`, so the VWAP-slope buffer only
starts filling then too. Effective earliest entry is therefore roughly:

```
10:06  +  slope_lookback  minutes
```

For lookback 30 that is ~10:37, which is *later* than the 45-minute gate, so
the gate never bound and changing it did nothing.

Confirmed by falsification: a 90-minute gate correctly moved the floor to
11:15:00 and cut trades from 393 to 154. The parameter works; the experiment
was confounded.

**Two consequences beyond this experiment:**

- The live book's 10:30 start is mostly an **artefact of indicator warmup**,
  not a tuned decision.
- **Slope lookback silently controls effective start time** (lb10 ~10:17,
  lb20 ~10:27, lb30 ~10:37). The per-stock lookback tuning in
  `config/tune30_live.toml` was therefore partly tuning "how late this stock
  starts trading", which was never the intent. This is a live confound and
  probably worth revisiting.

**Correct design:** `.with_window(TimeWindow::FullDay)` so everything warms
from 09:15, then `.with_min_entry_time()` for the start and
`.with_max_entry_time()` pinned so only the start varies.

## 2. Entry start sweep (correct design)

FullDay window, slope lookback 20, cutoff pinned at 11:30, one shared exit
config (risk 0.75%, MRM 3.0, trail 0.75%), all 15 live stocks.

| start | trades | earliest entry | Avg R | win% |
|---|---|---|---|---|
| 09:45 | 5,386 | 09:55 | 0.1871 | 51.0 |
| 10:00 | 5,222 | 10:00 | 0.1873 | 51.0 |
| 10:15 | 4,711 | 10:15 | 0.1896 | 51.3 |
| **10:30** | 4,034 | 10:30 | **0.2077** | 52.1 |

Monotonic — every earlier start lowers Avg R. The marginal trades each slice
adds are worth 0.18 (09:45-10:00), 0.166 (10:00-10:15) and 0.082
(10:15-10:30), all below the 0.208 baseline and the last barely above the
0.061 breakeven.

### But it does not survive validation

| | pre-2024 | held out 2024+ |
|---|---|---|
| start 09:45 | 0.1869 | 0.1874 |
| start 10:30 | 0.2220 | 0.1922 |
| **advantage of 10:30** | **+0.035** | **+0.005** |

The in-sample advantage is mostly period-specific. Out of sample the two are
nearly identical, so there is no case for changing the start in either
direction. Keeping 10:30 is fine.

## 3. Time of day matters — but not where the universe says

Within the `allday` runs (entries permitted to 14:00), Avg R by half-hour.

**Full 250-stock universe** (67k+ trades per bucket) — inverted U peaking at
11:30-11:59:

| slot | n | Avg R | x breakeven |
|---|---|---|---|
| 10:30 | 67,082 | 0.0399 | 0.65 (below cost) |
| 11:00 | 58,364 | 0.0681 | 1.12 |
| 11:30 | 49,862 | 0.0736 | **1.21 peak** |
| 12:00 | 43,463 | 0.0418 | 0.69 |
| 13:00 | 34,481 | -0.0046 | negative |

**The 15 live stocks** — different shape entirely. The universe's peak slot is
a trough here:

| slot | n | Avg R | win% | x breakeven |
|---|---|---|---|---|
| 10:30 | 4,034 | 0.1359 | 49.5 | 2.23 |
| **11:00** | 3,604 | **0.2037** | **53.5** | **3.34 peak** |
| 11:30 | 3,268 | 0.1195 | 50.8 | 1.96 |
| 12:00 | 2,739 | 0.1336 | 51.9 | 2.19 |
| 12:30 | 2,446 | 0.1531 | 51.8 | 2.51 |
| 13:00 | 2,183 | 0.0967 | 51.1 | 1.59 |
| 13:30 | 2,132 | 0.0571 | 52.2 | 0.94 |

Book average across these slots is 0.135, so the 11:00-11:29 half-hour runs
~50% above it. And 3,604 trades over ~1,900 sessions is ~1.9 signals/day in
that half-hour alone — more than the single position slot can absorb, so
concentrating there would cost no trading opportunity.

**Caution:** these 15 were selected for being atypical, so universe-level
patterns should not be assumed to transfer — as this table demonstrates.

## 4. The 11:00 window — tested, and it fails

The 11:00-11:29 slot looked like the one real find of the session. Backtested
directly (FullDay, lb20, 15 stocks, shared exit config):

| window | trades | Avg R | win% |
|---|---|---|---|
| 11:00-11:29 | 2,370 | **0.2278** | 54.4 |
| 10:30-11:29 (current) | 4,034 | 0.2077 | 52.1 |
| 10:30-11:59 | 5,774 | 0.1940 | 51.8 |

Narrowing the start helps; extending the end hurts. Held out (2024+) it looked
stronger still — 0.2606 vs 0.1922, win 57.4% — and the advantage survived
restricting to the 9 full-history stocks, so it was not a
recent-listings artefact.

**But year by year it evaporates.** Difference (11:00 slice minus 10:30 slice):

| year | 10:30 slice | 11:00 slice | diff |
|---|---|---|---|
| 2019 | 0.119 | 0.122 | +0.003 |
| 2020 | 0.402 | 0.086 | -0.316 |
| 2021 | 0.161 | 0.390 | +0.229 |
| 2022 | 0.215 | 0.216 | +0.001 |
| 2023 | 0.328 | 0.194 | -0.134 |
| **2024** | 0.038 | 0.327 | **+0.289** |
| 2025 | 0.241 | 0.211 | -0.030 |
| 2026 | 0.123 | 0.191 | +0.068 |

Signs alternate with no trend. **The whole effect is 2024**, which happened to
sit at the start of the held-out window. 2025 is slightly negative, 2026
slightly positive.

**Conclusion: do not change the entry window.** Keep 10:30-11:30.

### Lesson for future work

A single date split is not enough on this dataset. An effect can clear an
in-sample/held-out comparison purely because one strong year lands on the
right side of the cut. **Always break a surviving effect down by year** — if
the sign alternates, it is noise regardless of how clean the aggregate looked.

## Method note

Everything above follows the protocol in the `intraday-edge-research` skill:
pooled across 15 stocks for power (~4-5k trades/variant, not ~400), one
variable at a time, and validated on a held-out period. The decisive number is
always **Avg R per trade taken** against the **0.061 R breakeven**, never win%.
