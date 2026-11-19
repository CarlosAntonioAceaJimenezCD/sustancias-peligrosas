# Formatos de paquetes en Linux y distribuciones

Linux no tiene un único formato de instalación. Cada familia de distribuciones usa el suyo, además de los formatos universales.

## Paquetes nativos por familia

| Formato | Gestor | Distribuciones |
|---|---|---|
| `.deb` | `dpkg` / `apt` | Debian, Ubuntu, Linux Mint, Pop!_OS, elementary OS, Zorin OS, MX Linux, Devuan, Kali Linux, Raspberry Pi OS |
| `.rpm` | `rpm` / `dnf` / `yum` | Fedora, RHEL, CentOS Stream, Rocky Linux, AlmaLinux, Oracle Linux. También openSUSE (usa `zypper`) |
| `.pkg.tar.zst` (antes `.pkg.tar.xz`) | `pacman` | Arch Linux, Manjaro, EndeavourOS, Garuda Linux, Artix Linux |
| `.apk` | `apk` | Alpine Linux, postmarketOS (**no confundir con el APK de Android**) |
| `.xbps` | `xbps-install` | Void Linux |
| `.eopkg` | `eopkg` | Solus |
| `.tgz` / `.txz` | `slackpkg` | Slackware |
| `.pisi` | `pisi` | Pardus |
| Nix derivation / `nix-env` | `nix` | NixOS; también Nix instalado en otras distros |

## Formatos universales (no dependen de la distro)

| Formato | Cómo funciona | Ventajas | Inconvenientes |
|---|---|---|---|
| **AppImage** | Un ejecutable autocontenido. Se descarga, `chmod +x` y se ejecuta | Sin root, sin instalación, funciona en casi cualquier distro con glibc | Necesita FUSE (`libfuse2` en Ubuntu 22.04+); no se integra en el gestor de actualizaciones |
| **Flatpak** | Paquete con sandbox + runtime (Flathub) | Aislamiento, actualizaciones centralizadas, funciona en muchas distros | Requiere `flatpak` y el runtime; permisos limitados por sandbox |
| **Snap** | Paquete Canonical con `snapd` | Actualizaciones automáticas, funciona en varias distros | Mejor integrado en Ubuntu; arranque más lento, sandbox estricto |
| **tar.gz / tar.xz** | Archivo comprimido genérico | Funciona en cualquier sitio | Instalación manual, sin desinstalación integrada |

## Recomendación para esta aplicación

1. **AppImage** como paquete universal principal: el usuario lo descarga y ejecuta sin root.
2. **`.deb`** para Debian/Ubuntu y derivados.
3. **`.rpm`** para Fedora/RHEL y derivados.
4. **Flatpak** como opción aislada si se quiere publicar en Flathub.

Dioxus CLI (`dx bundle`) genera paquetes nativos; los formatos exactos admitidos dependen de la versión, así que conviene comprobar `dx bundle --help` antes de fijar el pipeline de publicación.

## Instalación típica

```bash
# .deb (Debian/Ubuntu)
sudo apt install ./app.deb

# .rpm (Fedora)
sudo dnf install ./app.rpm

# Arch
sudo pacman -U ./app.pkg.tar.zst

# AppImage
chmod +x app.AppImage && ./app.AppImage

# Flatpak
flatpak install app.flatpak
```
