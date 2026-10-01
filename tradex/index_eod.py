"""Daily OHLC for the index underlyings (spot), via jugaad-data.

jugaad-data pulls these from niftyindices.com. We chunk the request by year so a
single flaky call doesn't lose the whole range, and append to a CSV per index.
"""
from __future__ import annotations

from datetime import date

import pandas as pd
from jugaad_data.nse import index_df

import config


def fetch_index(nse_name: str, start: date, end: date) -> pd.DataFrame:
    frames: list[pd.DataFrame] = []
    yr = start.year
    while yr <= end.year:
        chunk_start = max(start, date(yr, 1, 1))
        chunk_end = min(end, date(yr, 12, 31))
        try:
            part = index_df(symbol=nse_name, from_date=chunk_start, to_date=chunk_end)
            if part is not None and len(part):
                frames.append(part)
        except Exception as exc:  # noqa: BLE001 - upstream raises bare Exceptions
            print(f"  ! {nse_name} {yr}: {exc}")
        yr += 1
    if not frames:
        return pd.DataFrame()
    df = pd.concat(frames, ignore_index=True)
    df.columns = [c.strip().upper().replace(" ", "_") for c in df.columns]
    # niftyindices.com sometimes returns duplicate/label columns - keep OHLC + date.
    df = df.loc[:, ~df.columns.duplicated()]
    date_col = next((c for c in df.columns
                     if c in ("HISTORICALDATE", "DATE", "INDEX_DATE", "TIMESTAMP")), df.columns[0])
    df = df.rename(columns={date_col: "DATE"})
    df["DATE"] = pd.to_datetime(df["DATE"])
    keep = ["DATE"] + [c for c in ("OPEN", "HIGH", "LOW", "CLOSE") if c in df.columns]
    df = df[keep].apply(
        lambda s: pd.to_numeric(s, errors="coerce") if s.name != "DATE" else s
    )
    df.insert(1, "INDEX", nse_name)
    return df.drop_duplicates(subset="DATE").sort_values("DATE").reset_index(drop=True)


def save_all(start: date, end: date) -> None:
    for nse_name, stem in config.INDEX_SPOT.items():
        print(f"[index] {nse_name} -> {stem}.csv")
        df = fetch_index(nse_name, start, end)
        if df.empty:
            print(f"  ! no data for {nse_name}")
            continue
        out = config.INDEX_DIR / f"{stem}.csv"
        df.to_csv(out, index=False)
        print(f"  saved {len(df)} rows -> {out}")
