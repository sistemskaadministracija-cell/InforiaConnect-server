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

if [ "${#id}" -lt 6 ]; then
  echo "ID must contain at least six digits." >&2
  exit 1
fi

mkdir -p "$(dirname "$file")"
touch "$file"
if grep -qx "$id" "$file"; then
  echo "ID $id is already blocked."
  exit 0
fi

printf '%s\n' "$id" >> "$file"
echo "Blocked ID $id. hbbs will reload the file automatically."
