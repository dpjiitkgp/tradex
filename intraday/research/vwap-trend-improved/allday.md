# high-winrate — exit-rule research

Exit rule: risk_pct=0.750% (hard stop), armed at 3.0x risk (2.250%), give-back trail 0.750% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 250 instruments, 2021-03-23..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| vwap_trend_short_allday | 145404 | 0.031 | 0.042 | 46.2% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| UBL | vwap_trend_short_allday | 84 | 0.086 | 0.115 | 64.3% |
| UNITDSPR | vwap_trend_short_allday | 46 | 0.244 | 0.325 | 60.9% |
| AWL | vwap_trend_short_allday | 793 | 0.169 | 0.225 | 59.0% |
| LICI | vwap_trend_short_allday | 688 | 0.143 | 0.190 | 59.0% |
| JIOFIN | vwap_trend_short_allday | 450 | 0.199 | 0.265 | 57.8% |
| PRESTIGE | vwap_trend_short_allday | 49 | 0.410 | 0.546 | 57.1% |
| NTPCGREEN | vwap_trend_short_allday | 298 | 0.140 | 0.187 | 56.4% |
| BAJAJHFL | vwap_trend_short_allday | 340 | 0.114 | 0.153 | 55.6% |
| TMPV | vwap_trend_short_allday | 58 | 0.403 | 0.537 | 55.2% |
| ZYDUSLIFE | vwap_trend_short_allday | 64 | 0.195 | 0.260 | 54.7% |
| SJVN | vwap_trend_short_allday | 94 | -0.033 | -0.044 | 54.3% |
| LINDEINDIA | vwap_trend_short_allday | 1000 | 0.155 | 0.206 | 53.6% |
| TORNTPOWER | vwap_trend_short_allday | 73 | 0.169 | 0.225 | 53.4% |
| ITCHOTELS | vwap_trend_short_allday | 256 | 0.148 | 0.197 | 53.1% |
| IREDA | vwap_trend_short_allday | 429 | 0.152 | 0.203 | 52.9% |
| RVNL | vwap_trend_short_allday | 70 | 0.256 | 0.341 | 52.9% |
| RELIANCE | vwap_trend_short_allday | 57 | 0.149 | 0.198 | 52.6% |
| ANTHEM | vwap_trend_short_allday | 177 | 0.066 | 0.088 | 52.5% |
| LENSKART | vwap_trend_short_allday | 132 | 0.192 | 0.256 | 52.3% |
| TATAELXSI | vwap_trend_short_allday | 75 | 0.274 | 0.365 | 52.0% |
| CONCOR | vwap_trend_short_allday | 747 | 0.084 | 0.112 | 51.9% |
| MAZDOCK | vwap_trend_short_allday | 1003 | 0.134 | 0.179 | 51.7% |
| THERMAX | vwap_trend_short_allday | 70 | 0.055 | 0.073 | 51.4% |
| HONAUT | vwap_trend_short_allday | 918 | 0.073 | 0.097 | 51.3% |
| JSWINFRA | vwap_trend_short_allday | 474 | 0.087 | 0.116 | 51.3% |
| HEROMOTOCO | vwap_trend_short_allday | 756 | 0.075 | 0.099 | 50.9% |
| TECHM | vwap_trend_short_allday | 57 | 0.107 | 0.143 | 50.9% |
| COCHINSHIP | vwap_trend_short_allday | 979 | 0.085 | 0.113 | 50.9% |
| EXIDEIND | vwap_trend_short_allday | 826 | 0.084 | 0.112 | 50.8% |
| HDFCLIFE | vwap_trend_short_allday | 714 | 0.074 | 0.099 | 50.8% |
| DIXON | vwap_trend_short_allday | 812 | 0.117 | 0.156 | 50.7% |
| GROWW | vwap_trend_short_allday | 136 | 0.112 | 0.150 | 50.7% |
| MANKIND | vwap_trend_short_allday | 515 | 0.118 | 0.158 | 50.7% |
| HAL | vwap_trend_short_allday | 800 | 0.112 | 0.150 | 50.6% |
| BAJAJ-AUTO | vwap_trend_short_allday | 749 | 0.056 | 0.075 | 50.6% |
| ITC | vwap_trend_short_allday | 724 | 0.042 | 0.056 | 50.4% |
| BDL | vwap_trend_short_allday | 956 | 0.131 | 0.174 | 50.2% |
| IRCTC | vwap_trend_short_allday | 830 | 0.063 | 0.084 | 50.1% |
| HUDCO | vwap_trend_short_allday | 997 | 0.090 | 0.120 | 50.1% |
| GICRE | vwap_trend_short_allday | 1043 | 0.101 | 0.135 | 50.0% |
| SBICARD | vwap_trend_short_allday | 64 | 0.253 | 0.338 | 50.0% |
| UPL | vwap_trend_short_allday | 60 | 0.128 | 0.170 | 50.0% |
| SHRIRAMFIN | vwap_trend_short_allday | 66 | 0.073 | 0.097 | 50.0% |
| NLCINDIA | vwap_trend_short_allday | 940 | 0.133 | 0.178 | 49.9% |
| ONGC | vwap_trend_short_allday | 790 | 0.036 | 0.048 | 49.9% |
| BERGEPAINT | vwap_trend_short_allday | 794 | 0.046 | 0.062 | 49.7% |
| KALYANKJIL | vwap_trend_short_allday | 961 | 0.107 | 0.143 | 49.7% |
| ADANIPORTS | vwap_trend_short_allday | 786 | 0.110 | 0.147 | 49.6% |
| BHARTIARTL | vwap_trend_short_allday | 712 | 0.004 | 0.005 | 49.6% |
| GAIL | vwap_trend_short_allday | 791 | 0.089 | 0.119 | 49.4% |
| GLAXO | vwap_trend_short_allday | 963 | 0.072 | 0.096 | 49.4% |
| HDFCBANK | vwap_trend_short_allday | 678 | 0.007 | 0.009 | 49.4% |
| MARUTI | vwap_trend_short_allday | 741 | 0.048 | 0.064 | 49.4% |
| SWIGGY | vwap_trend_short_allday | 65 | 0.075 | 0.100 | 49.2% |
| LLOYDSME | vwap_trend_short_allday | 516 | 0.104 | 0.138 | 49.0% |
| HCLTECH | vwap_trend_short_allday | 699 | 0.035 | 0.046 | 48.9% |
| LUPIN | vwap_trend_short_allday | 742 | 0.049 | 0.065 | 48.9% |
| AXISBANK | vwap_trend_short_allday | 720 | 0.011 | 0.015 | 48.9% |
| ATGL | vwap_trend_short_allday | 954 | 0.045 | 0.060 | 48.8% |
| 3MINDIA | vwap_trend_short_allday | 1016 | 0.027 | 0.036 | 48.8% |
| DRREDDY | vwap_trend_short_allday | 706 | -0.005 | -0.006 | 48.7% |
| DIVISLAB | vwap_trend_short_allday | 754 | 0.019 | 0.025 | 48.7% |
| LTTS | vwap_trend_short_allday | 816 | 0.056 | 0.075 | 48.7% |
| DMART | vwap_trend_short_allday | 855 | 0.074 | 0.098 | 48.5% |
| SBIN | vwap_trend_short_allday | 62 | 0.182 | 0.243 | 48.4% |
| APOLLOHOSP | vwap_trend_short_allday | 729 | 0.047 | 0.062 | 48.3% |
| LTM | vwap_trend_short_allday | 744 | 0.047 | 0.062 | 48.3% |
| INDUSINDBK | vwap_trend_short_allday | 767 | 0.061 | 0.081 | 48.2% |
| INFY | vwap_trend_short_allday | 675 | 0.004 | 0.006 | 48.1% |
| HAVELLS | vwap_trend_short_allday | 742 | 0.035 | 0.047 | 48.1% |
| KPITTECH | vwap_trend_short_allday | 882 | 0.117 | 0.156 | 48.1% |
| PAGEIND | vwap_trend_short_allday | 762 | -0.005 | -0.007 | 48.0% |
| OIL | vwap_trend_short_allday | 879 | 0.120 | 0.160 | 48.0% |
| NIACL | vwap_trend_short_allday | 1055 | 0.044 | 0.059 | 48.0% |
| EICHERMOT | vwap_trend_short_allday | 757 | 0.021 | 0.027 | 48.0% |
| APOLLOTYRE | vwap_trend_short_allday | 799 | 0.059 | 0.078 | 47.9% |
| HDFCAMC | vwap_trend_short_allday | 800 | 0.062 | 0.083 | 47.9% |
| SHREECEM | vwap_trend_short_allday | 65 | 0.219 | 0.292 | 47.7% |
| ADANIPOWER | vwap_trend_short_allday | 957 | 0.073 | 0.097 | 47.6% |
| BRITANNIA | vwap_trend_short_allday | 741 | -0.006 | -0.008 | 47.6% |
| ASIANPAINT | vwap_trend_short_allday | 732 | 0.014 | 0.019 | 47.5% |
| M&MFIN | vwap_trend_short_allday | 794 | 0.077 | 0.103 | 47.5% |
| BANKINDIA | vwap_trend_short_allday | 911 | 0.082 | 0.109 | 47.4% |
| LGEINDIA | vwap_trend_short_allday | 135 | -0.037 | -0.049 | 47.4% |
| BIOCON | vwap_trend_short_allday | 840 | 0.057 | 0.076 | 47.4% |
| LT | vwap_trend_short_allday | 684 | 0.015 | 0.019 | 47.4% |
| BLUESTARCO | vwap_trend_short_allday | 871 | 0.068 | 0.091 | 47.3% |
| KEI | vwap_trend_short_allday | 895 | 0.097 | 0.130 | 47.3% |
| BOSCHLTD | vwap_trend_short_allday | 818 | 0.014 | 0.019 | 47.2% |
| ETERNAL | vwap_trend_short_allday | 776 | 0.097 | 0.130 | 47.2% |
| MOTILALOFS | vwap_trend_short_allday | 899 | 0.090 | 0.119 | 47.2% |
| DABUR | vwap_trend_short_allday | 717 | 0.033 | 0.044 | 47.1% |
| BAJFINANCE | vwap_trend_short_allday | 764 | 0.035 | 0.047 | 47.1% |
| IOC | vwap_trend_short_allday | 815 | 0.061 | 0.081 | 47.1% |
| COALINDIA | vwap_trend_short_allday | 780 | 0.036 | 0.048 | 47.1% |
| CHOLAFIN | vwap_trend_short_allday | 761 | 0.042 | 0.055 | 47.0% |
| ACC | vwap_trend_short_allday | 817 | 0.036 | 0.048 | 47.0% |
| ABBOTINDIA | vwap_trend_short_allday | 917 | -0.012 | -0.015 | 47.0% |
| KOTAKBANK | vwap_trend_short_allday | 719 | -0.026 | -0.035 | 46.9% |
| LICHSGFIN | vwap_trend_short_allday | 767 | 0.048 | 0.064 | 46.8% |
