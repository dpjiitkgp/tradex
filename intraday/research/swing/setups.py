#!/usr/bin/env python3
"""The standard long swing setups, on daily bars.

Ten textbook entries spanning the three mechanisms swing trading actually
uses: breakout, trend-continuation, and mean-reversion. They are implemented
as published rather than tuned, so the comparison is between *ideas* and not
between someone's parameter choices.

Every function returns the indices of signal days. The caller fills at the
NEXT day's open, so a signal at index i may use bars 0..i inclusive and
nothing after — the one invariant that makes any of this meaningful.

Parameters are the conventional ones (Donchian 20, Connors RSI-2 below 10,
MACD 12/26/9, golden cross 50/200). Where a filter is part of the published
rule it is kept — Connors' RSI-2 is explicitly a *pullback in an uptrend*
system and is not the same idea without its 200-day filter.
"""
import statistics


# ---------------------------------------------------------------- indicators

def sma(v, n):
    out = [None] * len(v)
    if len(v) < n:
        return out
    run = sum(v[:n])
    out[n - 1] = run / n
    for i in range(n, len(v)):
        run += v[i] - v[i - n]
        out[i] = run / n
    return out


def ema(v, n):
    out = [None] * len(v)
    if len(v) < n:
        return out
    k = 2.0 / (n + 1.0)
    prev = statistics.fmean(v[:n])
    out[n - 1] = prev
    for i in range(n, len(v)):
        prev = (v[i] - prev) * k + prev
        out[i] = prev
    return out


def rsi(v, n=2):
    """Wilder's RSI. At n=2 it is the Connors variant: extremely twitchy by
    design, spending most of its time at the extremes."""
    out = [None] * len(v)
    if len(v) <= n:
        return out
    gains = losses = 0.0
    for i in range(1, n + 1):
        ch = v[i] - v[i - 1]
        gains += max(ch, 0.0)
        losses += max(-ch, 0.0)
    ag, al = gains / n, losses / n
    out[n] = 100.0 if al == 0 else 100 - 100 / (1 + ag / al)
    for i in range(n + 1, len(v)):
        ch = v[i] - v[i - 1]
        ag = (ag * (n - 1) + max(ch, 0.0)) / n
        al = (al * (n - 1) + max(-ch, 0.0)) / n
        out[i] = 100.0 if al == 0 else 100 - 100 / (1 + ag / al)
    return out


def rolling_max(v, n):
    return [None if i < n else max(v[i - n:i]) for i in range(len(v))]


def rolling_min(v, n):
    return [None if i < n else min(v[i - n:i]) for i in range(len(v))]


def stdev(v, n):
    out = [None] * len(v)
    for i in range(n - 1, len(v)):
        out[i] = statistics.pstdev(v[i - n + 1:i + 1])
    return out


def atr(bars, n=14):
    """True range, Wilder-smoothed."""
    out = [None] * len(bars)
    trs = []
    for i, (_, o, h, l, c, _) in enumerate(bars):
        pc = bars[i - 1][4] if i else c
        trs.append(max(h - l, abs(h - pc), abs(l - pc)))
    if len(trs) < n:
        return out
    prev = statistics.fmean(trs[:n])
    out[n - 1] = prev
    for i in range(n, len(trs)):
        prev = (prev * (n - 1) + trs[i]) / n
        out[i] = prev
    return out


def cols(bars):
    return ([b[1] for b in bars], [b[2] for b in bars],
            [b[3] for b in bars], [b[4] for b in bars], [b[5] for b in bars])


def _cross_up(series, i):
    a, b = series
    return (None not in (a[i], b[i], a[i - 1], b[i - 1])
            and a[i - 1] <= b[i - 1] and a[i] > b[i])


# -------------------------------------------------------------------- setups

def donchian_breakout(bars, lookback=20):
    """Turtle. Close takes out the highest high of the prior `lookback` days.

    The canonical breakout system, and the reference every other breakout is a
    variation on.
    """
    _, h, _, c, _ = cols(bars)
    hh = rolling_max(h, lookback)
    return [i for i in range(len(bars)) if hh[i] is not None and c[i] > hh[i]]


def fiftytwo_week_high(bars):
    """Close makes a new 250-day high — the momentum anomaly in its plainest
    form. Slower and rarer than Donchian 20."""
    return donchian_breakout(bars, 250)


def golden_cross(bars):
    """SMA 50 crosses above SMA 200. The best-known trend signal there is, and
    a deliberately slow one — included as the long-horizon reference."""
    _, _, _, c, _ = cols(bars)
    return [i for i in range(1, len(bars)) if _cross_up((sma(c, 50), sma(c, 200)), i)] \
        if len(bars) > 200 else []


def ema_cross(bars, fast=10, slow=30):
    """The medium-term trend cross. Faster than the golden cross, slower than
    anything intraday."""
    _, _, _, c, _ = cols(bars)
    f, s = ema(c, fast), ema(c, slow)
    return [i for i in range(1, len(bars)) if _cross_up((f, s), i)]


def macd_cross(bars):
    """MACD line crosses above its signal, 12/26/9. Momentum turning up,
    smoothed twice."""
    _, _, _, c, _ = cols(bars)
    e12, e26 = ema(c, 12), ema(c, 26)
    line = [None if None in (e12[i], e26[i]) else e12[i] - e26[i] for i in range(len(c))]
    vals = [x for x in line if x is not None]
    if len(vals) < 9:
        return []
    off = len(line) - len(vals)
    sig9 = ema(vals, 9)
    sig = [None] * off + sig9
    return [i for i in range(1, len(bars)) if _cross_up((line, sig), i)]


def rsi2_pullback(bars, threshold=10):
    """Connors RSI-2: buy a deeply oversold day while the trend is up.

    The 200-day filter is part of the rule, not an addition — without it this
    is catching falling knives rather than buying dips in an uptrend.
    """
    _, _, _, c, _ = cols(bars)
    r, s200 = rsi(c, 2), sma(c, 200)
    return [i for i in range(len(bars))
            if None not in (r[i], s200[i]) and c[i] > s200[i] and r[i] < threshold]


def bollinger_reversion(bars, n=20, k=2.0):
    """Close below the lower Bollinger band while above the 200-day mean.

    Same family as RSI-2 — stretched price in an intact uptrend — but measured
    in standard deviations rather than by an oscillator, so the two disagree
    often enough to be worth running both.
    """
    _, _, _, c, _ = cols(bars)
    m, sd, s200 = sma(c, n), stdev(c, n), sma(c, 200)
    return [i for i in range(len(bars))
            if None not in (m[i], sd[i], s200[i])
            and c[i] > s200[i] and c[i] < m[i] - k * sd[i]]


def ma_pullback(bars):
    """Trend continuation: an uptrend (close > SMA 50 > SMA 200) pulls back to
    touch the 20-day mean and closes back above it. Buy the resumption, not
    the dip itself."""
    _, _, l, c, _ = cols(bars)
    s20, s50, s200 = sma(c, 20), sma(c, 50), sma(c, 200)
    out = []
    for i in range(1, len(bars)):
        if None in (s20[i], s50[i], s200[i]):
            continue
        if not (c[i] > s50[i] > s200[i]):
            continue
        if l[i] <= s20[i] < c[i]:   # touched it, closed above it
            out.append(i)
    return out


def nr7_breakout(bars):
    """Volatility contraction: today's range is the narrowest of the last
    seven, and the close is above the 50-day mean.

    The premise is that a coiled range precedes expansion; the trend filter
    decides which way to bet on it.
    """
    _, h, l, c, _ = cols(bars)
    s50 = sma(c, 50)
    out = []
    for i in range(7, len(bars)):
        rng = [h[j] - l[j] for j in range(i - 6, i + 1)]
        if rng[-1] == min(rng) and s50[i] is not None and c[i] > s50[i]:
            out.append(i)
    return out


def volume_breakout(bars, lookback=20, vol_mult=2.0):
    """Donchian breakout that insists on participation: a 20-day-high close on
    at least twice the 20-day average volume.

    The most common retail refinement of a plain breakout, and a direct test
    of whether volume confirmation is worth anything.
    """
    _, h, _, c, v = cols(bars)
    hh, av = rolling_max(h, lookback), sma(v, lookback)
    return [i for i in range(len(bars))
            if None not in (hh[i], av[i]) and av[i] > 0
            and c[i] > hh[i] and v[i] >= vol_mult * av[i]]


SETUPS = {
    "donchian20":     (donchian_breakout, "breakout"),
    "high_52w":       (fiftytwo_week_high, "breakout"),
    "volume_breakout": (volume_breakout,  "breakout"),
    "golden_cross":   (golden_cross,      "trend"),
    "ema_10_30":      (ema_cross,         "trend"),
    "macd_cross":     (macd_cross,        "trend"),
    "ma_pullback":    (ma_pullback,       "continuation"),
    "nr7_breakout":   (nr7_breakout,      "continuation"),
    "rsi2_pullback":  (rsi2_pullback,     "reversion"),
    "bollinger_rev":  (bollinger_reversion, "reversion"),
}
