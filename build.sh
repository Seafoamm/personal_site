#!/bin/bash
trap "kill 0" EXIT

echo "🦀 Cargo Watcher Starting..."
cargo watch -w src -w assets/styles.css -x run
