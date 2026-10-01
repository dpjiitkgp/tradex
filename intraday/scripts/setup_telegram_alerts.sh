#!/usr/bin/env bash
# Wires Telegram up as the alert channel, without the bot token ever leaving
# this machine.
#
# The chat id is the fiddly part: a Telegram bot cannot message you first, so
# the id only exists once YOU have messaged IT. This script waits for that to
# happen, reads the id back, and writes config/alerts.toml.
#
# Usage:  scripts/setup_telegram_alerts.sh <bot_token>
#
# Getting a token: message @BotFather in Telegram, send /newbot, follow the
# prompts. It replies with something like 123456789:AAH...
set -euo pipefail

cd "$(dirname "$0")/.."
CONFIG="config/alerts.toml"
TOKEN="${1:-}"

if [[ -z "$TOKEN" ]]; then
  echo "usage: $0 <bot_token>" >&2
  echo "get one from @BotFather in Telegram: send /newbot" >&2
  exit 1
fi

# Deliberately not `curl -f`: a bad token answers 404 with a JSON body, and
# under `set -e` a failing curl would abort before the friendlier check below
# ever runs. Transient network errors are swallowed the same way, so a blip
# during the wait loop does not end the whole setup.
api() { curl -sS --max-time 15 "https://api.telegram.org/bot${TOKEN}/$1" 2>/dev/null || true; }

echo "Checking the token..."
BOT_NAME=$(api getMe | sed -n 's/.*"username":"\([^"]*\)".*/\1/p')
if [[ -z "$BOT_NAME" ]]; then
  echo "That token was rejected by Telegram. Check it and try again." >&2
  exit 1
fi
echo "  bot is @${BOT_NAME}"

# getUpdates only returns messages the bot can see, so this doubles as the
# check that the chat actually exists from the bot's side.
echo
echo "Where should alerts go?"
echo
echo "  PRIVATE CHAT — open Telegram, find @${BOT_NAME}, send it any message."
echo
echo "  GROUP        — add @${BOT_NAME} to the group, then send  /start@${BOT_NAME}"
echo "                 in the group. It must be a command addressed to the bot:"
echo "                 bots run with privacy mode ON and cannot see ordinary"
echo "                 group messages, so a plain \"hi\" will not register."
echo
echo -n "Waiting"

CHAT_IDS=""
for _ in $(seq 1 60); do
  # Every distinct chat the bot has seen, newest last. Collected in full
  # rather than taking the first match, so a bot that has been messaged
  # privately AND added to a group does not silently pick the wrong one.
  CHAT_IDS=$(api getUpdates \
    | grep -o '"chat":{"id":-\{0,1\}[0-9]*' \
    | grep -o '\-\{0,1\}[0-9]*$' \
    | awk '!seen[$0]++')
  [[ -n "$CHAT_IDS" ]] && break
  echo -n "."
  sleep 2
done
echo

if [[ -z "$CHAT_IDS" ]]; then
  echo "Nothing arrived in two minutes." >&2
  echo "For a group, check you sent /start@${BOT_NAME} and not a plain message." >&2
  exit 1
fi

COUNT=$(echo "$CHAT_IDS" | wc -l | tr -d ' ')
if [[ "$COUNT" -eq 1 ]]; then
  CHAT_ID="$CHAT_IDS"
else
  echo "This bot has seen more than one chat. Pick where alerts should go:"
  i=1
  while read -r id; do
    # Negative ids are groups, supergroups and channels; positive are the
    # one-to-one chat with a person.
    if [[ "$id" == -* ]]; then KIND="group/channel"; else KIND="private chat"; fi
    echo "  $i) $id  ($KIND)"
    i=$((i + 1))
  done <<< "$CHAT_IDS"
  echo -n "number: "
  read -r PICK
  CHAT_ID=$(echo "$CHAT_IDS" | sed -n "${PICK}p")
  [[ -z "$CHAT_ID" ]] && { echo "not a valid choice" >&2; exit 1; }
fi

if [[ "$CHAT_ID" == -* ]]; then
  echo "  chat id: ${CHAT_ID} (group/channel)"
  echo
  echo "  NOTE: converting this group to a supergroup later CHANGES its id and"
  echo "        alerts will silently stop. Re-run this script if that happens."
else
  echo "  chat id: ${CHAT_ID} (private chat)"
fi

if [[ -f "$CONFIG" ]]; then
  BACKUP="${CONFIG}.bak-$(date +%Y%m%d%H%M%S)"
  cp "$CONFIG" "$BACKUP"
  echo "  existing config backed up to ${BACKUP}"
fi

# Preserve an existing heartbeat_url rather than silently dropping the other
# half of the alerting setup.
HEARTBEAT=""
if [[ -f "$CONFIG" ]]; then
  HEARTBEAT=$(sed -n 's/^heartbeat_url *= *"\(.*\)"/\1/p' "$CONFIG" | head -1)
fi

{
  echo "# Written by scripts/setup_telegram_alerts.sh on $(date +%Y-%m-%d)."
  echo "# The bot token grants read access to these alerts — this file is gitignored."
  echo
  echo "[channel]"
  echo 'kind = "telegram"'
  echo "bot_token = \"${TOKEN}\""
  echo "chat_id = \"${CHAT_ID}\""
  if [[ -n "$HEARTBEAT" ]]; then
    echo
    echo "heartbeat_url = \"${HEARTBEAT}\""
  else
    echo
    echo "# Dead-man's switch still unset — see config/alerts.example.toml."
    echo "# Without it, nothing tells you when this process stops running."
    echo '# heartbeat_url = "https://hc-ping.com/YOUR-CHECK-UUID"'
  fi
} > "$CONFIG"
chmod 600 "$CONFIG"

echo
echo "Wrote ${CONFIG}. Sending a test alert..."
cargo run --quiet --bin papertrader -- live-trade test-alert
