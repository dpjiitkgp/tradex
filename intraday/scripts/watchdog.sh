#!/usr/bin/env bash
# Notices when a trading process has died, and says so on Telegram.
#
# Exists because every alert the system sends is sent BY the process that
# would be failing: a crash produces exactly zero notifications, and the gap
# looks identical to a quiet day where no signal fired. This runs separately,
# on a timer, and reports absence.
#
# What it does NOT cover: this machine going to sleep or losing power, since
# launchd sleeps with it — the same monotonic-clock problem that silently
# killed the collector overnight on 2026-09-30. Only an off-machine
# dead-man's switch covers that (see config/alerts.example.toml).
#
# Deliberately alert-only for the live process. Auto-restarting something that
# places real orders risks a double-start or a restart into a state nobody has
# looked at; a human decides. The collector is read-only, so it is restarted
# automatically.
#
# Nothing here authenticates to Kite. That is a hard requirement, not a
# detail: a login rotates the cached access token, and the running live
# process holds its own from startup — so a watchdog that logged in every
# minute would break the thing it is watching. `calendar today` and `notify`
# are both auth-free; do not add a command here that is not.
set -uo pipefail

cd "$(dirname "$0")/.." || exit 1
BIN="./target/debug/papertrader"
STATE_DIR="data/watchdog"
mkdir -p "$STATE_DIR"

# Only complain while the market is open. Outside those hours the processes are
# *supposed* to be gone, and a watchdog that cries every night is one that gets
# switched off.
#
# Ends at 15:20 rather than the 15:30 close: the live loop exits at 15:18 by
# design, so checking past that would report a clean shutdown as a failure.
# `10#` forces base 10: bash reads a bare `09` as an invalid octal literal and
# aborts under `set -u`, which would make the watchdog die silently every day
# between 08:00 and 09:59 — exactly the hour before the open.
now_h=$(TZ=Asia/Kolkata date +%H)
now_m=$(TZ=Asia/Kolkata date +%M)
now_min=$((10#$now_h * 60 + 10#$now_m))
open_min=$((9 * 60 + 15))
close_min=$((15 * 60 + 20))
if [[ "$now_min" -lt "$open_min" || "$now_min" -gt "$close_min" ]]; then
  exit 0
fi

# Holidays and weekends come from the same NSE calendar the engine uses, so the
# watchdog and the trading processes can never disagree about whether today is
# a session.
if ! "$BIN" calendar today 2>/dev/null | grep -q "is a trading day"; then
  exit 0
fi

# One alert per outage, not one per minute. The marker is cleared when the
# process comes back, so a second outage later in the day still reports.
alert_once() {
  local key="$1" message="$2"
  local marker="$STATE_DIR/$key.alerted"
  if [[ -f "$marker" ]]; then
    return
  fi
  touch "$marker"
  "$BIN" notify --title "WATCHDOG" "$message" >/dev/null 2>&1
  logger -t papertrader-watchdog "$message"
}

clear_alert() {
  rm -f "$STATE_DIR/$1.alerted"
}

# --- the live trading process: alert, never auto-restart ------------------
if pgrep -f "papertrader live-trade run --live" >/dev/null; then
  clear_alert live
else
  alert_once live \
    "LIVE PROCESS IS GONE during market hours. No trades will be taken and any open position is unmanaged. Check Kite, then restart it."
fi

# --- the collector: read-only, so bring it back by itself ----------------
if pgrep -f "papertrader collect" >/dev/null; then
  clear_alert collector
else
  LOG="data/collect_$(TZ=Asia/Kolkata date +%Y-%m-%d).log"
  nohup "$BIN" collect >>"$LOG" 2>&1 &
  disown
  alert_once collector \
    "Collector was down during market hours — restarted automatically. Research candles for the gap will need a backfill."
fi
