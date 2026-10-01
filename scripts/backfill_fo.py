"""Backfill the NIFTY + BANKNIFTY F&O bhavcopy, one trading day at a time.

Usage:
    python scripts/backfill_fo.py                     # 2020-01-01 -> yesterday
    python scripts/backfill_fo.py --from 2023-01-01
    python scripts/backfill_fo.py --from 2024-06-25 --to 2024-07-15
    python scripts/backfill_fo.py --all-contracts     # keep every symbol, not just index

Idempotent: a day whose interim parquet already exists is skipped, and days
with no data (weekend holidays, trading holidays) are remembered so re-runs
don't re-hit NSE for them.
"""
from __future__ import annotations

import argparse
import sys
import time
from datetime import date, datetime, timedelta
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

import config
from tradex import calendar_utils
from tradex.fo_bhavcopy import fetch_fo_bhavcopy, fetch_underlyings


def _parse(d: str) -> date:
    return datetime.strptime(d, "%Y-%m-%d").date()


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--from", dest="start", type=_parse, default=config.START_DATE)
    ap.add_argument("--to", dest="end", type=_parse,
                    default=date.today() - timedelta(days=1))
    ap.add_argument("--all-contracts", action="store_true",
                    help="store every underlying, not just NIFTY/BANKNIFTY index F&O")
    args = ap.parse_args()

    misses = calendar_utils.load_misses()
    days = list(calendar_utils.weekdays(args.start, args.end))
    print(f"range {args.start} -> {args.end}  ({len(days)} weekdays)")

    got = skipped = empty = failed = 0
    for i, d in enumerate(days, 1):
        stamp = d.isoformat()
        out = config.INTERIM_FO / f"{stamp}.parquet"
        if out.exists():
            skipped += 1
            continue
        if stamp in misses:
            empty += 1
            continue

        try:
            if args.all_contracts:
                df = fetch_fo_bhavcopy(d)
            else:
                df = fetch_underlyings(d, config.UNDERLYINGS)
        except RuntimeError as exc:
            print(f"[{i}/{len(days)}] {stamp}  FAIL  {exc}")
            failed += 1
            continue

        if df is None or df.empty:
            calendar_utils.record_miss(d)
            empty += 1
            continue

        df.to_parquet(out, index=False)
        got += 1
        if got % 25 == 0 or i == len(days):
            print(f"[{i}/{len(days)}] {stamp}  saved {len(df):>6} rows  "
                  f"(ok={got} skip={skipped} nodata={empty} fail={failed})")
        time.sleep(config.REQUEST_PAUSE_SEC)

    print(f"\ndone. saved={got} already-had={skipped} no-data-days={empty} failed={failed}")
    if failed:
        print("re-run the same command to retry the failed days.")


if __name__ == "__main__":
    main()
