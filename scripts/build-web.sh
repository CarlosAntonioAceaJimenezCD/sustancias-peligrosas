#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

rustup target add wasm32-unknown-unknown >/dev/null 2>&1 || true
cargo build -p sustancias-web --target wasm32-unknown-unknown --release

echo "WASM compilado en target/wasm32-unknown-unknown/release/"
echo "Para generar el sitio completo: dx build --release"
