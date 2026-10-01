-- Each run's requested date range, so the UI can show "this backtest
-- covered Jun 1 - Sep 22" without guessing from trade timestamps (a run
-- with few/no trades in some sub-range would otherwise look like it
-- covered less than it actually did).
ALTER TABLE runs ADD COLUMN from_date TEXT;
ALTER TABLE runs ADD COLUMN to_date TEXT;

-- NSE's index constituent CSVs (ind_nifty*list.csv) carry an Industry
-- column we were previously discarding — needed to backtest "all stocks
-- in a sector" from the UI.
ALTER TABLE instruments ADD COLUMN sector TEXT;

DROP VIEW v_strategy_performance;
CREATE VIEW v_strategy_performance AS
SELECT t.run_id, r.label AS run_label, r.from_date, r.to_date,
       t.strategy_id, t.window, t.mode, t.instrument_token,
       COUNT(*) AS trades,
       AVG(t.pnl_pct) AS avg_pnl_pct,
       AVG(t.r_multiple) AS avg_r,
       SUM(CASE WHEN t.pnl_abs > 0 THEN 1 ELSE 0 END) * 1.0 / COUNT(*) AS win_rate
FROM trades t
JOIN runs r ON r.id = t.run_id
WHERE t.exit_ts IS NOT NULL
GROUP BY t.run_id, t.strategy_id, t.window, t.mode, t.instrument_token;
