"""Quick end-to-end check against live NSE: 3 dates across both formats."""
from __future__ import annotations

import sys
from datetime import date
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from tradex.fo_bhavcopy import fetch_underlyings

TEST_DATES = [
    date(2023, 3, 15),   # legacy format
    date(2024, 7, 5),    # UDiFF cutover day
    date(2026, 8, 28),   # recent
]

for d in TEST_DATES:
    print(f"\n=== {d} ===")
    try:
        df = fetch_underlyings(d, ["NIFTY", "BANKNIFTY"])
    except Exception as exc:  # noqa: BLE001
        print(f"  ERROR: {exc}")
        continue
    if df is None or df.empty:
        print("  no data (holiday or endpoint gone)")
        continue
    print(f"  rows={len(df)}  symbols={sorted(df['symbol'].unique())}  "
          f"instruments={sorted(df['instrument'].unique())}")
    print(f"  expiries={sorted(str(x.date()) for x in df['expiry'].unique())[:6]} ...")
    g = df[(df.symbol == 'NIFTY') & (df.instrument == 'OPTIDX')]
    print(f"  NIFTY option rows={len(g)}  strike range={g['strike'].min()}-{g['strike'].max()}")
    print(df.head(3).to_string())
