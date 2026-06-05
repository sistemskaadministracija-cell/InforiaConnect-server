#!/bin/sh
set -eu

file="${BLOCKED_IDS_FILE:-/home/inforiaadmin/rustdesk/blocked_ids.txt}"
if [ ! -s "$file" ]; then
  echo "No IDs are currently blocked."
  exit 0
fi

echo "Blocked InforiaConnect IDs:"
sed '/^[[:space:]]*#/d; /^[[:space:]]*$/d' "$file"
