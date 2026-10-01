#!/usr/bin/env python3
"""Folds the 1-minute archive into daily bars, into its own database.

The live database holds 164M one-minute bars and no daily series, so every
swing question would otherwise re-aggregate the whole archive. This builds the
daily series once.

Writes to `research/swing/data/daily.sqlite` rather than into
`data/papertrader.db`. Two reasons: research must never contend for the live
database's write lock during a session, and a separate file makes it obvious
that this is derived data that can be deleted and rebuilt.

Open and close come from the day's first and last one-minute bar, so a day's
open is the 09:15 bar's open — not the previous close. Gaps are therefore real
in this series, which is exactly what a multi-day stop has to survive.

    python3 build_daily.py              # all instruments
    python3 build_daily.py --symbols HAL,ITC
"""
import argparse
import os
import sqlite3
import time

LIVE_DB = "data/papertrader.db"
OUT_DB = "research/swing/data/daily.sqlite"

SCHEMA = """
CREATE TABLE IF NOT EXISTS daily (
    instrument_token INTEGER NOT NULL,
    tradingsymbol    TEXT    NOT NULL,
    d                TEXT    NOT NULL,
    open  REAL NOT NULL,
    high  REAL NOT NULL,
    low   REAL NOT NULL,
    close REAL NOT NULL,
    volume INTEGER NOT NULL,
    bars   INTEGER NOT NULL,   -- 1m bars behind this day; a thin day is suspect
    PRIMARY KEY (instrument_token, d)
);
CREATE INDEX IF NOT EXISTS idx_daily_sym ON daily(tradingsymbol, d);
"""


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--live-db", default=LIVE_DB)
    ap.add_argument("--out", default=OUT_DB)
    ap.add_argument("--symbols", help="comma-separated; default all")
    ap.add_argument("--min-bars", type=int, default=60,
                    help="drop days with fewer 1m bars than this (halts, part days)")
    a = ap.parse_args()

    os.makedirs(os.path.dirname(a.out), exist_ok=True)
    # Read-only on the live DB: this must be incapable of disturbing a session.
    src = sqlite3.connect(f"file:{a.live_db}?mode=ro", uri=True)
    src.execute("PRAGMA busy_timeout=60000")
    dst = sqlite3.connect(a.out)
    dst.executescript(SCHEMA)

    # Only instruments that actually have candles. The instruments table
    # carries every NSE listing the API returned — 10,156 rows including bonds
    # and sovereign gold — of which ~256 have any history. Iterating all of
    # them wastes most of the run producing zero-day results.
    #
    # `DISTINCT instrument_token` is cheap despite the table's size: it is the
    # leading column of the primary key, so SQLite skip-scans the index rather
    # than reading 164M rows.
    rows = src.execute(
        "SELECT i.instrument_token, i.tradingsymbol FROM instruments i "
        "WHERE i.instrument_token IN (SELECT DISTINCT instrument_token FROM candles) "
        "ORDER BY i.tradingsymbol"
    ).fetchall()
    if a.symbols:
        want = {s.strip().upper() for s in a.symbols.split(",")}
        rows = [r for r in rows if r[1].upper() in want]
    print(f"Folding {len(rows)} instruments into daily bars -> {a.out}")

    t0 = time.time()
    total = 0
    for i, (token, sym) in enumerate(rows, 1):
        # SQLite guarantees that with a single MIN() or MAX() aggregate, the
        # bare columns come from the row that matched it. That gives the day's
        # first open and last close in two grouped scans instead of pulling
        # every minute bar through Python.
        firsts = dict(src.execute(
            "SELECT date(ts), open FROM candles "
            "WHERE instrument_token=? AND interval='1m' GROUP BY date(ts) "
            "HAVING ts = MIN(ts)", (token,)).fetchall())
        lasts = dict(src.execute(
            "SELECT date(ts), close FROM candles "
            "WHERE instrument_token=? AND interval='1m' GROUP BY date(ts) "
            "HAVING ts = MAX(ts)", (token,)).fetchall())
        agg = src.execute(
            "SELECT date(ts) d, MAX(high), MIN(low), SUM(volume), COUNT(*) "
            "FROM candles WHERE instrument_token=? AND interval='1m' GROUP BY d",
            (token,)).fetchall()

        out = []
        for d, hi, lo, vol, n in agg:
            if n < a.min_bars or d not in firsts or d not in lasts:
                continue
            out.append((token, sym, d, firsts[d], hi, lo, lasts[d], int(vol or 0), n))
        dst.executemany(
            "INSERT OR REPLACE INTO daily "
            "(instrument_token,tradingsymbol,d,open,high,low,close,volume,bars) "
            "VALUES (?,?,?,?,?,?,?,?,?)", out)
        dst.commit()
        total += len(out)
        if i % 25 == 0 or i == len(rows):
            print(f"  [{i}/{len(rows)}] {sym:<12} {len(out):>5} days "
                  f"({total:,} total, {time.time()-t0:.0f}s)")

    n, s, lo, hi = dst.execute(
        "SELECT COUNT(*), COUNT(DISTINCT tradingsymbol), MIN(d), MAX(d) FROM daily"
    ).fetchone()
    print(f"\nDone: {n:,} daily bars, {s} symbols, {lo} .. {hi}")


if __name__ == "__main__":
    main()
