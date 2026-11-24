# Sustancias Peligrosas

> **¿Te resultó útil?** Dale una estrella al repositorio en GitHub: ayuda a que más profesionales de seguridad química, PRL y laboratorio lo encuentren. [★ Repositorio](https://github.com/CarlosAntonioAceaJimenezCD/sustancias-peligrosas)

Base de conocimiento multilingüe de sustancias peligrosas: **marco legal por país, SGA/GHS, fichas técnicas completas de 16 secciones, convenios internacionales y transporte de mercancías peligrosas**.

| | |
|---|---|
| **Sitio web (vista previa)** | https://carlosantonioaceajimenezcd.github.io/sustancias-peligrosas/ |
| **Repositorio** | https://github.com/CarlosAntonioAceaJimenezCD/sustancias-peligrosas |
| **Plataformas objetivo** | Web (WASM), Windows, macOS, Linux, Android e iOS |
| **Datos de sustancias** | En vivo, desde PubChem (NIH) |

> **La web es la vista previa; el producto es la aplicación nativa.** La interfaz web se publica en GitHub Pages para que cualquiera pueda probar la aplicación sin instalar nada. La aplicación real se compila de forma nativa para escritorio y móvil, con caché sin conexión, descarga de documentos y almacenamiento local de opciones.

---

## Qué hace la aplicación

1. **Elige jurisdicción.** 175 jurisdicciones agrupadas por continente. Cada continente es una «carpeta» desplegable: primero abres *Europa* y después seleccionas *Suiza*. Todas las banderas están embebidas (SVG para las originales, PNG de 40 px para el resto), sin emojis ni dependencias externas. En móvil el selector es desplazable y la búsqueda permanece siempre visible.
2. **Consulta el marco legal en capas**, siempre en este orden:
   - **Marco internacional** (ONU, UNECE, OMI, OIT, convenciones de Basilea, Róterdam, Estocolmo, Minamata) — para todos los países.
   - **Organizaciones regionales** — solo si el país pertenece a una con normativa verificada y enlace directo: Unión Europea (REACH, CLP, Directiva 2008/68/CE) y Comunidad Andina (Decisiones 602 y 547). MERCOSUR quedó excluido porque su sistema normativo bloquea el acceso y no hay enlaces directos comprobables.
   - **Legislación nacional** — del país seleccionado, con enlaces a los textos oficiales.
3. **Busca cualquier sustancia**, en cualquier idioma y por cualquier identificador:
   - Nombre químico en español, inglés, francés, alemán, italiano, portugués, ruso, chino, árabe… (traducción automática al inglés antes de consultar PubChem).
   - **Código CAS** (`7664-93-9`), **fórmula molecular** (`H2SO4`, `NaCl`, `CH3COOH`) y **códigos de producto** (`AEO7`, `TX-100`, `SDS`, `SLS`, `EDTA`, `BHT`, `TEA`). Los términos ambiguos muestran una nota aclaratoria.
4. **Abre la ficha completa** con los 16 apartados del SGA y datos reales de PubChem PUG View (propiedades, clasificación GHS, todas las frases H, sinónimos y enlaces oficiales). Todo se muestra dentro de la app, sin saltar a sitios externos.
5. **Compara sustancias** por parejas: nivel de peligro, tipo de reacción, productos, consecuencias y acciones recomendadas (`data/interactions.json`). Las parejas sin regla explícita usan una regla general de «sin reacción registrada».
6. **Envía feedback** mediante un formulario que genera automáticamente un issue estructurado (tipo, descripción, país, idioma, búsqueda actual y plataforma). En la versión nativa puedes activar además el reporte automático de errores.

---

## Traducción: una capa, siete plataformas

La traducción vive en `crates/i18n`, **no en la web**, para que la misma implementación sirva a WASM, Windows, macOS, Linux, Android e iOS. El HTTP se abstrae una sola vez (`gloo-net` en WASM, `reqwest` en nativo) y los proveedores implementan un trait común:

| Proveedor | Cuándo se usa | Características |
|---|---|---|
| **DeepL** | Solo si el usuario activa su propia key en *Opciones* | Máxima calidad; endpoint gratuito o Pro; la key se guarda únicamente en el dispositivo |
| **Google Translate** | Por defecto | Sin clave, sin configuración, `sl=auto` detecta cualquier idioma, CORS abierto en web |
| **Diccionario local** | Respaldo offline | 209 términos químicos multilingües + alias industriales del core; funciona sin internet |

La cadena prueba los proveedores en ese orden y devuelve el primero que responde. La **entrada** se traduce al inglés antes de consultar PubChem y la **salida** al idioma elegido en el selector (es, en, fr, de, it, pt, ru, zh, ja, ar, hi, nl, sv, pl, tr, ko o «auto»).

*Privacidad:* al traducir, el texto de la búsqueda se envía a Google Translate (o a DeepL si lo configuraste). El diccionario local no envía nada.

---

## Política de fuentes

**Normativa: solo fuentes oficiales.** Gobiernos (`.gob.*`, `.gov.*`, `.mil.*`), organismos internacionales (ONU/UNECE, OMI, OIT, OPCW, convenciones) o instituciones públicas reconocidas (LATU). Se excluyen empresas, consultoras, blogs, agregadores comerciales, enciclopedias y repositorios de presentaciones.

**Datos de sustancias y FDS**, además de PubChem: ChemSpider, Chemical Safety, SDS Manager, ChemBlink, VWR, Fisher Scientific, TCI Chemicals y Fluorochem. Estas fuentes **no** se usan como autoridad normativa.

Cada fuente lleva un campo `verificacion`:

| Valor | Significado |
|---|---|
| `contenido_verificado` | Página abierta y contenido confirmado |
| `dominio_snippet` | Dominio gubernamental confirmado; el sitio bloquea al lector automático |
| `portal_validar` | URL canónica pendiente de confirmación manual |
| `base_local_usuario` | Fuente del usuario pendiente de importación |

La auditoría completa, las fuentes rechazadas y los enlaces directos verificados están en [`docs/fuentes-oficiales.md`](docs/fuentes-oficiales.md) y [`docs/fuentes-datos-en-linea.md`](docs/fuentes-datos-en-linea.md).

---

## Estructura del repositorio

| Ruta | Contenido |
|---|---|
| `crates/core` | Modelos, cachés `OnceLock`, búsqueda difusa y resolución del marco legal. Sin interfaz. |
| `crates/query` → `core/src/query.rs` | Capa de consultas: traducción local, detección de CAS, fórmulas y códigos de producto. |
| `crates/i18n` | Capa de traducción compartida: `Translator`, Google, DeepL (opcional), diccionario local y cadena de respaldo. |
| `crates/storage` | Caché sin conexión, opciones, exportación a `Descargas` y saneamiento de nombres (nativo). |
| `crates/web` | Interfaz Dioxus (RSX), banderas embebidas, formulario de feedback y panel de opciones. |
| `data/countries.json` | 175 jurisdicciones agrupadas por continente. |
| `data/sources.json` | Fuentes oficiales con institución, tema, URL y verificación. |
| `data/legislation.json` | Portales e instrumentos por país, con el texto íntegro de las normas cubanas. |
| `data/leyes/cuba/*.pdf` | Gacetas oficiales originales de las tres normas cubanas incluidas. |
| `data/organizations.json` | Organizaciones regionales y sus miembros (UE, Comunidad Andina). |
| `data/aliases.json` | Códigos de producto y términos ambiguos resueltos a CIDs de PubChem. |
| `data/interactions.json` | 15 reglas de interacción entre sustancias. |
| `data/cameo_groups.json` | 68 grupos reactivos de NOAA/EPA CAMEO Chemicals. |
| `data/cameo_reactions.json` | Prototipo de 3 pares de grupos con peligros, gases y citas bibliográficas (Capa 1). |
| `data/substances.json` | **Vacío a propósito.** No se almacenan fichas en el repositorio. |
| `preview/` | Maqueta HTML/CSS/JS editable de la interfaz (solo diseño; no se despliega). |
| `.github/workflows/deploy-pages.yml` | Compilación y publicación del WASM en GitHub Pages. |

 Todas las fichas se generan en vivo desde PubChem y se marcan como **`DATOS EN LÍNEA · PUBCHEM`**. `data/substances.json` contiene `"sustancias": []` deliberadamente.

---

## Compilación

> Esta sección es la **fuente canónica** de comandos. Los archivos de `docs/` son complementos, no copias.

### Preparación común (todas las plataformas)

```bash
rustup update stable
rustup target add wasm32-unknown-unknown
cargo install dioxus-cli --locked
```

Verifica la instalación con `cargo --version`, `rustc --version` y `dx --version`.

### Comandos generales

| Acción | Comando |
|---|---|
| Tests del workspace | `cargo test --workspace` |
| Comprobar WASM | `cargo check -p sustancias-web --target wasm32-unknown-unknown` |
| Sitio completo (igual que el CI) | `dx build --release --package sustancias-web` |
| Desarrollo web | `dx serve` |
| Desarrollo escritorio | `dx serve --platform desktop` |
| Desarrollo Android | `dx serve --platform android` |
| Binario nativo optimizado | `cargo build -p sustancias-web --features desktop --profile native-release` |

El sitio se genera en `target/dx/sustancias-web/release/web/public/`. El workflow copia `index.html` a `404.html` e inyecta la CSP y `robots.txt`.

### Perfiles de compilación

| Perfil | Uso | Ajustes |
|---|---|---|
| `release` | Web / WASM | `opt-level = "z"`, `lto = "thin"`, `codegen-units = 1`. **Sin `strip`**: rompe `wasm-bindgen` |
| `native-release` | Escritorio y móvil | Hereda de `release`, `opt-level = 2`, `lto = "thin"`, `codegen-units = 1`, `strip = true`, `panic = "abort"` |

### Linux (verificado en CI)

```bash
sudo apt-get update
sudo apt-get install -y \
  libwebkit2gtk-4.1-dev libgtk-3-dev libsoup-3.0-dev \
  libjavascriptcoregtk-4.1-dev libayatana-appindicator3-dev \
  librsvg2-dev libxdo-dev patchelf

cargo build -p sustancias-web --features desktop --profile native-release
```

Sin `libxdo-dev` el enlazado falla con `unable to find library -lxdo`. En distribuciones distintas de Debian/Ubuntu, instala los equivalentes de WebKitGTK 4.1 y GTK 3.

### Windows

1. Instala **Visual Studio Build Tools** con la carga de trabajo *Desktop development with C++*.
2. Instala **WebView2 Runtime**.
3. Compila:

```powershell
cargo build -p sustancias-web --features desktop --profile native-release
```

El instalador de `installers/windows/` compila en la máquina del usuario y deja el ejecutable en `Descargas\Sustancias Peligrosas\`.

### macOS

Solo se puede compilar en una máquina Apple:

```bash
xcode-select --install
cargo build -p sustancias-web --features desktop --profile native-release
```

### Android

```bash
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android
cargo install cargo-ndk
# Configura ANDROID_HOME / ANDROID_NDK_HOME y JDK 17
cargo ndk -t arm64 -t armv7 -t x86_64 -o ./jniLibs build --release -p sustancias-web --features mobile
dx build --platform android --release
```

### iOS

Requiere macOS + Xcode:

```bash
rustup target add aarch64-apple-ios x86_64-apple-ios
cargo build -p sustancias-web --features mobile --target aarch64-apple-ios --profile native-release
```

### Verificación automática

Cada `push` a `main` ejecuta cinco jobs: compilación WASM, tests y compilación nativa (con las dependencias de Linux), auditoría RustSec, accesibilidad con axe y despliegue a Pages. El job de auditoría es informativo; los avisos transitivos están documentados en `docs/fuentes-oficiales.md`.

Opcional: `wasm-opt` (Binaryen) para optimizar el bundle WASM. `wasm-bindgen` lo gestiona `dioxus-cli`.

Guía ampliada: [`docs/compilacion.md`](docs/compilacion.md) e [`docs/instalacion.md`](docs/instalacion.md).

---

## Publicación en tiendas y canales de descarga

Requisitos verificados en la documentación oficial de cada plataforma. La app today solo está publicada en la web; el resto son pasos pendientes.

| Plataforma | Canal | Requisitos principales | Estado |
|---|---|---|---|
| **Web** | GitHub Pages | Repositorio público, workflow de Actions, CSP y `robots.txt` | ✅ Publicada |
| **Android** | Google Play | Cuenta de desarrollador; **AAB firmado** con Play App Signing; `versionCode` creciente (máx. 2100000000); cumplir los *target API level requirements*; declaraciones de políticas; ficha con nombre (30), descripción corta (80) y completa (4000 caracteres), 2–8 capturas y correo de contacto; las cuentas personales nuevas tienen requisitos de testing previo | Pendiente |
| **Android** | F-Droid | Código abierto con licencia **FLOSS** reconocida; **solo dependencias FLOSS** (prohibidos Google Play Services, Firebase y SDKs privativos); toolchain FLOSS, compilación por CLI; ID de aplicación único; tags de versión (`v1.0`); metadata `fastlane/metadata/android/en-US/`; solicitud por merge request en `fdroiddata`; revisión manual y 24–48 h hasta publicación. **Ojo:** F-Droid no incluye claves API de terceros; las claves de DeepL/CAS las introduce el usuario, y el uso de Google Translate puede marcarse como *AntiFeature* de servicio no libre | Pendiente, con avisos |
| **Linux** | Flathub | Manifiesto de construcción y metainfo AppStream; compilar con `org.flatpak.Builder`; pasar el linter (`flatpak-builder-lint`); PR a `flathub/flathub` contra la rama `new-pr` (no `master`); revisión por voluntarios; publicación 1–2 h tras merge | Pendiente |
| **Windows** | Microsoft Store | Cuenta de desarrollador en Partner Center (individual o empresa); paquete **MSIX**, MSI/EXE o PWA; reservar el nombre de la app; pasar certificación; aceptar políticas de la Store; se puede automatizar con Microsoft Store Developer CLI y GitHub Actions | Pendiente |
| **macOS** | Mac App Store | Apple Developer Program; **sandboxing** obligatorio; paquete firmado y notarizado, sin instaladores de terceros; actualizaciones solo por App Store; revisión de App Store. La guía 1.4.1 examina con más rigor apps que puedan dar datos inexactos: mantener visible el aviso de que no sustituye la FDS del fabricante | Pendiente |
| **iOS** | App Store / TestFlight | Apple Developer Program; Xcode; revisión de App Store; TestFlight para betas; mismas normas de contenido y privacidad | Pendiente |

Fuentes oficiales: [Google Play](https://support.google.com/googleplay/android-developer/answer/9859152), [F-Droid](https://f-droid.org/en/docs/Submitting_to_F-Droid_Quick_Start_Guide) y su [política de inclusión](https://f-droid.org/en/docs/Inclusion_Policy), [Flathub](https://docs.flathub.org/docs/for-app-authors/submission), [Microsoft Store](https://learn.microsoft.com/en-us/windows/apps/publish/), [App Store Review Guidelines](https://developer.apple.com/app-store/review/guidelines/).

### Sitios de descarga legales alternativos

Si no se publica en una tienda, la app se puede distribuir como descarga directa firmada desde el repositorio (ya existe un instalador en `installers/windows/` que compila en la máquina del usuario). Para Linux, un repo propio compatible con F-Droid permite llegar a esos usuarios sin pasar por el repositorio principal.

## Descargas y caché sin conexión

| Versión | Qué hace al descargar |
|---|---|
| **Web (WASM)** | Descarga directamente al directorio del navegador. No guarda nada en la app. |
| **Nativa** | 1) Guarda el documento en el directorio de datos de la app; 2) lo copia a `Descargas/Sustancias Peligrosas\`. Ambas opciones se pueden configurar. |

`crates/storage` implementa `offline_cache`, `copy_to_downloads`, `downloads_dir` personalizado, indexado, lectura, borrado y saneamiento de nombres de archivo. Estas opciones —y el proveedor de traducción— aparecen en el panel **«Documentos sin conexión»**, que solo se muestra en la versión nativa.

Para las fichas generadas en línea existe **«Guardar ficha como PDF (imprimir)»**, con una hoja de estilo que oculta paneles, menús y botones y deja solo la ficha de 16 secciones.

---

## Acceso a las leyes cubanas

Al seleccionar **Cuba**, cada norma ofrece **«Leer norma íntegra»** (texto extraído de la Gaceta Oficial, sin salir de la app) y **«Abrir PDF original»** (archivo alojado en `data/leyes/cuba/`). Incluidas: Resolución 253/2021, Resolución 96/2004 y el Reglamento del Sistema Nacional de Control de Sustancias Químicas. La Ley 81/1997 aparece como referencia citada.

---

## Seguridad

- **Auditoría automática:** el workflow ejecuta `rustsec/audit-check` en cada `push` y bloquea el despliegue si aparece una vulnerabilidad conocida (CVE) en las dependencias.
- **CSP:** el workflow inyecta una *Content-Security-Policy* en `index.html` y `404.html`. GitHub Pages no permite cabeceras HTTP propias, pero la meta tag restringe scripts, estilos, imágenes y destinos de red a los dominios necesarios (PubChem, Google Translate y BOE).
- **Validación de entrada:** se rechazan búsquedas formadas solo por emoji, puntuación o dígitos sueltos, que generaban resultados aleatorios en PubChem.
- **Errores de red diferenciados:** un fallo de conexión se distingue de «sin resultados» y se indica al usuario.
- **Sin `unsafe`:** no hay bloques `unsafe` en el workspace.

- **Avisos transitivos aceptados:** `cargo audit` informa 5 avisos de severidad *warning* en dependencias transitivas de Dioxus/WebKit que no se pueden actualizar desde este repositorio. El CI los ignora explícitamente, con los IDs registrados:

| ID | Crate | Motivo |
|---|---|---|
| RUSTSEC-2025-0057 | `fxhash` | No mantenida; sin versión corregida |
| RUSTSEC-2024-0436 | `paste` | No mantenida; sin versión corregida |
| RUSTSEC-2024-0370 | `proc-macro-error` | No mantenida; sin versión corregida |
| RUSTSEC-2024-0429 | `glib` | Requiere `>=0.20` (lo fija el backend WebKitGTK) |
| RUSTSEC-2026-0097 | `rand` | Requiere `>=0.8.6` (dependencia transitiva) |
| RUSTSEC-2026-0543 | `chrono` | No mantenido, sin versión corregida; solo se usa para leer la hora local |
| RUSTSEC-2026-0592 | `url` | Stack overflow en `Url::origin()` con URLs `blob:` anidadas; la app no construye ese tipo de URL |

Ninguno es explotable desde la interfaz web: afectan a macros en tiempo de compilación y al backend nativo de escritorio. Se revisarán al actualizar Dioxus. El HTML dinámico se limita a iconos y banderas controladas; los datos externos se renderizan como texto escapado.

## Despliegue

El repositorio es público y el despliegue está automatizado en `.github/workflows/deploy-pages.yml`: cada `push` a `main` compila el WASM con `dx build --release` y publica el resultado en GitHub Pages.

Paso único: **Settings → Pages → Build and deployment → Source: GitHub Actions**. También se puede lanzar a mano desde **Actions → Deploy Dioxus a GitHub Pages → Run workflow**.

---

## Estado verificado

- `cargo test --workspace`: ✅ 23 tests (core 17 + i18n 3 + storage 3)
- `cargo check -p sustancias-web --target wasm32-unknown-unknown`: ✅
- `cargo check -p sustancias-web --features desktop`: ✅
- Despliegue en GitHub Pages: ✅
- Búsquedas verificadas en vivo: `ácido sulfúrico`, `alcohol etoxilado`, `AEO7`, `éter`, `H2SO4`, `7664-93-9`, `TX-100`
- Frases H del ácido sulfúrico: 9 (`H290`, `H303`, `H314`, `H318`, `H330`, `H370`, `H372`, `H402`, `H410`)

---

## Pendiente

- Ampliar las reglas de `data/interactions.json` a más pares de sustancias.
- Validar en navegador los enlaces con `verificacion: portal_validar`.
- Generar los paquetes de instalación para escritorio y móvil (iOS/macOS requieren Apple).

## Licencia

MIT OR Apache-2.0.
