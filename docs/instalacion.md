# Instalación de herramientas de compilación

## 1. Rust (todas las plataformas)

```bash
rustup toolchain install stable
rustup component add rust-src
rustup target add \
  wasm32-unknown-unknown \
  x86_64-unknown-linux-gnu \
  aarch64-linux-android \
  armv7-linux-androideabi \
  i686-linux-android \
  x86_64-linux-android
```

## 2. Dioxus CLI y cargo-ndk

```bash
# CLI de Dioxus (elige UNO)
curl -sSL https://dioxus.dev/install.sh | bash
# cargo install dioxus-cli --locked

# Para Android
cargo install cargo-ndk
```

## 3. Linux (Arch — esta máquina)

```bash
sudo pacman -S --needed \
  webkit2gtk-4.1 base-devel curl wget file openssl \
  appmenu-gtk-module libappindicator-gtk3 librsvg xdotool
```

`xdotool` aporta `libxdo.so`. Sin él el enlazado falla con `unable to find library -lxdo`.

Verificación:

```bash
pkg-config --exists webkit2gtk-4.1 && echo "webkit OK"
ldconfig -p | grep 'libxdo\.so' && echo "libxdo OK"
./scripts/build-linux.sh
```

### Debian / Ubuntu

```bash
sudo apt update
sudo apt install -y \
  libwebkit2gtk-4.1-dev build-essential curl wget file \
  libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
```

## 4. Android

1. Instalar Android Studio: https://developer.android.com/studio
2. SDK Manager → SDK Tools: **Android SDK**, **Command-line tools**, **NDK (side by side)** y **CMake**.
3. Variables de entorno (añadir a `~/.bashrc` o `~/.zshrc`):

```bash
export JAVA_HOME="$HOME/Android/Sdk/jbr"
export ANDROID_HOME="$HOME/Android/Sdk"
export NDK_HOME="$ANDROID_HOME/ndk/<version-instalada>"
export PATH="$PATH:$ANDROID_HOME/emulator:$ANDROID_HOME/platform-tools"
```

4. Compilar:

```bash
source ~/.bashrc
cargo install cargo-ndk
./scripts/build-android.sh
# APK: dx build --platform android --release
```

## 5. Windows

En una máquina Windows (recomendado sobre cross-compilación):

```powershell
winget install Microsoft.VisualStudio.2022.BuildTools --override "--wait --add Microsoft.VisualStudio.Workload.VCTools"
winget install Microsoft.WebView2Runtime
rustup target add x86_64-pc-windows-msvc
cargo build -p sustancias-web --features desktop --release
```

## 6. Apple (más adelante)

```bash
xcode-select --install
rustup target add aarch64-apple-darwin aarch64-apple-ios aarch64-apple-ios-sim
```

## Bloqueos actuales en esta máquina

- `sudo` requiere contraseña → no se pudo instalar `xdotool` (`libxdo.so`).
- No hay `ANDROID_HOME`, NDK ni `cargo-ndk` → no se puede generar el APK.
- No hay MSVC ni `mingw-w64` → no se puede compilar Windows desde aquí.
