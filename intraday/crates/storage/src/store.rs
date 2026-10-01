use crate::convert::{
    broker_order_action_to_str, direction_to_str, exit_reason_to_str, mode_to_str,
    order_side_to_str, order_status_to_str, order_type_to_str, sqlite_err, window_to_str,
};
use crate::schema;
use crate::{RunMeta, StrategyPerformanceRow, StrategySummaryRow};
use core_types::{
    Candle, CandleInterval, Exchange, Instrument, NewRun, NewTrade, RunId, SignalId, SignalRecord,
    StopMove, Storage, TradeExit, TradeId,
};
use rusqlite::{params, Connection};
use std::path::Path;
use std::sync::Mutex;

/// `core_types::Storage` implementation backed by a local SQLite file (WAL
/// mode — a report/read can run concurrently with the writing daemon).
/// Also exposes candle cache methods beyond the trait: those aren't part of
/// the broker-agnostic contract (nothing outside SQLite needs them), but
/// `backtest` (M8) and `universe` (M4) both want a local candle cache to
/// avoid re-hitting Kite for the same range.
pub struct SqliteStorage {
    conn: Mutex<Connection>,
}

impl SqliteStorage {
    pub fn open(path: &Path) -> anyhow::Result<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut conn = Connection::open(path)?;
        conn.pragma_update(None, "journal_mode", "WAL")?;
        // WAL allows concurrent readers with one writer, but a *second*
        // writer gets SQLITE_BUSY — and with no busy_timeout set (the
        // default), that's an immediate hard error rather than a wait.
        // That's what crashed a research run mid-replay earlier when a
        // backfill was writing concurrently, and it's a far worse failure
        // for the live trading engine: a failed `open_trade` right after a
        // real order was placed would leave a real broker position with no
        // local record of it. 30s of patient retrying instead.
        conn.busy_timeout(std::time::Duration::from_secs(30))?;
        schema::migrations()
            .to_latest(&mut conn)
            .map_err(|e| anyhow::anyhow!("running storage migrations: {e}"))?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    /// A throwaway in-memory database — used by this crate's own tests, and
    /// a convenient way for other crates (`paper-engine`, `backtest`) to
    /// exercise the real `Storage` implementation in their own tests
    /// without touching disk.
    pub fn open_in_memory() -> anyhow::Result<Self> {
        let mut conn = Connection::open_in_memory()?;
        schema::migrations()
            .to_latest(&mut conn)
            .map_err(|e| anyhow::anyhow!("running storage migrations: {e}"))?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    /// Upserts the tradingsymbol <-> instrument_token mapping so
    /// `trades`/`signals`/`candles` rows (which key on `instrument_token`
    /// directly, not an FK) can be joined back to a human-readable symbol.
    /// Keyed on `tradingsymbol`'s UNIQUE constraint — safe to call
    /// repeatedly (e.g. once per universe screen) as tokens occasionally
    /// drift on instrument-master refreshes.
    pub fn upsert_instruments(&self, instruments: &[core_types::Instrument]) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        // A partial upsert, not INSERT OR REPLACE: the latter would wipe
        // `sector` back to NULL on every call (this runs on every
        // backtest/live/collect start), silently destroying data set by
        // `upsert_sectors` — a real bug caught before it shipped.
        let mut stmt = conn.prepare(
            "INSERT INTO instruments (tradingsymbol, instrument_token, exchange, lot_size, tick_size)
             VALUES (?1, ?2, ?3, ?4, ?5)
             ON CONFLICT(tradingsymbol) DO UPDATE SET
                instrument_token = excluded.instrument_token,
                exchange = excluded.exchange,
                lot_size = excluded.lot_size,
                tick_size = excluded.tick_size",
        )?;
        for i in instruments {
            let exchange = match i.exchange {
                core_types::Exchange::Nse => "NSE",
                core_types::Exchange::Nfo => "NFO",
            };
            stmt.execute(params![
                i.tradingsymbol,
                i.instrument_token as i64,
                exchange,
                i.lot_size,
                i.tick_size,
            ])?;
        }
        Ok(())
    }

    /// Sets `sector` (NSE's "Industry" column) for tradingsymbols already
    /// present in `instruments` — call `upsert_instruments` first for any
    /// symbol that might not have a row yet.
    pub fn upsert_sectors(&self, mapping: &[(String, String)]) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("UPDATE instruments SET sector = ?1 WHERE tradingsymbol = ?2")?;
        for (symbol, sector) in mapping {
            stmt.execute(params![sector, symbol])?;
        }
        Ok(())
    }

    /// Distinct sectors with how many tagged instruments fall in each —
    /// for the UI's sector picker.
    pub fn distinct_sectors(&self) -> anyhow::Result<Vec<(String, i64)>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT sector, COUNT(*) FROM instruments
             WHERE sector IS NOT NULL AND sector != ''
             GROUP BY sector ORDER BY sector",
        )?;
        let rows = stmt
            .query_map([], |row| Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?)))?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    /// Instruments tagged with `sector` (or every tagged instrument, if
    /// `sector` is `None`) — the backtest-trigger scope resolution for
    /// "all stocks" / "a sector".
    pub fn instruments_by_sector(&self, sector: Option<&str>) -> anyhow::Result<Vec<Instrument>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT tradingsymbol, instrument_token, exchange, lot_size, tick_size
             FROM instruments
             WHERE sector IS NOT NULL AND sector != '' AND (?1 IS NULL OR sector = ?1)
             ORDER BY tradingsymbol",
        )?;
        let rows = stmt
            .query_map(params![sector], |row| {
                let exchange_s: String = row.get(2)?;
                let exchange = if exchange_s == "NFO" { Exchange::Nfo } else { Exchange::Nse };
                Ok(Instrument {
                    tradingsymbol: row.get(0)?,
                    instrument_token: row.get::<_, i64>(1)? as u64,
                    exchange,
                    lot_size: row.get(3)?,
                    tick_size: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    /// Same filter as `instruments_by_sector`, but also returns each row's
    /// sector — for the UI's stock-listing endpoint (which needs to show
    /// the sector, not just use it as a filter).
    pub fn tagged_instruments(&self, sector: Option<&str>) -> anyhow::Result<Vec<(Instrument, Option<String>)>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT tradingsymbol, instrument_token, exchange, lot_size, tick_size, sector
             FROM instruments
             WHERE sector IS NOT NULL AND sector != '' AND (?1 IS NULL OR sector = ?1)
             ORDER BY tradingsymbol",
        )?;
        let rows = stmt
            .query_map(params![sector], |row| {
                let exchange_s: String = row.get(2)?;
                let exchange = if exchange_s == "NFO" { Exchange::Nfo } else { Exchange::Nse };
                let inst = Instrument {
                    tradingsymbol: row.get(0)?,
                    instrument_token: row.get::<_, i64>(1)? as u64,
                    exchange,
                    lot_size: row.get(3)?,
                    tick_size: row.get(4)?,
                };
                let sector: Option<String> = row.get(5)?;
                Ok((inst, sector))
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    /// Every instrument that has at least `min_bars` cached candles at
    /// `interval` — the research/backtest universe when you want "whatever
    /// we've actually collected enough of," independent of the liquidity
    /// screen or sector tagging (which many collector-only symbols never
    /// get, since sector tagging only runs for the trading universe).
    pub fn instruments_with_candle_coverage(
        &self,
        interval: CandleInterval,
        min_bars: i64,
    ) -> anyhow::Result<Vec<Instrument>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT i.tradingsymbol, i.instrument_token, i.exchange, i.lot_size, i.tick_size
             FROM instruments i
             JOIN candles c ON c.instrument_token = i.instrument_token AND c.interval = ?1
             GROUP BY i.instrument_token
             HAVING COUNT(*) >= ?2
             ORDER BY i.tradingsymbol",
        )?;
        let rows = stmt
            .query_map(params![interval.as_str(), min_bars], |row| {
                let exchange_s: String = row.get(2)?;
                let exchange = if exchange_s == "NFO" { Exchange::Nfo } else { Exchange::Nse };
                Ok(Instrument {
                    tradingsymbol: row.get(0)?,
                    instrument_token: row.get::<_, i64>(1)? as u64,
                    exchange,
                    lot_size: row.get(3)?,
                    tick_size: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    /// One instrument by its exact tradingsymbol (case-insensitive) — for
    /// the UI's single-stock backtest scope.
    pub fn instrument_by_symbol(&self, symbol: &str) -> anyhow::Result<Option<Instrument>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT tradingsymbol, instrument_token, exchange, lot_size, tick_size
             FROM instruments WHERE tradingsymbol = ?1 COLLATE NOCASE LIMIT 1",
        )?;
        let mut rows = stmt.query(params![symbol])?;
        Ok(match rows.next()? {
            Some(row) => {
                let exchange_s: String = row.get(2)?;
                let exchange = if exchange_s == "NFO" { Exchange::Nfo } else { Exchange::Nse };
                Some(Instrument {
                    tradingsymbol: row.get(0)?,
                    instrument_token: row.get::<_, i64>(1)? as u64,
                    exchange,
                    lot_size: row.get(3)?,
                    tick_size: row.get(4)?,
                })
            }
            None => None,
        })
    }

    /// Candle coverage for one instrument — `None` if nothing's cached.
    pub fn candle_coverage(&self, instrument_token: u64) -> anyhow::Result<Option<(String, String, i64)>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT MIN(ts), MAX(ts), COUNT(*) FROM candles WHERE instrument_token = ?1 AND interval = '1m'",
        )?;
        let mut rows = stmt.query(params![instrument_token as i64])?;
        Ok(match rows.next()? {
            Some(row) => {
                let min: Option<String> = row.get(0)?;
                let max: Option<String> = row.get(1)?;
                let count: i64 = row.get(2)?;
                match (min, max) {
                    (Some(a), Some(b)) => Some((a, b, count)),
                    _ => None,
                }
            }
            None => None,
        })
    }

    /// Overall candle coverage across every cached instrument — for the
    /// UI's top-level "data coverage" summary.
    pub fn overall_candle_coverage(&self) -> anyhow::Result<Option<(String, String, i64)>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT MIN(ts), MAX(ts), COUNT(DISTINCT instrument_token) FROM candles WHERE interval = '1m'",
        )?;
        let mut rows = stmt.query([])?;
        Ok(match rows.next()? {
            Some(row) => {
                let min: Option<String> = row.get(0)?;
                let max: Option<String> = row.get(1)?;
                let count: i64 = row.get(2)?;
                match (min, max) {
                    (Some(a), Some(b)) => Some((a, b, count)),
                    _ => None,
                }
            }
            None => None,
        })
    }

    /// All runs, newest first — for the UI's runs list.
    pub fn list_runs(&self) -> anyhow::Result<Vec<RunMeta>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, label, mode, started_at, ended_at, from_date, to_date, config_snapshot_json
             FROM runs ORDER BY id DESC",
        )?;
        let rows = stmt
            .query_map([], |row| {
                Ok(RunMeta {
                    id: row.get(0)?,
                    label: row.get(1)?,
                    mode: row.get(2)?,
                    started_at: row.get(3)?,
                    ended_at: row.get(4)?,
                    from_date: row.get(5)?,
                    to_date: row.get(6)?,
                    config_snapshot_json: row.get(7)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    /// Looks up a symbol by instrument_token, for display purposes (report
    /// printing etc.) — `None` if it was never upserted.
    pub fn symbol_for_token(&self, instrument_token: u64) -> anyhow::Result<Option<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT tradingsymbol FROM instruments WHERE instrument_token = ?1 LIMIT 1")?;
        let mut rows = stmt.query(params![instrument_token as i64])?;
        Ok(match rows.next()? {
            Some(row) => Some(row.get(0)?),
            None => None,
        })
    }

    pub fn insert_candles(
        &self,
        instrument_token: u64,
        interval: CandleInterval,
        candles: &[Candle],
    ) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "INSERT OR REPLACE INTO candles
             (instrument_token, interval, ts, open, high, low, close, volume)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        )?;
        for c in candles {
            stmt.execute(params![
                instrument_token as i64,
                interval.as_str(),
                c.ts.to_rfc3339(),
                c.open,
                c.high,
                c.low,
                c.close,
                c.volume as i64,
            ])?;
        }
        Ok(())
    }

    /// Reads `v_strategy_performance` — the fine-grained (per strategy x
    /// window x instrument) "what actually worked" report, scoped to one
    /// run when `run_id` is given, across every run otherwise (each row
    /// still carries its own `run_id`/`run_label`, so runs are never
    /// silently pooled together even in the unfiltered case). Left-joins
    /// `instruments` for a human-readable symbol where one's been upserted
    /// (via `upsert_instruments`) — `None`/instrument_token-as-string
    /// otherwise.
    pub fn strategy_performance(&self, run_id: Option<RunId>) -> anyhow::Result<Vec<StrategyPerformanceRow>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT v.run_id, v.run_label, v.strategy_id, v.window, v.mode, v.instrument_token, i.tradingsymbol,
                    v.trades, v.avg_pnl_pct, v.avg_r, v.win_rate
             FROM v_strategy_performance v
             LEFT JOIN instruments i ON i.instrument_token = v.instrument_token
             WHERE ?1 IS NULL OR v.run_id = ?1
             ORDER BY v.run_id DESC, v.avg_r DESC",
        )?;
        let rows = stmt
            .query_map(params![run_id], |row| {
                Ok(StrategyPerformanceRow {
                    run_id: row.get(0)?,
                    run_label: row.get(1)?,
                    strategy_id: row.get(2)?,
                    window: row.get(3)?,
                    mode: row.get(4)?,
                    instrument_token: row.get::<_, i64>(5)? as u64,
                    tradingsymbol: row.get(6)?,
                    trades: row.get(7)?,
                    avg_pnl_pct: row.get(8)?,
                    avg_r: row.get(9)?,
                    win_rate: row.get(10)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    /// Coarse per-run x per-strategy rollup (no instrument/window split) —
    /// computed directly from `trades`, not by averaging
    /// `v_strategy_performance`'s already-averaged rows, since averaging
    /// per-instrument averages would silently misweight instruments with
    /// different trade counts. This is the shape `compare` uses to put two
    /// or more tuning runs side by side.
    pub fn strategy_summary(&self, run_ids: Option<&[RunId]>) -> anyhow::Result<Vec<StrategySummaryRow>> {
        let conn = self.conn.lock().unwrap();
        let sql = "SELECT t.run_id, r.label, t.strategy_id, t.mode,
                          COUNT(*) AS trades,
                          AVG(t.pnl_pct) AS avg_pnl_pct,
                          AVG(t.r_multiple) AS avg_r,
                          SUM(CASE WHEN t.pnl_abs > 0 THEN 1 ELSE 0 END) * 1.0 / COUNT(*) AS win_rate
                   FROM trades t
                   JOIN runs r ON r.id = t.run_id
                   WHERE t.exit_ts IS NOT NULL
                   {run_filter}
                   GROUP BY t.run_id, t.strategy_id, t.mode
                   ORDER BY t.strategy_id, t.run_id";

        let row_mapper = |row: &rusqlite::Row| {
            Ok(StrategySummaryRow {
                run_id: row.get(0)?,
                run_label: row.get(1)?,
                strategy_id: row.get(2)?,
                mode: row.get(3)?,
                trades: row.get(4)?,
                avg_pnl_pct: row.get(5)?,
                avg_r: row.get(6)?,
                win_rate: row.get(7)?,
            })
        };

        let rows = match run_ids {
            None => {
                let sql = sql.replace("{run_filter}", "");
                let mut stmt = conn.prepare(&sql)?;
                let mapped = stmt.query_map([], row_mapper)?.collect::<Result<Vec<_>, _>>()?;
                mapped
            }
            Some(ids) => {
                let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
                let sql = sql.replace("{run_filter}", &format!("AND t.run_id IN ({placeholders})"));
                let mut stmt = conn.prepare(&sql)?;
                let params: Vec<&dyn rusqlite::ToSql> =
                    ids.iter().map(|id| id as &dyn rusqlite::ToSql).collect();
                let mapped = stmt.query_map(params.as_slice(), row_mapper)?.collect::<Result<Vec<_>, _>>()?;
                mapped
            }
        };
        Ok(rows)
    }

    pub fn query_candles(
        &self,
        instrument_token: u64,
        interval: CandleInterval,
        from: core_types::DateTime,
        to: core_types::DateTime,
    ) -> anyhow::Result<Vec<Candle>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT ts, open, high, low, close, volume FROM candles
             WHERE instrument_token = ?1 AND interval = ?2 AND ts >= ?3 AND ts <= ?4
             ORDER BY ts ASC",
        )?;
        let rows = stmt.query_map(
            params![
                instrument_token as i64,
                interval.as_str(),
                from.to_rfc3339(),
                to.to_rfc3339(),
            ],
            |row| {
                let ts_s: String = row.get(0)?;
                Ok((
                    ts_s,
                    row.get::<_, f64>(1)?,
                    row.get::<_, f64>(2)?,
                    row.get::<_, f64>(3)?,
                    row.get::<_, f64>(4)?,
                    row.get::<_, i64>(5)?,
                ))
            },
        )?;

        let mut out = Vec::new();
        for row in rows {
            let (ts_s, open, high, low, close, volume) = row?;
            let ts = chrono::DateTime::parse_from_rfc3339(&ts_s)?.with_timezone(&chrono::Utc);
            out.push(Candle {
                ts,
                open,
                high,
                low,
                close,
                volume: volume as u64,
            });
        }
        Ok(out)
    }

    /// Fold the write-ahead log back into the database and truncate it.
    ///
    /// In WAL mode SQLite can only checkpoint when no reader holds an older
    /// snapshot. A long analytical query pins one for its whole duration, so
    /// during a batch of research runs the WAL grows unbounded — it reached
    /// 29GB beside a 28GB database here, which both filled the disk and made
    /// every subsequent query slower (readers must search the WAL index), a
    /// feedback loop that turned 30-second queries into multi-hour ones.
    ///
    /// Call this at the end of any job that writes a lot. It is a no-op
    /// when nothing is pending, and a failure is never fatal: a checkpoint
    /// that loses a race with another connection is an optimisation missed,
    /// not data lost, so the caller should log and carry on.
    pub fn checkpoint(&self) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.pragma_update(None, "wal_checkpoint", "TRUNCATE")?;
        Ok(())
    }
}

impl Storage for SqliteStorage {
    fn start_run(&self, new_run: &NewRun) -> core_types::Result<RunId> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO runs (started_at, mode, config_snapshot_json, label, from_date, to_date)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                chrono::Utc::now().to_rfc3339(),
                mode_to_str(new_run.mode),
                new_run.config_snapshot_json,
                new_run.label,
                new_run.from_date.map(|d| d.to_string()),
                new_run.to_date.map(|d| d.to_string()),
            ],
        )
        .map_err(sqlite_err)?;
        Ok(conn.last_insert_rowid())
    }

    fn end_run(&self, run_id: RunId) -> core_types::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE runs SET ended_at = ?1 WHERE id = ?2",
            params![chrono::Utc::now().to_rfc3339(), run_id],
        )
        .map_err(sqlite_err)?;
        Ok(())
    }

    fn record_signal(&self, signal: &SignalRecord) -> core_types::Result<SignalId> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO signals
             (run_id, ts, instrument_token, strategy_id, window, direction, entry_price_hint, rationale, params_json)
             VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)",
            params![
                signal.run_id,
                signal.ts.to_rfc3339(),
                signal.instrument_token as i64,
                signal.strategy_id,
                window_to_str(signal.window),
                direction_to_str(signal.direction),
                signal.entry_price_hint,
                signal.rationale,
                signal.params_json,
            ],
        )
        .map_err(sqlite_err)?;
        Ok(conn.last_insert_rowid())
    }

    fn open_trade(&self, trade: &NewTrade) -> core_types::Result<TradeId> {
        let conn = self.conn.lock().unwrap();
        let trail_policy_json = serde_json::to_string(&trade.trail_policy)
            .map_err(|e| core_types::Error::Storage(e.to_string()))?;
        conn.execute(
            "INSERT INTO trades
             (run_id, signal_id, instrument_token, strategy_id, window, mode, direction, qty,
              capital_allocated, entry_ts, entry_price, hard_stop_price, min_target_price, trail_policy_json)
             VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14)",
            params![
                trade.run_id,
                trade.signal_id,
                trade.instrument_token as i64,
                trade.strategy_id,
                window_to_str(trade.window),
                mode_to_str(trade.mode),
                direction_to_str(trade.direction),
                trade.qty,
                trade.capital_allocated,
                trade.entry_ts.to_rfc3339(),
                trade.entry_price,
                trade.hard_stop_price,
                trade.min_target_price,
                trail_policy_json,
            ],
        )
        .map_err(sqlite_err)?;
        Ok(conn.last_insert_rowid())
    }

    fn record_stop_move(&self, trade_id: TradeId, mv: &StopMove) -> core_types::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO trade_stop_events (trade_id, ts, new_stop_price) VALUES (?1,?2,?3)",
            params![trade_id, mv.ts.to_rfc3339(), mv.new_stop_price],
        )
        .map_err(sqlite_err)?;
        Ok(())
    }

    fn close_trade(&self, trade_id: TradeId, exit: &TradeExit) -> core_types::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE trades SET exit_ts=?1, exit_price=?2, exit_reason=?3, pnl_abs=?4, pnl_pct=?5, r_multiple=?6
             WHERE id=?7",
            params![
                exit.exit_ts.to_rfc3339(),
                exit.exit_price,
                exit_reason_to_str(exit.reason),
                exit.pnl_abs,
                exit.pnl_pct,
                exit.r_multiple,
                trade_id,
            ],
        )
        .map_err(sqlite_err)?;
        Ok(())
    }

    fn record_broker_order(&self, event: &core_types::BrokerOrderEvent) -> core_types::Result<i64> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO broker_orders
             (run_id, trade_id, instrument_token, tradingsymbol, kite_order_id, action, variety,
              transaction_type, order_type, product, requested_price, trigger_price, quantity,
              status, filled_quantity, average_price, is_fallback_market, rejection_reason,
              raw_response_json, placed_at)
             VALUES (?1,?2,?3,?4,?5,?6,'regular',?7,?8,'MIS',?9,?10,?11,?12,?13,?14,?15,?16,?17,?18)",
            params![
                event.run_id,
                event.trade_id,
                event.instrument_token as i64,
                event.tradingsymbol,
                event.kite_order_id,
                broker_order_action_to_str(event.action),
                order_side_to_str(event.transaction_type),
                order_type_to_str(event.order_type),
                event.requested_price,
                event.trigger_price,
                event.quantity,
                order_status_to_str(&event.status),
                event.filled_quantity,
                event.average_price,
                event.is_fallback_market as i64,
                event.rejection_reason,
                event.raw_response_json,
                event.placed_at.to_rfc3339(),
            ],
        )
        .map_err(sqlite_err)?;
        Ok(conn.last_insert_rowid())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core_types::{
        Direction, ExitReason, RunMode, SignalRecord, TimeWindow, TradeExit, TrailPolicy,
    };

    fn candle(open: f64, high: f64, low: f64, close: f64, volume: u64) -> Candle {
        Candle {
            ts: chrono::Utc::now(),
            open,
            high,
            low,
            close,
            volume,
        }
    }

    #[test]
    fn full_run_signal_trade_lifecycle_round_trips() {
        let db = SqliteStorage::open_in_memory().unwrap();

        let run_id = db
            .start_run(&core_types::NewRun {
                mode: RunMode::Backtest,
                config_snapshot_json: "{}".to_string(),
                label: Some("test-run".to_string()),
                from_date: None,
                to_date: None,
            })
            .unwrap();

        let signal_id = db
            .record_signal(&SignalRecord {
                run_id,
                ts: chrono::Utc::now(),
                instrument_token: 12345,
                strategy_id: "orb".to_string(),
                window: TimeWindow::Opening,
                direction: Direction::Long,
                entry_price_hint: 100.0,
                rationale: "opening range breakout".to_string(),
                params_json: "{}".to_string(),
            })
            .unwrap();

        let trade_id = db
            .open_trade(&NewTrade {
                run_id,
                signal_id: Some(signal_id),
                instrument_token: 12345,
                strategy_id: "orb".to_string(),
                window: TimeWindow::Opening,
                mode: RunMode::Backtest,
                direction: Direction::Long,
                qty: 10,
                capital_allocated: 1000.0,
                entry_ts: chrono::Utc::now(),
                entry_price: 100.0,
                hard_stop_price: 99.75,
                min_target_price: 100.5,
                trail_policy: TrailPolicy::PercentFromPeak(0.15),
            })
            .unwrap();

        db.record_stop_move(
            trade_id,
            &StopMove {
                ts: chrono::Utc::now(),
                new_stop_price: 100.5,
            },
        )
        .unwrap();

        db.close_trade(
            trade_id,
            &TradeExit {
                exit_ts: chrono::Utc::now(),
                exit_price: 101.0,
                reason: ExitReason::TrailingStop,
                pnl_abs: 10.0,
                pnl_pct: 1.0,
                r_multiple: 4.0,
            },
        )
        .unwrap();

        db.end_run(run_id).unwrap();

        let conn = db.conn.lock().unwrap();
        let trade_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM trades WHERE id = ?1", [trade_id], |r| r.get(0))
            .unwrap();
        assert_eq!(trade_count, 1);
        drop(conn);

        let perf = db.strategy_performance(None).unwrap();
        assert_eq!(perf.len(), 1);
        assert_eq!(perf[0].run_id, run_id);
        assert_eq!(perf[0].run_label.as_deref(), Some("test-run"));
        assert_eq!(perf[0].strategy_id, "orb");
        assert_eq!(perf[0].trades, 1);
        assert_eq!(perf[0].win_rate, 1.0);
        assert!((perf[0].avg_r - 4.0).abs() < 1e-9);

        // strategy_performance scoped to this run_id returns the same row.
        let scoped = db.strategy_performance(Some(run_id)).unwrap();
        assert_eq!(scoped.len(), 1);

        // strategy_summary rolls the same trade up without an instrument split.
        let summary = db.strategy_summary(None).unwrap();
        assert_eq!(summary.len(), 1);
        assert_eq!(summary[0].strategy_id, "orb");
        assert_eq!(summary[0].trades, 1);
        assert!((summary[0].avg_r - 4.0).abs() < 1e-9);
    }

    #[test]
    fn candle_cache_round_trips_and_filters_by_range() {
        let db = SqliteStorage::open_in_memory().unwrap();
        let t0 = chrono::Utc::now() - chrono::Duration::days(1);
        let t1 = chrono::Utc::now();
        let mut c0 = candle(1.0, 2.0, 0.5, 1.5, 100);
        c0.ts = t0;
        let mut c1 = candle(1.5, 2.5, 1.0, 2.0, 200);
        c1.ts = t1;

        db.insert_candles(999, CandleInterval::OneMinute, &[c0, c1]).unwrap();

        let all = db
            .query_candles(
                999,
                CandleInterval::OneMinute,
                t0 - chrono::Duration::seconds(1),
                t1 + chrono::Duration::seconds(1),
            )
            .unwrap();
        assert_eq!(all.len(), 2);

        let only_first = db
            .query_candles(
                999,
                CandleInterval::OneMinute,
                t0 - chrono::Duration::seconds(1),
                t0 + chrono::Duration::seconds(1),
            )
            .unwrap();
        assert_eq!(only_first.len(), 1);
        assert_eq!(only_first[0].volume, 100);
    }

    fn instrument(token: u64, symbol: &str) -> core_types::Instrument {
        core_types::Instrument {
            tradingsymbol: symbol.to_string(),
            instrument_token: token,
            exchange: core_types::Exchange::Nse,
            lot_size: 1,
            tick_size: 0.05,
        }
    }

    #[test]
    fn upsert_instruments_does_not_wipe_a_previously_set_sector() {
        let db = SqliteStorage::open_in_memory().unwrap();
        db.upsert_instruments(&[instrument(1, "INFY")]).unwrap();
        db.upsert_sectors(&[("INFY".to_string(), "IT".to_string())]).unwrap();

        // Re-upsert the same instrument (as every backtest/live/collect
        // run does on startup) — sector must survive.
        db.upsert_instruments(&[instrument(1, "INFY")]).unwrap();

        let sectors = db.distinct_sectors().unwrap();
        assert_eq!(sectors, vec![("IT".to_string(), 1)]);

        let tagged = db.instruments_by_sector(Some("IT")).unwrap();
        assert_eq!(tagged.len(), 1);
        assert_eq!(tagged[0].tradingsymbol, "INFY");
    }

    #[test]
    fn instrument_by_symbol_is_case_insensitive() {
        let db = SqliteStorage::open_in_memory().unwrap();
        db.upsert_instruments(&[instrument(1, "INFY")]).unwrap();
        assert!(db.instrument_by_symbol("infy").unwrap().is_some());
        assert!(db.instrument_by_symbol("NOPE").unwrap().is_none());
    }

    #[test]
    fn list_runs_and_candle_coverage_round_trip() {
        let db = SqliteStorage::open_in_memory().unwrap();
        let from_date = chrono::NaiveDate::from_ymd_opt(2026, 6, 1).unwrap();
        let to_date = chrono::NaiveDate::from_ymd_opt(2026, 9, 1).unwrap();
        let run_id = db
            .start_run(&core_types::NewRun {
                mode: RunMode::Backtest,
                config_snapshot_json: "{}".to_string(),
                label: Some("coverage-test".to_string()),
                from_date: Some(from_date),
                to_date: Some(to_date),
            })
            .unwrap();

        let runs = db.list_runs().unwrap();
        assert_eq!(runs.len(), 1);
        assert_eq!(runs[0].id, run_id);
        assert_eq!(runs[0].from_date.as_deref(), Some("2026-06-01"));
        assert_eq!(runs[0].to_date.as_deref(), Some("2026-09-01"));

        assert!(db.candle_coverage(42).unwrap().is_none());
        let mut c = candle(1.0, 1.0, 1.0, 1.0, 10);
        c.ts = chrono::Utc::now();
        db.insert_candles(42, CandleInterval::OneMinute, &[c]).unwrap();
        let (_, _, count) = db.candle_coverage(42).unwrap().unwrap();
        assert_eq!(count, 1);
    }
}
