"""Download and normalize the NSE F&O bhavcopy for a single date.

Two upstream formats exist:

* **Legacy** (up to 2024-07-04):
  ``.../content/historical/DERIVATIVES/2020/JAN/fo01JAN2020bhav.csv.zip``
  Columns: INSTRUMENT, SYMBOL, EXPIRY_DT, STRIKE_PR, OPTION_TYP, OPEN, HIGH,
  LOW, CLOSE, SETTLE_PR, CONTRACTS, VAL_INLAKH, OPEN_INT, CHG_IN_OI, TIMESTAMP

* **UDiFF** (from 2024-07-05):
  ``.../content/fo/BhavCopy_NSE_FO_0_0_0_20240705_F_0000.csv.zip``
  Columns: TradDt, FinInstrmTp, TckrSymb, XpryDt, StrkPric, OptnTp, OpnPric,
  HghPric, LwPric, ClsPric, SttlmPric, OpnIntrst, ChngInOpnIntrst,
  TtlTradgVol, TtlTrfVal, UndrlygPric, ...

Both are mapped to this canonical row schema:

    date, symbol, instrument, expiry, strike, option_type,
    open, high, low, close, settle, oi, change_in_oi,
    contracts, value_cr, underlying

``instrument`` is normalized to the legacy vocabulary: FUTIDX / OPTIDX /
FUTSTK / OPTSTK.
"""
from __future__ import annotations

import io
import zipfile
from datetime import date

import pandas as pd

import config
from tradex.nse_client import client

_MONTHS = ["JAN", "FEB", "MAR", "APR", "MAY", "JUN",
          "JUL", "AUG", "SEP", "OCT", "NOV", "DEC"]

CANON_COLS = [
    "date", "symbol", "instrument", "expiry", "strike", "option_type",
    "open", "high", "low", "close", "settle", "oi", "change_in_oi",
    "contracts", "value_cr", "underlying",
]

_UDIFF_INSTR = {
    "IDF": "FUTIDX", "IDO": "OPTIDX",
    "STF": "FUTSTK", "STO": "OPTSTK",
    # some files already use the long form
    "FUTIDX": "FUTIDX", "OPTIDX": "OPTIDX",
    "FUTSTK": "FUTSTK", "OPTSTK": "OPTSTK",
}


# ---------------------------------------------------------------------------
# URLs
# ---------------------------------------------------------------------------
def _legacy_url(d: date) -> str:
    mon = _MONTHS[d.month - 1]
    return (
        f"https://nsearchives.nseindia.com/content/historical/DERIVATIVES/"
        f"{d.year}/{mon}/fo{d.day:02d}{mon}{d.year}bhav.csv.zip"
    )


def _udiff_url(d: date) -> str:
    return (
        f"https://nsearchives.nseindia.com/content/fo/"
        f"BhavCopy_NSE_FO_0_0_0_{d:%Y%m%d}_F_0000.csv.zip"
    )


def _unzip_first_csv(blob: bytes) -> str:
    with zipfile.ZipFile(io.BytesIO(blob)) as zf:
        name = zf.namelist()[0]
        return zf.read(name).decode("utf-8", errors="replace")


# ---------------------------------------------------------------------------
# Normalizers
# ---------------------------------------------------------------------------
def _norm_legacy(csv_text: str, d: date) -> pd.DataFrame:
    df = pd.read_csv(io.StringIO(csv_text))
    df.columns = [c.strip() for c in df.columns]
    out = pd.DataFrame({
        "date": pd.Timestamp(d),
        "symbol": df["SYMBOL"].str.strip(),
        "instrument": df["INSTRUMENT"].str.strip(),
        "expiry": pd.to_datetime(df["EXPIRY_DT"], format="%d-%b-%Y"),
        "strike": pd.to_numeric(df["STRIKE_PR"], errors="coerce"),
        "option_type": df["OPTION_TYP"].astype(str).str.strip().replace("XX", ""),
        "open": df["OPEN"], "high": df["HIGH"], "low": df["LOW"], "close": df["CLOSE"],
        "settle": df["SETTLE_PR"],
        "oi": df["OPEN_INT"], "change_in_oi": df["CHG_IN_OI"],
        "contracts": df["CONTRACTS"],
        "value_cr": pd.to_numeric(df["VAL_INLAKH"], errors="coerce") / 100.0,
        "underlying": pd.NA,
    })
    _blank_future_fields(out)
    return out[CANON_COLS]


def _norm_udiff(csv_text: str, d: date) -> pd.DataFrame:
    df = pd.read_csv(io.StringIO(csv_text))
    df.columns = [c.strip() for c in df.columns]
    instr = df["FinInstrmTp"].astype(str).str.strip().map(_UDIFF_INSTR)
    opt = df["OptnTp"].astype(str).str.strip().replace({"nan": "", "XX": "", "-": ""})
    out = pd.DataFrame({
        "date": pd.to_datetime(df["TradDt"]),
        "symbol": df["TckrSymb"].astype(str).str.strip(),
        "instrument": instr,
        "expiry": pd.to_datetime(df["XpryDt"], errors="coerce"),
        "strike": pd.to_numeric(df["StrkPric"], errors="coerce"),
        "option_type": opt,
        "open": pd.to_numeric(df["OpnPric"], errors="coerce"),
        "high": pd.to_numeric(df["HghPric"], errors="coerce"),
        "low": pd.to_numeric(df["LwPric"], errors="coerce"),
        "close": pd.to_numeric(df["ClsPric"], errors="coerce"),
        "settle": pd.to_numeric(df["SttlmPric"], errors="coerce"),
        "oi": pd.to_numeric(df["OpnIntrst"], errors="coerce"),
        "change_in_oi": pd.to_numeric(df["ChngInOpnIntrst"], errors="coerce"),
        "contracts": pd.to_numeric(df["TtlTradgVol"], errors="coerce"),
        "value_cr": pd.to_numeric(df["TtlTrfVal"], errors="coerce") / 1e7,
        "underlying": pd.to_numeric(df.get("UndrlygPric"), errors="coerce"),
    })
    _blank_future_fields(out)
    return out[CANON_COLS]


def _blank_future_fields(out: pd.DataFrame) -> None:
    """Futures rows carry no strike / option type - null them for consistency."""
    fut = out["instrument"].isin(["FUTIDX", "FUTSTK"])
    out.loc[fut, "strike"] = pd.NA
    out.loc[fut, "option_type"] = ""
    out["option_type"] = out["option_type"].fillna("")


# ---------------------------------------------------------------------------
# Public entry point
# ---------------------------------------------------------------------------
def fetch_fo_bhavcopy(d: date) -> pd.DataFrame | None:
    """Return the full normalized F&O bhavcopy for ``d`` or None if no data.

    Picks the format by date but falls back to the other URL if the primary
    one 404s (there is some overlap / backfill around the cutover).
    """
    primary_udiff = d >= config.UDIFF_CUTOVER
    order = [(_udiff_url, _norm_udiff), (_legacy_url, _norm_legacy)]
    if not primary_udiff:
        order.reverse()

    for url_fn, norm_fn in order:
        blob = client().get_bytes(url_fn(d))
        if blob is None:
            continue
        try:
            csv_text = _unzip_first_csv(blob)
        except zipfile.BadZipFile:
            continue
        df = norm_fn(csv_text, d)
        if len(df):
            return df
    return None


def fetch_underlyings(d: date, symbols: list[str]) -> pd.DataFrame | None:
    """Same as :func:`fetch_fo_bhavcopy` but filtered to index F&O for ``symbols``."""
    df = fetch_fo_bhavcopy(d)
    if df is None:
        return None
    mask = df["symbol"].isin(symbols) & df["instrument"].isin(["FUTIDX", "OPTIDX"])
    return df.loc[mask].reset_index(drop=True)
