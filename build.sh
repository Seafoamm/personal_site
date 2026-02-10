#!/bin/bash
# Ensures all background processes (Tailwind) die when you Ctrl+C
trap "kill 0" EXIT

ROOT=$(pwd)

npx @tailwindcss/cli -i "$ROOT/input.css" -o "$ROOT/assets/output.css" --watch=always < /dev/null &

echo "🦀 Cargo Watcher Starting..."
# Requires 'cargo install cargo-watch'
# This restarts your Axum server whenever .rs or .html files change
cargo watch -w src -w assets -x run
