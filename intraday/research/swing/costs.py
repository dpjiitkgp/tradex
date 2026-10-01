#!/usr/bin/env python3
"""Round-trip cost for a multi-day hold, by instrument.

This is not a detail. The intraday book's breakeven is 0.061 R because MIS
costs about 0.046% of notional per round trip. Equity DELIVERY costs roughly
five times that, almost entirely because STT is charged at 0.1% on both legs
instead of 0.025% on the sell alone. Carry the intraday 0.75% stop into a
delivery trade and nearly a third of every risk unit is gone before the trade
has done anything.

Numbers are Zerodha's published equity rates as of 2026-09. They move — SEBI
and exchange charges have changed several times — so treat these as a
calibration to re-check rather than a constant.

    python3 costs.py                 # the comparison table
    python3 costs.py --notional 5e5  # for a specific position size
"""
import argparse
from dataclasses import dataclass


@dataclass(frozen=True)
class CostModel:
    name: str
    brokerage_per_order: float   # rupees, flat
    brokerage_pct: float         # fraction of turnover, per leg
    brokerage_cap: float         # rupees per order
    stt_buy: float               # fraction of buy turnover
    stt_sell: float              # fraction of sell turnover
    exch_txn: float              # fraction, per leg
    stamp_buy: float             # fraction of buy turnover
    sebi: float = 0.000001       # Rs 10 per crore, per leg
    gst: float = 0.18            # on brokerage + txn + sebi
    dp_per_sell: float = 0.0     # rupees, flat, delivery sells only
    can_hold_short: bool = True
    note: str = ""

    def round_trip(self, notional: float) -> dict:
        """Total cost of one buy and one sell at `notional`, in rupees."""
        def brokerage(t):
            return min(max(self.brokerage_per_order, self.brokerage_pct * t),
                       self.brokerage_cap) if self.brokerage_cap else \
                   max(self.brokerage_per_order, self.brokerage_pct * t)

        brk = brokerage(notional) * 2
        stt = self.stt_buy * notional + self.stt_sell * notional
        txn = self.exch_txn * notional * 2
        sebi = self.sebi * notional * 2
        stamp = self.stamp_buy * notional
        gst = self.gst * (brk + txn + sebi)
        total = brk + stt + txn + sebi + stamp + gst + self.dp_per_sell
        return {
            "brokerage": brk, "stt": stt, "exchange": txn, "sebi": sebi,
            "stamp": stamp, "gst": gst, "dp": self.dp_per_sell,
            "total": total, "pct": total / notional * 100,
        }

    def breakeven_r(self, notional: float, stop_pct: float) -> float:
        """Cost expressed in R, where 1 R = `stop_pct` of notional.

        The number that actually matters: a strategy must clear this before it
        has made anything.
        """
        return self.round_trip(notional)["total"] / (stop_pct * notional)


# Intraday, for reference — this is what the live book pays.
MIS_INTRADAY = CostModel(
    name="Equity MIS (intraday)",
    brokerage_per_order=20.0, brokerage_pct=0.0003, brokerage_cap=20.0,
    stt_buy=0.0, stt_sell=0.00025,
    exch_txn=0.0000297, stamp_buy=0.00003,
    can_hold_short=False,
    note="Shorts legal, but must be closed the same day.",
)

# A multi-day LONG. Cannot be used for a short at all.
CNC_DELIVERY = CostModel(
    name="Equity CNC (delivery)",
    brokerage_per_order=0.0, brokerage_pct=0.0, brokerage_cap=0.0,
    stt_buy=0.001, stt_sell=0.001,
    exch_txn=0.0000297, stamp_buy=0.00015,
    dp_per_sell=15.34,
    can_hold_short=False,
    note="LONG ONLY overnight. A cash-segment short cannot be carried: it is "
         "short delivery, settled by exchange auction with penalties.",
)

# The only way to carry a multi-day SHORT, and cheaper than delivery.
STOCK_FUTURES = CostModel(
    name="Stock futures (NRML)",
    brokerage_per_order=20.0, brokerage_pct=0.0003, brokerage_cap=20.0,
    stt_buy=0.0, stt_sell=0.0002,
    exch_txn=0.0000173, stamp_buy=0.00002,
    can_hold_short=True,
    note="Shorts can be carried. Costs: lot-size granularity, monthly expiry "
         "and rollover, and only ~180-220 F&O-eligible names.",
)

MODELS = [MIS_INTRADAY, CNC_DELIVERY, STOCK_FUTURES]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--notional", type=float, default=500_000.0)
    ap.add_argument("--stops", default="0.0075,0.02,0.03,0.05",
                    help="stop sizes to express the cost in R")
    a = ap.parse_args()
    stops = [float(x) for x in a.stops.split(",")]

    print(f"Round-trip cost on Rs.{a.notional:,.0f} notional\n")
    print(f"{'instrument':<24}{'cost Rs':>10}{'% of notional':>15}{'short ok':>10}")
    print("-" * 59)
    for m in MODELS:
        rt = m.round_trip(a.notional)
        print(f"{m.name:<24}{rt['total']:>10,.0f}{rt['pct']:>14.3f}%"
              f"{('yes' if m.can_hold_short else 'NO'):>10}")

    print(f"\nThe same cost as BREAKEVEN R, by stop size")
    print(f"{'instrument':<24}" + "".join(f"{s*100:>9.2f}%" for s in stops))
    print("-" * (24 + 9 * len(stops)))
    for m in MODELS:
        cells = "".join(f"{m.breakeven_r(a.notional, s):>10.3f}" for s in stops)
        print(f"{m.name:<24}{cells}")

    print("\nRead the columns: a wider stop dilutes a fixed cost. The intraday")
    print("book's 0.75% stop is viable at MIS rates and ruinous on delivery.")
    print()
    for m in MODELS:
        if m.note:
            print(f"  {m.name}: {m.note}")


if __name__ == "__main__":
    main()
