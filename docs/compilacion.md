# Compilación por plataforma

Un solo código (Rust + Dioxus) para todas las plataformas. El crate se llama `sustancias-web` por razones históricas, pero es multiplataforma.

## Requisitos comunes

- Rust stable **1.85+** (`rustup`), `cargo`.
- target WebAssembly: `rustup target add wasm32-unknown-unknown`.
- Compilador C (para SQLite incluida en `dbtool`): MSVC en Windows, Xcode CLT en macOS, GCC/Clang en Linux.
- Opcional: `dioxus-cli` (`dx`) para serve/bundle: `cargo install dioxus-cli --locked` o el instalador precompilado de https://dioxus.dev/install.sh.

Features del crate `sustancias-web`:

| Feature | Plataforma |
|---|---|
| (por defecto) | Web / WASM |
| `desktop` | Windows, Linux, macOS |
| `mobile` | Android, iOS |

## Web (ya en GitHub Pages)

```bash
cargo build -p sustancias-web --target wasm32-unknown-unknown --release
# Para generar el sitio completo:
dx build --release
```

## Linux

Dependencias (Arch):

```bash
sudo pacman -S webkit2gtk-4.1 base-devel curl wget file openssl \
  appmenu-gtk-module libappindicator-gtk3 librsvg xdotool
```

`xdotool` aporta `libxdo.so`, necesario en el enlazado (Dioxus desktop usa atajos globales). Sin él, `cargo check` pasa pero `cargo build` falla con `unable to find library -lxdo`.

Debian/Ubuntu:

```bash
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file \
  libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
```

Compilación:

```bash
cargo build -p sustancias-web --features desktop --release
# binario: target/release/sustancias-web
```

Esta máquina ya tiene `webkit2gtk-4.1`, por lo que el build de Linux es viable aquí.

## Windows

Se recomienda **compilar en Windows** (MSVC + WebView2):

```powershell
cargo build -p sustancias-web --features desktop --release
# binario: target\release\sustancias-web.exe
```

Requisitos: Visual Studio Build Tools con carga de C++ (MSVC) y WebView2 Runtime (incluido con Edge en Windows 10/11).

Cross-compilación desde Linux es posible con `mingw-w64` y el target `x86_64-pc-windows-gnu`, pero no está verificada y requiere `x86_64-w64-mingw32-gcc` más las dependencias GTK. No es la vía recomendada.

## Android

1. Targets de Rust:

```bash
rustup target add aarch64-linux-android armv7-linux-androideabi \
  i686-linux-android x86_64-linux-android
```

2. Android Studio → SDK Manager: SDK, Command-line tools, NDK (side by side) y CMake.

3. Variables de entorno (Linux):

```bash
export JAVA_HOME="/ruta/a/android-studio/jbr"
export ANDROID_HOME="$HOME/Android/Sdk"
export NDK_HOME="$ANDROID_HOME/ndk/<version>"
export PATH="$PATH:$ANDROID_HOME/emulator:$ANDROID_HOME/platform-tools"
```

4. `cargo install cargo-ndk` y compilación:

```bash
cargo ndk -t arm64-v8a -t armeabi-v7a \
  -o ./target/android/jniLibs \
  build -p sustancias-web --features mobile --release

# Con Dioxus CLI (genera el APK y lo instala en el emulador):
dx build --platform android --release
```

Para probar: iniciar un emulador (`emulator -avd Pixel_6_API_34 -netdelay none -netspeed full`) y `dx serve`.

**Estado en esta máquina:** Java 27 está disponible, pero no hay `ANDROID_HOME`, NDK ni `cargo-ndk`. Hay que instalar Android Studio/SDK+NDK para poder generar el APK.

## Apple (pendiente, como pediste)

macOS e iOS requieren obligatoriamente una máquina Apple con Xcode:

```bash
rustup target add aarch64-apple-darwin aarch64-apple-ios-sim
cargo build -p sustancias-web --features desktop --release   # macOS
dx build --platform ios --release                            # iOS
```

No se puede compilar desde Linux.

## Estado verificado en esta máquina (2026-11-18)

| Plataforma | Comprobación | Resultado |
|---|---|---|
| Web (WASM) | `cargo check --target wasm32-unknown-unknown` | ✅ OK |
| Nativo Linux | `cargo check --features desktop` | ✅ OK |
| Nativo Linux (binario) | `cargo build --features desktop --release` | ❌ falla al enlazar: falta `libxdo.so` (paquete `xdotool`). No se puede instalar: `sudo` requiere contraseña |
| Android (targets) | `rustup target add ...` | ✅ 4 targets instalados |
| Android (check) | `cargo check --features mobile --target aarch64-linux-android` | ❌ falta `aarch64-linux-android-clang` (NDK) |
| Android (APK) | — | ❌ sin `ANDROID_HOME`/NDK ni `cargo-ndk` |
| Windows | — | ❌ sin MSVC ni `mingw-w64` (`x86_64-w64-mingw32-gcc` no presente) |
| Apple | — | ❌ requiere macOS + Xcode |

Los targets instalados son: `aarch64-linux-android`, `armv7-linux-androideabi`, `i686-linux-android`, `x86_64-linux-android`, `wasm32-unknown-unknown`, `x86_64-unknown-linux-gnu`.

## Scripts

```bash
./scripts/build-web.sh       # WASM (sin servidor)
./scripts/build-linux.sh     # binario nativo Linux (requiere libxdo)
./scripts/build-android.sh   # comprueba prerequisites y compila con cargo-ndk
```

## Caché offline

El módulo `sustancias-storage` solo entra en las compilaciones nativas (`cfg(not(target_arch = "wasm32"))`). La web no incluye almacenamiento ni cliente HTTP nativo.
