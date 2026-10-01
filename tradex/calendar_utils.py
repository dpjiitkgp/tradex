"""Trading-day iteration.

We don't ship a hard-coded NSE holiday table (it drifts every year and is a
common source of silent bugs). Instead we iterate weekdays and let the download
layer tell us a day had no data; those misses are cached so re-runs are fast.
"""
from __future__ import annotations

import json
from datetime import date, timedelta
from pathlib import Path

import config

_MISS_FILE: Path = config.RAW_FO / "_no_data_dates.json"


def weekdays(start: date, end: date):
    d = start
    one = timedelta(days=1)
    while d <= end:
        if d.weekday() < 5:  # Mon-Fri
            yield d
        d += one


def load_misses() -> set[str]:
    if _MISS_FILE.exists():
        return set(json.loads(_MISS_FILE.read_text()))
    return set()


def record_miss(d: date) -> None:
    misses = load_misses()
    misses.add(d.isoformat())
    _MISS_FILE.write_text(json.dumps(sorted(misses), indent=0))
