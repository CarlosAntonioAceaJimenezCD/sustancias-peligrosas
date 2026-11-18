/* Generado automaticamente desde data/*.json. No editar a mano. */
window.DATA = {
  "countries": [
    {
      "id": "internacional",
      "nombre": "Marco internacional",
      "codigo": "INT",
      "tipo": "global",
      "idioma": "es",
      "bandera": "🌐",
      "saludo": "Bienvenido al marco internacional",
      "descripcion": "SGA/GHS, convenios de la ONU y Reglamentacion Modelo para el Transporte de Mercancias Peligrosas. Aplica como base comun a todos los paises."
    },
    {
      "id": "colombia",
      "nombre": "Colombia",
      "codigo": "CO",
      "tipo": "nacional",
      "idioma": "es",
      "bandera": "🇨🇴",
      "saludo": "Bienvenido a Colombia",
      "descripcion": "Marco legal colombiano de sustancias peligrosas, residuos y transporte."
    },
    {
      "id": "espana",
      "nombre": "España",
      "codigo": "ES",
      "tipo": "nacional",
      "bloque": "union_europea",
      "idioma": "es",
      "bandera": "🇪🇸",
      "saludo": "Bienvenido a España",
      "descripcion": "Legislacion espanola de riesgos quimicos, PRL, REACH-CLP y transporte de mercancias peligrosas."
    },
    {
      "id": "mexico",
      "nombre": "México",
      "codigo": "MX",
      "tipo": "nacional",
      "idioma": "es",
      "bandera": "🇲🇽",
      "saludo": "Bienvenido a México",
      "descripcion": "Normas Oficiales Mexicanas de la STPS y la SEMARNAT."
    },
    {
      "id": "peru",
      "nombre": "Perú",
      "codigo": "PE",
      "tipo": "nacional",
      "idioma": "es",
      "bandera": "🇵🇪",
      "saludo": "Bienvenido al Perú",
      "descripcion": "Normativa del MINAM sobre materiales y residuos peligrosos y su transporte terrestre."
    },
    {
      "id": "venezuela",
      "nombre": "Venezuela",
      "codigo": "VE",
      "tipo": "nacional",
      "idioma": "es",
      "bandera": "🇻🇪",
      "saludo": "Bienvenido a Venezuela",
      "descripcion": "Ley sobre Sustancias, Materiales y Desechos Peligrosos y normativa tecnica de transporte."
    },
    {
      "id": "cuba",
      "nombre": "Cuba",
      "codigo": "CU",
      "tipo": "nacional",
      "origen_legislacion": "base_local_usuario",
      "estado": "pendiente_importacion",
      "idioma": "es",
      "bandera": "🇨🇺",
      "saludo": "Bienvenido a Cuba",
      "descripcion": "El usuario dispone de la legislacion cubana completa en una base local. No se realizan busquedas web para este pais; la base se importara cuando se defina su formato."
    },
    {
      "id": "eeuu",
      "nombre": "Estados Unidos",
      "codigo": "US",
      "tipo": "referencia",
      "aplicable": false,
      "idioma": "en",
      "bandera": "🇺🇸",
      "saludo": "Welcome to the United States",
      "descripcion": "Referencia complementaria (OSHA). No es marco aplicable en los paises del proyecto."
    }
  ],
  "topics": [
    {
      "id": "sga",
      "nombre": "SGA / Clasificación de peligros",
      "descripcion": "Sistema Globalmente Armonizado: clasificacion de peligros fisicos, para la salud y para el medio ambiente; pictogramas, palabras de advertencia, frases H y P, etiquetado.",
      "fuente_clave": "unece-ghs-rev11",
      "gobierno_o_institucion": "UNECE (ONU), MinAmbiente, DOF-STPS, LATU"
    },
    {
      "id": "fds",
      "nombre": "Fichas de Datos de Seguridad (FDS/SDS)",
      "descripcion": "Estructura de las 16 secciones de la FDS segun el SGA, diferencias frente a la ficha tecnica y antiguas MSDS, y bases oficiales de fichas por sustancia.",
      "fuente_clave": "oit-icsc",
      "gobierno_o_institucion": "OIT, INSST, OSHA (referencia)"
    },
    {
      "id": "convenios",
      "nombre": "Convenios internacionales",
      "descripcion": "Convenios de Basilea, Rotterdam, Estocolmo, Minamata y convenios maritimos de la OMI (HNS, MARPOL, SOLAS).",
      "fuente_clave": "pic-convenios",
      "gobierno_o_institucion": "Convencion de Rotterdam (PIC), OMI, DIMAR"
    },
    {
      "id": "transporte",
      "nombre": "Regulación del transporte",
      "descripcion": "Reglamentacion Modelo de la ONU (Libro Naranja), ADR (carretera), RID (ferrocarril), ADN (vias navegables), IMDG (mar) e IATA DGR (aire).",
      "fuente_clave": "unece-dangerous-goods",
      "gobierno_o_institucion": "UNECE, OMI, MITMA, INTT, SENCAMER, MINAM"
    },
    {
      "id": "legislacion",
      "nombre": "Legislación nacional",
      "descripcion": "Manipulacion, almacenamiento, control y comercializacion de sustancias peligrosas segun el pais seleccionado.",
      "fuente_clave": "insst-normativa-quimicos",
      "gobierno_o_institucion": "BOE, INSST, DOF, SINIA-MINAM, Asamblea Nacional, base local de Cuba"
    }
  ],
  "sources": [
    {
      "id": "unece-ghs-rev11",
      "pais": "internacional",
      "institucion": "UNECE (Naciones Unidas)",
      "tipo_institucion": "organismo_internacional",
      "tema": "sga",
      "titulo": "Sistema Globalmente Armonizado de Clasificacion y Etiquetado de Productos Quimicos (GHS), Rev.11 (2025)",
      "url": "https://unece.org/transport/dangerous-goods/ghs-rev11-2025",
      "descripcion": "Edicion vigente del SGA: clasificacion de peligros, etiquetado, pictogramas, frases H y fichas de datos de seguridad.",
      "verificacion": "dominio_snippet",
      "oficial": true,
      "nota": "El sitio devuelve 403 al lector automatico."
    },
    {
      "id": "unece-dangerous-goods",
      "pais": "internacional",
      "institucion": "UNECE (Naciones Unidas)",
      "tipo_institucion": "organismo_internacional",
      "tema": "transporte",
      "titulo": "Centro de mercancias peligrosas de la UNECE",
      "url": "https://unece.org/transport/dangerous-goods",
      "descripcion": "Hub oficial de la Reglamentacion Modelo (Libro Naranja), Manual de Pruebas y Criterios, ADR, ADN y SGA.",
      "verificacion": "dominio_snippet",
      "oficial": true
    },
    {
      "id": "unece-manual-pruebas",
      "pais": "internacional",
      "institucion": "UNECE (Naciones Unidas)",
      "tipo_institucion": "organismo_internacional",
      "tema": "transporte",
      "titulo": "Manual de Pruebas y Criterios de las Naciones Unidas",
      "url": "https://unece.org/transport/dangerous-goods/rev7-files",
      "descripcion": "Criterios y metodos de ensayo para clasificar mercancias peligrosas y peligros fisicos del SGA.",
      "verificacion": "dominio_snippet",
      "oficial": true
    },
    {
      "id": "unece-adr",
      "pais": "internacional",
      "institucion": "UNECE (Naciones Unidas)",
      "tipo_institucion": "organismo_internacional",
      "tema": "transporte",
      "titulo": "Acuerdo Europeo sobre el Transporte Internacional de Mercancias Peligrosas por Carretera (ADR)",
      "url": "https://unece.org/about-adr",
      "descripcion": "Texto oficial y ediciones vigentes del ADR.",
      "verificacion": "dominio_snippet",
      "oficial": true
    },
    {
      "id": "imo-convenios",
      "pais": "internacional",
      "institucion": "Organizacion Maritima Internacional (OMI)",
      "tipo_institucion": "organismo_internacional",
      "tema": "convenios",
      "titulo": "Lista de convenios de la OMI",
      "url": "https://www.imo.org/es/about/conventions/pages/listofconventions.aspx",
      "descripcion": "SOLAS, MARPOL, STCW, Convenio HNS 1996 y su Protocolo de 2010, CLC, FUND, BWM y AFS.",
      "verificacion": "contenido_verificado",
      "oficial": true
    },
    {
      "id": "imo-imdg",
      "pais": "internacional",
      "institucion": "Organizacion Maritima Internacional (OMI)",
      "tipo_institucion": "organismo_internacional",
      "tema": "transporte",
      "titulo": "Codigo Maritimo Internacional de Mercancias Peligrosas (IMDG)",
      "url": "https://www.imo.org/es/publications/Pages/IMDG%20Code.aspx",
      "descripcion": "Edicion 2024 con enmienda 42-24, obligatoria desde el 1 de enero de 2026.",
      "verificacion": "contenido_verificado",
      "oficial": true
    },
    {
      "id": "oit-icsc",
      "pais": "internacional",
      "institucion": "Oficina Internacional del Trabajo (OIT)",
      "tipo_institucion": "organismo_internacional",
      "tema": "fds",
      "titulo": "Fichas Internacionales de Seguridad Quimica (ICSC)",
      "url": "https://chemicalsafety.ilo.org/dyn/icsc/showcard.listCards3?p_lang=es",
      "descripcion": "Fichas de seguridad y salud de sustancias quimicas producidas conjuntamente por la OMS y la OIT.",
      "verificacion": "dominio_snippet",
      "oficial": true
    },
    {
      "id": "pic-convenios",
      "pais": "internacional",
      "institucion": "Convencion de Rotterdam (Secretaria PIC)",
      "tipo_institucion": "organismo_internacional",
      "tema": "convenios",
      "titulo": "Los convenios sobre productos quimicos y desechos peligrosos",
      "url": "https://www.pic.int/",
      "descripcion": "Marco conjunto de Basilea, Rotterdam y Estocolmo para la gestion del ciclo de vida de los productos quimicos.",
      "verificacion": "dominio_snippet",
      "oficial": true
    },
    {
      "id": "hns-convention",
      "pais": "internacional",
      "institucion": "Convencion HNS (OMI)",
      "tipo_institucion": "organismo_internacional",
      "tema": "convenios",
      "titulo": "Convenio internacional sobre responsabilidad e indemnizacion por danos del transporte maritimo de sustancias nocivas y potencialmente peligrosas",
      "url": "https://www.hnsconvention.org/es/el-convenio/",
      "descripcion": "Texto oficial del Convenio SNP/HNS, 1996, y su Protocolo de 2010.",
      "verificacion": "dominio_snippet",
      "oficial": true
    },
    {
      "id": "opcw-transporte",
      "pais": "internacional",
      "institucion": "Organizacion para la Prohibicion de las Armas Quimicas (OPCW)",
      "tipo_institucion": "organismo_internacional",
      "tema": "transporte",
      "titulo": "Directrices orientativas para el transporte seguro y protegido de sustancias quimicas peligrosas",
      "url": "https://www.opcw.org/sites/default/files/documents/2025/06/Final%20version%20guidelines_rev%206.3-%20clean%20Spanish.pdf",
      "descripcion": "Directrices de 2025 elaboradas a partir de talleres de expertos.",
      "verificacion": "dominio_snippet",
      "oficial": true
    },
    {
      "id": "latu-ghs-sga",
      "pais": "internacional",
      "institucion": "LATU (Laboratorio Tecnologico del Uruguay)",
      "tipo_institucion": "institucion_publica",
      "tema": "sga",
      "titulo": "Clasificacion de peligros segun el Sistema Globalmente Armonizado",
      "url": "https://ghs-sga.com/clasificacion-de-peligros-segun-sga/",
      "descripcion": "Explicacion tecnica en espanol de los tres pasos de identificacion de peligros y tablas de ensayos por clase.",
      "verificacion": "contenido_verificado",
      "oficial": true,
      "nota": "Unica fuente no gubernamental conservada: LATU es ente publico uruguayo, centro coordinador del Convenio de Basilea y centro regional del Convenio de Estocolmo para America Latina y el Caribe."
    },
    {
      "id": "insst-normativa-quimicos",
      "pais": "espana",
      "institucion": "Instituto Nacional de Seguridad y Salud en el Trabajo (INSST)",
      "tipo_institucion": "gobierno",
      "tema": "legislacion",
      "titulo": "Normativa nacional de riesgos quimicos",
      "url": "https://www.insst.es/normativa/riesgos-quimicos",
      "descripcion": "Ley 31/1995 de Prevencion de Riesgos Laborales, RD 374/2001 de agentes quimicos, RD 363/1995, RD 145/1989 de puertos, RD 1566/1999 de consejeros de seguridad, guias tecnicas de almacenamiento y accesos a REACH-CLP.",
      "verificacion": "contenido_verificado",
      "oficial": true
    },
    {
      "id": "insst-fisq",
      "pais": "espana",
      "institucion": "Instituto Nacional de Seguridad y Salud en el Trabajo (INSST)",
      "tipo_institucion": "gobierno",
      "tema": "fds",
      "titulo": "Fichas Internacionales de Seguridad Quimica (FISQ)",
      "url": "https://www.insst.es/documentacion/colecciones-tecnicas/fisq",
      "descripcion": "Version espanola de las ICSC. Buscable por numero FISQ, CAS, CE, ONU o nombre de la sustancia.",
      "verificacion": "contenido_verificado",
      "oficial": true
    },
    {
      "id": "boe",
      "pais": "espana",
      "institucion": "Boletin Oficial del Estado (BOE)",
      "tipo_institucion": "gobierno",
      "tema": "legislacion",
      "titulo": "Boletin Oficial del Estado",
      "url": "https://www.boe.es/",
      "descripcion": "Textos consolidados y vigentes de leyes, reales decretos y ordenes ministeriales.",
      "verificacion": "dominio_snippet",
      "oficial": true
    },
    {
      "id": "miteco-reach-clp",
      "pais": "espana",
      "institucion": "Ministerio para la Transicion Ecologica (MITECO)",
      "tipo_institucion": "gobierno",
      "tema": "legislacion",
      "titulo": "Portal de informacion REACH-CLP",
      "url": "https://www.miteco.gob.es/es/calidad-y-evaluacion-ambiental/temas/productos-quimicos/portal-reach-clp/",
      "descripcion": "Registro, evaluacion, autorizacion y restriccion de sustancias quimicas; clasificacion, etiquetado y envasado.",
      "verificacion": "dominio_snippet",
      "oficial": true
    },
    {
      "id": "echa",
      "pais": "espana",
      "institucion": "Agencia Europea de Sustancias y Mezclas Quimicas (ECHA)",
      "tipo_institucion": "organismo_internacional",
      "tema": "legislacion",
      "titulo": "Guías de orientacion sobre REACH y CLP",
      "url": "https://echa.europa.eu/es/",
      "descripcion": "Guías de aplicacion de los reglamentos REACH y CLP de la Union Europea.",
      "verificacion": "dominio_snippet",
      "oficial": true
    },
    {
      "id": "mitma-mercancias",
      "pais": "espana",
      "institucion": "Ministerio de Transportes y Movilidad Sostenible (MITMA)",
      "tipo_institucion": "gobierno",
      "tema": "transporte",
      "titulo": "Transporte de mercancias peligrosas y perecederas",
      "url": "https://www.mitma.es/transporte-terrestre/mercancias-peligrosas-y-perecederas",
      "descripcion": "Normativa y gestion del transporte terrestre de mercancias peligrosas en Espana.",
      "verificacion": "dominio_snippet",
      "oficial": true
    },
    {
      "id": "minambiente-clasificacion-sga",
      "pais": "colombia",
      "institucion": "Ministerio de Ambiente y Desarrollo Sostenible (MinAmbiente)",
      "tipo_institucion": "gobierno",
      "tema": "sga",
      "titulo": "Guia de clasificacion de peligros segun el SGA (2017)",
      "url": "https://quimicos.minambiente.gov.co/wp-content/uploads/2021/06/4.-Guia-de-clasificacion-de-peligros-segun-SGA-2017.pdf",
      "descripcion": "Guia oficial colombiana para clasificar sustancias quimicas segun el SGA.",
      "verificacion": "dominio_snippet",
      "oficial": true,
      "nota": "El PDF no es procesable por el lector automatico."
    },
    {
      "id": "minambiente-comunicacion-sga",
      "pais": "colombia",
      "institucion": "Ministerio de Ambiente y Desarrollo Sostenible (MinAmbiente)",
      "tipo_institucion": "gobierno",
      "tema": "sga",
      "titulo": "Guia de comunicacion de peligros segun el SGA (2017)",
      "url": "https://quimicos.minambiente.gov.co/wp-content/uploads/2021/06/5.-Guia-de-comunicacion-de-peligros-segun-el-SGA-2017.pdf",
      "descripcion": "Etiquetado, pictogramas, palabras de advertencia, frases H y P y fichas de datos de seguridad.",
      "verificacion": "dominio_snippet",
      "oficial": true,
      "nota": "El PDF no es procesable por el lector automatico."
    },
    {
      "id": "dapre-manipulacion-almacenamiento",
      "pais": "colombia",
      "institucion": "Departamento Administrativo de la Presidencia de la Republica (DAPRE)",
      "tipo_institucion": "gobierno",
      "tema": "legislacion",
      "titulo": "Guia de manipulacion y almacenamiento de sustancias quimicas (G-TH-09)",
      "url": "https://dapre.presidencia.gov.co/dapre/DocumentosSIGEPRE/G-TH-09-Guia-Manipulacion-Almacenamiento-Sustancias-Quimicas.pdf",
      "descripcion": "Peligro quimico derivado de la manipulacion, uso, almacenamiento y transporte por parte de los operarios.",
      "verificacion": "dominio_snippet",
      "oficial": true,
      "nota": "El PDF no es procesable por el lector automatico."
    },
    {
      "id": "dimar-convenios",
      "pais": "colombia",
      "institucion": "Direccion General Maritima (DIMAR)",
      "tipo_institucion": "gobierno",
      "tema": "convenios",
      "titulo": "Convenios internacionales implementados por Colombia",
      "url": "https://www.dimar.mil.co/proteccion_del_medio_marino/convenios-internacionales",
      "descripcion": "Convenios de la OMI aplicados por Colombia como Estado miembro.",
      "verificacion": "dominio_snippet",
      "oficial": true
    },
    {
      "id": "dof-nom-005",
      "pais": "mexico",
      "institucion": "Diario Oficial de la Federacion / Secretaria del Trabajo y Prevision Social",
      "tipo_institucion": "gobierno",
      "tema": "legislacion",
      "titulo": "NOM-005-STPS-1998 - Condiciones de seguridad e higiene para el manejo, transporte y almacenamiento de sustancias quimicas peligrosas",
      "url": "https://dof.gob.mx/nota_detalle_popup.php?codigo=2052048",
      "descripcion": "Norma oficial mexicana sobre manejo, transporte y almacenamiento de sustancias quimicas peligrosas en los centros de trabajo.",
      "verificacion": "dominio_snippet",
      "oficial": true
    },
    {
      "id": "dof-nom-018",
      "pais": "mexico",
      "institucion": "Diario Oficial de la Federacion / Secretaria del Trabajo y Prevision Social",
      "tipo_institucion": "gobierno",
      "tema": "sga",
      "titulo": "NOM-018-STPS-2015 - Sistema armonizado para la identificacion y comunicacion de peligros y riesgos",
      "url": "https://dof.gob.mx/normasOficiales/9496/stps/stps.html",
      "descripcion": "Implementacion del SGA en centros de trabajo en Mexico.",
      "verificacion": "dominio_snippet",
      "oficial": true
    },
    {
      "id": "sinia-ley-28256",
      "pais": "peru",
      "institucion": "Sistema de Informacion Nacional de Ambiente del Ministerio del Ambiente (SINIA-MINAM)",
      "tipo_institucion": "gobierno",
      "tema": "legislacion",
      "titulo": "Ley N 28256 - Ley que regula el transporte terrestre de materiales y residuos peligrosos",
      "url": "https://sinia.minam.gob.pe/normas/ley-que-regula-transporte-terrestre-materiales-residuos-peligrosos",
      "descripcion": "Ley peruana que regula el transporte terrestre de materiales y residuos peligrosos.",
      "verificacion": "dominio_snippet",
      "oficial": true,
      "nota": "El sitio devuelve 403 al lector automatico."
    },
    {
      "id": "sinia-reglamento-transporte",
      "pais": "peru",
      "institucion": "Sistema de Informacion Nacional de Ambiente del Ministerio del Ambiente (SINIA-MINAM)",
      "tipo_institucion": "gobierno",
      "tema": "transporte",
      "titulo": "Reglamento Nacional de Transporte Terrestre de Materiales y Residuos Peligrosos",
      "url": "https://sinia.minam.gob.pe/normas/aprueban-reglamento-nacional-transporte-terrestre-materiales-residuos",
      "descripcion": "Reglamento que desarrolla la Ley 28256 en el Peru.",
      "verificacion": "dominio_snippet",
      "oficial": true,
      "nota": "El sitio devuelve 403 al lector automatico."
    },
    {
      "id": "asamblea-ley-55",
      "pais": "venezuela",
      "institucion": "Asamblea Nacional de Venezuela",
      "tipo_institucion": "gobierno",
      "tema": "legislacion",
      "titulo": "Ley sobre Sustancias, Materiales y Desechos Peligrosos (Ley N 55, Gaceta Oficial 5.554 Extraordinario, 13/11/2001)",
      "url": "https://www.asambleanacional.gob.ve/leyes/sancionadas/ley-sobre-sustancias-materiales-y-desechos-peligrosos",
      "descripcion": "Regula la generacion, uso, recoleccion, almacenamiento, transporte, tratamiento y disposicion final de sustancias, materiales y desechos peligrosos.",
      "verificacion": "dominio_snippet",
      "oficial": true,
      "nota": "El sitio no responde al lector automatico."
    },
    {
      "id": "minec-marco-juridico",
      "pais": "venezuela",
      "institucion": "Ministerio del Poder Popular de Economia y Comercio (MINEC)",
      "tipo_institucion": "gobierno",
      "tema": "legislacion",
      "titulo": "Marco juridico y registro de empresas manejadoras de sustancias peligrosas",
      "url": "https://www.minec.gob.ve/minec/marco-juridico/",
      "descripcion": "Ley N 55, decretos y tramite de registro y autorizacion de empresas manejadoras.",
      "verificacion": "dominio_snippet",
      "oficial": true
    },
    {
      "id": "intt-ntf-3060",
      "pais": "venezuela",
      "institucion": "Instituto Nacional de Transporte Terrestre (INTT)",
      "tipo_institucion": "gobierno",
      "tema": "transporte",
      "titulo": "NTF 3060:2011 - Materiales peligrosos: clasificacion e identificacion para su transporte terrestre",
      "url": "https://www.intt.gob.ve/wp-content/uploads/2026/08/ntf-3060-2011-materiales-peligrosos-clasificacion-e-identificacion-para-su-transporte-terrestre.pdf",
      "descripcion": "Norma tecnica venezolana de clasificacion, simbolos y senales de identificacion.",
      "verificacion": "dominio_snippet",
      "oficial": true
    },
    {
      "id": "sencamer-covenin-2670",
      "pais": "venezuela",
      "institucion": "Servicio Autonomo Nacional de Normalizacion, Certificacion y Metrologia (SENCAMER)",
      "tipo_institucion": "gobierno",
      "tema": "transporte",
      "titulo": "COVENIN 2670-2001 - Materiales peligrosos: guia de respuesta a emergencias",
      "url": "https://sigbs.sencamer.gob.ve/cgi-bin/koha/opac-detail.pl?biblionumber=549",
      "descripcion": "Guia de respuesta a emergencias para materiales peligrosos.",
      "verificacion": "dominio_snippet",
      "oficial": true
    },
    {
      "id": "resquimc-normativas",
      "pais": "venezuela",
      "institucion": "Registro Nacional Unico de Operadores de Sustancias Quimicas Controladas (RESQUIMC)",
      "tipo_institucion": "gobierno",
      "tema": "legislacion",
      "titulo": "Normativas legales vigentes de Venezuela",
      "url": "https://www.resquimc.gob.ve/normativas-legales/",
      "descripcion": "Regulacion vigente, gacetas oficiales y resoluciones obligatorias.",
      "verificacion": "dominio_snippet",
      "oficial": true
    },
    {
      "id": "chemicalsafety-com",
      "pais": "internacional",
      "institucion": "Chemical Safety (chemicalsafety.com)",
      "tipo_institucion": "agregador_comercial",
      "tema": "fds",
      "titulo": "Base de datos de fichas de datos de seguridad (SDS) de fabricantes",
      "url": "https://chemicalsafety.com/es/sds-search-page-es/",
      "descripcion": "Buscador de FDS de multiples fabricantes. Usado como una de las fuentes del repo find_sds.",
      "verificacion": "dominio_snippet",
      "oficial": false,
      "categoria": "base_de_datos_fds",
      "aplicable": true,
      "nota": "Aceptado por decision del usuario como fuente de datos de FDS y como referencia. No es fuente normativa oficial. Acceso fallido en esta sesion (timeout de conexion); reintentar mas tarde."
    },
    {
      "id": "sdsmanager-es",
      "pais": "internacional",
      "institucion": "SDS Manager (sdsmanager.es)",
      "tipo_institucion": "agregador_comercial",
      "tema": "fds",
      "titulo": "Biblioteca online de fichas de seguridad",
      "url": "https://sdsmanager.es/search/",
      "descripcion": "Biblioteca de FDS con busqueda por nombre, CAS y codigo de barras, gestion de versiones y acceso por QR.",
      "verificacion": "contenido_verificado",
      "oficial": false,
      "categoria": "base_de_datos_fds",
      "aplicable": true,
      "nota": "Aceptado por decision del usuario como fuente de datos de FDS y como referencia. No es fuente normativa oficial. Pagina abierta y verificada con scraper (HTTP 200)."
    },
    {
      "id": "pubchem",
      "pais": "internacional",
      "institucion": "National Center for Biotechnology Information (NCBI), National Institutes of Health - Estados Unidos",
      "tipo_institucion": "gobierno",
      "tema": "fds",
      "titulo": "PubChem - Base de datos de sustancias quimicas",
      "url": "https://pubchem.ncbi.nlm.nih.gov/",
      "descripcion": "Base de datos publica de sustancias quimicas con identificadores CAS, formulas, propiedades y clasificaciones. Propuesta por SGAcopilot.",
      "verificacion": "dominio_snippet",
      "oficial": true,
      "categoria": "base_de_datos_sustancias",
      "aplicable": true,
      "nota": "Fuente gubernamental de Estados Unidos aceptada por decision del usuario como fuente de datos de sustancias. Acceso por API publica."
    },
    {
      "id": "chemspider",
      "pais": "internacional",
      "institucion": "Royal Society of Chemistry",
      "tipo_institucion": "institucion_reconocida",
      "tema": "fds",
      "titulo": "ChemSpider - Base de datos quimica",
      "url": "https://www.chemspider.com/",
      "descripcion": "Base de datos de sustancias quimicas de la Royal Society of Chemistry. Propuesta por SGAcopilot.",
      "verificacion": "dominio_snippet",
      "oficial": true,
      "categoria": "base_de_datos_sustancias",
      "aplicable": true,
      "nota": "Institucion cientifica reconocida, aceptada por decision del usuario como fuente de datos de sustancias."
    },
    {
      "id": "chemblink",
      "pais": "internacional",
      "institucion": "ChemBlink",
      "tipo_institucion": "agregador_comercial",
      "tema": "fds",
      "titulo": "ChemBlink - Base de datos de productos quimicos y FDS",
      "url": "https://www.chemblink.com/",
      "descripcion": "Fichas y enlaces a FDS por numero CAS. Una de las fuentes usadas por find_sds.",
      "verificacion": "dominio_snippet",
      "oficial": false,
      "categoria": "base_de_datos_fds",
      "aplicable": true,
      "nota": "Empresa privada aceptada por decision del usuario como fuente de datos. No es fuente normativa oficial."
    },
    {
      "id": "vwr",
      "pais": "internacional",
      "institucion": "VWR (Avantor)",
      "tipo_institucion": "fabricante",
      "tema": "fds",
      "titulo": "VWR - Buscador de fichas de seguridad",
      "url": "https://us.vwr.com/store/search/searchMSDS.jsp",
      "descripcion": "FDS de productos VWR, buscables por CAS. Fuente usada por find_sds.",
      "verificacion": "dominio_snippet",
      "oficial": false,
      "categoria": "base_de_datos_fds",
      "aplicable": true,
      "nota": "Fabricante/distribuidor privado aceptado por decision del usuario como fuente de datos. No es fuente normativa oficial."
    },
    {
      "id": "fishersci",
      "pais": "internacional",
      "institucion": "Fisher Scientific (Thermo Fisher)",
      "tipo_institucion": "fabricante",
      "tema": "fds",
      "titulo": "Fisher Scientific - Centro de hojas de datos de seguridad",
      "url": "https://www.fishersci.com/us/en/catalog/search/sdshome.html",
      "descripcion": "FDS de productos Fisher Scientific, buscables por CAS. Fuente usada por find_sds.",
      "verificacion": "dominio_snippet",
      "oficial": false,
      "categoria": "base_de_datos_fds",
      "aplicable": true,
      "nota": "Fabricante/distribuidor privado aceptado por decision del usuario como fuente de datos. No es fuente normativa oficial."
    },
    {
      "id": "tci-chemicals",
      "pais": "internacional",
      "institucion": "TCI Chemicals",
      "tipo_institucion": "fabricante",
      "tema": "fds",
      "titulo": "TCI Chemicals - Buscador de SDS",
      "url": "https://www.tcichemicals.com/US/en/sds/",
      "descripcion": "FDS de productos TCI en formato PDF. Fuente usada por find_sds.",
      "verificacion": "dominio_snippet",
      "oficial": false,
      "categoria": "base_de_datos_fds",
      "aplicable": true,
      "nota": "Fabricante privado aceptado por decision del usuario como fuente de datos. No es fuente normativa oficial."
    },
    {
      "id": "fluorochem",
      "pais": "internacional",
      "institucion": "Fluorochem",
      "tipo_institucion": "fabricante",
      "tema": "fds",
      "titulo": "Fluorochem - Buscador de productos y fichas",
      "url": "http://www.fluorochem.co.uk/",
      "descripcion": "FDS de productos Fluorochem. Fuente usada por find_sds.",
      "verificacion": "dominio_snippet",
      "oficial": false,
      "categoria": "base_de_datos_fds",
      "aplicable": true,
      "nota": "Fabricante privado aceptado por decision del usuario como fuente de datos. No es fuente normativa oficial."
    },
    {
      "id": "osha-sds-ejemplo",
      "pais": "eeuu",
      "institucion": "Occupational Safety and Health Administration (OSHA)",
      "tipo_institucion": "gobierno",
      "tema": "fds",
      "titulo": "Ejemplo de Hoja de Datos de Seguridad",
      "url": "https://www.osha.gov/sites/default/files/2022-01/Ejemplo%20de%20Hoja%20de%20Datos%20de%20Seguridad.pdf",
      "descripcion": "Plantilla oficial estadounidense de hoja de datos de seguridad.",
      "verificacion": "dominio_snippet",
      "oficial": true,
      "aplicable": false,
      "nota": "Referencia complementaria. No es norma aplicable en los paises de este proyecto."
    }
  ],
  "legislation": {
    "colombia": {
      "portales_oficiales": [
        {
          "id": "co-portal-minambiente",
          "institucion": "Ministerio de Ambiente y Desarrollo Sostenible",
          "url": "https://www.minambiente.gov.co/",
          "verificacion": "portal_validar"
        },
        {
          "id": "co-suin-juriscol",
          "institucion": "Sistema Unico de Informacion Normativa (SUIN-Juriscol)",
          "url": "https://www.suin-juriscol.gov.co/",
          "verificacion": "portal_validar"
        },
        {
          "id": "co-diario-oficial",
          "institucion": "Diario Oficial de Colombia (Ministerio del Interior)",
          "url": "https://www.diariooficial.interior.gob.co/",
          "verificacion": "portal_validar"
        }
      ],
      "instrumentos": [
        {
          "id": "co-decreto-4741-2005",
          "titulo": "Decreto 4741 de 2005 - Reglamento para la gestion de residuos peligrosos",
          "tema": "legislacion",
          "url": "https://www.minvivienda.gov.co/normativa/decreto-4741-2005",
          "verificacion": "contenido_verificado",
          "alternativas": [
            "https://www.anla.gov.co/wanla/eureka/normativa/decretos/decreto-reglamentario-4741-de-2005-gestion-integral-de-los-residuos-o-desechos-peligrosos-residuos-peligrosos-repsel"
          ]
        },
        {
          "id": "co-decreto-1079-2015",
          "titulo": "Decreto 1079 de 2015 - Decreto Unico Reglamentario del Sector Transporte",
          "tema": "transporte",
          "url": "https://mintransporte.gov.co/documentos/593/decreto-1079-de-2015-unico-reglamentario-del-sector-transporte/",
          "verificacion": "contenido_verificado"
        },
        {
          "id": "co-guia-sga-clasificacion",
          "titulo": "Guia de clasificacion de peligros segun el SGA (2017)",
          "tema": "sga",
          "url": "https://quimicos.minambiente.gov.co/wp-content/uploads/2021/06/4.-Guia-de-clasificacion-de-peligros-segun-SGA-2017.pdf",
          "verificacion": "dominio_snippet"
        },
        {
          "id": "co-guia-sga-comunicacion",
          "titulo": "Guia de comunicacion de peligros segun el SGA (2017)",
          "tema": "sga",
          "url": "https://quimicos.minambiente.gov.co/wp-content/uploads/2021/06/5.-Guia-de-comunicacion-de-peligros-segun-el-SGA-2017.pdf",
          "verificacion": "dominio_snippet"
        },
        {
          "id": "co-gth09",
          "titulo": "Guia G-TH-09 de manipulacion y almacenamiento de sustancias quimicas",
          "tema": "legislacion",
          "url": "https://dapre.presidencia.gov.co/dapre/DocumentosSIGEPRE/G-TH-09-Guia-Manipulacion-Almacenamiento-Sustancias-Quimicas.pdf",
          "verificacion": "dominio_snippet"
        }
      ]
    },
    "espana": {
      "portales_oficiales": [
        {
          "id": "es-boe",
          "institucion": "Boletin Oficial del Estado",
          "url": "https://www.boe.es/",
          "verificacion": "dominio_snippet"
        },
        {
          "id": "es-insst",
          "institucion": "Instituto Nacional de Seguridad y Salud en el Trabajo",
          "url": "https://www.insst.es/normativa/riesgos-quimicos",
          "verificacion": "contenido_verificado"
        },
        {
          "id": "es-eurlex",
          "institucion": "EUR-Lex (Union Europea)",
          "url": "https://eur-lex.europa.eu/",
          "verificacion": "portal_validar"
        }
      ],
      "instrumentos": [
        {
          "id": "es-ley-31-1995",
          "titulo": "Ley 31/1995, de Prevencion de Riesgos Laborales",
          "tema": "legislacion",
          "url": "https://www.boe.es/buscar/act.php?id=BOE-A-1995-24292",
          "verificacion": "dominio_snippet"
        },
        {
          "id": "es-rd-374-2001",
          "titulo": "Real Decreto 374/2001, proteccion frente a riesgos de agentes quimicos",
          "tema": "legislacion",
          "url": "https://www.boe.es/buscar/act.php?id=BOE-A-2001-8436",
          "verificacion": "dominio_snippet"
        },
        {
          "id": "es-rd-363-1995",
          "titulo": "Real Decreto 363/1995, clasificacion, envasado y etiquetado de sustancias peligrosas (adaptado a REACH por RD 1802/2008)",
          "tema": "legislacion",
          "url": "https://www.boe.es/buscar/act.php?id=BOE-A-1995-13535",
          "verificacion": "dominio_snippet"
        },
        {
          "id": "es-rd-145-1989",
          "titulo": "Real Decreto 145/1989, admision, manipulacion y almacenamiento de mercancias peligrosas en puertos",
          "tema": "legislacion",
          "url": "https://www.boe.es/buscar/act.php?id=BOE-A-1989-3496",
          "verificacion": "dominio_snippet"
        },
        {
          "id": "es-rd-1566-1999",
          "titulo": "Real Decreto 1566/1999, consejeros de seguridad para el transporte de mercancias peligrosas",
          "tema": "transporte",
          "url": "https://www.boe.es/buscar/act.php?id=BOE-A-1999-20640",
          "verificacion": "dominio_snippet"
        },
        {
          "id": "es-rd-97-2014",
          "titulo": "Real Decreto 97/2014, operaciones de transporte de mercancias peligrosas por carretera en territorio espanol",
          "tema": "transporte",
          "url": "https://www.boe.es/buscar/act.php?id=BOE-A-2014-2110",
          "verificacion": "contenido_verificado"
        },
        {
          "id": "es-ley-9-2013",
          "titulo": "Ley 9/2013, modificacion de la Ley 16/1987 de Ordenacion de los Transportes Terrestres (sanciones mercancias peligrosas)",
          "tema": "transporte",
          "url": "https://www.boe.es/buscar/act.php?id=BOE-A-2013-7320",
          "verificacion": "contenido_verificado"
        },
        {
          "id": "es-rd-656-2017",
          "titulo": "Real Decreto 656/2017, Reglamento de Almacenamiento de Productos Quimicos y sus ITC (norma vigente)",
          "tema": "legislacion",
          "url": "https://www.boe.es/diario_boe/txt.php?id=BOE-A-2017-8755",
          "verificacion": "contenido_verificado",
          "nota": "Sustituye al Real Decreto 379/2001."
        },
        {
          "id": "es-rd-379-2001",
          "titulo": "Real Decreto 379/2001, Reglamento de almacenamiento de productos quimicos y sus ITC (historico)",
          "tema": "legislacion",
          "url": "https://www.boe.es/buscar/doc.php?id=BOE-A-2001-8971",
          "verificacion": "contenido_verificado",
          "nota": "Referencia historica; consultar RD 656/2017 como norma vigente."
        },
        {
          "id": "es-directiva-2008-68-ce",
          "titulo": "Directiva 2008/68/CE del Parlamento Europeo y del Consejo, transporte terrestre de mercancias peligrosas",
          "tema": "transporte",
          "url": "https://eur-lex.europa.eu/eli/dir/2008/68/oj/eng",
          "verificacion": "contenido_verificado",
          "alternativas": [
            "https://www.boe.es/buscar/doc.php?id=DOUE-L-2008-81911"
          ]
        },
        {
          "id": "es-portal-normativa-mitma",
          "titulo": "Normativa vigente de transporte de mercancias peligrosas y perecederas (MITMA)",
          "tema": "transporte",
          "url": "https://www.transportes.gob.es/transporte-terrestre/normativa/normativa-vigente-transporte-de-mercancias-peligrosas-y-perecederas",
          "verificacion": "contenido_verificado",
          "tipo": "portal"
        }
      ]
    },
    "mexico": {
      "portales_oficiales": [
        {
          "id": "mx-dof",
          "institucion": "Diario Oficial de la Federacion",
          "url": "https://www.dof.gob.mx/",
          "verificacion": "dominio_snippet"
        },
        {
          "id": "mx-stps",
          "institucion": "Secretaria del Trabajo y Prevision Social",
          "url": "https://www.stps.gob.mx/",
          "verificacion": "portal_validar"
        },
        {
          "id": "mx-semarnat",
          "institucion": "Secretaria de Medio Ambiente y Recursos Naturales",
          "url": "https://www.semarnat.gob.mx/",
          "verificacion": "portal_validar"
        },
        {
          "id": "mx-sct",
          "institucion": "Secretaria de Comunicaciones y Transportes",
          "url": "https://www.sct.gob.mx/",
          "verificacion": "portal_validar"
        }
      ],
      "instrumentos": [
        {
          "id": "mx-nom-005-stps-1998",
          "titulo": "NOM-005-STPS-1998 - Condiciones de seguridad e higiene para el manejo, transporte y almacenamiento de sustancias quimicas peligrosas",
          "tema": "legislacion",
          "url": "https://dof.gob.mx/nota_detalle_popup.php?codigo=2052048",
          "verificacion": "contenido_verificado",
          "nota": "Accedido con scraper (HTTP 200, titulo DOF - Diario Oficial de la Federacion)."
        },
        {
          "id": "mx-nom-018-stps-2015",
          "titulo": "NOM-018-STPS-2015 - Sistema armonizado para la identificacion y comunicacion de peligros y riesgos",
          "tema": "sga",
          "url": "https://dof.gob.mx/normasOficiales/9496/stps/stps.html",
          "verificacion": "dominio_snippet"
        },
        {
          "id": "mx-lgpgir",
          "titulo": "Ley General para la Prevencion y Gestion Integral de los Residuos",
          "tema": "legislacion",
          "url": "https://www.gob.mx/profepa/documentos/ley-general-para-la-prevencion-y-gestion-integral-de-los-residuos-62914",
          "verificacion": "contenido_verificado",
          "nota": "Texto publicado por la Procuraduria Federal de Proteccion al Ambiente. El sitio de la Camara de Diputados (diputados.gob.mx) dio timeout en esta sesion."
        }
      ]
    },
    "peru": {
      "portales_oficiales": [
        {
          "id": "pe-sinia-minam",
          "institucion": "SINIA - Ministerio del Ambiente",
          "url": "https://sinia.minam.gob.pe/",
          "verificacion": "dominio_snippet"
        },
        {
          "id": "pe-elperuano",
          "institucion": "Diario Oficial El Peruano",
          "url": "https://www.elperuano.pe/",
          "verificacion": "portal_validar"
        },
        {
          "id": "pe-mtc",
          "institucion": "Ministerio de Transportes y Comunicaciones",
          "url": "https://www.mtc.gob.pe/",
          "verificacion": "portal_validar"
        }
      ],
      "instrumentos": [
        {
          "id": "pe-ley-28256",
          "titulo": "Ley N 28256 - Transporte terrestre de materiales y residuos peligrosos",
          "tema": "legislacion",
          "url": "https://www.gob.pe/institucion/minam/normas-legales/3597-28256",
          "verificacion": "contenido_verificado",
          "alternativas": [
            "https://sinia.minam.gob.pe/normas/ley-que-regula-transporte-terrestre-materiales-residuos-peligrosos"
          ]
        },
        {
          "id": "pe-reglamento-transporte",
          "titulo": "Reglamento Nacional de Transporte Terrestre de Materiales y Residuos Peligrosos",
          "tema": "transporte",
          "url": "https://sinia.minam.gob.pe/normas/aprueban-reglamento-nacional-transporte-terrestre-materiales-residuos",
          "verificacion": "dominio_snippet"
        },
        {
          "id": "pe-plan-contingencia",
          "titulo": "Lineamientos para la elaboracion de un Plan de Contingencia para el Transporte Terrestre de Materiales y/o Residuos Peligrosos",
          "tema": "transporte",
          "url": "https://sinia.minam.gob.pe/normas/aprueban-lineamientos-elaboracion-un-plan-contingencia-transporte",
          "verificacion": "dominio_snippet"
        },
        {
          "id": "pe-convenio-basilea",
          "titulo": "Convenio de Basilea sobre el control de los movimientos transfronterizos de desechos peligrosos",
          "tema": "convenios",
          "url": "https://sinia.minam.gob.pe/normas/convenio-basilea-control-movimientos-transfronterizos-desechos-peligrosos",
          "verificacion": "dominio_snippet"
        },
        {
          "id": "pe-convenio-rotterdam",
          "titulo": "Convenio de Rotterdam - Procedimiento de Consentimiento Fundamentado Previo",
          "tema": "convenios",
          "url": "https://sinia.minam.gob.pe/normas/convenio-rotterdam-procedimiento-consentimiento-fundamentado-previo",
          "verificacion": "dominio_snippet"
        },
        {
          "id": "pe-plan-estocolmo",
          "titulo": "Plan Nacional de Implementacion del Convenio de Estocolmo sobre Contaminantes Organicos Persistentes",
          "tema": "convenios",
          "url": "https://sinia.minam.gob.pe/documentos/plan-nacional-implementacion-convenio-estocolmo-contaminantes",
          "verificacion": "dominio_snippet"
        },
        {
          "id": "pe-lista-residuos",
          "titulo": "Anexo 4: Listado de Residuos Peligrosos - Reglamento de la Ley General de Residuos Solidos",
          "tema": "legislacion",
          "url": "https://sinia.minam.gob.pe/normas/reglamento-ley-general-residuos-solidos",
          "verificacion": "dominio_snippet"
        }
      ]
    },
    "venezuela": {
      "portales_oficiales": [
        {
          "id": "ve-asamblea",
          "institucion": "Asamblea Nacional de Venezuela",
          "url": "https://www.asambleanacional.gob.ve/",
          "verificacion": "dominio_snippet"
        },
        {
          "id": "ve-minec",
          "institucion": "Ministerio del Poder Popular de Economia y Comercio",
          "url": "https://www.minec.gob.ve/minec/marco-juridico/",
          "verificacion": "dominio_snippet"
        },
        {
          "id": "ve-resquimc",
          "institucion": "Registro Nacional Unico de Operadores de Sustancias Quimicas Controladas",
          "url": "https://www.resquimc.gob.ve/normativas-legales/",
          "verificacion": "dominio_snippet"
        }
      ],
      "instrumentos": [
        {
          "id": "ve-ley-55",
          "titulo": "Ley sobre Sustancias, Materiales y Desechos Peligrosos (Ley N 55, Gaceta Oficial 5.554 Extraordinario, 13/11/2001)",
          "tema": "legislacion",
          "url": "https://www.asambleanacional.gob.ve/leyes/sancionadas/ley-sobre-sustancias-materiales-y-desechos-peligrosos",
          "verificacion": "contenido_verificado"
        },
        {
          "id": "ve-decreto-2635",
          "titulo": "Decreto 2635 - Normas para el control de la recuperacion de materiales peligrosos y el manejo de desechos peligrosos",
          "tema": "legislacion",
          "url": "https://www.bolipuertos.gob.ve/?page_id=9965",
          "verificacion": "dominio_snippet"
        },
        {
          "id": "ve-ntf-3060",
          "titulo": "NTF 3060:2011 - Materiales peligrosos: clasificacion e identificacion para su transporte terrestre",
          "tema": "transporte",
          "url": "https://www.intt.gob.ve/wp-content/uploads/2026/08/ntf-3060-2011-materiales-peligrosos-clasificacion-e-identificacion-para-su-transporte-terrestre.pdf",
          "verificacion": "dominio_snippet"
        },
        {
          "id": "ve-covenin-2670",
          "titulo": "COVENIN 2670-2001 - Materiales peligrosos: guia de respuesta a emergencias",
          "tema": "transporte",
          "url": "https://sigbs.sencamer.gob.ve/cgi-bin/koha/opac-detail.pl?biblionumber=549",
          "verificacion": "dominio_snippet"
        }
      ]
    },
    "cuba": {
      "portales_oficiales": [],
      "instrumentos": [
        {
          "id": "cu-base-local",
          "titulo": "Base local de legislacion cubana (proporcionada por el usuario)",
          "tema": "legislacion",
          "url": "",
          "verificacion": "base_local_usuario",
          "nota": "No se realizan busquedas web. La base se importara al repositorio cuando se defina su formato."
        }
      ]
    }
  },
  "substances": [
    {
      "id": "acetona",
      "nombre": "Acetona",
      "sinonimos": [
        "Propanona",
        "Propano-2-ona"
      ],
      "cas": "67-64-1",
      "ce": "200-662-2",
      "onu": "1090",
      "peligrosidad": {
        "clasificacion": [
          "Líquido inflamable, categoría 2",
          "Irritación ocular, categoría 2",
          "Toxicidad específica en órganos diana (exposición única), categoría 3"
        ],
        "pictogramas": [
          "GHS02",
          "GHS07"
        ],
        "frases_h": [
          {
            "codigo": "H225",
            "texto": "Líquido y vapores muy inflamables."
          },
          {
            "codigo": "H319",
            "texto": "Provoca irritación ocular grave."
          },
          {
            "codigo": "H336",
            "texto": "Puede provocar somnolencia o vértigo."
          }
        ],
        "significado": "Peligro principal: incendio. Sus vapores son más pesados que el aire, pueden desplazarse por el suelo e inflamarse a distancia de la fuente. La exposición prolongada puede afectar al sistema nervioso central."
      },
      "almacenamiento": "Almacenar en lugar fresco, bien ventilado y alejado de fuentes de ignición, chispas y llama abierta. Mantener el recipiente herméticamente cerrado. Proteger de la luz solar directa. Separar de agentes oxidantes fuertes, ácidos fuertes y bases fuertes. Zona con ventilación exhaustiva y conexión a tierra de los recipientes para evitar electricidad estática.",
      "derrame": "Eliminar fuentes de ignición y ventilar la zona. Usar ropa de protección y guantes resistentes a disolventes. No vertir al alcantarillado. Absorber con material inerte (arena, vermiculita o aglutinante universal) y recoger en recipientes cerrados para residuo peligroso. Si entra en contacto con ojos, enjuagar con agua abundante al menos 15 minutos y acudir al médico.",
      "propiedades": {
        "estado_fisico": "Líquido incoloro",
        "olor": "Dulce, característico (a acetona)",
        "punto_de_inflamacion": "-18 °C (vaso cerrado)",
        "punto_de_ebullicion": "56 °C",
        "densidad": "0,79 g/cm³ (20 °C)",
        "solubilidad_en_agua": "Miscible",
        "presion_de_vapor": "24 kPa (20 °C)"
      },
      "interacciones": {
        "incompatibilidades": [
          "Agentes oxidantes fuertes (peróxidos, permanganatos)",
          "Ácidos fuertes",
          "Bases fuertes",
          "Materiales plásticos no resistentes a disolventes"
        ],
        "nota": "Puede formar mezclas explosivas con el aire. El contacto con oxidantes fuertes puede provocar incendio o explosión."
      },
      "fuentes": [
        {
          "titulo": "Base de datos ICSC (OIT/OMS)",
          "url": "https://chemicalsafety.ilo.org/dyn/icsc/showcard.listCards3?p_lang=es"
        },
        {
          "titulo": "PubChem - Acetona (CAS 67-64-1)",
          "url": "https://pubchem.ncbi.nlm.nih.gov/"
        }
      ],
      "estado": "demo"
    },
    {
      "id": "etanol",
      "nombre": "Etanol",
      "sinonimos": [
        "Alcohol etílico",
        "Etanol absoluto"
      ],
      "cas": "64-17-5",
      "ce": "200-578-6",
      "onu": "1170",
      "peligrosidad": {
        "clasificacion": [
          "Líquido inflamable, categoría 2",
          "Irritación ocular, categoría 2"
        ],
        "pictogramas": [
          "GHS02",
          "GHS07"
        ],
        "frases_h": [
          {
            "codigo": "H225",
            "texto": "Líquido y vapores muy inflamables."
          },
          {
            "codigo": "H319",
            "texto": "Provoca irritación ocular grave."
          }
        ],
        "significado": "Peligro principal: incendio. La llama de los alcoholes es casi invisible en ambientes iluminados y sus vapores pueden propagarse por el suelo."
      },
      "almacenamiento": "Conservar en recipientes bien cerrados, en lugar fresco y ventilado, lejos de fuentes de calor, chispas y llama abierta. Mantener separado de agentes oxidantes, ácidos y metales alcalinos. Usar material antichispa y conexión a tierra.",
      "derrame": "Eliminar ignición, ventilar y absorber el líquido con material inerte. Recoger en recipientes cerrados. No verter a desagües. En caso de contacto ocular, lavar con agua abundante.",
      "propiedades": {
        "estado_fisico": "Líquido incoloro",
        "olor": "Alcohol, característico",
        "punto_de_inflamacion": "12 °C (vaso cerrado)",
        "punto_de_ebullicion": "78 °C",
        "densidad": "0,79 g/cm³ (20 °C)",
        "solubilidad_en_agua": "Miscible",
        "presion_de_vapor": "5,9 kPa (20 °C)"
      },
      "interacciones": {
        "incompatibilidades": [
          "Agentes oxidantes fuertes",
          "Sodio y potasio metálicos",
          "Ácidos fuertes"
        ],
        "nota": "Reacciona violentamente con metales alcalinos liberando hidrógeno inflamable."
      },
      "fuentes": [
        {
          "titulo": "Base de datos ICSC (OIT/OMS)",
          "url": "https://chemicalsafety.ilo.org/dyn/icsc/showcard.listCards3?p_lang=es"
        },
        {
          "titulo": "PubChem - Etanol (CAS 64-17-5)",
          "url": "https://pubchem.ncbi.nlm.nih.gov/"
        }
      ],
      "estado": "demo"
    },
    {
      "id": "acido-clorhidrico",
      "nombre": "Ácido clorhídrico",
      "sinonimos": [
        "Cloruro de hidrógeno en disolución",
        "HCl acuoso"
      ],
      "cas": "7647-01-0",
      "ce": "231-595-7",
      "onu": "1789",
      "peligrosidad": {
        "clasificacion": [
          "Corrosivo para metales, categoría 1",
          "Corrosión cutánea, categoría 1B",
          "Lesiones oculares graves, categoría 1",
          "Toxicidad específica en órganos diana (exposición única), categoría 3"
        ],
        "pictogramas": [
          "GHS05",
          "GHS07"
        ],
        "frases_h": [
          {
            "codigo": "H290",
            "texto": "Puede ser corrosivo para los metales."
          },
          {
            "codigo": "H314",
            "texto": "Provoca quemaduras graves en la piel y lesiones oculares graves."
          },
          {
            "codigo": "H335",
            "texto": "Puede irritar las vías respiratorias."
          }
        ],
        "significado": "Peligro principal: corrosión. Los vapores de HCl son irritantes y corrosivos para vías respiratorias, ojos y mucosas. El contacto con metales puede liberar hidrógeno inflamable."
      },
      "almacenamiento": "Almacenar en recipientes resistentes a ácidos, en zona bien ventilada y fresca. Separar de bases, metales en polvo, agentes oxidantes, cianuros y sulfitos. Usar bandejas de contención y materiales de construcción resistentes a la corrosión. No almacenar junto a hipocloritos (libera cloro gaseoso).",
      "derrame": "Ventilar intensamente. Usar EPP: guantes de nitrilo o neopreno, gafas de seguridad y protección facial. Neutralizar con carbonato de sodio o cal, absorber con material inerte y recoger como residuo peligroso. No verter sin neutralizar. En contacto con piel o ojos, lavar con agua abundante al menos 15 minutos y acudir al médico.",
      "propiedades": {
        "estado_fisico": "Líquido incoloro o ligeramente amarillento (disolución)",
        "olor": "Picante, irritante",
        "punto_de_ebullicion": "Depende de la concentración; azeótropo ~108 °C",
        "densidad": "1,18 g/cm³ (37 %, 20 °C)",
        "solubilidad_en_agua": "Muy soluble",
        "ph": "< 1 (disolución concentrada)"
      },
      "interacciones": {
        "incompatibilidades": [
          "Bases fuertes",
          "Hipoclorito de sodio (libera cloro)",
          "Cianuros (libera ácido cianhídrico)",
          "Metales en polvo",
          "Agentes oxidantes"
        ],
        "nota": "La mezcla con lejía (hipoclorito) genera gas cloro tóxico: nunca combinarlos."
      },
      "fuentes": [
        {
          "titulo": "Base de datos ICSC (OIT/OMS)",
          "url": "https://chemicalsafety.ilo.org/dyn/icsc/showcard.listCards3?p_lang=es"
        },
        {
          "titulo": "PubChem - Ácido clorhídrico (CAS 7647-01-0)",
          "url": "https://pubchem.ncbi.nlm.nih.gov/"
        }
      ],
      "estado": "demo"
    },
    {
      "id": "amoniaco",
      "nombre": "Amoníaco",
      "sinonimos": [
        "Amoniaco anhidro",
        "Trihidruro de nitrógeno"
      ],
      "cas": "7664-41-7",
      "ce": "231-441-9",
      "onu": "1005",
      "peligrosidad": {
        "clasificacion": [
          "Gas inflamable, categoría 2",
          "Toxicidad aguda por inhalación, categoría 3",
          "Corrosión cutánea, categoría 1B",
          "Peligro para el medio ambiente acuático, categoría 1"
        ],
        "pictogramas": [
          "GHS02",
          "GHS05",
          "GHS06",
          "GHS09"
        ],
        "frases_h": [
          {
            "codigo": "H221",
            "texto": "Gas inflamable."
          },
          {
            "codigo": "H331",
            "texto": "Tóxico en caso de inhalación."
          },
          {
            "codigo": "H314",
            "texto": "Provoca quemaduras graves en la piel y lesiones oculares graves."
          },
          {
            "codigo": "H400",
            "texto": "Muy tóxico para los organismos acuáticos."
          }
        ],
        "significado": "Peligro principal: toxicidad por inhalación y corrosión. El gas es más ligero que el aire pero puede acumularse en espacios cerrados. Su olor picante es detectable antes de alcanzar concentraciones peligrosas, aunque la fatiga olfativa puede enmascararlo."
      },
      "almacenamiento": "Almacenar en recipientes a presión certificados, en lugar fresco, seco y muy bien ventilado, lejos de fuentes de ignición. Proteger de la luz solar y de temperaturas elevadas. Separar de ácidos, halógenos, óxidos de metales y agentes oxidantes. Disponer de detectores de gas y duchas de emergencia.",
      "derrame": "Evacuar y ventilar la zona. Usar EPP con protección respiratoria adecuada (máscara con filtro para amoníaco o aparato autónomo). Cerrar la fuga si es seguro. Disolver el gas en agua pulverizada desde zonas protegidas. No dirigir chorros directamente sobre la fuga. Evitar que entre en alcantarillado o cursos de agua.",
      "propiedades": {
        "estado_fisico": "Gas incoloro (licuado a presión)",
        "olor": "Picante, fuerte, característico",
        "punto_de_inflamacion": "651 °C (temperatura de autoignición)",
        "punto_de_ebullicion": "-33 °C",
        "densidad_de_gas": "0,73 kg/m³ (más ligero que el aire)",
        "solubilidad_en_agua": "Muy soluble (forma amoniaco acuoso)"
      },
      "interacciones": {
        "incompatibilidades": [
          "Ácidos (forman sales amónicas)",
          "Cloro e hipocloritos (pueden formar cloraminas tóxicas)",
          "Halógenos",
          "Óxidos de metales",
          "Agentes oxidantes fuertes"
        ],
        "nota": "Nunca mezclar amoniaco con lejía: se generan cloraminas irritantes para el sistema respiratorio."
      },
      "fuentes": [
        {
          "titulo": "Base de datos ICSC (OIT/OMS)",
          "url": "https://chemicalsafety.ilo.org/dyn/icsc/showcard.listCards3?p_lang=es"
        },
        {
          "titulo": "PubChem - Amoníaco (CAS 7664-41-7)",
          "url": "https://pubchem.ncbi.nlm.nih.gov/"
        }
      ],
      "estado": "demo"
    },
    {
      "id": "acido-sulfurico",
      "nombre": "Ácido sulfúrico",
      "sinonimos": [
        "Vitriolo",
        "H2SO4"
      ],
      "cas": "7664-93-9",
      "ce": "231-639-5",
      "onu": "1830",
      "peligrosidad": {
        "clasificacion": [
          "Corrosivo para metales, categoría 1",
          "Corrosión cutánea, categoría 1A"
        ],
        "pictogramas": [
          "GHS05"
        ],
        "frases_h": [
          {
            "codigo": "H290",
            "texto": "Puede ser corrosivo para los metales."
          },
          {
            "codigo": "H314",
            "texto": "Provoca quemaduras graves en la piel y lesiones oculares graves."
          }
        ],
        "significado": "Peligro principal: corrosión severa y reacción violenta con el agua. La dilución debe hacerse añadiendo siempre el ácido sobre el agua, nunca al revés, por el calor liberado."
      },
      "almacenamiento": "Conservar en recipientes de vidrio o acero recubierto, bien cerrados, en lugar fresco y seco. Separar de agua, bases, metales, agentes oxidantes y materiales orgánicos. Usar bandejas de contención resistentes al ácido. Proteger de la humedad.",
      "derrame": "Ventilar y usar EPP completo. Neutralizar con carbonato de sodio, cal o bicarbonato. Absorber con material inerte y recoger como residuo peligroso. No añadir agua directamente sobre ácido concentrado. En contacto con piel, retirar la ropa contaminada y lavar con agua abundante al menos 15 minutos.",
      "propiedades": {
        "estado_fisico": "Líquido incoloro, aceitoso (concentrado)",
        "olor": "Inodoro",
        "punto_de_ebullicion": "337 °C (descompone)",
        "densidad": "1,84 g/cm³ (98 %, 20 °C)",
        "solubilidad_en_agua": "Miscible, con fuerte desprendimiento de calor",
        "ph": "< 1"
      },
      "interacciones": {
        "incompatibilidades": [
          "Agua (reacción exotérmica violenta)",
          "Bases fuertes",
          "Metales (liberan hidrógeno)",
          "Agentes oxidantes",
          "Materiales orgánicos y azúcares"
        ],
        "nota": "El contacto con azúcares o celulosa puede provocar carbonización e incendio."
      },
      "fuentes": [
        {
          "titulo": "Base de datos ICSC (OIT/OMS)",
          "url": "https://chemicalsafety.ilo.org/dyn/icsc/showcard.listCards3?p_lang=es"
        },
        {
          "titulo": "PubChem - Ácido sulfúrico (CAS 7664-93-9)",
          "url": "https://pubchem.ncbi.nlm.nih.gov/"
        }
      ],
      "estado": "demo"
    },
    {
      "id": "hidroxido-sodio",
      "nombre": "Hidróxido de sodio",
      "sinonimos": [
        "Sosa cáustica",
        "NaOH"
      ],
      "cas": "1310-73-2",
      "ce": "215-185-5",
      "onu": "1823",
      "peligrosidad": {
        "clasificacion": [
          "Corrosivo para metales, categoría 1",
          "Corrosión cutánea, categoría 1A",
          "Lesiones oculares graves, categoría 1"
        ],
        "pictogramas": [
          "GHS05"
        ],
        "frases_h": [
          {
            "codigo": "H290",
            "texto": "Puede ser corrosivo para los metales."
          },
          {
            "codigo": "H314",
            "texto": "Provoca quemaduras graves en la piel y lesiones oculares graves."
          }
        ],
        "significado": "Peligro principal: corrosión. Disuelve grasas y proteínas, por lo que las quemaduras son profundas y pueden no doler inicialmente. La disolución en agua es fuertemente exotérmica."
      },
      "almacenamiento": "Almacenar en recipientes secos y herméticos, en lugar fresco y bien ventilado. Proteger de la humedad y del CO2 del aire (absorbe ambos). Separar de ácidos, aluminio, zinc, estaño y otros metales anfóteros. Usar bandejas de contención.",
      "derrame": "Usar EPP completo. No usar agua a presión directamente sobre sólido. Recoger el sólido con herramientas limpias, evitando el polvo, y disolverlo lentamente en agua fría para su neutralización controlada. Neutralizar con ácido diluido. No verter sin neutralizar. En contacto con ojos, lavar al menos 15 minutos y acudir al médico.",
      "propiedades": {
        "estado_fisico": "Sólido blanco, pellets o escamas (también en disolución)",
        "olor": "Inodoro",
        "punto_de_fusion": "318 °C",
        "densidad": "2,13 g/cm³ (20 °C)",
        "solubilidad_en_agua": "Muy soluble, con fuerte desprendimiento de calor",
        "ph": "> 13 (disolución 1 %)"
      },
      "interacciones": {
        "incompatibilidades": [
          "Ácidos (reacción violenta)",
          "Aluminio, zinc, estaño (liberan hidrógeno)",
          "Cloro",
          "Agua (exotérmica)"
        ],
        "nota": "El contacto con aluminio o zinc genera hidrógeno inflamable; peligro de incendio o explosión."
      },
      "fuentes": [
        {
          "titulo": "Base de datos ICSC (OIT/OMS)",
          "url": "https://chemicalsafety.ilo.org/dyn/icsc/showcard.listCards3?p_lang=es"
        },
        {
          "titulo": "PubChem - Hidróxido de sodio (CAS 1310-73-2)",
          "url": "https://pubchem.ncbi.nlm.nih.gov/"
        }
      ],
      "estado": "demo"
    }
  ],
  "warning": "FICHAS DE DEMOSTRACION. Los datos son de ejemplo y deben verificarse en las fuentes oficiales antes de cualquier uso real. Cada ficha enlaza a su fuente."
};
