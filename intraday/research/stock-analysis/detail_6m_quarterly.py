#!/usr/bin/env python3
"""Per-period detail for the 6-month / quarterly selection, with a control.

Writes the basket chosen at each rebalance and how each stock actually did in
the quarter that followed — the quarter the selection never saw.

Differs from `detail_report.py` in three ways that matter:

  * it reads the trade-level database directly rather than a pre-aggregated
    TSV, so the selection rule and the min-trades threshold are parameters
    here and not baked into whatever produced the file;
  * every period carries a RANDOM-ENTRY column, so the entry's contribution
    is visible per period instead of only in the pooled total. A coin flip on
    the same basket is the only honest baseline for "did the signal do this";
  * PIIND is excluded (corrupt pre-2020 series) alongside the two indices.

The min-trades threshold is the point of the exercise: ranking a stock on four
trades measures luck, and a book built from such rankings mean-reverts. The
report prints how many stocks cleared the bar at each rebalance so a threshold
that is quietly rejecting the whole universe is visible rather than hidden.

    python3 detail_6m_quarterly.py --out research/stock-selection/detail-6m-quarterly.md
"""
import argparse
import math
import sqlite3
import statistics

BREAKEVEN = 0.061
SIGMA = 1.4
EXCLUDE = {"PIIND", "NIFTY 50", "INDIA VIX"}


def month_of(d):
    return int(d[:4]) * 12 + int(d[5:7]) - 1


def mlabel(m):
    return f"{m // 12}-{m % 12 + 1:02d}"


def qlabel(m):
    return f"{m // 12}Q{m % 12 // 3 + 1}"


def load(con, run_id):
    out = {}
    for s, d, r in con.execute(
        """SELECT i.tradingsymbol, date(t.entry_ts), t.r_multiple
           FROM trades t JOIN instruments i ON i.instrument_token = t.instrument_token
           WHERE t.run_id = ? AND t.r_multiple IS NOT NULL""", (run_id,)):
        if s in EXCLUDE:
            continue
        out.setdefault((s, month_of(d)), []).append(r)
    return out


def agg(data, sym, months):
    rs = [r for m in months for r in data.get((sym, m), [])]
    if not rs:
        return 0, 0.0, 0.0
    return len(rs), statistics.fmean(rs), 100.0 * sum(1 for r in rs if r > 0) / len(rs)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--db", default="data/papertrader.db")
    ap.add_argument("--strategy-run", type=int, default=110)
    ap.add_argument("--random-run", type=int, default=263)
    ap.add_argument("--window", type=int, default=6, help="months of trailing data")
    ap.add_argument("--refresh", type=int, default=3, help="months between rebalances")
    ap.add_argument("--topn", type=int, default=25)
    ap.add_argument("--first-test", default=None, metavar="YYYY-MM",
                    help="force the first test period to start here, so two "
                         "different windows cover the same years")
    ap.add_argument("--min-trades", type=int, default=15)
    ap.add_argument("--min-win", type=float, default=48.0)
    ap.add_argument("--min-turnover-per-min", type=float, default=0.0,
                    help="liquidity floor in rupees of traded value per minute, "
                         "measured on trailing months only; 0 disables")
    ap.add_argument("--daily-db", default="research/swing/data/daily.sqlite")
    ap.add_argument("--out", default="research/stock-selection/detail-6m-quarterly.md")
    ap.add_argument("--r-value", type=float, default=3750.0,
                    help="rupees per 1 R (0.75%% of ~Rs.5L notional at Rs.1L margin)")
    ap.add_argument("--live-trades-per-year", type=int, default=250,
                    help="the single position slot takes roughly one trade a day")
    a = ap.parse_args()

    con = sqlite3.connect(a.db)
    con.execute("PRAGMA busy_timeout=60000")
    S, R = load(con, a.strategy_run), load(con, a.random_run)

    # Trailing turnover per (symbol, month): median daily traded value that
    # month. Applied on TRAILING months only, so the floor never sees the
    # period it is selecting for.
    turnover = {}
    if a.min_turnover_per_min > 0:
        dc = sqlite3.connect(f"file:{a.daily_db}?mode=ro", uri=True)
        acc = {}
        for sym, d, v in dc.execute("SELECT tradingsymbol, d, close*volume FROM daily"):
            acc.setdefault((sym, month_of(d)), []).append(v or 0)
        turnover = {k: statistics.median(v) for k, v in acc.items()}
    floor_day = a.min_turnover_per_min * 375
    syms = sorted({s for s, _ in S})
    months = sorted({m for _, m in S})

    L = []
    w = L.append
    w(f"# Selection detail — {a.window}-month lookback, rebalanced every {a.refresh} months\n")
    w(f"At each rebalance: top **{a.topn}** by Avg R over the trailing {a.window} months, "
      f"requiring **>= {a.min_trades} trades**, win% >= {a.min_win}"
      + (f" and **>= Rs.{a.min_turnover_per_min/1e6:.1f}M/min traded value**"
         if a.min_turnover_per_min > 0 else " (no liquidity floor)") + ". "
      f"The basket then trades the next {a.refresh} months only.\n")
    if a.first_test:
        w(f"First test period forced to **{a.first_test}** so this is comparable "
          f"to a longer window, which cannot start any earlier.\n")
    w("Avg R is gross; **breakeven 0.061 R**. `random` is a coin-flip entry on the "
      "*same basket*, managed by the same stop, trail and square-off — the baseline "
      "that says whether the signal did the work. PIIND and the indices are excluded.\n")
    w(f"Strategy run {a.strategy_run}, random control run {a.random_run}.\n")
    w("---\n")

    periods = []
    m = months[0] + a.window
    # A long window cannot start testing until it has that much history, so
    # two windows compared at their natural starts are also compared over
    # different years. --first-test forces a common start.
    if a.first_test:
        want = month_of(a.first_test + "-01")
        while m < want:
            m += a.refresh
        if m != want:
            raise SystemExit(
                f"--first-test {a.first_test} is not on the {a.refresh}-month "
                f"grid from {mlabel(months[0] + a.window)}; nearest is {mlabel(m)}")
    while m <= months[-1]:
        train = list(range(m - a.window, m))
        test = list(range(m, min(m + a.refresh, months[-1] + 1)))
        cands = []
        for s in syms:
            n, r, win = agg(S, s, train)
            if not (n >= a.min_trades and win >= a.min_win):
                continue
            if floor_day > 0:
                vs = [turnover[(s, q)] for q in train if (s, q) in turnover]
                if not vs or statistics.median(vs) < floor_day:
                    continue
            cands.append((s, n, r, win))
        cands.sort(key=lambda x: -x[2])
        picks = cands[:a.topn]
        if len(picks) < 5:
            m += a.refresh
            continue

        w(f"## Train {mlabel(train[0])}..{mlabel(train[-1])}  ->  "
          f"test {mlabel(test[0])}..{mlabel(test[-1])}\n")
        w(f"{len(cands)} of {len(syms)} stocks cleared the filter; top {len(picks)} taken.\n")
        w("| stock | train n | train Avg R | train win% | test n | test Avg R | "
          "test win% | beat cost? |")
        w("|---|---|---|---|---|---|---|---|")
        prs, prr = [], []
        for s, n, r, win in picks:
            tn, tr, tw = agg(S, s, test)
            prs += [x for mm in test for x in S.get((s, mm), [])]
            prr += [x for mm in test for x in R.get((s, mm), [])]
            beat = "yes" if tn and tr > BREAKEVEN else ("NO" if tn else "-")
            w(f"| {s} | {n} | {r:.3f} | {win:.1f} | {tn} | "
              f"**{tr:+.3f}** | {tw:.1f} | {beat} |")
        if prs:
            g = statistics.fmean(prs)
            rnd = statistics.fmean(prr) if prr else float("nan")
            beat_n = sum(1 for s, *_ in picks
                         if (lambda t: t[0] and t[1] > BREAKEVEN)(agg(S, s, test)))
            w("")
            w(f"**{len(prs)} trades · Avg R {g:+.4f} · net {g-BREAKEVEN:+.4f} · "
              f"random {rnd-BREAKEVEN:+.4f} · entry {g-rnd:+.4f} · "
              f"{beat_n}/{len(picks)} stocks beat cost**\n")
            periods.append((test[0], len(prs), g - BREAKEVEN, rnd - BREAKEVEN, beat_n, len(picks)))
        m += a.refresh

    # ---- summary
    w("---\n")
    w("## Summary by period\n")
    w("| test period | trades | net R | random | entry | stocks beating cost |")
    w("|---|---|---|---|---|---|")
    for t0, n, net, rnd, bn, tot in periods:
        w(f"| {qlabel(t0)} | {n} | {net:+.4f} | {rnd:+.4f} | {net-rnd:+.4f} | {bn}/{tot} |")

    allnet = [p[2] for p in periods]
    tn = sum(p[1] for p in periods)
    wnet = sum(p[1] * p[2] for p in periods) / tn
    wrnd = sum(p[1] * p[3] for p in periods) / tn
    se = SIGMA / math.sqrt(tn)
    pos = sum(1 for x in allnet if x > 0)
    w("")
    w(f"**Pooled: {tn} trades · net {wnet:+.4f} R/trade · random {wrnd:+.4f} · "
      f"entry {wnet-wrnd:+.4f} (SE {se:.4f}) · "
      f"{pos}/{len(periods)} periods beat cost**\n")

    # ---- year view, the check that a pooled edge is not one good year
    w("## By year\n")
    w(f"1 R = Rs.{a.r_value:,.0f}. **book P&L** is every trade the unconstrained "
      f"book took that year — what the shadow book would earn. **1-slot P&L** "
      f"scales to {a.live_trades_per_year} trades/year, the rate a single position "
      f"slot actually permits.\n")
    w("| year | trades | net R | random | entry | book P&L | 1-slot P&L |")
    w("|---|---|---|---|---|---|---|")
    yrs = {}
    for t0, n, net, rnd, *_ in periods:
        yrs.setdefault(t0 // 12, []).append((n, net, rnd))
    ypos = 0
    book_total = slot_total = 0.0
    full = []            # 1-slot P&L of the years that are actually full years
    for y in sorted(yrs):
        rows = yrs[y]
        n = sum(r[0] for r in rows)
        nv = sum(r[0] * r[1] for r in rows) / n
        rv = sum(r[0] * r[2] for r in rows) / n
        ypos += nv > 0
        book = nv * n * a.r_value
        slot = nv * a.live_trades_per_year * a.r_value
        book_total += book
        slot_total += slot
        # A long window cannot start testing in January, and the data ends
        # mid-year, so the first and last rows are usually part-years. Averaging
        # a single quarter in as a "year" is how a lucky quarter becomes a
        # headline, so mark them and keep them out of the mean.
        nq = 12 // a.refresh
        partial = "" if len(rows) == nq else f" ({len(rows)}/{nq} q)"
        if not partial:
            full.append(slot)
        w(f"| {y}{partial} | {n} | {nv:+.4f} | {rv:+.4f} | {nv-rv:+.4f} | "
          f"Rs.{book:+,.0f} | Rs.{slot:+,.0f} |")
    w(f"| **total** | **{tn}** | **{wnet:+.4f}** | **{wrnd:+.4f}** | "
      f"**{wnet-wrnd:+.4f}** | **Rs.{book_total:+,.0f}** | **Rs.{slot_total:+,.0f}** |")
    w("")
    w(f"**Profitable in {ypos}/{len(yrs)} years.** Mean 1-slot year over the "
      f"{len(full)} FULL years only: **Rs.{statistics.fmean(full):+,.0f}** on "
      f"Rs.1,00,000 of margin, before live slippage. Rows marked `(k/{12 // a.refresh} q)` "
      f"are part-years and are excluded from that mean — their 1-slot figure is "
      f"an annual *rate* projected from a few months, not a year's earnings.\n")
    w("> The 1-slot column is an approximation, not a simulation. One slot does not "
      "sample these trades at random — it takes whichever stock signals first each "
      "day and discards the rest, so the realised mix differs from the book's. "
      "Treat it as the right order of magnitude.\n")
    w("> Caveats: the universe is today's Nifty 250, so names listed recently "
      "appear only in later periods. The random control shares the basket but "
      "not the entry timing, so `entry` is the signal's marginal value on a book "
      "already chosen. Live fills are adversely selected in a way no backtest "
      "here models — see the APARINDS/NMDC pair from 2026-09-30.\n")

    import os
    os.makedirs(os.path.dirname(a.out), exist_ok=True)
    open(a.out, "w").write("\n".join(L))
    print(f"wrote {a.out}  ({len(periods)} periods, {tn} trades, "
          f"net {wnet:+.4f}, entry {wnet-wrnd:+.4f})")


if __name__ == "__main__":
    main()
