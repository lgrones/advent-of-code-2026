#!/usr/bin/env bash
set -euo pipefail

if [[ ! "${1:-}" =~ ^[0-9]+$ ]]; then
	echo "Usage: ./create.sh 12" >&2
	exit 1
fi

cd "$(dirname "$0")"

# 10# forces base 10, otherwise 08 and 09 are invalid octal.
day="$(printf "day%02d" "$((10#$1))")"
bin="src/bin/$day.rs"

mkdir -p src/bin inputs

if [[ -e "$bin" ]]; then
	echo "$bin already exists, keeping it"
else
	cp template.rs "$bin"
fi

touch "inputs/$day.txt"
echo "Paste your puzzle input into inputs/$day.txt"
