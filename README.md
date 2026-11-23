# Sustancias Peligrosas

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
| `data/substances.json` | **Vacío a propósito.** No se almacenan fichas en el repositorio. |
| `preview/` | Maqueta HTML/CSS/JS editable de la interfaz (solo diseño; no se despliega). |
| `.github/workflows/deploy-pages.yml` | Compilación y publicación del WASM en GitHub Pages. |

 Todas las fichas se generan en vivo desde PubChem y se marcan como **`DATOS EN LÍNEA · PUBCHEM`**. `data/substances.json` contiene `"sustancias": []` deliberadamente.

---

## Compilación

> Esta sección es la **fuente canónica** de comandos. Los archivos de `docs/` son complementos, no copias.

### Comunes a todas las plataformas

```bash
rustup update stable
rustup target add wasm32-unknown-unknown
cargo install dioxus-cli --locked
```

### Comandos

| Acción | Comando |
|---|---|
| Tests de todo el workspace | `cargo test --workspace` |
| Comprobar WASM | `cargo check -p sustancias-web --target wasm32-unknown-unknown` |
| Sitio completo (Igual que CI) | `dx build --release --package sustancias-web` |
| Escritorio (desarrollo) | `dx serve --platform desktop` |
| Móvil (desarrollo) | `dx serve --platform android` |
| Binario nativo suelto | `cargo build -p sustancias-web --features desktop --release` |

### Requisitos por plataforma

| Plataforma | Requisitos adicionales |
|---|---|
| **Web** | Ninguno además del target WASM y `dx`. Se publica como sitio estático. |
| **Windows** | WebView2 Runtime y Visual Studio Build Tools con carga de C++. Instalador en `installers/windows/`. |
| **macOS** | Xcode Command Line Tools y macOS SDK. Solo se puede compilar en una máquina Apple. |
| **Linux** | `libwebkit2gtk-4.1-dev`, `libgtk-3-dev`, `libsoup-3.0-dev`, `pkg-config`. |
| **Android** | Android SDK + NDK, JDK 17, `cargo install cargo-ndk`, targets `aarch64-linux-android`, `armv7-linux-androideabi`, `x86_64-linux-android`, Gradle. |
| **iOS** | macOS + Xcode + targets `aarch64-apple-ios` y `x86_64-apple-ios`. No se puede compilar en Linux. |

Opcional: `wasm-opt` (Binaryen) para optimizar el bundle. `wasm-bindgen` lo gestiona `dioxus-cli`.

**Importante:** el perfil de release usa `opt-level = "z"`, `lto = "thin"` y `codegen-units = 1`, pero **no** `strip = true`: elimina la sección `target_features` del WASM y rompe `wasm-bindgen` durante el despliegue.

Guía ampliada y estado verificado por plataforma: [`docs/compilacion.md`](docs/compilacion.md) e [`docs/instalacion.md`](docs/instalacion.md).

---

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
