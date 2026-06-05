#!/bin/sh
set -eu

file="${BLOCKED_IDS_FILE:-/home/inforiaadmin/rustdesk/blocked_ids.txt}"
id="$(printf '%s' "${1:-}" | tr -d '[:space:]-')"

case "$id" in
  ''|*[!0-9]*)
    echo "Usage: $0 NUMERICAL_ID" >&2
    exit 1
    ;;
esac

if [ ! -f "$file" ]; then
  echo "The blocklist does not exist; ID $id is not blocked."
  exit 0
fi

tmp="${file}.tmp.$$"
trap 'rm -f "$tmp"' EXIT
grep -vx "$id" "$file" > "$tmp" || true
cat "$tmp" > "$file"
echo "Unblocked ID $id. hbbs will reload the file automatically."
