# ma-conf-stop100 — exit-rule research

Exit rule: risk_pct=1.000% (hard stop), armed at 3.0x risk (3.000%), give-back trail 1.000% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 165 instruments, 2021-03-23..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| ma_alignment | 332371 | 0.031 | 0.031 | 45.9% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LICI | ma_alignment | 1437 | 0.093 | 0.093 | 51.1% |
| ASIANPAINT | ma_alignment | 1736 | 0.027 | 0.027 | 50.2% |
| ONGC | ma_alignment | 1860 | 0.070 | 0.070 | 49.8% |
| APOLLOHOSP | ma_alignment | 1798 | 0.046 | 0.046 | 49.8% |
| HINDUNILVR | ma_alignment | 1692 | 0.029 | 0.029 | 49.3% |
| ITC | ma_alignment | 1714 | 0.027 | 0.027 | 49.2% |
| NTPC | ma_alignment | 1851 | 0.029 | 0.029 | 49.1% |
| INFY | ma_alignment | 1735 | -0.008 | -0.008 | 49.1% |
| BAJAJHLDNG | ma_alignment | 1894 | 0.096 | 0.096 | 48.9% |
| HCLTECH | ma_alignment | 1829 | -0.011 | -0.011 | 48.9% |
| BHARTIARTL | ma_alignment | 1777 | 0.020 | 0.020 | 48.8% |
| GODREJCP | ma_alignment | 1868 | 0.007 | 0.007 | 48.8% |
| MANKIND | ma_alignment | 1218 | 0.065 | 0.065 | 48.8% |
| INDHOTEL | ma_alignment | 1996 | 0.078 | 0.078 | 48.7% |
| COLPAL | ma_alignment | 1821 | -0.008 | -0.008 | 48.7% |
| NESTLEIND | ma_alignment | 1754 | 0.007 | 0.007 | 48.6% |
| DABUR | ma_alignment | 1744 | 0.028 | 0.028 | 48.6% |
| DLF | ma_alignment | 2012 | 0.080 | 0.080 | 48.4% |
| HONAUT | ma_alignment | 1819 | 0.051 | 0.051 | 48.4% |
| ABBOTINDIA | ma_alignment | 1941 | 0.024 | 0.024 | 48.3% |
| ACC | ma_alignment | 1933 | 0.023 | 0.023 | 48.3% |
| MPHASIS | ma_alignment | 2037 | 0.058 | 0.058 | 48.3% |
| AWL | ma_alignment | 1795 | 0.033 | 0.033 | 48.2% |
| 3MINDIA | ma_alignment | 2015 | 0.050 | 0.050 | 48.2% |
| IRCTC | ma_alignment | 1923 | 0.068 | 0.068 | 48.2% |
| BAJAJFINSV | ma_alignment | 1836 | 0.053 | 0.053 | 48.1% |
| GLENMARK | ma_alignment | 2015 | 0.045 | 0.045 | 48.1% |
| BAJAJ-AUTO | ma_alignment | 1797 | -0.001 | -0.001 | 48.1% |
| MARICO | ma_alignment | 1804 | -0.012 | -0.012 | 48.1% |
| AXISBANK | ma_alignment | 1816 | 0.008 | 0.008 | 48.0% |
| DMART | ma_alignment | 1852 | 0.066 | 0.066 | 47.9% |
| LUPIN | ma_alignment | 1846 | 0.016 | 0.016 | 47.9% |
| KPRMILL | ma_alignment | 2217 | 0.111 | 0.111 | 47.9% |
| EICHERMOT | ma_alignment | 1838 | 0.009 | 0.009 | 47.8% |
| BRITANNIA | ma_alignment | 1747 | 0.005 | 0.005 | 47.7% |
| PAGEIND | ma_alignment | 1862 | 0.006 | 0.006 | 47.7% |
| LT | ma_alignment | 1780 | -0.001 | -0.001 | 47.6% |
| HAL | ma_alignment | 2020 | 0.032 | 0.032 | 47.6% |
| CIPLA | ma_alignment | 1800 | -0.013 | -0.013 | 47.6% |
| LTM | ma_alignment | 1892 | 0.045 | 0.045 | 47.6% |
| LICHSGFIN | ma_alignment | 1991 | 0.042 | 0.042 | 47.6% |
| CRISIL | ma_alignment | 2115 | 0.063 | 0.063 | 47.5% |
| HDFCLIFE | ma_alignment | 1824 | 0.016 | 0.016 | 47.5% |
| BERGEPAINT | ma_alignment | 1937 | 0.016 | 0.016 | 47.4% |
| CHOLAFIN | ma_alignment | 2057 | 0.031 | 0.031 | 47.4% |
| HEROMOTOCO | ma_alignment | 1824 | 0.029 | 0.029 | 47.3% |
| BOSCHLTD | ma_alignment | 1896 | 0.022 | 0.022 | 47.3% |
| ADANIENT | ma_alignment | 2171 | 0.129 | 0.129 | 47.3% |
| MUTHOOTFIN | ma_alignment | 1952 | -0.013 | -0.013 | 47.2% |
| ABB | ma_alignment | 2061 | 0.040 | 0.040 | 47.2% |
| GODREJPROP | ma_alignment | 2155 | 0.077 | 0.077 | 47.1% |
| JKCEMENT | ma_alignment | 2039 | 0.040 | 0.040 | 47.1% |
| JUBLFOOD | ma_alignment | 2031 | 0.027 | 0.027 | 47.0% |
| DIXON | ma_alignment | 2109 | 0.078 | 0.078 | 47.0% |
| COALINDIA | ma_alignment | 1892 | 0.027 | 0.027 | 47.0% |
| ETERNAL | ma_alignment | 2049 | 0.158 | 0.158 | 46.9% |
| AUROPHARMA | ma_alignment | 1975 | 0.024 | 0.024 | 46.9% |
| ICICIBANK | ma_alignment | 1745 | -0.019 | -0.019 | 46.9% |
| IOC | ma_alignment | 1889 | 0.058 | 0.058 | 46.9% |
| KEI | ma_alignment | 2124 | 0.102 | 0.102 | 46.9% |
| EXIDEIND | ma_alignment | 1986 | 0.029 | 0.029 | 46.9% |
| NLCINDIA | ma_alignment | 2328 | 0.117 | 0.117 | 46.8% |
| ADANIPORTS | ma_alignment | 2048 | 0.061 | 0.061 | 46.8% |
| APLAPOLLO | ma_alignment | 2126 | 0.040 | 0.040 | 46.8% |
| ICICIPRULI | ma_alignment | 1938 | 0.012 | 0.012 | 46.7% |
| INDIGO | ma_alignment | 1920 | 0.039 | 0.039 | 46.7% |
| IDFCFIRSTB | ma_alignment | 1940 | 0.050 | 0.050 | 46.7% |
| COFORGE | ma_alignment | 1986 | 0.074 | 0.074 | 46.7% |
| INDUSINDBK | ma_alignment | 1968 | 0.040 | 0.040 | 46.6% |
| HINDALCO | ma_alignment | 2000 | -0.005 | -0.005 | 46.6% |
| ABCAPITAL | ma_alignment | 2085 | 0.043 | 0.043 | 46.5% |
| FORTIS | ma_alignment | 2062 | 0.051 | 0.051 | 46.5% |
| MARUTI | ma_alignment | 1791 | -0.016 | -0.016 | 46.5% |
| M&M | ma_alignment | 1934 | -0.001 | -0.001 | 46.4% |
| AUBANK | ma_alignment | 2012 | 0.029 | 0.029 | 46.4% |
| NAUKRI | ma_alignment | 1995 | 0.043 | 0.043 | 46.4% |
| NYKAA | ma_alignment | 1874 | 0.084 | 0.084 | 46.3% |
| APOLLOTYRE | ma_alignment | 2036 | 0.017 | 0.017 | 46.3% |
| GRASIM | ma_alignment | 1880 | -0.026 | -0.026 | 46.3% |
| ASTRAL | ma_alignment | 2012 | 0.017 | 0.017 | 46.3% |
| BIOCON | ma_alignment | 1954 | 0.048 | 0.048 | 46.3% |
| HAVELLS | ma_alignment | 1898 | -0.007 | -0.007 | 46.3% |
| BEL | ma_alignment | 1957 | 0.066 | 0.066 | 46.2% |
| JINDALSTEL | ma_alignment | 2076 | 0.053 | 0.053 | 46.2% |
| HDFCBANK | ma_alignment | 1722 | -0.022 | -0.022 | 46.2% |
| HDFCAMC | ma_alignment | 1923 | 0.016 | 0.016 | 46.2% |
| OIL | ma_alignment | 2153 | 0.065 | 0.065 | 46.2% |
| BPCL | ma_alignment | 1917 | 0.016 | 0.016 | 46.2% |
| ESCORTS | ma_alignment | 2000 | 0.009 | 0.009 | 46.1% |
| DIVISLAB | ma_alignment | 1842 | 0.010 | 0.010 | 46.0% |
| HINDPETRO | ma_alignment | 2137 | 0.043 | 0.043 | 46.0% |
| HINDZINC | ma_alignment | 2007 | 0.039 | 0.039 | 46.0% |
| IPCALAB | ma_alignment | 2116 | -0.021 | -0.021 | 45.9% |
| AMBUJACEM | ma_alignment | 2006 | 0.048 | 0.048 | 45.9% |
| ASHOKLEY | ma_alignment | 1968 | 0.065 | 0.065 | 45.8% |
| DRREDDY | ma_alignment | 1774 | -0.032 | -0.032 | 45.8% |
| MFSL | ma_alignment | 2050 | 0.009 | 0.009 | 45.8% |
| BALKRISIND | ma_alignment | 2034 | -0.005 | -0.005 | 45.8% |
| KOTAKBANK | ma_alignment | 1788 | -0.037 | -0.037 | 45.7% |
| ENDURANCE | ma_alignment | 2147 | 0.022 | 0.022 | 45.7% |
