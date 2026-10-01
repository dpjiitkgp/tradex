CREATE TABLE runs (
    id INTEGER PRIMARY KEY,
    started_at TEXT NOT NULL,
    ended_at TEXT,
    mode TEXT NOT NULL CHECK(mode IN ('live','backtest')),
    config_snapshot_json TEXT NOT NULL,
    git_commit_hash TEXT
);

-- Reference cache of tradingsymbol <-> instrument_token, not a strict FK
-- target: candles/signals/trades key on instrument_token directly so
-- writing those doesn't require an instruments row to exist first.
CREATE TABLE instruments (
    id INTEGER PRIMARY KEY,
    tradingsymbol TEXT NOT NULL UNIQUE,
    instrument_token INTEGER NOT NULL,
    exchange TEXT NOT NULL,
    lot_size INTEGER,
    tick_size REAL
);

-- Historical candle cache: also the backtest (M8) data source and the
-- universe screener's (M4) turnover/ATR/volume inputs, so a backfill only
-- has to hit Kite once per range.
CREATE TABLE candles (
    instrument_token INTEGER NOT NULL,
    interval TEXT NOT NULL,
    ts TEXT NOT NULL,
    open REAL NOT NULL,
    high REAL NOT NULL,
    low REAL NOT NULL,
    close REAL NOT NULL,
    volume INTEGER NOT NULL,
    PRIMARY KEY (instrument_token, interval, ts)
);

CREATE TABLE signals (
    id INTEGER PRIMARY KEY,
    run_id INTEGER NOT NULL REFERENCES runs(id),
    ts TEXT NOT NULL,
    instrument_token INTEGER NOT NULL,
    strategy_id TEXT NOT NULL,
    window TEXT NOT NULL,
    direction TEXT NOT NULL,
    entry_price_hint REAL NOT NULL,
    rationale TEXT NOT NULL,
    params_json TEXT NOT NULL
);

CREATE TABLE trades (
    id INTEGER PRIMARY KEY,
    run_id INTEGER NOT NULL REFERENCES runs(id),
    signal_id INTEGER REFERENCES signals(id),
    instrument_token INTEGER NOT NULL,
    strategy_id TEXT NOT NULL,
    window TEXT NOT NULL,
    mode TEXT NOT NULL CHECK(mode IN ('live','backtest')),
    direction TEXT NOT NULL,
    qty INTEGER NOT NULL,
    capital_allocated REAL NOT NULL,
    entry_ts TEXT NOT NULL,
    entry_price REAL NOT NULL,
    hard_stop_price REAL NOT NULL,
    min_target_price REAL NOT NULL,
    trail_policy_json TEXT NOT NULL,
    exit_ts TEXT,
    exit_price REAL,
    exit_reason TEXT CHECK(exit_reason IN
        ('stop_loss','trailing_stop','vwap_cross','eod_square_off','max_hold','manual')),
    pnl_abs REAL,
    pnl_pct REAL,
    r_multiple REAL
);
CREATE INDEX idx_trades_analysis ON trades(strategy_id, window, instrument_token, mode);

CREATE TABLE trade_stop_events (
    id INTEGER PRIMARY KEY,
    trade_id INTEGER NOT NULL REFERENCES trades(id),
    ts TEXT NOT NULL,
    new_stop_price REAL NOT NULL
);

-- Convenience view for the post-hoc "what actually worked" analysis and for
-- ad-hoc `sqlite3` exploration even before any dashboard exists.
CREATE VIEW v_strategy_performance AS
SELECT strategy_id, window, mode, instrument_token,
       COUNT(*) AS trades,
       AVG(pnl_pct) AS avg_pnl_pct,
       AVG(r_multiple) AS avg_r,
       SUM(CASE WHEN pnl_abs > 0 THEN 1 ELSE 0 END) * 1.0 / COUNT(*) AS win_rate
FROM trades
WHERE exit_ts IS NOT NULL
GROUP BY strategy_id, window, mode, instrument_token;
