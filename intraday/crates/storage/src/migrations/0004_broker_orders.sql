-- Every real broker order-lifecycle event (place/modify/cancel/status
-- poll), for the live real-money trading engine's audit trail — a row per
-- attempt, not just the final outcome, so a rejected-then-retried order or
-- a limit-then-market-fallback sequence is fully reconstructable later.
-- `trade_id` is nullable: an order rejected before ever filling has no
-- corresponding `trades` row.
CREATE TABLE broker_orders (
    id INTEGER PRIMARY KEY,
    run_id INTEGER NOT NULL REFERENCES runs(id),
    trade_id INTEGER REFERENCES trades(id),
    instrument_token INTEGER NOT NULL,
    tradingsymbol TEXT NOT NULL,
    kite_order_id TEXT,
    action TEXT NOT NULL CHECK(action IN ('place','modify','cancel','status_poll')),
    variety TEXT NOT NULL DEFAULT 'regular',
    transaction_type TEXT NOT NULL CHECK(transaction_type IN ('BUY','SELL')),
    order_type TEXT NOT NULL CHECK(order_type IN ('MARKET','LIMIT','SL','SL-M')),
    product TEXT NOT NULL DEFAULT 'MIS',
    requested_price REAL,
    trigger_price REAL,
    quantity INTEGER NOT NULL,
    status TEXT NOT NULL,
    filled_quantity INTEGER NOT NULL DEFAULT 0,
    average_price REAL,
    is_fallback_market INTEGER NOT NULL DEFAULT 0 CHECK(is_fallback_market IN (0,1)),
    rejection_reason TEXT,
    raw_response_json TEXT NOT NULL,
    placed_at TEXT NOT NULL
);

CREATE INDEX idx_broker_orders_run_id ON broker_orders(run_id);
CREATE INDEX idx_broker_orders_kite_order_id ON broker_orders(kite_order_id);
CREATE INDEX idx_broker_orders_trade_id ON broker_orders(trade_id);

-- The overwhelmingly common query pattern all session has been "give me
-- this run's trades" — added directly to a live db earlier via a bare
-- CREATE INDEX (fixed a real multi-minute-per-query slowdown once the
-- table passed ~13M rows); folded into migrations here so a fresh db gets
-- it from the start instead of relying on that one-off fix.
CREATE INDEX IF NOT EXISTS idx_trades_run_id ON trades(run_id);
