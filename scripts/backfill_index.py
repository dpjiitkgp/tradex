"""Backfill index spot OHLC (NIFTY 50, NIFTY BANK) into data/index/*.csv.

Usage:
    python scripts/backfill_index.py
    python scripts/backfill_index.py --from 2020-01-01 --to 2026-08-29
"""
from __future__ import annotations

import argparse
import sys
from datetime import date, datetime, timedelta
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

import config
from tradex import index_eod


def _parse(d: str) -> date:
    return datetime.strptime(d, "%Y-%m-%d").date()


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--from", dest="start", type=_parse, default=config.START_DATE)
    ap.add_argument("--to", dest="end", type=_parse,
                    default=date.today() - timedelta(days=1))
    args = ap.parse_args()
    index_eod.save_all(args.start, args.end)


if __name__ == "__main__":
    main()
