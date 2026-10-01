-- Human-readable run names (e.g. "orb-relvol-2x") so backtests taken while
-- tuning a strategy can be told apart without memorizing numeric ids.
ALTER TABLE runs ADD COLUMN label TEXT;

-- Re-scope the performance view per run_id: previously every backtest ever
-- run got merged into one aggregate, which silently hid whether a
-- parameter change actually helped. Now each run's numbers stay separate,
-- and callers who genuinely want everything pooled can aggregate the rows
-- themselves.
DROP VIEW v_strategy_performance;
CREATE VIEW v_strategy_performance AS
SELECT t.run_id, r.label AS run_label, t.strategy_id, t.window, t.mode, t.instrument_token,
       COUNT(*) AS trades,
       AVG(t.pnl_pct) AS avg_pnl_pct,
       AVG(t.r_multiple) AS avg_r,
       SUM(CASE WHEN t.pnl_abs > 0 THEN 1 ELSE 0 END) * 1.0 / COUNT(*) AS win_rate
FROM trades t
JOIN runs r ON r.id = t.run_id
WHERE t.exit_ts IS NOT NULL
GROUP BY t.run_id, t.strategy_id, t.window, t.mode, t.instrument_token;
