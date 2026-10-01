"""Merge the per-day interim parquet files into per-underlying datasets.

Output (in data/processed/):
    NIFTY_options.parquet     all OPTIDX rows for NIFTY, every day
    NIFTY_futures.parquet     all FUTIDX rows for NIFTY, every day
    BANKNIFTY_options.parquet
    BANKNIFTY_futures.parquet

These are the tables a backtester reads: one row = one contract on one day.
"""
from __future__ import annotations

import pandas as pd

import config


def build() -> None:
    files = sorted(config.INTERIM_FO.glob("*.parquet"))
    if not files:
        print("no interim files - run backfill_fo.py first")
        return
    print(f"reading {len(files)} daily files ...")
    df = pd.concat((pd.read_parquet(f) for f in files), ignore_index=True)
    df = df.sort_values(["symbol", "instrument", "expiry", "strike", "option_type", "date"])

    for sym in config.UNDERLYINGS:
        for instr, tag in (("OPTIDX", "options"), ("FUTIDX", "futures")):
            sub = df[(df["symbol"] == sym) & (df["instrument"] == instr)]
            if sub.empty:
                continue
            out = config.PROCESSED / f"{sym}_{tag}.parquet"
            sub.to_parquet(out, index=False)
            print(f"  {out.name}: {len(sub):,} rows, "
                  f"{sub['date'].min().date()} -> {sub['date'].max().date()}")


if __name__ == "__main__":
    build()
