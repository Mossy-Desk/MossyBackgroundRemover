#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[1/6] Frontend install"
cd mossbgr
npm ci

echo "[2/6] Frontend typecheck/lint"
npm run check

echo "[3/6] Frontend build"
npm run build

echo "[4/6] Rust format check"
cd src-tauri
cargo fmt --all -- --check

echo "[5/6] Rust clippy"
cargo clippy --all-targets --all-features -- -D warnings

echo "[6/6] Rust tests"
cargo test --all-features

echo "OK: local pre-push checks passed."
