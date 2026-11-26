# Fuentes oficiales del proyecto

Este documento registra la auditoría de fuentes y los criterios usados. La base legible por máquina está en `data/sources.json` y `data/legislation.json`.

## Criterio de aceptación

Se aceptan **únicamente**:

1. Dominios gubernamentales: `.gob.*`, `.gov.*`, `.mil.*`.
2. Organismos internacionales y convenciones: ONU/UNECE, OMI, OIT, OPCW, Convención de Rotterdam (PIC), Convención HNS.
3. Instituciones públicas reconocidas: LATU (Uruguay).

Se rechazan empresas privadas, consultoras, blogs, agregadores comerciales, enciclopedias y repositorios de presentaciones.

## Estados de verificación

| Estado | Significado |
|---|---|
| `contenido_verificado` | La página fue abierta y se confirmó su contenido. |
| `dominio_snippet` | Institucionalidad confirmada por dominio gubernamental y resultado de búsqueda. El sitio bloquea al lector automático. Validar en navegador. |
| `portal_validar` | URL canónica del organismo, pendiente de confirmación manual. |
| `base_local_usuario` | Fuente proporcionada por el usuario, pendiente de importación. |

## Cobertura de jurisdicciones

La ampliación incorporó **167 jurisdicciones** a `data/countries.json` y `data/legislation.json`, con **230 instrumentos** y **341 portales oficiales**. De los instrumentos, **104 enlazan a un documento o sección concreta** y **126 siguen apuntando al portal** del organismo (el usuario debe buscar la norma allí). El criterio de inclusión fue estricto: solo normativa sobre SGA/GHS, fichas de datos de seguridad, convenios químicos, transporte de mercancías peligrosas, gestión de residuos peligrosos y seguridad química laboral. Quedaron fuera constituciones, leyes generales y normativa sin relación con el tema.

### Niveles de cobertura

- **Cobertura plena**: portales oficiales e instrumentos concretos con URL verificada (por ejemplo Reino Unido, Alemania, Italia, Unión Europea, Chile, México, Perú, Australia, Japón, India, Sudáfrica, Kenia).
- **Solo portales**: jurisdicciones donde no se identificó un texto normativo concreto y verificable. Se listan el diario oficial y el ministerio competente, y la descripción del país lo señala. Es el caso de varios Estados pequeños y de jurisdicciones con legislación no digitalizada.
- **Sin fuente propia**: 26 jurisdicciones conservan su entrada pero no tienen portales ni instrumentos; su descripción indica que solo se muestran las fuentes internacionales. Se prefirió mantenerlas visibles antes que ocultar la falta de información.
- **Referencia**: EE. UU. (OSHA) sigue marcado como `aplicable: false`.

### Tratados globales, sin duplicados

Los textos de Basilea, Róterdam, Estocolmo y Minamata, la Reglamentación Modelo de la ONU, ADR/RID/ADN, el Código IMDG y las Instrucciones Técnicas de la OACI están en `sources.json` con `pais: "internacional"`. La interfaz los muestra para **todos** los países, por lo que no se repiten como instrumentos nacionales. Se añadieron además: Convención de Basilea, Convención de Estocolmo, Convención de Minamata, PNUMA, OACI e IATA.

### Método de verificación de URLs

1. Se recopilaron las URLs de cada jurisdicción desde fuentes gubernamentales y de instituciones públicas reconocidas (diarios oficiales, ministerios de ambiente, autoridades de transporte). Los *Legal Information Institute* del Commonwealth (KenyaLII, GhaLII, NigeriaLII, ULII, ZambiaLII, TanzLII, SAFLII, entre otros) se aceptan como instituciones legales públicas.
2. Se comprobó cada URL de forma concurrente. Un `200` confirma el portal; en instrumentos solo se marca `contenido_verificado` si la URL apunta a un documento concreto, no a la homepage del organismo.
3. Los códigos `401/403/406/429` se marcan `dominio_snippet`: el sitio es oficial pero bloquea al lector automático.
4. Los errores de red o `5xx` se dejan como `portal_validar`: pueden ser problemas temporales de conectividad, no pruebas de que el enlace esté caído.
5. Solo los `404/410` confirmados se descartan. Tras este filtro se eliminó Irak, que no conservaba ninguna fuente válida.

### Profundización de enlaces

Se hizo una pasada específica para sustituir homepages por enlaces directos al texto normativo en jurisdicciones con identificadores estables: Portugal (DRE), Austria (RIS), Polonia (ISAP), Suiza (Fedlex/ELI), Irlanda (Irish Statute Book), Argentina (normativa.gob.ar), Australia (Federal Register), Nueva Zelanda (legislation.govt.nz), Costa Rica (SINALEVI) y Noruega (Lovdata). En total **15 enlaces pasaron de homepage a documento concreto** (13 sustituciones y 2 normas irlandesas recuperadas que se habían perdido por duplicado de URL).

Los títulos se corrigieron donde el enlace original era engañoso: `au-legislacion` es la *Industrial Chemicals Act 2019* (no "AICIS Act"), `ie-clp` es el *S.I. 196/2015* que deroga las normas antiguas de clasificación (el CLP se aplica por reglamento UE directo) y `no-clp` incluye el número de forskrift.

Quedan como portal los casos sin identificador público estable: Italia (Normattiva redirige la URN a otra norma), Países Bajos, Dinamarca, Suecia, Finlandia, Chequia, Brasil y varios más. En esos países el enlace abre la base legal y el usuario debe buscar la norma.

> Los enlaces marcados `portal_validar` o `dominio_snippet` deben revisarse en un navegador normal antes de darlos por definitivos.

## Fuentes aceptadas

### Internacionales

| Institución | Contenido | URL |
|---|---|---|
| UNECE (ONU) | SGA/GHS, edición vigente Rev.11 (2025) | https://unece.org/transport/dangerous-goods/ghs-rev11-2025 |
| UNECE (ONU) | Reglamentación Modelo, Manual de Pruebas y Criterios, ADR y ADN | https://unece.org/transport/dangerous-goods |
| UNECE (ONU) | Manual de Pruebas y Criterios | https://unece.org/transport/dangerous-goods/rev7-files |
| UNECE (ONU) | ADR | https://unece.org/about-adr |
| OMI (ONU) | Lista de convenios (SOLAS, MARPOL, HNS 1996 + Protocolo 2010) | https://www.imo.org/es/about/conventions/pages/listofconventions.aspx |
| OMI (ONU) | Código IMDG, ed. 2024, enmienda 42-24, vigente desde 1-ene-2026 | https://www.imo.org/es/publications/Pages/IMDG%20Code.aspx |
| OIT (ONU) | Fichas ICSC (índice CAS → ficha en `data/icsc_cards.json`, enlace profundo por idioma; el texto de la ficha se consulta siempre en el sitio oficial) | https://chemicalsafety.ilo.org/dyn/icsc/showcard.listCards3?p_lang=es |
| Convención de Rotterdam (PIC) | Basilea, Róterdam y Estocolmo | https://www.pic.int/ |
| Convención HNS | Texto oficial del convenio SNP | https://www.hnsconvention.org/es/el-convenio/ |
| OPCW | Directrices de transporte seguro de sustancias químicas peligrosas (2025) | https://www.opcw.org/ |
| LATU (Uruguay) | Clasificación de peligros según el SGA, en español | https://ghs-sga.com/clasificacion-de-peligros-segun-sga/ |

> LATU es la única fuente no gubernamental conservada: es un ente público uruguayo, centro coordinador del Convenio de Basilea y centro regional del Convenio de Estocolmo para América Latina y el Caribe. Si se desea un filtro estrictamente `.gob/.gov`, sustituir por los documentos en español de UNECE.

### Organizaciones regionales

El marco legal se presenta en tres capas: **marco internacional** (para todos los países), **organizaciones regionales** (solo si el país es miembro y la organización tiene normativa oficial verificada con enlace directo) y **legislación nacional**.

| Organización | Miembros incluidos | Normativa verificada (enlace directo) |
|---|---|---|
| Unión Europea | 27 Estados miembros | [REACH — Reglamento (CE) 1907/2006](https://eur-lex.europa.eu/eli/reg/2006/1907/oj), [CLP — Reglamento (CE) 1272/2008](https://eur-lex.europa.eu/eli/reg/2008/1272/oj), [Directiva 2008/68/CE](https://eur-lex.europa.eu/eli/dir/2008/68/oj) |
| Comunidad Andina | Bolivia, Colombia, Ecuador, Perú y Venezuela | [Decisión 602 — control de sustancias químicas](https://www.comunidadandina.org/StaticFiles/DocOf/DEC602.pdf), [Decisión 547 — seguridad y salud en el trabajo](https://www.comunidadandina.org/StaticFiles/DocOf/DEC547.pdf) |

> MERCOSUR no se incluye: su sistema normativo (`normas.mercosur.int`) devuelve HTTP 403 por Cloudflare y no fue posible verificar enlaces directos al texto legal. La función `organizations_for()` excluye automáticamente cualquier organización que no tenga al menos una norma oficial, verificada y con URL directa, aunque aparezca en `data/organizations.json`.

### España

| Institución | Contenido | URL |
|---|---|---|
| INSST | Normativa nacional de riesgos químicos | https://www.insst.es/normativa/riesgos-quimicos |
| INSST | FISQ, versión española de las ICSC | https://www.insst.es/documentacion/colecciones-tecnicas/fisq |
| BOE | Textos consolidados de leyes y reales decretos | https://www.boe.es/ |
| MITECO | Portal REACH-CLP | https://www.miteco.gob.es/es/calidad-y-evaluacion-ambiental/temas/productos-quimicos/portal-reach-clp/ |
| ECHA | Guías REACH y CLP | https://echa.europa.eu/es/ |
| MITMA | Transporte de mercancías peligrosas | https://www.mitma.es/transporte-terrestre/mercancias-peligrosas-y-perecederas |

### Colombia

| Institución | Contenido | URL |
|---|---|---|
| MinAmbiente | Guía de clasificación de peligros SGA 2017 | https://quimicos.minambiente.gov.co/wp-content/uploads/2021/06/4.-Guia-de-clasificacion-de-peligros-segun-SGA-2017.pdf |
| MinAmbiente | Guía de comunicación de peligros SGA 2017 | https://quimicos.minambiente.gov.co/wp-content/uploads/2021/06/5.-Guia-de-comunicacion-de-peligros-segun-el-SGA-2017.pdf |
| DAPRE | Guía G-TH-09 de manipulación y almacenamiento | https://dapre.presidencia.gov.co/dapre/DocumentosSIGEPRE/G-TH-09-Guia-Manipulacion-Almacenamiento-Sustancias-Quimicas.pdf |
| DIMAR | Convenios internacionales implementados por Colombia | https://www.dimar.mil.co/proteccion_del_medio_marino/convenios-internacionales |

### México

| Institución | Contenido | URL |
|---|---|---|
| DOF / STPS | NOM-005-STPS-1998 — manejo, transporte y almacenamiento | https://dof.gob.mx/nota_detalle_popup.php?codigo=2052048 |
| DOF / STPS | NOM-018-STPS-2015 — sistema armonizado de comunicación de peligros | https://dof.gob.mx/normasOficiales/9496/stps/stps.html |

### Perú

| Institución | Contenido | URL |
|---|---|---|
| SINIA – MINAM | Ley 28256 y Reglamento Nacional de Transporte Terrestre de Materiales y Residuos Peligrosos | https://sinia.minam.gob.pe/ |

### Venezuela

| Institución | Contenido | URL |
|---|---|---|
| Asamblea Nacional | Ley sobre Sustancias, Materiales y Desechos Peligrosos (Ley N° 55) | https://www.asambleanacional.gob.ve/leyes/sancionadas/ley-sobre-sustancias-materiales-y-desechos-peligrosos |
| MINEC | Marco jurídico y registro de empresas manejadoras | https://www.minec.gob.ve/minec/marco-juridico/ |
| INTT | NTF 3060:2011 — clasificación e identificación para transporte terrestre | https://www.intt.gob.ve/ |
| SENCAMER | COVENIN 2670-2001 — guía de respuesta a emergencias | https://sigbs.sencamer.gob.ve/ |
| RESQUIMC | Normativas legales vigentes | https://www.resquimc.gob.ve/normativas-legales/ |

### Cuba

El usuario dispone de la **base de legislación cubana completa en local**. No se realizan búsquedas web para este país. El registro `cu-base-local` en `data/legislation.json` está marcado como `base_local_usuario` y pendiente de importación.

### Referencia (no aplicable)

| Institución | Contenido | URL |
|---|---|---|
| OSHA (EE. UU.) | Ejemplo de Hoja de Datos de Seguridad | https://www.osha.gov/sites/default/files/2022-01/Ejemplo%20de%20Hoja%20de%20Datos%20de%20Seguridad.pdf |

## Enlaces legislativos directos verificados con scraper

Se sustituyeron varios enlaces de portal por URLs directas, confirmadas con HTTP 200 y título coincidente:

- **Colombia**: Decreto 4741/2005 (Minvivienda, ANLA) y Decreto 1079/2015 (MinTransporte).
- **España**: RD 97/2014, Ley 9/2013, RD 656/2017 (vigente) y RD 379/2001 (histórico) en BOE; Directiva 2008/68/CE en EUR-Lex y versión española en BOE; portal de normativa vigente del MITMA.
- **México**: NOM-005-STPS-1998 en DOF y LGPGIR en PROFEPA.
- **Perú**: Ley 28256 en la Plataforma del Estado Peruano (`gob.pe`), con SINIA-MINAM como alternativa.
- **Venezuela**: Ley N° 55 en la Asamblea Nacional.

Aun con 403: UNECE, SINIA-MINAM y ECHA. Con timeout: chemicalsafety.com y `diputados.gob.mx`.

## Fuentes de datos de sustancias (decisión del usuario)

El usuario aceptó estas fuentes como **fuente de datos de sustancias y FDS**. Las que no son gubernamentales **no se usan como fuente normativa oficial**.

| Fuente | Naturaleza | URL | Verificación |
|---|---|---|---|
| PubChem (NIH, EE. UU.) | Gobierno | https://pubchem.ncbi.nlm.nih.gov/ | `dominio_snippet` |
| ChemSpider (Royal Society of Chemistry) | Institución científica reconocida | https://www.chemspider.com/ | `dominio_snippet` |
| Chemical Safety | Agregador comercial | https://chemicalsafety.com/es/sds-search-page-es/ | `dominio_snippet` — timeout en esta sesión |
| SDS Manager | Agregador comercial | https://sdsmanager.es/search/ | `contenido_verificado` — HTTP 200 con scraper |
| ChemBlink | Agregador comercial | https://www.chemblink.com/ | `dominio_snippet` |
| VWR (Avantor) | Fabricante/distribuidor | https://us.vwr.com/store/search/searchMSDS.jsp | `dominio_snippet` |
| Fisher Scientific (Thermo Fisher) | Fabricante/distribuidor | https://www.fishersci.com/us/en/catalog/search/sdshome.html | `dominio_snippet` |
| TCI Chemicals | Fabricante | https://www.tcichemicals.com/US/en/sds/ | `dominio_snippet` |
| Fluorochem | Fabricante | http://www.fluorochem.co.uk/ | `dominio_snippet` |

`find_sds` (https://github.com/khoivan88/find_sds) es la herramienta de referencia que consulta por CAS a ChemBlink, VWR, Fisher Scientific, TCI Chemicals, ChemicalSafety y Fluorochem. No incluye base de datos propia: descarga los PDF a una carpeta local.

Los seis repositorios clonados se mantienen en `referencias/`, carpeta excluida de git mediante `.gitignore`. Ninguno contiene una base de datos de sustancias; solo plantillas (sdsforge), esquemas XSD (esdscom), reglas embebidas (x3-sds-reader) o documentación (SGAcopilot).

## Fuentes rechazadas

| Fuente | Motivo | Reemplazo oficial |
|---|---|---|
| ACCUDIRE | Empresa privada (plataforma e-ADR) | UNECE (ADR) + BOE/MITMA |
| LUR Consultores | Consultora privada | Enlaces directos a UNECE, OMI y OPCW |
| Escuela de Seguridad Química / seguridadquimica.com | Empresa privada | Anexo 4 del GHS (UNECE) para las 16 secciones de la FDS |
| Instituto Ambiental (Perú) | Instituto privado | SINIA – MINAM |
| Wikipedia | No oficial | — |
| Prezi, Scribd | Repositorios de presentaciones | — |
| preciocombustible, simcoconsulting, sparxlogistics, Savino Del Bene, DSV, terminal98 | Blogs y empresas de logística | — |
| chemicalsafety.com, sdsmanager.es | Agregadores comerciales de FDS | Aceptados por decisión del usuario como fuentes de datos (ver sección "Fuentes de datos de sustancias") |

## Limitaciones de la búsqueda

Durante la ampliación de búsqueda los motores bloquearon el acceso automatizado:

- **DuckDuckGo**: desafío CAPTCHA con `webfetch`, pero **accesible mediante el scraper local** (`scrapling`/`curl_cffi`) con impersonación `chrome120`. Ya se pudo buscar normativa (p. ej., localizar el RD 97/2014 en BOE).
- **Bing**: devolvió "sin resultados" o resultados sin relación con la consulta.
- **Brave Search y Mojeek**: HTTP 403 / desafío JavaScript.
- **Herramienta `websearch` integrada**: error de ejecución.

Con el scraper local se logró acceso real a DOF, Asamblea Nacional, ICSC (OIT), sdsmanager.es y DuckDuckGo. UNECE, SINIA-MINAM y ECHA siguen devolviendo 403 (Cloudflare / Azure WAF), y chemicalsafety.com dio timeout de conexión.

Los portales marcados como `portal_validar` son URLs canónicas conocidas de los organismos oficiales, **no resultados confirmados en esta sesión**. Deben validarse en navegador antes de publicarse en la aplicación. No se han inventado números de norma ni enlaces directos a instrumentos no verificados; cuando no se obtuvo una URL directa, se enlaza al portal oficial y se indica la búsqueda a realizar.

## Reactividad química: NOAA/EPA CAMEO Chemicals (Capa 1)

Para predecir reacciones entre sustancias se usa la metodología de **CAMEO Chemicals**, desarrollada por la NOAA (Office of Response and Restoration) junto con la EPA:

- **Grupos reactivos:** 68 categorías estructurales; cada sustancia se asigna a una o más.
- **Predicción por pares:** se combinan los grupos de dos sustancias; el resultado es *Incompatible*, *Caution* o *Compatible*, con peligros (corrosivo, gas, calor, explosivo, tóxico) y gases subproductos.
- **Trazabilidad:** cada par incluye citas bibliográficas concretas (Bretherick's Handbook, Sax's, Mellor, Kirk-Othmer, MCA Case Histories).
- **Condiciones del escenario:** proporciones similares, mezcla completa, 1 atm, 24–49 °C y sin impurezas catalíticas. No predice reacciones de tres o más sustancias.

Fuentes: https://cameochemicals.noaa.gov/help/reactivity/how_reactivity_is_predicted.htm y https://cameochemicals.noaa.gov/reactivity/documentation/RG1-RG10

**Uso de datos:** los datos de reactividad son elaborados por NOAA/EPA y se usan con atribución y el descargo de responsabilidad de la agencia. No se copian los campos con copyright de terceros (CAS, NFPA, DuPont, AEGL, ERPG). Ver https://cameochemicals.noaa.gov/help/reference/terms_and_conditions.htm
