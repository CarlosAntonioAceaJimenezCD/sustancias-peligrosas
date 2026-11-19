# Instalador de Windows (compila en el equipo destino)

Como la compilación cruzada de Windows desde Linux no funciona de forma fiable, este instalador **compila la aplicación dentro del Windows del usuario**.

## Uso

1. Copia esta carpeta `installers/windows/` al equipo Windows.
2. Ejecuta **`Instalar.bat`** con doble clic (no necesita administrador; instala en `%LOCALAPPDATA%`).
3. El instalador:
   - instala Git, Rust y WebView2 si faltan;
   - instala Visual Studio Build Tools con la carga C++ (MSVC) si falta `cl`;
   - obtiene el código (de `source/` si se incluye, o clona el repositorio público);
   - ejecuta `cargo build -p sustancias-web --features desktop --release`;
   - copia el `.exe` a `%LOCALAPPDATA%\Programs\SustanciasPeligrosas\`;
   - crea un acceso directo en el menú de inicio.

## Requisitos

- Windows 10/11 con conexión a internet para descargar las herramientas y el código.
- winget (incluido en Windows 10/11 modernos).

## Instalador con código incluido (sin internet para el código)

Si quieres que el instalador no dependa de GitHub, copia el repositorio completo a:

```
installers/windows/source/
```

El instalador detectará `source\.git` y compilará desde ahí. Seguirá necesitando internet para instalar Rust/MSVC/WebView2 la primera vez.

## Desinstalación

Ejecuta `desinstalar.ps1` desde la carpeta de instalación, o:

```powershell
powershell -ExecutionPolicy Bypass -File "%LOCALAPPDATA%\Programs\SustanciasPeligrosas\desinstalar.ps1"
```

## Nota

El binario compilado es el mismo que el de Linux: usa SQLite incluida, caché offline en los datos del usuario y exporta los PDF a `Descargas\Sustancias Peligrosas\`.
