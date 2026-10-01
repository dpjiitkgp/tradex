# revtest — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 3.0x risk (2.250%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 25 instruments, 2018-11-09..2026-09-30.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_reversion | 40190 | -0.043 | -0.058 | 38.3% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| ASIANPAINT | vwap_reversion | 1926 | -0.007 | -0.009 | 45.5% |
| DABUR | vwap_reversion | 1911 | -0.028 | -0.037 | 43.8% |
| ITC | vwap_reversion | 1842 | -0.057 | -0.076 | 42.9% |
| LICI | vwap_reversion | 985 | -0.048 | -0.064 | 41.6% |
| TVSMOTOR | vwap_reversion | 1993 | 0.001 | 0.002 | 40.4% |
| ICICIPRULI | vwap_reversion | 1998 | -0.006 | -0.008 | 40.3% |
| VOLTAS | vwap_reversion | 1987 | -0.041 | -0.055 | 39.7% |
| HAL | vwap_reversion | 1971 | -0.023 | -0.031 | 38.9% |
| PNB | vwap_reversion | 1679 | -0.028 | -0.037 | 38.5% |
| TMPV | vwap_reversion | 1991 | -0.030 | -0.040 | 38.1% |
| ATGL | vwap_reversion | 1971 | -0.004 | -0.006 | 38.1% |
| LLOYDSME | vwap_reversion | 872 | 0.021 | 0.028 | 38.0% |
| LENSKART | vwap_reversion | 237 | 0.002 | 0.002 | 37.1% |
| DIXON | vwap_reversion | 2029 | -0.044 | -0.058 | 37.1% |
| GMRAIRPORT | vwap_reversion | 1430 | -0.069 | -0.093 | 37.0% |
| TIINDIA | vwap_reversion | 2019 | -0.056 | -0.075 | 36.7% |
| PRESTIGE | vwap_reversion | 2082 | -0.031 | -0.041 | 36.6% |
| NAUKRI | vwap_reversion | 1933 | -0.096 | -0.129 | 36.5% |
| LODHA | vwap_reversion | 1448 | -0.052 | -0.069 | 36.0% |
| APARINDS | vwap_reversion | 2028 | -0.053 | -0.071 | 35.8% |
| YESBANK | vwap_reversion | 1105 | -0.060 | -0.080 | 35.7% |
| CANBK | vwap_reversion | 1896 | -0.060 | -0.081 | 35.7% |
| WAAREEENER | vwap_reversion | 505 | -0.063 | -0.083 | 35.0% |
| NMDC | vwap_reversion | 1472 | -0.144 | -0.192 | 33.2% |
| IDEA | vwap_reversion | 880 | -0.149 | -0.198 | 29.5% |
