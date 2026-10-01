# vwap-trend-improved/vol2x — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 3.0x risk (2.250%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 165 instruments, 2021-03-23..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_allday_vol2x | 99941 | 0.029 | 0.039 | 46.5% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LICI | vwap_trend_short_allday_vol2x | 511 | 0.125 | 0.167 | 57.9% |
| AWL | vwap_trend_short_allday_vol2x | 611 | 0.107 | 0.142 | 57.1% |
| LINDEINDIA | vwap_trend_short_allday_vol2x | 862 | 0.160 | 0.213 | 54.2% |
| CONCOR | vwap_trend_short_allday_vol2x | 528 | 0.107 | 0.142 | 52.1% |
| MANKIND | vwap_trend_short_allday_vol2x | 407 | 0.111 | 0.149 | 51.8% |
| ITC | vwap_trend_short_allday_vol2x | 441 | 0.055 | 0.073 | 51.7% |
| DIXON | vwap_trend_short_allday_vol2x | 566 | 0.100 | 0.133 | 51.4% |
| BERGEPAINT | vwap_trend_short_allday_vol2x | 631 | 0.065 | 0.086 | 51.3% |
| IRCTC | vwap_trend_short_allday_vol2x | 569 | 0.072 | 0.096 | 51.3% |
| KALYANKJIL | vwap_trend_short_allday_vol2x | 725 | 0.105 | 0.141 | 51.3% |
| HONAUT | vwap_trend_short_allday_vol2x | 843 | 0.074 | 0.099 | 51.2% |
| BDL | vwap_trend_short_allday_vol2x | 740 | 0.131 | 0.175 | 51.2% |
| HCLTECH | vwap_trend_short_allday_vol2x | 427 | 0.055 | 0.074 | 51.1% |
| HUDCO | vwap_trend_short_allday_vol2x | 814 | 0.083 | 0.111 | 50.9% |
| INDUSINDBK | vwap_trend_short_allday_vol2x | 455 | 0.054 | 0.071 | 50.8% |
| MAZDOCK | vwap_trend_short_allday_vol2x | 734 | 0.116 | 0.155 | 50.7% |
| HEROMOTOCO | vwap_trend_short_allday_vol2x | 519 | 0.062 | 0.083 | 50.5% |
| NLCINDIA | vwap_trend_short_allday_vol2x | 747 | 0.120 | 0.160 | 50.3% |
| COALINDIA | vwap_trend_short_allday_vol2x | 519 | 0.083 | 0.111 | 50.3% |
| BPCL | vwap_trend_short_allday_vol2x | 519 | 0.073 | 0.097 | 50.3% |
| COCHINSHIP | vwap_trend_short_allday_vol2x | 714 | 0.083 | 0.111 | 50.3% |
| ETERNAL | vwap_trend_short_allday_vol2x | 508 | 0.170 | 0.227 | 50.2% |
| GLAXO | vwap_trend_short_allday_vol2x | 841 | 0.072 | 0.096 | 50.2% |
| 3MINDIA | vwap_trend_short_allday_vol2x | 918 | 0.039 | 0.053 | 50.1% |
| EXIDEIND | vwap_trend_short_allday_vol2x | 621 | 0.064 | 0.085 | 50.1% |
| DMART | vwap_trend_short_allday_vol2x | 654 | 0.061 | 0.081 | 50.0% |
| DIVISLAB | vwap_trend_short_allday_vol2x | 561 | 0.029 | 0.039 | 49.9% |
| HDFCLIFE | vwap_trend_short_allday_vol2x | 459 | 0.060 | 0.080 | 49.9% |
| GAIL | vwap_trend_short_allday_vol2x | 556 | 0.096 | 0.128 | 49.8% |
| ONGC | vwap_trend_short_allday_vol2x | 522 | 0.025 | 0.033 | 49.8% |
| ADANIPORTS | vwap_trend_short_allday_vol2x | 504 | 0.073 | 0.097 | 49.8% |
| GICRE | vwap_trend_short_allday_vol2x | 868 | 0.091 | 0.122 | 49.8% |
| HAL | vwap_trend_short_allday_vol2x | 557 | 0.073 | 0.097 | 49.7% |
| DRREDDY | vwap_trend_short_allday_vol2x | 460 | 0.018 | 0.024 | 49.6% |
| MARUTI | vwap_trend_short_allday_vol2x | 487 | 0.043 | 0.057 | 49.3% |
| AXISBANK | vwap_trend_short_allday_vol2x | 414 | -0.005 | -0.007 | 49.3% |
| ABB | vwap_trend_short_allday_vol2x | 581 | 0.092 | 0.122 | 49.2% |
| EICHERMOT | vwap_trend_short_allday_vol2x | 509 | 0.030 | 0.040 | 49.1% |
| APOLLOHOSP | vwap_trend_short_allday_vol2x | 499 | 0.038 | 0.051 | 49.1% |
| ADANIENT | vwap_trend_short_allday_vol2x | 544 | 0.135 | 0.179 | 49.1% |
| CIPLA | vwap_trend_short_allday_vol2x | 483 | -0.011 | -0.015 | 49.1% |
| IOC | vwap_trend_short_allday_vol2x | 603 | 0.064 | 0.086 | 48.9% |
| BRITANNIA | vwap_trend_short_allday_vol2x | 526 | 0.003 | 0.005 | 48.9% |
| CHOLAFIN | vwap_trend_short_allday_vol2x | 520 | 0.047 | 0.063 | 48.8% |
| BAJAJ-AUTO | vwap_trend_short_allday_vol2x | 515 | 0.049 | 0.065 | 48.7% |
| APOLLOTYRE | vwap_trend_short_allday_vol2x | 621 | 0.074 | 0.099 | 48.6% |
| BIOCON | vwap_trend_short_allday_vol2x | 629 | 0.065 | 0.087 | 48.5% |
| BHARTIARTL | vwap_trend_short_allday_vol2x | 415 | 0.007 | 0.009 | 48.4% |
| LUPIN | vwap_trend_short_allday_vol2x | 536 | 0.043 | 0.057 | 48.3% |
| GRASIM | vwap_trend_short_allday_vol2x | 539 | -0.018 | -0.024 | 48.2% |
| ATGL | vwap_trend_short_allday_vol2x | 703 | 0.033 | 0.044 | 48.2% |
| KPITTECH | vwap_trend_short_allday_vol2x | 640 | 0.116 | 0.154 | 48.1% |
| LTM | vwap_trend_short_allday_vol2x | 501 | 0.020 | 0.026 | 48.1% |
| GLENMARK | vwap_trend_short_allday_vol2x | 574 | 0.017 | 0.023 | 48.1% |
| PAGEIND | vwap_trend_short_allday_vol2x | 600 | 0.002 | 0.003 | 48.0% |
| LT | vwap_trend_short_allday_vol2x | 424 | 0.034 | 0.045 | 47.9% |
| ACC | vwap_trend_short_allday_vol2x | 610 | 0.016 | 0.021 | 47.9% |
| JSWSTEEL | vwap_trend_short_allday_vol2x | 514 | 0.062 | 0.082 | 47.9% |
| NIACL | vwap_trend_short_allday_vol2x | 871 | 0.037 | 0.049 | 47.8% |
| M&M | vwap_trend_short_allday_vol2x | 446 | 0.030 | 0.039 | 47.8% |
| MOTILALOFS | vwap_trend_short_allday_vol2x | 731 | 0.087 | 0.116 | 47.7% |
| HAVELLS | vwap_trend_short_allday_vol2x | 530 | 0.032 | 0.043 | 47.7% |
| BOSCHLTD | vwap_trend_short_allday_vol2x | 656 | 0.012 | 0.016 | 47.6% |
| KEI | vwap_trend_short_allday_vol2x | 694 | 0.086 | 0.115 | 47.6% |
| HDFCAMC | vwap_trend_short_allday_vol2x | 586 | 0.065 | 0.087 | 47.4% |
| OIL | vwap_trend_short_allday_vol2x | 662 | 0.079 | 0.105 | 47.4% |
| COLPAL | vwap_trend_short_allday_vol2x | 579 | 0.004 | 0.006 | 47.3% |
| INDHOTEL | vwap_trend_short_allday_vol2x | 541 | 0.033 | 0.044 | 47.3% |
| INDIANB | vwap_trend_short_allday_vol2x | 656 | 0.038 | 0.050 | 47.3% |
| AUROPHARMA | vwap_trend_short_allday_vol2x | 534 | 0.025 | 0.033 | 47.2% |
| HINDALCO | vwap_trend_short_allday_vol2x | 458 | 0.051 | 0.068 | 47.2% |
| IDFCFIRSTB | vwap_trend_short_allday_vol2x | 615 | 0.028 | 0.037 | 47.2% |
| LTTS | vwap_trend_short_allday_vol2x | 628 | 0.048 | 0.064 | 47.1% |
| ABBOTINDIA | vwap_trend_short_allday_vol2x | 798 | -0.012 | -0.016 | 47.1% |
| ABCAPITAL | vwap_trend_short_allday_vol2x | 605 | 0.025 | 0.034 | 47.1% |
| DABUR | vwap_trend_short_allday_vol2x | 531 | 0.026 | 0.034 | 47.1% |
| MARICO | vwap_trend_short_allday_vol2x | 541 | -0.017 | -0.023 | 47.0% |
| GODFRYPHLP | vwap_trend_short_allday_vol2x | 810 | 0.074 | 0.098 | 46.9% |
| BAJAJFINSV | vwap_trend_short_allday_vol2x | 518 | 0.006 | 0.008 | 46.9% |
| KOTAKBANK | vwap_trend_short_allday_vol2x | 448 | -0.030 | -0.040 | 46.9% |
| BLUESTARCO | vwap_trend_short_allday_vol2x | 726 | 0.055 | 0.074 | 46.8% |
| OFSS | vwap_trend_short_allday_vol2x | 670 | 0.047 | 0.062 | 46.7% |
| ENDURANCE | vwap_trend_short_allday_vol2x | 793 | 0.036 | 0.049 | 46.7% |
| NTPC | vwap_trend_short_allday_vol2x | 515 | 0.025 | 0.033 | 46.6% |
| APARINDS | vwap_trend_short_allday_vol2x | 786 | 0.092 | 0.123 | 46.6% |
| MOTHERSON | vwap_trend_short_allday_vol2x | 553 | 0.003 | 0.003 | 46.5% |
| IPCALAB | vwap_trend_short_allday_vol2x | 684 | -0.015 | -0.019 | 46.3% |
| GODREJIND | vwap_trend_short_allday_vol2x | 885 | -0.025 | -0.034 | 46.3% |
| NATIONALUM | vwap_trend_short_allday_vol2x | 557 | 0.105 | 0.141 | 46.3% |
| ASIANPAINT | vwap_trend_short_allday_vol2x | 467 | -0.005 | -0.007 | 46.3% |
| INDIGO | vwap_trend_short_allday_vol2x | 517 | 0.024 | 0.032 | 46.2% |
| BAJFINANCE | vwap_trend_short_allday_vol2x | 459 | 0.012 | 0.016 | 46.2% |
| ICICIBANK | vwap_trend_short_allday_vol2x | 459 | -0.058 | -0.077 | 46.2% |
| ADANIPOWER | vwap_trend_short_allday_vol2x | 652 | 0.052 | 0.069 | 46.2% |
| DLF | vwap_trend_short_allday_vol2x | 516 | 0.070 | 0.093 | 46.1% |
| AMBUJACEM | vwap_trend_short_allday_vol2x | 553 | 0.040 | 0.053 | 46.1% |
| ESCORTS | vwap_trend_short_allday_vol2x | 655 | 0.023 | 0.030 | 46.1% |
| MEDANTA | vwap_trend_short_allday_vol2x | 489 | 0.042 | 0.056 | 46.0% |
| CRISIL | vwap_trend_short_allday_vol2x | 835 | 0.014 | 0.019 | 46.0% |
| BALKRISIND | vwap_trend_short_allday_vol2x | 596 | 0.004 | 0.005 | 46.0% |
