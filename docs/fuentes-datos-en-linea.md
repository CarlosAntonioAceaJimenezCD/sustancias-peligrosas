# Extracción de datos sin servidor propio

Informe revisado el 2026-11-18 sobre `robots.txt`, cabeceras CORS y condiciones legales de cada fuente.

Tres barreras distintas:

1. **robots.txt** — instrucciones voluntarias para crawlers; no son una prohibición legal ni técnica.
2. **CORS** — lo que el navegador permite realmente. Sin `Access-Control-Allow-Origin`, una web estática **no puede** leer la fuente: haría falta un proxy propio.
3. **Condiciones de reutilización / copyright** — lo que autoriza legalmente copiar y publicar los datos.

## Fuentes explotables desde el navegador (sin servidor)

### PubChem (NIH) — integrada

- API: `https://pubchem.ncbi.nlm.nih.gov/rest/pug/...`, `Access-Control-Allow-Origin: *` verificado.
- `robots.txt`: `User-agent: *` → `Crawl-delay: 5`. Prohíbe bots de entrenamiento (GPTBot, ClaudeBot, CCBot…), no al uso de la API por aplicaciones.
- Política de uso: **máximo 5 peticiones por segundo**, sin claves API, con limitación automática (throttling).
- Datos públicos del NIH.
- El buscador usa *debounce* de 450 ms (≈2 req/s), dentro del límite.

### BOE (España) — integrada

- API de datos abiertos: `https://www.boe.es/datosabiertos/api/legislacion-consolidada`.
- **Obligatorio** enviar `Accept: application/json`; sin esa cabecera responde 400/500.
- `Access-Control-Allow-Origin: *` y `Access-Control-Allow-Headers` con `Accept` → funciona desde el navegador.
- El parámetro `query` es un JSON en string:
  `{"query":{"query_string":{"query":"titulo:mercancias"}},"sort":[]}`
- Verificado en vivo: devuelve `BOE-A-2014-2110` (Real Decreto 97/2014).
- Condiciones de reutilización (Resolución AEBOE de 27-jun-2024):
  - Citar **"Fuente de los datos: Agencia Estatal Boletín Oficial del Estado"** con enlace a `https://www.boe.es`.
  - Si se transforma: **"Basado en datos de la Agencia Estatal Boletín Oficial del Estado"**.
  - Indicar que el texto consolidado es **de carácter meramente informativo**.
  - Conservar la fecha de última actualización.
  - No sugerir carácter oficial ni patrocinio.

La app cumple estas condiciones en cada resultado del BOE.

## Fuentes que solo deben enlazarse

| Fuente | Motivo |
|---|---|
| INSST / FISQ | `robots.txt` permite búsqueda con `Content-Signal: search=yes, ai-input=yes`, pero no expone `Access-Control-Allow-Origin`; bloquea rutas de login/filtros |
| ICSC (OIT) | Sin `robots.txt`. © 1996–2018 ILO; las publicaciones anteriores al 3-may-2023 requieren permiso de reproducción. Solo CC BY 4.0 para las posteriores |
| ECHA | WAF (HTTP 403), sin API JSON pública abierta |
| UNECE / OMI / PIC | 403 / WAF / `robots.txt` no válido. OPCW permite `Allow: /` pero con `ai-train=no` |
| Gaceta Oficial de Cuba | `User-agent: *`, `Crawl-delay: 10`, sin CORS. Las 3 normas incluidas se alojan como PDF en este repositorio |
| ChemSpider (RSC) | API con clave; `robots.txt` bloquea `*.aspx` |
| chemicalsafety.com / sdsmanager.es | Agregadores con FDS de terceros; sin CORS y condiciones de uso restrictivas |

Para extraer estas fuentes haría falta un servidor proxy **y** autorización de derechos. No se hace.

## Regla práctica del proyecto

- Si la fuente tiene API con CORS y condiciones de licencia claras → consulta en vivo (PubChem, BOE).
- Si no → enlace a la fuente oficial y, como mucho, documento alojado en este repositorio con su procedencia.
- Nunca presentar un dato parcial como una FDS completa: la app lo marca explícitamente.
