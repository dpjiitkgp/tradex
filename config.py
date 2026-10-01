"""Central configuration for the tradex data pipeline."""
from datetime import date
from pathlib import Path

ROOT = Path(__file__).resolve().parent

DATA = ROOT / "data"
RAW_FO = DATA / "raw" / "fo"            # raw bhavcopy zips exactly as downloaded from NSE
INTERIM_FO = DATA / "interim" / "fo_daily"   # one normalized parquet per trading day
PROCESSED = DATA / "processed"          # consolidated per-underlying datasets
INDEX_DIR = DATA / "index"             # spot OHLC for the underlyings

for _p in (RAW_FO, INTERIM_FO, PROCESSED, INDEX_DIR):
    _p.mkdir(parents=True, exist_ok=True)

# ---- What to download -------------------------------------------------------
START_DATE = date(2020, 1, 1)
# END_DATE is resolved at run time to "yesterday" unless overridden on the CLI.

# Underlyings we care about in phase 1. NSE uses these tickers in the bhavcopy.
UNDERLYINGS = ["NIFTY", "BANKNIFTY"]

# Index spot series: NSE index name -> output filename stem
INDEX_SPOT = {
    "NIFTY 50": "NIFTY50",
    "NIFTY BANK": "NIFTYBANK",
}

# NSE switched the F&O bhavcopy to the UDiFF format on this date.
UDIFF_CUTOVER = date(2024, 7, 5)

# Politeness / resilience knobs for hitting NSE.
REQUEST_PAUSE_SEC = 0.6     # sleep between successful downloads
MAX_RETRIES = 4
RETRY_BACKOFF_SEC = 3       # multiplied by attempt number
