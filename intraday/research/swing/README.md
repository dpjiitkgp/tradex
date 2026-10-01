# Swing research — multi-day holds

Exploring holds of roughly 1–10 trading days, exited on a stop, on a broken
setup, or on time. Separate from the intraday book in every respect: different
instrument, different cost structure, different simulator.

Nothing here touches the live engine or the live database.

---

## Read this before designing a setup

### 1. A multi-day SHORT is illegal in equity cash

The intraday book is short-only (`allow_long: false`). **That strategy cannot
be extended to a multi-day hold in the cash segment at all.** India settles
T+1, so a short you have not covered by the close is short delivery: the
exchange buys the shares in for you by auction, typically well above the close,
plus a penalty.

So the direction of the trade dictates the instrument:

| Direction | Instrument | Viable? |
|---|---|---|
| Long, multi-day | Equity **CNC** (delivery) | Yes |
| Short, multi-day | **Stock futures** (NRML) | Yes |
| Short, multi-day | Equity cash | **No** — short delivery |
| Short, intraday | Equity **MIS** | Yes — what the live book does |

`swing_backtest.py` refuses `--direction short --instrument cnc` rather than
silently producing numbers for a trade that could not be placed.

Stock futures bring their own costs that a backtest on cash prices will not
show: lot-size granularity, monthly expiry and rollover, and a universe
restricted to the ~180–220 F&O-eligible names — which is *not* the same list
as the intraday book's 25.

### 2. Delivery costs about five times intraday

`python3 costs.py` on ₹5,00,000 notional:

| Instrument | Round trip | % of notional |
|---|---|---|
| Equity MIS (intraday) | ₹223 | 0.045% |
| Equity CNC (delivery) | ₹1,127 | **0.225%** |
| Stock futures (NRML) | ₹179 | 0.036% |

Almost all of the delivery difference is STT: 0.1% on **both** legs, versus
0.025% on the sell alone for intraday.

The consequence is a design constraint, not a footnote. The same cost expressed
as breakeven R:

| Instrument | 0.75% stop | 2% stop | 3% stop | 5% stop |
|---|---|---|---|---|
| MIS | 0.060 | 0.022 | 0.015 | 0.009 |
| CNC | **0.300** | 0.113 | 0.075 | 0.045 |
| Futures | 0.048 | 0.018 | 0.012 | 0.007 |

**The intraday book's 0.75% stop is unusable on delivery** — 0.30 R gone per
round trip, against a measured intraday edge of ~0.07 R. A swing stop has to be
wide enough to dilute the cost: 2–5%, not 0.75%. That also means position sizes
are smaller for the same rupee risk, which cuts the market-impact problem the
intraday book has.

The cost model is calibrated against the intraday book's known breakeven: it
reads 0.060 R at a 0.75% MIS stop, where the live figure is 0.061.

### 3. Gaps are the new risk, and they are not optional

Intraday, a stop is a price you get. Overnight, it is a price you *hoped* for —
a stop at 95 fills at 91 if the stock opens at 91. This is the single easiest
thing to fake in a swing backtest and the reason many look better than they
trade.

`swing_backtest.py` checks the **open** before the intraday range on every day
of the hold, so a gap through the stop costs what it really costs. The exit-mix
report shows how often it happens.

---

## Files

| File | What it does |
|---|---|
| `costs.py` | Round-trip cost and breakeven R for MIS / CNC / futures |
| `build_daily.py` | Folds the 1-minute archive into `data/daily.sqlite` |
| `swing_backtest.py` | The simulator: stop, setup-broken, max hold |

`build_daily.py` reads the live database **read-only** and writes to its own
file, so it cannot contend for a write lock during a session. Derived data —
delete and rebuild freely.

---

## The entry rule is not yet defined

`swing_backtest.py` currently runs a **daily EMA 10/30 cross** as a placeholder.
It is there so the machinery has something to exercise and so a result is
comparable with the intraday book's own trigger. **It is not a recommendation
and has not been validated.**

The real question — what constitutes the setup, and what constitutes it
breaking — is still open. The current break rule is "price closes back through
the signal day's own extreme", judged on the close so a wick cannot eject a
position that finished the day fine. That is a defensible default, not a
studied one.

---

## Protocol

The same standard the intraday work is held to, because it is the standard that
killed fourteen of fourteen entry filters there:

1. **No look-ahead.** A signal on day D is filled at day D+1's open. Every
   indicator uses only bars strictly before the decision.
2. **Net of real costs**, for the instrument the trade would actually use.
3. **Out-of-sample split** before believing anything.
4. **Year by year**, not just pooled. A pooled edge living in one year is not
   an edge.
5. **Neighbouring parameters must agree.** A result that only works at one
   stop or one hold length is a fitted threshold.
6. **Judge a filter by what it removes**, out of sample — and beware comparing
   backtest runs by set arithmetic. In the intraday work that produced a
   spurious t = +3.77, because filtering out an early trade frees an instrument
   to take a later one, so runs are not nested subsets.

## Known caveats to carry forward

- The intraday book's 25 stocks were selected on trailing performance. Any
  universe borrowed from it inherits that bias; the F&O-eligible list is a
  different and largely independent screen.
- Daily bars are folded from 1-minute data, so a day's open is the 09:15 bar's
  open. Days with fewer than 60 one-minute bars are dropped as part-days or
  halts; the `bars` column keeps the count so thin days stay auditable.
- `PIIND` (token 6191105) carries a corrupted pre-2020 price series; it and the
  two cached indices (`NIFTY 50`, `INDIA VIX`) are deleted from `daily.sqlite`
  after each build. `build_daily.py` does not yet filter them itself, so redo
  this after a rebuild:
  `DELETE FROM daily WHERE tradingsymbol IN ('PIIND','NIFTY 50','INDIA VIX');`
