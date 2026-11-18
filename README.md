# Sustancias peligrosas

Base de conocimiento sobre sustancias peligrosas: marco legal por país, SGA/GHS, fichas de datos de seguridad, convenios internacionales y regulación del transporte.

Repositorio: https://github.com/CarlosAntonioAceaJimenezCD/sustancias-peligrosas

## Estado actual

Fase de **capa de datos**. Todavía no hay aplicación: primero se construye y valida la información, después se elegirá el stack para el sitio web y las aplicaciones multiplataforma.

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

## Flujo previsto de la aplicación

1. El usuario elige el **país** (o "Marco internacional").
2. Elige un **apartado** de `data/topics.json`.
3. Dentro del apartado, elige una **sustancia** para consultar su información.
4. La interfaz muestra el marco legal aplicable y enlaza siempre a la fuente oficial.

## Países incluidos

- **Marco internacional** — SGA/GHS, convenios y Reglamentación Modelo de la ONU.
- **Colombia** — MinAmbiente, DAPRE, DIMAR.
- **España** — INSST, BOE, MITECO, MITMA, ECHA.
- **México** — DOF/STPS, SEMARNAT, SCT.
- **Perú** — SINIA-MINAM, El Peruano, MTC.
- **Venezuela** — Asamblea Nacional, MINEC, INTT, SENCAMER, RESQUIMC.
- **Cuba** — base local del usuario, pendiente de importación.
- **Estados Unidos** — referencia complementaria (OSHA), no marco aplicable.

## Pendiente

- Validar en navegador los enlaces con `verificacion: portal_validar`.
- Importar la base de legislación cubana del usuario y definir su formato.
- Definir el modelo de datos de sustancias (identificadores CAS/CE/ONU, clasificación SGA y enlaces a FDS oficiales).
- Elegir el stack del sitio web y de las aplicaciones Android, iOS, macOS, Windows y Linux.
