#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

for var in ANDROID_HOME NDK_HOME JAVA_HOME; do
  if [ -z "${!var:-}" ]; then
    echo "Falta la variable de entorno $var (Android Studio / SDK / NDK / JDK)" >&2
    exit 1
  fi
done

if ! command -v cargo-ndk >/dev/null 2>&1 && ! cargo ndk --version >/dev/null 2>&1; then
  echo "Falta cargo-ndk: cargo install cargo-ndk" >&2
  exit 1
fi

rustup target add aarch64-linux-android armv7-linux-androideabi \
  i686-linux-android x86_64-linux-android

cargo ndk -t arm64-v8a -t armeabi-v7a \
  -o ./target/android/jniLibs \
  build -p sustancias-web --features mobile --release

echo "Librerías nativas en target/android/jniLibs"
echo "Para generar el APK: dx build --platform android --release"
