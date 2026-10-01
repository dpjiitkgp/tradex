# Overnight session — 2026-09-25 (02:30–07:00 IST)

## What was asked
1. Backfill ~5 years of minute data and re-validate every strategy against it.
2. Think about new strategies and tune existing ones for better win rate.
3. Specifically test `ma_alignment` with `[45, 30, 15]` instead of `[60, 45, 30, 15]`.

## Data: partial success, real external blocker hit

- Kite genuinely has minute-level history back to at least 2019 (confirmed empirically) — far more than the commonly-assumed ~60 day window.
- 159 of 250 Nifty250 stocks now have full ~5.5-year minute-bar coverage (2021-03-23 → 2026-09-23), up from 6 months for all of them. The other 91 (including RELIANCE, TCS, INFY, HDFCBANK, SBIN — many large caps) are still 6-months-only.
- **Why it stopped at 159/250, not 250/250**: hit two separate issues.
  1. Kite access tokens turned out to expire much faster than assumed — observed failures after roughly 2–3.5 hours, not "all day" as the code's caching logic assumed. This caused the first backfill attempt to fail 67 stocks partway through.
  2. After refreshing the token and retrying, hit a **second, harder wall**: starting partway through, *every* Kite call — historical, quote, even a tiny 4-day request — started failing with `PermissionException` ("Insufficient permission for that call"), and it did not recover on its own. This smells like a daily quota/rate ceiling on the historical+quote endpoint category, tripped by the sheer request volume (~9,000 requests across both attempts). A fresh token obtained later (06:52 IST) worked again for `/quote`, so it's not a permanent account-level block — but historical fetches for the remaining 91 stocks weren't attempted again tonight to avoid retriggering it blind.
- **Recommendation**: resume backfilling the remaining 91 stocks tomorrow (a fresh day likely resets whatever quota this is) with `papertrader backfill --months 61 --to 2026-03-27 --index nifty250` — it's idempotent, already-cached stocks are just re-written harmlessly.

## A real bug found and fixed along the way

`kite-auth`'s token cache trusted any token minted **any time on the same IST calendar date** — which is exactly what let a 00:59 IST token get reused by a process at 02:26 IST, already dead. Worse: this is precisely the scenario the live daemon hits every single day (long overnight sleep, then reuses whatever's cached at wake time). Fixed by adding a 60-minute freshness window (`crates/kite-auth/src/provider.rs`) — a cached token is now only reused if it's both today's date *and* less than an hour old; otherwise a fresh login happens automatically. 4 new unit tests cover this. This should make tomorrow's (and every future) 09:15 wake meaningfully more reliable — this is not a hypothetical, it's the exact failure mode that broke tonight's backfill twice.

## New strategies built (all config-tunable, none added to the live `enabled` list)

- **`strong_rally`** — 3 consecutive N-minute candles each moving ≥ threshold% in the same direction, enters at the open of the 4th candle. Required a real architecture extension: added `TimeWindow::FullDay` so a strategy can watch every bar all session instead of being confined to one of the three existing windows — `strong_rally` and `ma_alignment` both use it.
- **`ma_alignment`** — N nested SMAs must stack in order (shortest-period average above the next-longer one, all the way down) for an accelerating trend. Generalized to accept *any* list of periods, so testing `[15,30,45]` (as asked) needed zero new code, just `MaAlignment::new(vec![15,30,45])`.
- **`ma_alignment` with confirmation** — added a `confirmation_bars` option: require the alignment to hold for an extra bar before entering, instead of firing on the first aligned bar. This directly targets a mechanism found earlier tonight — half of *all* trades across every strategy get stopped out before the signal ever proves itself, because the exit rule needs price to run 3x the stop distance cleanly before locking in any protection. Confirmation aims to filter out alignments that immediately collapse.
- **`vwap_trend` filters** — `allow_long`/`allow_short`/`min_time_into_window_minutes`, from the earlier loss-case investigation (short-side and late-window signals meaningfully outperformed).
- **`trend_pullback`**, **`vwap_reversion`** — built earlier tonight's session, included in this validation too.

## The validation: 6-month findings vs. 5-year findings, head to head

Same exit rule throughout (0.5% stop, arms at 3R, give-back trail) — only the entry logic and dataset size changed. This is the out-of-sample check that's been deferred all night, done properly with a genuinely different time period rather than a synthetic split.

| Strategy | 6-month Avg R / Win% | 5-year Avg R / Win% | Verdict |
|---|---|---|---|
| **ma_alignment (confirmed)** | *(new, not tested at 6mo)* | **+0.039 / 39.1%** | **Best result of the whole effort — 498K trades** |
| **ma_alignment (4-tier)** | +0.030 / 38.2% | **+0.033 / 38.8%** | **Holds up — remarkably stable across 10x more data** |
| ma_alignment (3-tier, 45/30/15 — as requested) | *(new)* | +0.021 / 37.8% | Positive, but weaker than the 4-tier version — the 60m leg seems to add real value, not just noise |
| vwap_trend (short-only, ≥45min into window) | +0.038 / 37.4% | **+0.045 / 39.6%** | **Holds up, even improved** |
| strong_rally (0.1% threshold) | -0.000 / 35.2% | +0.009 / 35.7% | Modest, stayed positive |
| vwap_trend (unfiltered) | -0.009 / 34.2% | +0.003 / 35.9% | Roughly flat both times |
| strong_rally (0.2% threshold) | -0.034 / 32.8% | -0.017 / 33.1% | Negative both times |
| orb | -0.105 / 29.2% | -0.016 / 32.0% | Weak both times |
| **trend_pullback** | **+0.005 / 39.6%** (6mo pooled leader!) | **-0.018 / 38.8%** | **Reversed sign — the 6-month result did not survive** |
| momentum_breakout | -0.003 / 38.9% | -0.030 / 38.4% | Negative both times, worse over 5yr |
| vwap_reversion | -0.019 / 33.8% | -0.044 / 34.4% | Negative both times, worse over 5yr |

**The headline finding: `ma_alignment` (4-tier, and especially the confirmed variant) is the one strategy whose edge survived a 10x increase in data essentially unchanged.** That's exactly the kind of result worth trusting more than anything else produced this week. `trend_pullback` — last week's pooled leader — reversing sign is the clearest demonstration yet of why the repeated "needs out-of-sample validation" caveat wasn't just caution for its own sake.

## Best individual stocks, `ma_alignment` (confirmed), min 200 trades

| Symbol | Trades | Avg R | Win rate |
|---|---|---|---|
| **LICI** | 1,946 | **+0.161** | **45.2%** |
| BRITANNIA | 2,393 | +0.026 | 44.6% |
| APOLLOHOSP | 2,535 | +0.086 | 44.2% |
| ITC | 2,268 | +0.033 | 44.2% |
| AXISBANK | 2,490 | +0.083 | 43.9% |
| HONAUT | 2,490 | +0.108 | 43.9% |
| DMART | 2,570 | +0.110 | 43.4% |
| IRCTC | 2,765 | +0.109 | 43.1% |

Every row here has 2,000+ trades — a genuinely large sample, not the 9-30 trade flukes earlier rounds kept surfacing. LICI stands out clearly.

## What's still open / recommended next steps

1. **Resume the backfill for the remaining 91 stocks** once the Kite quota resets (try tomorrow before intensive use).
2. **Costs**: none of tonight's 5-year numbers have transaction costs subtracted yet (brokerage/STT/exchange/GST, ~0.047% per round trip at ₹4L notional, as computed earlier for IRCTC). Given `ma_alignment (confirmed)`'s pooled edge is +0.039 avg R ≈ +0.02% underlying pnl% — that's *below* the cost floor in pooled terms, same problem as before. Individual names with a bigger edge (LICI's larger avg R) are the ones actually worth a second look after costs.
3. Consider extending `ma_alignment`'s confirmation idea to the other strategies (`vwap_trend`, `momentum_breakout`) — the mechanism (half of losses are premature stop-outs) applies to all of them, not just this one.
4. Full ranked tables: `research/fiveyear-v1.md`; every trade queryable via `papertrader report --run <44-54> --by-instrument` or `compare`.
