#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

if ! pkg-config --exists webkit2gtk-4.1; then
  echo "Falta webkit2gtk-4.1 (libwebkit2gtk-4.1-dev / webkit2gtk-4.1)" >&2
  exit 1
fi

if ! ldconfig -p 2>/dev/null | grep -q 'libxdo\.so'; then
  echo "Falta libxdo.so: instala el paquete xdotool (o libxdo-dev)" >&2
  exit 1
fi

cargo build -p sustancias-web --features desktop --release
echo "Binario en target/release/sustancias-web"
