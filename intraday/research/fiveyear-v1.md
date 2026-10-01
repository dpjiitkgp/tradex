# fiveyear-v1 — exit-rule research

Exit rule: risk_pct=0.500% (hard stop), armed at 3.0x risk (1.500%), give-back trail 0.500% behind peak (no hard floor lock — see `TrailPolicy::GiveBackFromPeak`).

Universe: 165 instruments, 2021-03-23..2026-09-23.

## Per-strategy rollup

| Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|
| ma_alignment | 512177 | 0.017 | 0.033 | 38.8% |
| ma_alignment | 572034 | 0.011 | 0.021 | 37.8% |
| ma_alignment | 498287 | 0.020 | 0.039 | 39.1% |
| momentum_breakout | 389922 | -0.015 | -0.030 | 38.4% |
| orb | 83221 | -0.008 | -0.016 | 32.0% |
| strong_rally | 87780 | -0.009 | -0.017 | 33.1% |
| strong_rally | 258157 | 0.004 | 0.009 | 35.7% |
| trend_pullback | 487609 | -0.009 | -0.018 | 38.9% |
| vwap_reversion | 235277 | -0.022 | -0.044 | 34.4% |
| vwap_trend | 164870 | 0.002 | 0.003 | 35.9% |
| vwap_trend | 67691 | 0.022 | 0.045 | 39.6% |

## Top stock x strategy combos by win rate (min 5 trades)

| Symbol | Strategy | Trades | Avg PnL% | Avg R | Win rate |
|---|---|---|---|---|---|
| LICI | vwap_trend | 313 | 0.127 | 0.254 | 55.3% |
| AWL | vwap_trend | 347 | 0.123 | 0.247 | 50.7% |
| ITC | vwap_trend | 350 | 0.100 | 0.199 | 50.6% |
| 3MINDIA | vwap_trend | 535 | 0.074 | 0.148 | 47.3% |
| HDFCLIFE | vwap_trend | 343 | 0.094 | 0.187 | 47.2% |
| EXIDEIND | vwap_trend | 396 | 0.105 | 0.210 | 47.0% |
| COALINDIA | vwap_trend | 372 | 0.105 | 0.210 | 46.5% |
| HONAUT | vwap_trend | 480 | 0.038 | 0.077 | 46.2% |
| BAJAJ-AUTO | vwap_trend | 390 | 0.068 | 0.136 | 46.2% |
| LT | trend_pullback | 2407 | 0.017 | 0.033 | 45.7% |
| AXISBANK | momentum_breakout | 1701 | 0.023 | 0.045 | 45.4% |
| ASIANPAINT | momentum_breakout | 1758 | -0.011 | -0.021 | 45.3% |
| DIVISLAB | vwap_trend | 375 | 0.029 | 0.058 | 45.3% |
| LT | momentum_breakout | 1664 | 0.025 | 0.051 | 45.3% |
| MARUTI | momentum_breakout | 1788 | 0.019 | 0.038 | 45.2% |
| MARUTI | vwap_trend | 398 | 0.056 | 0.112 | 45.2% |
| LICI | ma_alignment | 1945 | 0.081 | 0.161 | 45.2% |
| INFY | trend_pullback | 2259 | 0.002 | 0.004 | 45.2% |
| HAVELLS | vwap_trend | 421 | 0.050 | 0.100 | 45.1% |
| LUPIN | vwap_trend | 366 | 0.097 | 0.195 | 45.1% |
| INFY | vwap_trend | 315 | 0.019 | 0.038 | 45.1% |
| ATGL | vwap_trend | 447 | 0.081 | 0.162 | 45.0% |
| ITC | vwap_trend | 857 | 0.026 | 0.053 | 44.8% |
| MARUTI | trend_pullback | 2369 | 0.020 | 0.040 | 44.7% |
| HINDUNILVR | ma_alignment | 2290 | 0.021 | 0.042 | 44.7% |
| BRITANNIA | momentum_breakout | 1908 | 0.003 | 0.006 | 44.7% |
| BRITANNIA | ma_alignment | 2392 | 0.013 | 0.026 | 44.6% |
| MAZDOCK | vwap_trend | 448 | 0.114 | 0.229 | 44.6% |
| LICI | ma_alignment | 1949 | 0.093 | 0.186 | 44.6% |
| M&M | trend_pullback | 2548 | 0.025 | 0.050 | 44.6% |
| HCLTECH | momentum_breakout | 1735 | -0.006 | -0.011 | 44.6% |
| INFY | momentum_breakout | 1585 | 0.000 | 0.000 | 44.5% |
| COCHINSHIP | vwap_trend | 494 | 0.067 | 0.135 | 44.5% |
| ITC | ma_alignment | 2301 | 0.023 | 0.047 | 44.5% |
| HDFCBANK | orb | 419 | 0.057 | 0.113 | 44.4% |
| HDFCAMC | vwap_trend | 424 | 0.098 | 0.197 | 44.3% |
| GLAXO | vwap_trend | 530 | 0.088 | 0.175 | 44.3% |
| EICHERMOT | momentum_breakout | 1978 | -0.002 | -0.004 | 44.2% |
| HCLTECH | trend_pullback | 2445 | -0.003 | -0.007 | 44.2% |
| ITC | ma_alignment | 2267 | 0.016 | 0.033 | 44.2% |
| DMART | momentum_breakout | 2114 | 0.045 | 0.090 | 44.2% |
| APOLLOHOSP | ma_alignment | 2535 | 0.043 | 0.086 | 44.2% |
| DMART | ma_alignment | 2855 | 0.075 | 0.150 | 44.2% |
| HUDCO | vwap_trend | 470 | 0.047 | 0.093 | 44.0% |
| MARICO | vwap_trend | 384 | 0.012 | 0.024 | 44.0% |
| BRITANNIA | trend_pullback | 2466 | 0.002 | 0.005 | 44.0% |
| ACC | vwap_trend | 405 | 0.028 | 0.057 | 44.0% |
| LINDEINDIA | vwap_trend | 471 | 0.068 | 0.136 | 43.9% |
| BRITANNIA | vwap_trend | 428 | 0.006 | 0.013 | 43.9% |
| ASIANPAINT | trend_pullback | 2361 | -0.000 | -0.000 | 43.9% |
| HINDUNILVR | ma_alignment | 2270 | 0.018 | 0.037 | 43.9% |
| AXISBANK | ma_alignment | 2489 | 0.042 | 0.083 | 43.9% |
| DMART | ma_alignment | 2626 | 0.067 | 0.135 | 43.9% |
| DRREDDY | momentum_breakout | 1870 | -0.008 | -0.015 | 43.9% |
| HEROMOTOCO | momentum_breakout | 1891 | 0.017 | 0.033 | 43.9% |
| HONAUT | ma_alignment | 2489 | 0.054 | 0.108 | 43.9% |
| HINDUNILVR | strong_rally | 748 | 0.059 | 0.118 | 43.9% |
| LTM | vwap_trend | 365 | 0.093 | 0.186 | 43.8% |
| ASIANPAINT | vwap_trend | 365 | 0.047 | 0.093 | 43.8% |
| ITC | ma_alignment | 2449 | 0.031 | 0.063 | 43.8% |
| BRITANNIA | ma_alignment | 2479 | 0.006 | 0.011 | 43.8% |
| APOLLOHOSP | ma_alignment | 2562 | 0.044 | 0.087 | 43.7% |
| HDFCLIFE | trend_pullback | 2522 | 0.020 | 0.039 | 43.7% |
| DABUR | momentum_breakout | 1933 | -0.007 | -0.014 | 43.7% |
| DABUR | trend_pullback | 2443 | 0.007 | 0.013 | 43.6% |
| EICHERMOT | trend_pullback | 2590 | 0.003 | 0.007 | 43.6% |
| HEROMOTOCO | trend_pullback | 2487 | 0.016 | 0.033 | 43.6% |
| HINDUNILVR | momentum_breakout | 1730 | -0.011 | -0.022 | 43.6% |
| BHARTIARTL | trend_pullback | 2352 | -0.001 | -0.001 | 43.6% |
| NLCINDIA | vwap_trend | 473 | 0.124 | 0.247 | 43.6% |
| LTM | trend_pullback | 2752 | 0.021 | 0.042 | 43.5% |
| COLPAL | trend_pullback | 2561 | 0.003 | 0.006 | 43.5% |
| BAJAJ-AUTO | ma_alignment | 2440 | 0.022 | 0.045 | 43.5% |
| DABUR | vwap_trend | 368 | 0.034 | 0.069 | 43.5% |
| IRCTC | momentum_breakout | 2024 | 0.029 | 0.058 | 43.5% |
| DIVISLAB | trend_pullback | 2588 | 0.011 | 0.022 | 43.5% |
| DMART | ma_alignment | 2569 | 0.055 | 0.110 | 43.4% |
| HINDUNILVR | ma_alignment | 2461 | 0.018 | 0.036 | 43.4% |
| HAL | vwap_trend | 396 | 0.121 | 0.243 | 43.4% |
| HCLTECH | ma_alignment | 2501 | 0.014 | 0.027 | 43.4% |
| LUPIN | trend_pullback | 2651 | 0.002 | 0.004 | 43.4% |
| HDFCBANK | momentum_breakout | 1534 | -0.011 | -0.022 | 43.4% |
| ASIANPAINT | ma_alignment | 2360 | 0.016 | 0.033 | 43.4% |
| MARUTI | ma_alignment | 2651 | 0.024 | 0.048 | 43.4% |
| LICI | ma_alignment | 2131 | 0.071 | 0.142 | 43.4% |
| HEROMOTOCO | vwap_trend | 390 | 0.044 | 0.089 | 43.3% |
| AXISBANK | ma_alignment | 2560 | 0.030 | 0.060 | 43.3% |
| KPITTECH | vwap_trend | 411 | 0.121 | 0.241 | 43.3% |
| BAJAJ-AUTO | ma_alignment | 2487 | 0.030 | 0.060 | 43.3% |
| HDFCBANK | ma_alignment | 2208 | 0.009 | 0.018 | 43.3% |
| BHARTIARTL | momentum_breakout | 1740 | -0.004 | -0.007 | 43.3% |
| BRITANNIA | ma_alignment | 2640 | 0.010 | 0.020 | 43.3% |
| HINDUNILVR | trend_pullback | 2305 | -0.008 | -0.016 | 43.3% |
| NESTLEIND | vwap_trend | 407 | 0.016 | 0.033 | 43.2% |
| CIPLA | momentum_breakout | 1864 | -0.014 | -0.028 | 43.2% |
| NESTLEIND | ma_alignment | 2396 | -0.002 | -0.004 | 43.2% |
| LT | ma_alignment | 2450 | 0.029 | 0.059 | 43.2% |
| ASIANPAINT | ma_alignment | 2430 | 0.006 | 0.012 | 43.2% |
| HDFCBANK | ma_alignment | 2377 | 0.021 | 0.043 | 43.2% |
| HDFCBANK | trend_pullback | 2235 | 0.011 | 0.022 | 43.2% |
