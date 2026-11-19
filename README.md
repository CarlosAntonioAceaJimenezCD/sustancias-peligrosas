# Sustancias peligrosas

Base de conocimiento sobre sustancias peligrosas: marco legal por país, SGA/GHS, fichas de datos de seguridad, convenios internacionales y regulación del transporte.

Repositorio: https://github.com/CarlosAntonioAceaJimenezCD/sustancias-peligrosas

## Estado actual

Capa de datos terminada y **interfaz en construcción**. Hay dos vistas del mismo diseño:

- **Vista previa editable** en `preview/` (HTML/CSS/JS): se abre directamente en el navegador y sirve para ver y ajustar el diseño sin compilar.
- **Aplicación definitiva** en `crates/web/`: interfaz **Dioxus** escrita en Rust, conectada al núcleo `crates/core` y a la base SQLite gestionada por `crates/dbtool`.

## Política de fuentes

**Solo fuentes oficiales**: gobiernos (`.gob.*`, `.gov.*`, `.mil.*`), organismos internacionales (ONU/UNECE, OMI, OIT, OPCW, convenciones) o instituciones públicas reconocidas (LATU).

Para la **normativa** se excluyen empresas, consultoras, blogs, agregadores comerciales, enciclopedias y repositorios de presentaciones.

Para los **datos de sustancias y FDS**, el usuario aceptó además: PubChem, ChemSpider, Chemical Safety, SDS Manager, ChemBlink, VWR, Fisher Scientific, TCI Chemicals y Fluorochem. Estas fuentes no se usan como autoridad normativa. El detalle de la auditoría, las fuentes rechazadas y las limitaciones de búsqueda están en [`docs/fuentes-oficiales.md`](docs/fuentes-oficiales.md).

Cada fuente lleva un campo `verificacion`:

| Valor | Significado |
|---|---|
| `contenido_verificado` | Página abierta y contenido confirmado. |
| `dominio_snippet` | Dominio gubernamental confirmado por búsqueda; el sitio bloquea al lector automático. |
| `portal_validar` | URL canónica del organismo pendiente de confirmación manual. |
| `base_local_usuario` | Fuente del usuario pendiente de importación. |

## Estructura de datos

| Archivo | Contenido |
|---|---|
| `data/countries.json` | Países del proyecto: internacional, Colombia, España, México, Perú, Venezuela, Cuba y EE. UU. (referencia). |
| `data/topics.json` | Apartados disponibles: SGA, FDS, convenios, transporte y legislación nacional. |
| `data/sources.json` | Registro de fuentes oficiales con institución, URL, tema y verificación. |
| `data/legislation.json` | Portales e instrumentos normativos por país. Cuba se alimenta de la base local del usuario. |
| `data/substances.json` | **Vacío a propósito.** No se almacenan fichas técnicas en el repositorio; las sustancias se obtienen en vivo de PubChem. |

## Vista previa (diseño)

Abre `preview/index.html` en cualquier navegador. No necesita servidor ni instalación: los datos están embebidos en `preview/data.js`.

Para editar el diseño:

| Archivo | Qué controla |
|---|---|
| `preview/index.html` | Estructura de la página |
| `preview/styles.css` | Colores, tamaños, disposición y animaciones |
| `preview/app.js` | Comportamiento (país, idioma, búsqueda, ficha, multi-sustancia) |
| `preview/data.js` | **Generado** desde `data/*.json`; no editar a mano |

Si cambias los JSON, regenera `preview/data.js` con:

```bash
python3 - <<'PY'
import json, pathlib
root = pathlib.Path('data')
def load(n): return json.loads((root / n).read_text(encoding='utf-8'))
sub = load('substances.json')
data = {
  'countries': load('countries.json')['paises'],
  'topics': load('topics.json')['temas'],
  'sources': load('sources.json')['fuentes'],
  'legislation': load('legislation.json')['paises'],
  'substances': sub['sustancias'],
  'warning': sub['advertencia'],
}
pathlib.Path('preview/data.js').write_text(
  'window.DATA = ' + json.dumps(data, ensure_ascii=False, indent=2) + ';\n',
  encoding='utf-8')
PY
```

## Arquitectura Rust

```
crates/core     Librería sin interfaz: modelos, búsqueda y resolución del marco legal.
crates/dbtool   Importa los JSON a SQLite (data/sustancias.db) con índices y FTS5.
crates/web      Aplicación Dioxus (RSX) que consume crates/core.
```

Comandos:

```bash
cargo test                                          # pruebas del núcleo
cargo run -p sustancias-dbtool -- build             # crear/actualizar la base SQLite
cargo run -p sustancias-dbtool -- stats             # contenido de la base
cargo run -p sustancias-dbtool -- search acetona    # búsqueda FTS5
cargo check -p sustancias-web --target wasm32-unknown-unknown
```

SQLite se compila incluida (`bundled`) y el frontend se compila a WebAssembly, de modo que no hay dependencias de servidor: el mismo código sirve para web y, más adelante, para escritorio/móvil con Dioxus.

## Flujo de la aplicación

1. El usuario elige el **país** (o "Marco internacional").
2. El idioma y el banner cambian al instante; el selector queda abajo a la derecha.
3. El panel izquierdo muestra el marco legal filtrable por apartado.
4. El panel derecho busca sustancias por nombre, CAS, CE u ONU y muestra la ficha completa.
5. El bloque multi-sustancia permite añadir varias sustancias y analiza **todas las parejas**: muestra nivel de peligro, tipo de reacción, productos, consecuencias y acciones recomendadas (`data/interactions.json`). Las parejas sin regla explícita usan una regla general de "sin reacción registrada".
6. El buscador tolera ausencia de acentos y errores tipográficos menores (distancia de edición 1–2); los identificadores CAS/CE/ONU se buscan de forma exacta.

## Países incluidos

- **Marco internacional** — SGA/GHS, convenios y Reglamentación Modelo de la ONU.
- **Colombia** — MinAmbiente, DAPRE, DIMAR.
- **España** — INSST, BOE, MITECO, MITMA, ECHA.
- **México** — DOF/STPS, SEMARNAT, SCT.
- **Perú** — SINIA-MINAM, El Peruano, MTC.
- **Venezuela** — Asamblea Nacional, MINEC, INTT, SENCAMER, RESQUIMC.
- **Cuba** — normas de la Gaceta Oficial localizadas en la base del usuario e incluidas en `data/leyes/cuba/`.
- **Estados Unidos** — referencia complementaria (OSHA), no marco aplicable.

## Publicación en GitHub Pages

El repositorio es público. El despliegue de la app Dioxus está automatizado en `.github/workflows/deploy-pages.yml`: cada `push` a `main` compila el WASM con `dx build --release` y publica el resultado en GitHub Pages.

Paso único en el repositorio: **Settings → Pages → Build and deployment → Source: GitHub Actions**. Después de eso, el workflow se ejecuta solo y la URL queda en **Settings → Pages**.

También se puede lanzar a mano desde la pestaña **Actions → Deploy Dioxus a GitHub Pages → Run workflow**.

## Acceso a las leyes cubanas

El usuario final no necesita la carpeta local. Al seleccionar **Cuba**, el panel de marco legal muestra cada norma con:

- **Leer norma íntegra**: abre el texto completo extraído de la Gaceta Oficial, sin salir de la app.
- **Abrir PDF original**: enlaza al archivo alojado en el repositorio (`data/leyes/cuba/`).

Normas incluidas: Resolución 253/2021 (productos químicos peligrosos y desechos peligrosos), Resolución 96/2004 (productos químicos tóxicos) y el Reglamento del Sistema Nacional de Control de Sustancias Químicas. La Ley 81/1997 aparece como referencia citada, pendiente de incorporar su texto.

## Herramientas de compilación

Guía completa, comandos y estado real de cada plataforma en [`docs/compilacion.md`](docs/compilacion.md). Scripts listos en `scripts/`: `build-web.sh`, `build-linux.sh` y `build-android.sh`.


Comunes a todas las plataformas:

| Herramienta | Uso |
|---|---|
| `rustup`, `rustc`, `cargo` | Toolchain Rust (versión 1.85 o superior) |
| `rustup target add wasm32-unknown-unknown` | Destino WebAssembly para la web |
| `cargo install dioxus-cli` | CLI `dx`: desarrollo, empaquetado y assets |
| Compilador C (`cc`, MSVC o Xcode CLT) | Necesario para compilar SQLite incluida (`rusqlite bundled`) |

Por plataforma:

| Plataforma | Requisitos adicionales |
|---|---|
| **Web** | Ninguno además de WASM + `dx`. Se publica como sitio estático. |
| **Windows** | WebView2 Runtime y Visual Studio Build Tools con carga de C++ (MSVC). |
| **macOS** | Xcode Command Line Tools y macOS SDK. **Solo se puede compilar en una máquina Apple.** |
| **Linux** | `libwebkit2gtk-4.1-dev`, `libgtk-3-dev`, `libsoup-3.0-dev`, `pkg-config`. |
| **Android** | Android SDK + NDK, JDK 17, `cargo install cargo-ndk`, targets `aarch64-linux-android`, `armv7-linux-androideabi`, `x86_64-linux-android`, Gradle. |
| **iOS** | macOS + Xcode + targets `aarch64-apple-ios` y `x86_64-apple-ios`. **No se puede compilar en Linux.** |

Opcional: `wasm-opt` (Binaryen) para optimizar el bundle WASM, y `wasm-bindgen` (lo gestiona `dioxus-cli`).

## Sin fichas locales

No hay **ninguna** sustancia almacenada en el repositorio. `data/substances.json` contiene `"sustancias": []` a propósito. Todas las fichas se generan en vivo desde PubChem (NIH), se marcan como **`DATOS EN LÍNEA · PUBCHEM`** y se construyen con las 16 secciones del SGA. Las secciones que exigen datos del fabricante lo indican y enlazan a fuentes oficiales.

Esto se probó con sustancias **no incluidas en ningún archivo del repo**: `ibuprofeno` (CID 3672), `paracetamol` (Acetaminophen) y `cafeína`. En los tres casos: 0 resultados locales, ficha de 16 secciones desde PubChem y comparación por parejas con sus nombres reales.

La comparación multi-sustancia acepta sustancias en línea: cada resultado de PubChem tiene **"Añadir a comparación"**. Las parejas sin regla en `data/interactions.json` usan la regla general de "sin reacción registrada". Para reacciones concretas, añade pares con `sustancias: [id_a, id_b]`, `nivel`, `tipo`, `descripcion`, `productos`, `consecuencias` y `acciones`.

Si en el futuro se incorpora una FDS verificada en PDF, el modelo sigue admitiendo `pdf_url` y `pdf_nombre` para mostrar **"Descargar FDS en PDF"**, y `estado: "verificado"` para la insignia verde.

## Descargas y caché sin conexión (nativo)

El comportamiento depende de la plataforma:

| Versión | Qué hace al descargar |
|---|---|
| **Web (WASM)** | Descarga directamente al directorio de descargas del navegador/sistema. **No guarda nada** en la app. |
| **Nativa (escritorio/móvil)** | 1) Guarda el PDF en el directorio de datos de la aplicación; 2) lo copia al directorio de descargas del sistema. Ambas cosas se pueden configurar. |

El módulo `crates/storage` implementa:

- `offline_cache`: guardar normas y fichas en `<datos del usuario>/sustancias-peligrosas/documents/` con un índice `index.json`, para consultarlas sin internet.
- `copy_to_downloads`: copiar además el archivo a `Descargas`.
- `downloads_dir`: directorio de descargas personalizado; vacío = el del sistema (`dirs::download_dir()`).
- Indexación, lectura, borrado y saneamiento de nombres de archivo.

Estas opciones aparecen en el panel **"Documentos sin conexión"**, que solo se muestra en la versión nativa (la web no incluye el módulo de almacenamiento ni el cliente HTTP).

Compilación nativa:

```bash
cargo build -p sustancias-web --features desktop --release
```

Requiere las dependencias de WebView de tu sistema (WebKitGTK en Linux, WebView2 en Windows, WKWebView en macOS). La compilación de iOS/macOS exige macOS + Xcode. La compilación web (`wasm32-unknown-unknown`) no incluye este módulo.

## Optimizaciones

- **Caché estática**: `crates/core` parsea cada JSON una sola vez con `OnceLock` en lugar de hacerlo en cada render de la interfaz. Esto elimina el coste de deserializar ~322 KB de legislación y ~310 KB de texto de leyes en cada actualización de pantalla.
- **Búsqueda difusa eficiente**: distancia de Levenshtein con dos filas y salida temprana; tokens con stopwords; los identificadores numéricos (CAS/CE/ONU) se comparan por substring exacto, sin fuzzy.
- **Perfil de release**: `opt-level = "z"`, `lto = "thin"`, `codegen-units = 1`, `strip = true` y `panic = "abort"` para reducir el tamaño del binario y del WASM.
- **Límites de API**: *debounce* de 450 ms en PubChem (≈2 req/s, por debajo del límite de 5 req/s) y 500 ms en el BOE.
- **Datos embebidos en la vista previa**: `preview/data.js` evita peticiones `fetch` locales y hace que la maqueta funcione abriendo el HTML directamente.

## Bases de datos oficiales en línea

La aplicación ya no se limita a los datos embebidos:

- **Sustancias**: al escribir en el buscador se consulta en vivo la API PUG REST de **PubChem (NIH, EE. UU.)** con CORS abierto. Los resultados aparecen bajo "Resultados en bases de datos oficiales · PubChem (NIH)". Al pulsar *Ver ficha en línea* se genera una **ficha parcial de 16 secciones** con propiedades reales (fórmula, peso, InChIKey, XLogP, etc.), marcada como `DATOS EN LÍNEA · PUBCHEM` y con enlaces a PubChem y a las fichas ICSC de la OIT. Las secciones que requieren FDS del fabricante lo indican explícitamente: **no se presenta como una FDS completa**.
- **Marco legal**: al seleccionar cualquier país se muestran **siempre** las fuentes del marco internacional (UNECE, OMI, OIT, convenciones) **junto a** la legislación nacional, más los portales oficiales del país. Ya no es necesario elegir "Marco internacional" por separado para verlo.
- No existen fichas locales: la búsqueda de sustancias se resuelve íntegramente contra PubChem en vivo.

## Descarga de documentos

- **Leyes cubanas**: el lector de normas muestra *Abrir PDF original* y *Descargar PDF*. La descarga usa `fetch` + `blob` (el hosting `raw.githubusercontent.com` permite CORS), con el nombre de archivo de la Gaceta.
- **Fichas de sustancias**: si una sustancia aporta `pdf_url`, aparece *Descargar FDS en PDF*. Para las fichas generadas en línea (sin PDF) existe *Guardar ficha como PDF (imprimir)*, que abre el diálogo de impresión del navegador con una hoja de estilo que oculta paneles, menús y botones, dejando solo la ficha de 16 secciones.
- La aplicación Dioxus realiza la misma descarga ejecutando el patrón `fetch`/`blob` desde Rust mediante `dioxus::document::eval`.

## Pendiente

- Ampliar las reglas de `data/interactions.json` a más pares de sustancias.
- Validar en navegador los enlaces con `verificacion: portal_validar`.
- Importar el resto de la base de legislación cubana del usuario (ya hay 3 normas incluidas).
- Generar los paquetes Dioxus para escritorio y móvil. Las compilaciones de iOS/macOS requieren macOS + Xcode.
