'use strict';

const API = '/api';

// ------------------------------------------------------------- utils ---

function $(sel, root = document) { return root.querySelector(sel); }
function $all(sel, root = document) { return [...root.querySelectorAll(sel)]; }

async function api(path, opts) {
  const res = await fetch(API + path, opts);
  if (!res.ok) {
    let msg = res.statusText;
    try {
      const j = await res.json();
      if (j && j.error) msg = j.error;
    } catch { /* body wasn't JSON */ }
    throw new Error(msg);
  }
  return res.json();
}

function esc(s) {
  const d = document.createElement('div');
  d.textContent = s ?? '';
  return d.innerHTML;
}

function fmtDate(s) {
  return s ? s.slice(0, 10) : '—';
}

function fmtNum(n, digits = 2) {
  return (n === null || n === undefined || Number.isNaN(n)) ? '—' : Number(n).toFixed(digits);
}

function fmtPct1(fraction) {
  return (fraction === null || fraction === undefined) ? '—' : (Number(fraction) * 100).toFixed(1) + '%';
}

function pnlClass(n) {
  if (n === null || n === undefined) return '';
  return n > 0 ? 'pos' : (n < 0 ? 'neg' : '');
}

function strategyBadge(id) {
  return `<span class="badge strategy-${esc(id)}">${esc(id)}</span>`;
}

function runLabel(r) {
  const id = r.run_id ?? r.id;
  const label = r.run_label ?? r.label;
  return label ? `#${id} ${esc(label)}` : `#${id}`;
}

function intVal(el) {
  const v = parseInt(el.value, 10);
  return Number.isFinite(v) ? v : 0;
}
function floatVal(el) {
  const v = parseFloat(el.value);
  return Number.isFinite(v) ? v : 0;
}

// -------------------------------------------------------------- tabs ---

function switchTab(name) {
  $all('.tab').forEach(b => b.classList.toggle('active', b.dataset.tab === name));
  $all('.panel').forEach(p => p.classList.toggle('active', p.id === 'panel-' + name));
  if (name === 'overview') loadOverview();
  if (name === 'trigger') loadTriggerDefaults();
  if (name === 'runs') loadRuns();
  if (name === 'compare') loadComparePicker();
}

function initTabs() {
  $all('.tab').forEach(btn => btn.addEventListener('click', () => switchTab(btn.dataset.tab)));
  $all('[data-tab-link]').forEach(btn => btn.addEventListener('click', () => switchTab(btn.dataset.tabLink)));
}

// ---------------------------------------------------------- coverage ---

async function loadCoveragePill() {
  try {
    const cov = await api('/coverage');
    $('#coverage-pill').textContent = cov.instrument_count
      ? `data: ${fmtDate(cov.from)} → ${fmtDate(cov.to)} · ${cov.instrument_count} stocks`
      : 'no candle data cached yet';
  } catch {
    $('#coverage-pill').textContent = 'coverage unavailable';
  }
}

// --------------------------------------------------------- symbol list ---

let cachedStocks = null;

/** Fetches /universe/stocks once and caches it; populates every datalist
 *  on the page that wants a symbol autocomplete. */
async function ensureStocks() {
  if (cachedStocks) return cachedStocks;
  try {
    cachedStocks = await api('/universe/stocks');
    const opts = cachedStocks
      .slice()
      .sort((a, b) => a.symbol.localeCompare(b.symbol))
      .map(s => `<option value="${esc(s.symbol)}">`)
      .join('');
    $all('datalist[id^="symbol-list"]').forEach(dl => { dl.innerHTML = opts; });
  } catch (e) {
    console.error('failed to load stock list', e);
    cachedStocks = [];
  }
  return cachedStocks;
}

// ----------------------------------------------------------- overview ---

async function loadOverview() {
  try {
    const cov = await api('/coverage');
    $('#overview-coverage').innerHTML = cov.instrument_count
      ? `${fmtDate(cov.from)} → ${fmtDate(cov.to)}<span class="sub">${cov.instrument_count} instruments cached</span>`
      : '<span class="muted">No candle data cached yet — run a backfill or trigger a backtest.</span>';
  } catch (e) {
    $('#overview-coverage').innerHTML = `<span class="error-state">${esc(e.message)}</span>`;
  }

  try {
    const runs = await api('/runs');
    $('#overview-run-count').innerHTML = `${runs.length}<span class="sub">total runs</span>`;
    renderRunsTable(runs.slice(0, 8), $('#overview-recent-runs'), { clickable: false });
  } catch (e) {
    $('#overview-recent-runs').innerHTML = `<div class="error-state">${esc(e.message)}</div>`;
  }

  try {
    const perf = await api('/performance');
    const withTrades = perf.filter(r => r.trades >= 5);
    withTrades.sort((a, b) => (b.avg_r ?? -Infinity) - (a.avg_r ?? -Infinity));
    renderBestCombos(withTrades.slice(0, 15));
  } catch (e) {
    $('#overview-best-combos').innerHTML = `<div class="error-state">${esc(e.message)}</div>`;
  }
}

function renderBestCombos(rows) {
  const container = $('#overview-best-combos');
  if (!rows.length) {
    container.innerHTML = '<div class="empty-state">No closed trades yet (or too few per combo — needs 5+). Trigger a backtest first.</div>';
    return;
  }
  container.innerHTML = `<table><thead><tr>
      <th>Strategy</th><th>Symbol</th><th>Run</th>
      <th class="num">Trades</th><th class="num">Avg PnL%</th><th class="num">Avg R</th><th class="num">Win rate</th>
    </tr></thead><tbody>${rows.map(r => `
      <tr>
        <td>${strategyBadge(r.strategy_id)}</td>
        <td class="mono">${esc(r.tradingsymbol ?? ('#' + r.instrument_token))}</td>
        <td class="mono small muted">${runLabel(r)}</td>
        <td class="num">${r.trades}</td>
        <td class="num ${pnlClass(r.avg_pnl_pct)}">${fmtNum(r.avg_pnl_pct)}%</td>
        <td class="num ${pnlClass(r.avg_r)}">${fmtNum(r.avg_r)}</td>
        <td class="num">${fmtPct1(r.win_rate)}</td>
      </tr>`).join('')}</tbody></table>`;
}

// --------------------------------------------------------------- runs ---

function renderRunsTable(runs, container, opts = {}) {
  if (!runs.length) {
    container.innerHTML = '<div class="empty-state">No runs yet.</div>';
    return;
  }
  container.innerHTML = `<table><thead><tr>
      <th>Run</th><th>Label</th><th>Mode</th><th>Coverage</th><th>Started</th><th>Status</th>
    </tr></thead><tbody>${runs.map(r => `
      <tr class="${opts.clickable === false ? '' : 'selectable'}" data-run-id="${r.id}">
        <td class="mono">#${r.id}</td>
        <td>${esc(r.label ?? '—')}</td>
        <td><span class="badge">${esc(r.mode)}</span></td>
        <td class="mono small">${fmtDate(r.from_date)} → ${fmtDate(r.to_date)}</td>
        <td class="mono small muted">${esc((r.started_at || '').slice(0, 16).replace('T', ' '))}</td>
        <td>${r.ended_at ? '<span class="badge">done</span>' : '<span class="badge">running…</span>'}</td>
      </tr>`).join('')}</tbody></table>`;
  if (opts.clickable !== false) {
    $all('tr.selectable', container).forEach(tr => {
      tr.addEventListener('click', () => showRunDetail(Number(tr.dataset.runId)));
    });
  }
}

async function loadRuns() {
  try {
    renderRunsTable(await api('/runs'), $('#runs-table'));
  } catch (e) {
    $('#runs-table').innerHTML = `<div class="error-state">${esc(e.message)}</div>`;
  }
}

let currentRunDetailId = null;

async function showRunDetail(runId) {
  currentRunDetailId = runId;
  $('#run-detail-card').hidden = false;
  $('#run-detail-title').textContent = `Run #${runId}`;
  $('#run-detail-card').scrollIntoView({ behavior: 'smooth', block: 'nearest' });
  await refreshRunDetail();
}

async function refreshRunDetail() {
  if (currentRunDetailId === null) return;
  const byInstrument = $('#run-detail-by-instrument').checked;
  try {
    const rows = byInstrument
      ? await api(`/runs/${currentRunDetailId}/detail`)
      : await api(`/runs/${currentRunDetailId}/summary`);
    renderPerformanceTable(rows, $('#run-detail-table'), byInstrument ? 'by_instrument' : 'summary');
  } catch (e) {
    $('#run-detail-table').innerHTML = `<div class="error-state">${esc(e.message)}</div>`;
  }
}

/** mode: 'summary' (strategy rollup) | 'by_instrument' (+ window + symbol)
 *  | 'by_run' (+ window + run, one stock across runs)
 *  | 'compare' (StrategySummaryRow: strategy + run + mode, no window —
 *    grouped with a blank row between strategies, same as the CLI). */
function renderPerformanceTable(rows, container, mode) {
  if (!rows.length) {
    container.innerHTML = '<div class="empty-state">No closed trades for this filter.</div>';
    return;
  }
  let head, body;
  if (mode === 'compare') {
    head = '<th>Strategy</th><th>Run</th><th>Mode</th><th class="num">Trades</th><th class="num">Avg PnL%</th><th class="num">Avg R</th><th class="num">Win rate</th>';
    let lastStrategy = null;
    body = rows.map(r => {
      const spacer = lastStrategy !== null && lastStrategy !== r.strategy_id
        ? '<tr><td colspan="7" style="border:none;height:8px;padding:0"></td></tr>' : '';
      lastStrategy = r.strategy_id;
      return `${spacer}<tr>
        <td>${strategyBadge(r.strategy_id)}</td>
        <td class="mono small">${runLabel(r)}</td>
        <td><span class="badge">${esc(r.mode)}</span></td>
        <td class="num">${r.trades}</td>
        <td class="num ${pnlClass(r.avg_pnl_pct)}">${fmtNum(r.avg_pnl_pct)}%</td>
        <td class="num ${pnlClass(r.avg_r)}">${fmtNum(r.avg_r)}</td>
        <td class="num">${fmtPct1(r.win_rate)}</td>
      </tr>`;
    }).join('');
  } else if (mode === 'by_instrument') {
    head = '<th>Strategy</th><th>Window</th><th>Symbol</th><th class="num">Trades</th><th class="num">Avg PnL%</th><th class="num">Avg R</th><th class="num">Win rate</th>';
    body = rows.map(r => `
      <tr>
        <td>${strategyBadge(r.strategy_id)}</td>
        <td class="small muted">${esc(r.window)}</td>
        <td class="mono">${esc(r.tradingsymbol ?? ('#' + r.instrument_token))}</td>
        <td class="num">${r.trades}</td>
        <td class="num ${pnlClass(r.avg_pnl_pct)}">${fmtNum(r.avg_pnl_pct)}%</td>
        <td class="num ${pnlClass(r.avg_r)}">${fmtNum(r.avg_r)}</td>
        <td class="num">${fmtPct1(r.win_rate)}</td>
      </tr>`).join('');
  } else if (mode === 'by_run') {
    head = '<th>Strategy</th><th>Window</th><th>Run</th><th class="num">Trades</th><th class="num">Avg PnL%</th><th class="num">Avg R</th><th class="num">Win rate</th>';
    body = rows.map(r => `
      <tr>
        <td>${strategyBadge(r.strategy_id)}</td>
        <td class="small muted">${esc(r.window)}</td>
        <td class="mono small">${runLabel(r)}</td>
        <td class="num">${r.trades}</td>
        <td class="num ${pnlClass(r.avg_pnl_pct)}">${fmtNum(r.avg_pnl_pct)}%</td>
        <td class="num ${pnlClass(r.avg_r)}">${fmtNum(r.avg_r)}</td>
        <td class="num">${fmtPct1(r.win_rate)}</td>
      </tr>`).join('');
  } else {
    head = '<th>Strategy</th><th class="num">Trades</th><th class="num">Avg PnL%</th><th class="num">Avg R</th><th class="num">Win rate</th>';
    body = rows.map(r => `
      <tr>
        <td>${strategyBadge(r.strategy_id)}</td>
        <td class="num">${r.trades}</td>
        <td class="num ${pnlClass(r.avg_pnl_pct)}">${fmtNum(r.avg_pnl_pct)}%</td>
        <td class="num ${pnlClass(r.avg_r)}">${fmtNum(r.avg_r)}</td>
        <td class="num">${fmtPct1(r.win_rate)}</td>
      </tr>`).join('');
  }
  container.innerHTML = `<table><thead><tr>${head}</tr></thead><tbody>${body}</tbody></table>`;
}

// ------------------------------------------------------------ compare ---

const compareSelected = new Set();

async function loadComparePicker() {
  try {
    const runs = await api('/runs');
    $('#compare-picker').innerHTML = `<table><thead><tr>
        <th></th><th>Run</th><th>Label</th><th>Mode</th><th>Coverage</th>
      </tr></thead><tbody>${runs.map(r => `
        <tr>
          <td><input type="checkbox" class="compare-check" data-run-id="${r.id}" ${compareSelected.has(r.id) ? 'checked' : ''}/></td>
          <td class="mono">#${r.id}</td>
          <td>${esc(r.label ?? '—')}</td>
          <td><span class="badge">${esc(r.mode)}</span></td>
          <td class="mono small">${fmtDate(r.from_date)} → ${fmtDate(r.to_date)}</td>
        </tr>`).join('')}</tbody></table>`;
    $all('.compare-check').forEach(cb => cb.addEventListener('change', () => {
      const id = Number(cb.dataset.runId);
      if (cb.checked) compareSelected.add(id); else compareSelected.delete(id);
    }));
  } catch (e) {
    $('#compare-picker').innerHTML = `<div class="error-state">${esc(e.message)}</div>`;
  }
}

async function runCompare() {
  const ids = [...compareSelected];
  const qs = ids.length ? `?runs=${ids.join(',')}` : '';
  $('#compare-result-card').hidden = false;
  try {
    renderPerformanceTable(await api(`/compare${qs}`), $('#compare-result'), 'compare');
  } catch (e) {
    $('#compare-result').innerHTML = `<div class="error-state">${esc(e.message)}</div>`;
  }
}

// ------------------------------------------------------------ trigger ---

async function loadSectors() {
  try {
    const sectors = await api('/universe/sectors');
    $('#scope-sector').innerHTML = sectors.map(s => `<option value="${esc(s.sector)}">${esc(s.sector)} (${s.count})</option>`).join('');
  } catch (e) {
    $('#scope-sector').innerHTML = `<option value="">failed to load sectors</option>`;
  }
}

function updateScopeVisibility() {
  const kind = $('#scope-kind').value;
  $('#scope-sector-wrap').hidden = kind !== 'sector';
  $('#scope-symbol-wrap').hidden = kind !== 'symbol';
  if (kind === 'sector' && $('#scope-sector').options.length === 0) loadSectors();
  if (kind === 'symbol') ensureStocks();
}

function updateTrailParamVisibility() {
  const policy = $('#trail-policy').value;
  $('#trail-param-wrap').hidden = policy === 'vwap_cross';
  const hint = $('#trail-param-hint');
  if (hint) hint.textContent = policy === 'atr_multiple' ? '(multiplier, e.g. 2.0)' : '(% from peak, e.g. 0.15)';
}

function fillStrategyDefaults(prefix, p) {
  const form = $('#trigger-form');
  form[`${prefix}_enabled`].checked = p.enabled;
  for (const field of ['relative_volume_period', 'min_relative_volume', 'fast_ema', 'slow_ema', 'atr_period', 'min_atr_multiple']) {
    const el = form[`${prefix}_${field}`];
    if (el) el.value = p[field];
  }
}

function fillRiskDefaults(r) {
  const form = $('#trigger-form');
  form.risk_pct_display.value = (r.risk_pct * 100).toFixed(4);
  form.min_reward_multiple.value = r.min_reward_multiple;
  form.trail_policy.value = r.trail_policy;
  form.trail_param_display.value = r.trail_policy === 'atr_multiple' ? r.trail_param : (r.trail_param * 100).toFixed(4);
  form.capital_per_trade.value = r.capital_per_trade;
  form.max_concurrent_positions.value = r.max_concurrent_positions;
  form.daily_loss_cap_pct_display.value = (r.daily_loss_cap_pct * 100).toFixed(2);
  form.eod_square_off.value = r.eod_square_off;
  form.max_hold_minutes.value = r.max_hold_minutes;
  form.exit_atr_period.value = r.exit_atr_period;
  updateTrailParamVisibility();
}

let defaultsLoaded = false;

async function loadTriggerDefaults() {
  if (defaultsLoaded) return;
  defaultsLoaded = true;
  const form = $('#trigger-form');

  const today = new Date();
  form.to.value = today.toISOString().slice(0, 10);
  const from = new Date(today);
  from.setDate(from.getDate() - 90);
  form.from.value = from.toISOString().slice(0, 10);

  try {
    const d = await api('/config/defaults');
    fillStrategyDefaults('orb', d.strategies.orb);
    fillStrategyDefaults('vwap_trend', d.strategies.vwap_trend);
    fillStrategyDefaults('momentum_breakout', d.strategies.momentum_breakout);
    fillRiskDefaults(d.risk);
  } catch (e) {
    $('#trigger-hint').textContent = 'Could not load config/default.toml defaults: ' + e.message;
  }
  updateScopeVisibility();
}

function collectStrategyPayload(prefix, form) {
  return {
    enabled: form[`${prefix}_enabled`].checked,
    relative_volume_period: form[`${prefix}_relative_volume_period`] ? intVal(form[`${prefix}_relative_volume_period`]) : 0,
    min_relative_volume: form[`${prefix}_min_relative_volume`] ? floatVal(form[`${prefix}_min_relative_volume`]) : 0,
    fast_ema: form[`${prefix}_fast_ema`] ? intVal(form[`${prefix}_fast_ema`]) : 0,
    slow_ema: form[`${prefix}_slow_ema`] ? intVal(form[`${prefix}_slow_ema`]) : 0,
    atr_period: form[`${prefix}_atr_period`] ? intVal(form[`${prefix}_atr_period`]) : 0,
    min_atr_multiple: form[`${prefix}_min_atr_multiple`] ? floatVal(form[`${prefix}_min_atr_multiple`]) : 0,
  };
}

async function submitTrigger(ev) {
  ev.preventDefault();
  const form = ev.target;
  const submitBtn = $('#trigger-submit');
  submitBtn.disabled = true;

  try {
    const scopeKind = form.scope_kind.value;
    const scope = {
      kind: scopeKind,
      value: scopeKind === 'sector' ? form.scope_sector.value
        : scopeKind === 'symbol' ? form.scope_symbol.value.trim().toUpperCase()
        : null,
    };

    const trailPolicy = form.trail_policy.value;
    const trailParamRaw = floatVal(form.trail_param_display);
    const trailParam = trailPolicy === 'atr_multiple' ? trailParamRaw : trailParamRaw / 100;

    const payload = {
      label: form.label.value.trim() || null,
      from: form.from.value,
      to: form.to.value,
      scope,
      strategies: {
        orb: collectStrategyPayload('orb', form),
        vwap_trend: collectStrategyPayload('vwap_trend', form),
        momentum_breakout: collectStrategyPayload('momentum_breakout', form),
      },
      risk: {
        risk_pct: floatVal(form.risk_pct_display) / 100,
        min_reward_multiple: floatVal(form.min_reward_multiple),
        trail_policy: trailPolicy,
        trail_param: trailParam,
        capital_per_trade: floatVal(form.capital_per_trade),
        max_concurrent_positions: intVal(form.max_concurrent_positions),
        daily_loss_cap_pct: floatVal(form.daily_loss_cap_pct_display) / 100,
        eod_square_off: form.eod_square_off.value,
        max_hold_minutes: intVal(form.max_hold_minutes),
        exit_atr_period: intVal(form.exit_atr_period),
      },
    };

    const res = await api('/backtest', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(payload),
    });

    $('#job-progress-card').hidden = false;
    $('#progress-fill').style.width = '2%';
    $('#progress-message').textContent = `Job started — ${res.instrument_count} instruments queued.`;
    $('#job-result').innerHTML = '';
    pollJob(res.job_id);
  } catch (e) {
    $('#job-progress-card').hidden = false;
    $('#progress-message').innerHTML = `<span class="error-state">${esc(e.message)}</span>`;
  } finally {
    submitBtn.disabled = false;
  }
}

let pollTimer = null;

function pollJob(jobId) {
  if (pollTimer) clearInterval(pollTimer);
  pollTimer = setInterval(async () => {
    try {
      const status = await api(`/backtest/${jobId}`);
      if (status.status === 'running') {
        const p = status.progress;
        if (p) {
          const pct = p.total ? Math.round((p.current / p.total) * 100) : 5;
          $('#progress-fill').style.width = Math.max(pct, 3) + '%';
          $('#progress-message').textContent = `[${p.current}/${p.total}] ${p.message}`;
        }
      } else if (status.status === 'done') {
        clearInterval(pollTimer);
        $('#progress-fill').style.width = '100%';
        $('#progress-message').textContent = 'Complete.';
        $('#job-result').innerHTML = `<p style="margin:10px 0 0">Run <strong>#${status.run_id}</strong> complete. <button type="button" class="btn btn-ghost" id="job-view-result">View report →</button></p>`;
        $('#job-view-result').addEventListener('click', () => {
          switchTab('runs');
          showRunDetail(status.run_id);
        });
        loadRuns().catch(() => {});
      } else if (status.status === 'failed') {
        clearInterval(pollTimer);
        $('#progress-message').innerHTML = `<span class="error-state">Failed: ${esc(status.error)}</span>`;
      }
    } catch (e) {
      clearInterval(pollTimer);
      $('#progress-message').innerHTML = `<span class="error-state">${esc(e.message)}</span>`;
    }
  }, 1200);
}

// ------------------------------------------------------- stock explorer ---

function renderStockChart(rows) {
  const container = $('#stock-chart');
  if (!rows.length) {
    container.innerHTML = '';
    return;
  }
  const maxAbs = Math.max(...rows.map(r => Math.abs(r.avg_r || 0)), 0.1);
  container.innerHTML = `<div class="bar-chart">${rows.map(r => {
    const val = r.avg_r || 0;
    const widthPct = Math.min((Math.abs(val) / maxAbs) * 50, 50);
    const cls = val >= 0 ? 'pos' : 'neg';
    return `<div class="bar-row">
        <div class="label">${strategyBadge(r.strategy_id)} <span class="small muted">${runLabel(r)}</span></div>
        <div class="bar-track"><div class="bar-zero"></div><div class="bar-fill ${cls}" style="width:${widthPct}%"></div></div>
        <div class="bar-value ${cls}">${fmtNum(val)}R</div>
      </div>`;
  }).join('')}</div>`;
}

async function showStock(symbol) {
  if (!symbol) return;
  symbol = symbol.trim().toUpperCase();
  $('#stock-title').textContent = symbol;
  $('#stock-coverage-card').hidden = false;
  $('#stock-result-card').hidden = false;
  $('#stock-coverage').textContent = 'loading…';
  $('#stock-result-table').innerHTML = '';
  $('#stock-chart').innerHTML = '';

  try {
    const stocks = await ensureStocks();
    const match = stocks.find(s => s.symbol.toUpperCase() === symbol);
    $('#stock-coverage').innerHTML = match && match.candle_count
      ? `${fmtDate(match.data_from)} → ${fmtDate(match.data_to)}<span class="sub">${match.candle_count.toLocaleString()} candles · ${esc(match.sector || 'unsectored')}</span>`
      : '<span class="muted">No cached candle data for this stock yet — trigger a backtest for it first.</span>';
  } catch (e) {
    $('#stock-coverage').innerHTML = `<span class="error-state">${esc(e.message)}</span>`;
  }

  try {
    const rows = await api(`/compare/stock?symbol=${encodeURIComponent(symbol)}`);
    renderPerformanceTable(rows, $('#stock-result-table'), 'by_run');
    renderStockChart(rows);
  } catch (e) {
    $('#stock-result-table').innerHTML = `<div class="error-state">${esc(e.message)}</div>`;
  }
}

// -------------------------------------------------------------- boot ---

document.addEventListener('DOMContentLoaded', () => {
  initTabs();
  loadCoveragePill();
  ensureStocks();
  loadOverview();

  $('#scope-kind').addEventListener('change', updateScopeVisibility);
  $('#trail-policy').addEventListener('change', updateTrailParamVisibility);
  $('#trigger-form').addEventListener('submit', submitTrigger);
  $('#runs-refresh').addEventListener('click', loadRuns);
  $('#run-detail-by-instrument').addEventListener('change', refreshRunDetail);
  $('#compare-go').addEventListener('click', runCompare);
  $('#stock-go').addEventListener('click', () => showStock($('#stock-search').value));
  $('#stock-search').addEventListener('keydown', (e) => {
    if (e.key === 'Enter') {
      e.preventDefault();
      showStock(e.target.value);
    }
  });
});
