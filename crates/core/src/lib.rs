//! Núcleo de la aplicación de sustancias peligrosas.
//!
//! Toda la lógica (países, marco legal, búsqueda de sustancias) vive aquí y es
//! independiente de la interfaz. Los datos se incluyen en el binario con
//! `include_str!`, de modo que la misma librería compila tanto para nativo como
//! para WebAssembly sin depender de un sistema de gestión de bases de datos.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::OnceLock;

const COUNTRIES_JSON: &str = include_str!("../../../data/countries.json");
const TOPICS_JSON: &str = include_str!("../../../data/topics.json");
const SOURCES_JSON: &str = include_str!("../../../data/sources.json");
const LEGISLATION_JSON: &str = include_str!("../../../data/legislation.json");
const SUBSTANCES_JSON: &str = include_str!("../../../data/substances.json");
const INTERACTIONS_JSON: &str = include_str!("../../../data/interactions.json");

// ---------------------------------------------------------------------------
// Modelos
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Country {
    pub id: String,
    pub nombre: String,
    pub codigo: String,
    #[serde(default)]
    pub tipo: String,
    #[serde(default)]
    pub idioma: String,
    #[serde(default)]
    pub bandera: String,
    #[serde(default)]
    pub saludo: String,
    #[serde(default)]
    pub descripcion: String,
    #[serde(default)]
    pub aplicable: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CountriesDb {
    pub paises: Vec<Country>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Topic {
    pub id: String,
    pub nombre: String,
    #[serde(default)]
    pub descripcion: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TopicsDb {
    pub temas: Vec<Topic>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Source {
    pub id: String,
    #[serde(default)]
    pub pais: String,
    #[serde(default)]
    pub institucion: String,
    #[serde(default)]
    pub tema: String,
    pub titulo: String,
    pub url: String,
    #[serde(default)]
    pub descripcion: String,
    #[serde(default)]
    pub verificacion: String,
    #[serde(default)]
    pub categoria: String,
    #[serde(default)]
    pub oficial: bool,
    #[serde(default)]
    pub nota: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SourcesDb {
    pub fuentes: Vec<Source>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Portal {
    pub id: String,
    #[serde(default)]
    pub institucion: String,
    pub url: String,
    #[serde(default)]
    pub verificacion: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Instrument {
    pub id: String,
    pub titulo: String,
    #[serde(default)]
    pub tema: String,
    pub url: String,
    #[serde(default)]
    pub verificacion: String,
    #[serde(default)]
    pub nota: String,
    #[serde(default)]
    pub alternativas: Vec<String>,
    #[serde(default)]
    pub tipo: String,
    #[serde(default)]
    pub archivo_local: String,
    #[serde(default)]
    pub paginas: String,
    #[serde(default)]
    pub texto: String,
    #[serde(default)]
    pub pdf_nombre: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CountryLegislation {
    #[serde(default)]
    pub portales_oficiales: Vec<Portal>,
    #[serde(default)]
    pub instrumentos: Vec<Instrument>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LegislationDb {
    pub paises: HashMap<String, CountryLegislation>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FraseH {
    pub codigo: String,
    pub texto: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Peligrosidad {
    #[serde(default)]
    pub clasificacion: Vec<String>,
    #[serde(default)]
    pub pictogramas: Vec<String>,
    #[serde(default)]
    pub frases_h: Vec<FraseH>,
    #[serde(default)]
    pub significado: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Propiedades {
    #[serde(default)]
    pub estado_fisico: String,
    #[serde(default)]
    pub olor: String,
    #[serde(default)]
    pub punto_de_inflamacion: String,
    #[serde(default)]
    pub punto_de_ebullicion: String,
    #[serde(default)]
    pub punto_de_fusion: String,
    #[serde(default)]
    pub densidad: String,
    #[serde(default)]
    pub densidad_de_gas: String,
    #[serde(default)]
    pub solubilidad_en_agua: String,
    #[serde(default)]
    pub presion_de_vapor: String,
    #[serde(default)]
    pub ph: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Interacciones {
    #[serde(default)]
    pub incompatibilidades: Vec<String>,
    #[serde(default)]
    pub nota: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FdsField {
    pub k: String,
    pub v: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FdsList {
    pub titulo: String,
    #[serde(default)]
    pub items: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FdsSection {
    pub numero: u32,
    pub id: String,
    pub titulo: String,
    #[serde(default)]
    pub texto: String,
    #[serde(default)]
    pub campos: Vec<FdsField>,
    #[serde(default)]
    pub listas: Vec<FdsList>,
    #[serde(default)]
    pub notas: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Fuente {
    pub titulo: String,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Substance {
    pub id: String,
    pub nombre: String,
    #[serde(default)]
    pub sinonimos: Vec<String>,
    #[serde(default)]
    pub cas: String,
    #[serde(default)]
    pub ce: String,
    #[serde(default)]
    pub onu: String,
    #[serde(default)]
    pub peligrosidad: Peligrosidad,
    #[serde(default)]
    pub almacenamiento: String,
    #[serde(default)]
    pub derrame: String,
    #[serde(default)]
    pub propiedades: Propiedades,
    #[serde(default)]
    pub interacciones: Interacciones,
    #[serde(default)]
    pub fuentes: Vec<Fuente>,
    #[serde(default)]
    pub estado: String,
    #[serde(default)]
    pub verificado_por: String,
    #[serde(default)]
    pub fecha_verificacion: String,
    #[serde(default)]
    pub pdf_url: String,
    #[serde(default)]
    pub pdf_nombre: String,
    #[serde(default)]
    pub palabra_advertencia: String,
    #[serde(default)]
    pub secciones: Vec<FdsSection>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SubstancesDb {
    pub advertencia: String,
    #[serde(default)]
    pub estructura: String,
    pub sustancias: Vec<Substance>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InteractionLevel {
    pub etiqueta: String,
    #[serde(default)]
    pub descripcion: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Interaction {
    pub sustancias: Vec<String>,
    pub nivel: String,
    pub tipo: String,
    #[serde(default)]
    pub descripcion: String,
    #[serde(default)]
    pub productos: Vec<String>,
    #[serde(default)]
    pub consecuencias: Vec<String>,
    #[serde(default)]
    pub acciones: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InteractionsDb {
    pub version: String,
    #[serde(default)]
    pub advertencia: String,
    #[serde(default)]
    pub niveles: HashMap<String, InteractionLevel>,
    #[serde(default)]
    pub interacciones: Vec<Interaction>,
}

#[derive(Debug, Clone)]
pub struct PairAnalysis {
    pub id_a: String,
    pub id_b: String,
    pub nombre_a: String,
    pub nombre_b: String,
    pub nivel: String,
    pub etiqueta: String,
    pub tipo: String,
    pub descripcion: String,
    pub productos: Vec<String>,
    pub consecuencias: Vec<String>,
    pub acciones: Vec<String>,
    pub es_regla_general: bool,
}

// ---------------------------------------------------------------------------
// Carga de datos
// ---------------------------------------------------------------------------

/// Carga y devuelve todos los países disponibles (datos cacheados).
pub fn countries() -> Vec<Country> {
    static CACHE: OnceLock<CountriesDb> = OnceLock::new();
    CACHE.get_or_init(|| parse(COUNTRIES_JSON)).paises.clone()
}

/// Carga y devuelve todos los apartados/temas (datos cacheados).
pub fn topics() -> Vec<Topic> {
    static CACHE: OnceLock<TopicsDb> = OnceLock::new();
    CACHE.get_or_init(|| parse(TOPICS_JSON)).temas.clone()
}

/// Carga y devuelve todas las fuentes (datos cacheados).
pub fn sources() -> Vec<Source> {
    static CACHE: OnceLock<SourcesDb> = OnceLock::new();
    CACHE.get_or_init(|| parse(SOURCES_JSON)).fuentes.clone()
}

/// Devuelve las fuentes aplicables a un país.
pub fn sources_for(country_id: &str) -> Vec<Source> {
    static CACHE: OnceLock<SourcesDb> = OnceLock::new();
    CACHE
        .get_or_init(|| parse(SOURCES_JSON))
        .fuentes
        .iter()
        .filter(|s| {
            country_id == "internacional"
                || s.pais == country_id
                || (country_id == "espana" && s.pais == "espana")
                || (country_id == "eeuu" && s.pais == "eeuu")
        })
        .cloned()
        .collect()
}

/// Base legislativa completa, cacheada (evita parsear 322 KB en cada render).
pub fn legislation_db() -> &'static LegislationDb {
    static CACHE: OnceLock<LegislationDb> = OnceLock::new();
    CACHE.get_or_init(|| parse(LEGISLATION_JSON))
}

/// Devuelve los portales oficiales de un país.
pub fn portals_for(country_id: &str) -> Vec<Portal> {
    legislation_db()
        .paises
        .get(country_id)
        .map(|c| c.portales_oficiales.clone())
        .unwrap_or_default()
}

/// Devuelve los instrumentos normativos de un país.
pub fn instruments_for(country_id: &str) -> Vec<Instrument> {
    legislation_db()
        .paises
        .get(country_id)
        .map(|c| c.instrumentos.clone())
        .unwrap_or_default()
}

/// Norma devuelta por la API de datos abiertos del BOE.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BoeNorm {
    pub identificador: String,
    pub titulo: String,
    #[serde(default)]
    pub fecha_actualizacion: String,
    #[serde(default)]
    pub departamento: Option<BoeDepartamento>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BoeDepartamento {
    #[serde(default)]
    pub texto: String,
}

impl BoeNorm {
    pub fn url(&self) -> String {
        format!("https://www.boe.es/buscar/act.php?id={}", self.identificador)
    }
    pub fn fecha_formateada(&self) -> String {
        let f = &self.fecha_actualizacion;
        if f.len() >= 8 {
            format!("{}-{}-{}", &f[0..4], &f[4..6], &f[6..8])
        } else {
            String::new()
        }
    }
}

/// Compuesto devuelto por la API PUG REST de PubChem (NIH).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PubChemCompound {
    #[serde(rename = "CID")]
    pub cid: u64,
    #[serde(rename = "Title", default)]
    pub title: String,
    #[serde(rename = "MolecularFormula", default)]
    pub molecular_formula: String,
    #[serde(rename = "MolecularWeight", default)]
    pub molecular_weight: String,
    #[serde(rename = "InChIKey", default)]
    pub inchi_key: String,
    #[serde(rename = "XLogP", default)]
    pub x_log_p: serde_json::Value,
    #[serde(rename = "ExactMass", default)]
    pub exact_mass: String,
    #[serde(rename = "HeavyAtomCount", default)]
    pub heavy_atom_count: serde_json::Value,
    #[serde(default)]
    pub complexity: serde_json::Value,
    #[serde(default)]
    pub synonyms: Vec<String>,
    /// Idioma de salida elegido por el usuario (no viene en el JSON de PubChem).
    #[serde(default)]
    pub lang: String,
}

/// Genera variantes de búsqueda para no depender del idioma del usuario.
/// PubChem indexa mejor los nombres en inglés, así que normalizamos acentos,
/// eliminamos la palabra "ácido" en varios idiomas y traducimos las raíces
/// químicas más habituales.
pub fn search_variants(query: &str) -> Vec<String> {
    let base = normalize(query);
    let mut variantes = vec![base.clone()];

    // Raíces químicas en español, francés, alemán, italiano y portugués.
    let traducciones: &[(&str, &str)] = &[
        // Ácidos
        ("citrico", "citric"), ("citrique", "citric"), ("zitronensaure", "citric"), ("citrico", "citric"),
        ("clorhidrico", "hydrochloric"), ("chlorhydrique", "hydrochloric"), ("chlorwasserstoffsaure", "hydrochloric"),
        ("sulfurico", "sulfuric"), ("sulfurique", "sulfuric"), ("schwefelsaure", "sulfuric"), ("solfurico", "sulfuric"),
        ("nitrico", "nitric"), ("nitrique", "nitric"), ("salpetersaure", "nitric"), ("nitrico", "nitric"),
        ("acetico", "acetic"), ("acetique", "acetic"), ("essigsaure", "acetic"), ("acetico", "acetic"),
        ("ascorbico", "ascorbic"), ("ascorbique", "ascorbic"), ("askorbinsaure", "ascorbic"), ("ascorbico", "ascorbic"),
        ("lactico", "lactic"), ("lactique", "lactic"), ("milchsaure", "lactic"), ("latico", "lactic"),
        ("benzoico", "benzoic"), ("benzoique", "benzoic"), ("benzoesaure", "benzoic"), ("benzoico", "benzoic"),
        ("fosforico", "phosphoric"), ("phosphorique", "phosphoric"), ("phosphorsaure", "phosphoric"), ("fosforico", "phosphoric"),
        ("carbonico", "carbonic"), ("carbonique", "carbonic"), ("kohlensaure", "carbonic"), ("carbonico", "carbonic"),
        ("acetilsalicilico", "acetylsalicylic"), ("acetylsalicylique", "acetylsalicylic"), ("aspirina", "aspirin"),
        ("tartarico", "tartaric"), ("tartarique", "tartaric"), ("weinsaure", "tartaric"),
        ("maleico", "maleic"), ("maleique", "maleic"), ("maleinsaure", "maleic"),
        ("oxalico", "oxalic"), ("oxalique", "oxalic"), ("oxalsaure", "oxalic"),
        ("salicilico", "salicylic"), ("salicylique", "salicylic"), ("salizylsaure", "salicylic"),
        ("nicotinico", "nicotinic"), ("nicotinique", "nicotinic"), ("nikotinsaure", "nicotinic"),
        ("pantotenico", "pantothenic"), ("pantothenique", "pantothenic"), ("pantothensaure", "pantothenic"),
        ("folico", "folic"), ("folique", "folic"), ("folgsaure", "folic"),
        ("hialuronico", "hyaluronic"), ("hyaluronique", "hyaluronic"), ("hyaluronsaure", "hyaluronic"),
        ("glicolico", "glycolic"), ("glycolique", "glycolic"), ("glykolsaure", "glycolic"),
        ("formico", "formic"), ("formique", "formic"), ("ameisensaure", "formic"),
        ("propionico", "propionic"), ("propionique", "propionic"), ("propionsaure", "propionic"),
        ("butirico", "butyric"), ("butyrique", "butyric"), ("buttersaure", "butyric"),
        ("sorbico", "sorbic"), ("sorbique", "sorbic"), ("sorbinsaure", "sorbic"),
        ("glutamico", "glutamic"), ("glutamique", "glutamic"), ("glutaminsaure", "glutamic"),
        ("aspartico", "aspartic"), ("aspartique", "aspartic"), ("asparaginsaure", "aspartic"),
        // Bases y sales
        ("amoniaco", "ammonia"), ("ammoniaque", "ammonia"), ("ammoniak", "ammonia"), ("amoniaco", "ammonia"),
        ("hidroxido", "hydroxide"), ("hydroxyde", "hydroxide"), ("hydroxid", "hydroxide"),
        ("sodio", "sodium"), ("sodium", "sodium"), ("natrium", "sodium"),
        ("potasio", "potassium"), ("potassium", "potassium"), ("kalium", "potassium"),
        ("calcio", "calcium"), ("calcium", "calcium"), ("magnesio", "magnesium"), ("magnesium", "magnesium"),
        ("amonio", "ammonium"), ("ammonium", "ammonium"),
        ("bicarbonato", "bicarbonate"), ("bicarbonate", "bicarbonate"), ("carbonato", "carbonate"), ("carbonate", "carbonate"),
        ("sulfato", "sulfate"), ("sulfate", "sulfate"), ("cloruro", "chloride"), ("chlorure", "chloride"),
        // Disolventes y compuestos comunes
        ("etanol", "ethanol"), ("ethanol", "ethanol"), ("alcohol etilico", "ethanol"), ("alcool ethylique", "ethanol"),
        ("metanol", "methanol"), ("methanol", "methanol"), ("acetona", "acetone"), ("acetone", "acetone"),
        ("cafeina", "caffeine"), ("cafeine", "caffeine"), ("koffein", "caffeine"), ("caffeina", "caffeine"),
        ("ibuprofeno", "ibuprofen"), ("ibuprofene", "ibuprofen"), ("ibuprofen", "ibuprofen"),
        ("paracetamol", "paracetamol"), ("acetaminofen", "acetaminophen"), ("acetaminophene", "acetaminophen"),
        ("dipirona", "metamizole"), ("novalgina", "metamizole"),
        ("agua", "water"), ("eau", "water"), ("wasser", "water"), ("acqua", "water"),
        ("cloro", "chlorine"), ("chlore", "chlorine"), ("chlor", "chlorine"),
        ("hidrogeno", "hydrogen"), ("hydrogene", "hydrogen"), ("wasserstoff", "hydrogen"),
        ("oxigeno", "oxygen"), ("oxygene", "oxygen"), ("sauerstoff", "oxygen"),
        ("nitrogeno", "nitrogen"), ("azote", "nitrogen"), ("stickstoff", "nitrogen"),
        ("glucosa", "glucose"), ("glucose", "glucose"), ("sacarosa", "sucrose"), ("saccharose", "sucrose"),
        ("almidon", "starch"), ("amidon", "starch"), ("staerke", "starch"),
        ("aceite", "oil"), ("huile", "oil"), ("oel", "oil"),
    ];

    let acidos: &[&str] = &["acido", "acide", "acid", "saure", "saur", "acidos", "acides"];
    let tokens: Vec<&str> = base.split_whitespace().collect();
    let tiene_acido = tokens.iter().any(|t| acidos.contains(t));

    let mut mapeados: Vec<String> = Vec::new();
    for token in tokens {
        if acidos.contains(&token) {
            continue;
        }
        let convertido = traducciones
            .iter()
            .find(|(origen, _)| *origen == token)
            .map(|(_, destino)| destino.to_string())
            .unwrap_or_else(|| token.to_string());
        mapeados.push(convertido);
    }

    if !mapeados.is_empty() {
        let mut frase = mapeados.join(" ");
        if tiene_acido && !frase.contains("acid") {
            frase = format!("{} acid", frase);
        }
        if frase != base {
            variantes.push(frase);
        }
    }
    variantes.dedup();
    variantes
}

/// Devuelve un sinónimo localizado a partir de las etiquetas que PubChem
/// añade a algunos nombres, por ejemplo `[INN-Spanish]` o `[French]`.
pub fn localized_name(synonyms: &[String], lang: &str) -> Option<String> {
    let marcas: &[&str] = match lang {
        "es" => &["[inn-spanish]", "[spanish]", "[es]"],
        "fr" => &["[inn-french]", "[french]", "[fr]"],
        "de" => &["[inn-german]", "[german]", "[de]"],
        "it" => &["[inn-italian]", "[italian]", "[it]"],
        _ => &[],
    };
    for marca in marcas {
        if let found @ Some(_) = synonyms
            .iter()
            .find(|s| s.to_lowercase().contains(marca))
            .cloned()
        {
            // Quitar la etiqueta entre corchetes para mostrar solo el nombre.
            return found.map(|s| {
                s.split('[')
                    .next()
                    .unwrap_or(&s)
                    .trim()
                    .to_string()
            });
        }
    }
    None
}

impl PubChemCompound {
    pub fn url(&self) -> String {
        format!("https://pubchem.ncbi.nlm.nih.gov/compound/{}", self.cid)
    }
}

/// Construye una ficha parcial de 16 secciones a partir de un compuesto de PubChem.
/// Se marca como `online` para que la interfaz no la confunda con una FDS verificada.
pub fn substance_from_pubchem(p: &PubChemCompound) -> Substance {
    let consultar = "Esta sección requiere la ficha de datos de seguridad del fabricante. Consulta los enlaces oficiales al final de la ficha.";
    let online_url = p.url();
    let nombre_mostrado = localized_name(&p.synonyms, &p.lang).unwrap_or_else(|| p.title.clone());
    let icsc = "https://chemicalsafety.ilo.org/dyn/icsc/showcard.listCards3?p_lang=es";

    let field = |k: &str, v: String| FdsField { k: k.to_string(), v };
    let sec = |numero: u32, id: &str, titulo: &str, texto: &str,
               campos: Vec<FdsField>, listas: Vec<FdsList>| FdsSection {
        numero,
        id: id.to_string(),
        titulo: titulo.to_string(),
        texto: texto.to_string(),
        campos,
        listas,
        notas: Vec::new(),
    };
    let links = |items: Vec<String>| vec![FdsList { titulo: "Enlaces oficiales".to_string(), items }];

    let mut propiedades = vec![
        field("Nombre (PubChem)", nombre_mostrado.clone()),
        field("CID de PubChem", p.cid.to_string()),
    ];
    if !p.molecular_formula.is_empty() {
        propiedades.push(field("Fórmula molecular", p.molecular_formula.clone()));
    }
    if !p.molecular_weight.is_empty() {
        propiedades.push(field("Peso molecular (g/mol)", p.molecular_weight.clone()));
    }
    if !p.inchi_key.is_empty() {
        propiedades.push(field("InChIKey", p.inchi_key.clone()));
    }
    if !p.x_log_p.is_null() {
        propiedades.push(field("XLogP", p.x_log_p.to_string()));
    }
    if !p.exact_mass.is_empty() {
        propiedades.push(field("Masa exacta", p.exact_mass.clone()));
    }
    if !p.heavy_atom_count.is_null() {
        propiedades.push(field("Átomos pesados", p.heavy_atom_count.to_string()));
    }
    if !p.complexity.is_null() {
        propiedades.push(field("Complejidad", p.complexity.to_string()));
    }

    let secciones = vec![
        sec(1, "identificacion", "Identificación del producto y del proveedor",
            "Sustancia identificada en PubChem, base de datos pública del National Institutes of Health (EE. UU.).",
            vec![field("Nombre (PubChem)", p.title.clone()), field("CID", p.cid.to_string())], vec![]),
        sec(2, "peligros", "Identificación de los peligros",
            "La clasificación GHS no se incluye en esta consulta. Verifica pictogramas, frases H y palabra de advertencia en PubChem o en la FDS del proveedor.",
            vec![], links(vec![online_url.clone()])),
        sec(3, "composicion", "Composición / información sobre los componentes",
            "Identidad química según PubChem. La pureza debe consultarse en el certificado del proveedor.",
            vec![field("Fórmula molecular", p.molecular_formula.clone())],
            if p.synonyms.is_empty() {
                vec![]
            } else {
                vec![FdsList {
                    titulo: "Sinónimos y nombres en otros idiomas".to_string(),
                    items: p.synonyms.iter().take(25).cloned().collect(),
                }]
            }),
        sec(4, "primeros_auxilios", "Primeros auxilios", consultar, vec![], vec![]),
        sec(5, "incendio", "Medidas de lucha contra incendios", consultar, vec![], vec![]),
        sec(6, "derrame", "Medidas en caso de vertido accidental", consultar, vec![], vec![]),
        sec(7, "manipulacion", "Manipulación y almacenamiento", consultar, vec![], vec![]),
        sec(8, "exposicion", "Controles de exposición / protección personal",
            "Consulta los límites de exposición profesional vigentes en tu país.", vec![], vec![]),
        sec(9, "propiedades", "Propiedades físicas y químicas",
            "Propiedades publicadas por PubChem.", propiedades, vec![]),
        sec(10, "estabilidad", "Estabilidad y reactividad", consultar, vec![], vec![]),
        sec(11, "toxicologica", "Información toxicológica",
            "Consulta la sección de toxicología de PubChem y la FDS del proveedor.", vec![], links(vec![online_url.clone()])),
        sec(12, "ecotoxicologica", "Información ecotoxicológica", consultar, vec![], vec![]),
        sec(13, "eliminacion", "Consideraciones sobre eliminación",
            "Eliminar conforme a la normativa local de residuos peligrosos.", vec![], vec![]),
        sec(14, "transporte", "Información relativa al transporte",
            "Número ONU, clase y grupo de embalaje deben obtenerse de la FDS del proveedor y verificarse en ADR/RID/IMDG/IATA.", vec![], vec![]),
        sec(15, "reglamentacion", "Información sobre la reglamentación",
            "Verificar REACH, CLP y SGA/GHS según el país seleccionado.", vec![], vec![]),
        sec(16, "otras", "Otras informaciones",
            "Ficha parcial generada en línea a partir de PubChem. No sustituye una FDS completa.",
            vec![field("Fuente", "PubChem (NIH)".to_string()), field("CID", p.cid.to_string())],
            links(vec![online_url.clone(), icsc.to_string()])),
    ];

    Substance {
        id: format!("online-{}", p.cid),
        nombre: nombre_mostrado.clone(),
        sinonimos: p.synonyms.iter().take(25).cloned().collect(),
        cas: String::new(),
        ce: String::new(),
        onu: String::new(),
        peligrosidad: Peligrosidad::default(),
        almacenamiento: String::new(),
        derrame: String::new(),
        propiedades: Propiedades::default(),
        interacciones: Interacciones::default(),
        fuentes: vec![
            Fuente { titulo: format!("PubChem - {} (CID {})", p.title, p.cid), url: online_url },
            Fuente { titulo: "ICSC - OIT/OMS".to_string(), url: icsc.to_string() },
        ],
        estado: "online".to_string(),
        verificado_por: String::new(),
        fecha_verificacion: String::new(),
        pdf_url: String::new(),
        pdf_nombre: String::new(),
        palabra_advertencia: "Consultar FDS".to_string(),
        secciones,
    }
}

/// Base de interacciones entre sustancias.
pub fn interactions_db() -> &'static InteractionsDb {
    static CACHE: OnceLock<InteractionsDb> = OnceLock::new();
    CACHE.get_or_init(|| parse(INTERACTIONS_JSON))
}

/// Nombre de archivo sugerido para descargar un PDF.
pub fn pdf_filename(url: &str, fallback: &str) -> String {
    if !fallback.is_empty() {
        return fallback.to_string();
    }
    url.rsplit('/')
        .next()
        .unwrap_or(fallback)
        .split('?')
        .next()
        .unwrap_or(fallback)
        .to_string()
}

/// Indica si una ficha ha sido verificada frente a fuentes oficiales.
pub fn substance_is_verified(s: &Substance) -> bool {
    s.estado == "verificado"
}

/// Advertencia sobre el uso de las interacciones.
pub fn interactions_warning() -> String {
    interactions_db().advertencia.clone()
}

/// Etiqueta legible de un nivel de peligro.
pub fn level_label(level: &str) -> String {
    interactions_db()
        .niveles
        .get(level)
        .map(|l| l.etiqueta.clone())
        .unwrap_or_else(|| level.to_string())
}

/// Analiza todas las parejas de una lista de sustancias.
///
/// Busca una regla explícita para cada par. Si no existe, devuelve una regla
/// general de "sin reacción conocida" para no dar una respuesta vacía.
pub fn analyze_interactions(ids: &[String]) -> Vec<PairAnalysis> {
    let nombres = HashMap::new();
    analyze_interactions_named(ids, &nombres)
}

/// Variante que recibe los nombres de las sustancias, necesaria cuando estas
/// proceden de una base de datos en línea y no están en el repositorio.
pub fn analyze_interactions_named(
    ids: &[String],
    nombres: &HashMap<String, String>,
) -> Vec<PairAnalysis> {
    let db = interactions_db();
    let all: Vec<Substance> = Vec::new();
    let mut results = Vec::new();

    for i in 0..ids.len() {
        for j in (i + 1)..ids.len() {
            let a = &ids[i];
            let b = &ids[j];
            if a == b {
                continue;
            }
            let mut pair = vec![a.clone(), b.clone()];
            pair.sort();

            let nombre_a = nombres
                .get(a)
                .cloned()
                .or_else(|| all.iter().find(|s| &s.id == a).map(|s| s.nombre.clone()))
                .unwrap_or_else(|| a.clone());
            let nombre_b = nombres
                .get(b)
                .cloned()
                .or_else(|| all.iter().find(|s| &s.id == b).map(|s| s.nombre.clone()))
                .unwrap_or_else(|| b.clone());

            let found = db.interacciones.iter().find(|it| {
                let mut sorted = it.sustancias.clone();
                sorted.sort();
                sorted == pair
            });

            let analysis = match found {
                Some(it) => PairAnalysis {
                    id_a: a.clone(),
                    id_b: b.clone(),
                    nombre_a: nombre_a.clone(),
                    nombre_b: nombre_b.clone(),
                    nivel: it.nivel.clone(),
                    etiqueta: level_label(&it.nivel),
                    tipo: it.tipo.clone(),
                    descripcion: it.descripcion.clone(),
                    productos: it.productos.clone(),
                    consecuencias: it.consecuencias.clone(),
                    acciones: it.acciones.clone(),
                    es_regla_general: false,
                },
                None => PairAnalysis {
                    id_a: a.clone(),
                    id_b: b.clone(),
                    nombre_a: nombre_a.clone(),
                    nombre_b: nombre_b.clone(),
                    nivel: "nulo".to_string(),
                    etiqueta: level_label("nulo"),
                    tipo: "Sin reacción registrada".to_string(),
                    descripcion: format!(
                        "No hay una interacción registrada entre {} y {} en la base actual. No se debe asumir que son compatibles: verificar en las fichas de datos de seguridad antes de cualquier mezcla.",
                        nombre_a, nombre_b
                    ),
                    productos: Vec::new(),
                    consecuencias: Vec::new(),
                    acciones: vec![
                        "No mezclar sin verificar la compatibilidad en fuentes oficiales".to_string(),
                    ],
                    es_regla_general: true,
                },
            };
            results.push(analysis);
        }
    }
    results
}

/// Devuelve la base de sustancias cacheada.
fn substances_db() -> &'static SubstancesDb {
    static CACHE: OnceLock<SubstancesDb> = OnceLock::new();
    CACHE.get_or_init(|| parse(SUBSTANCES_JSON))
}

/// Advertencia sobre el carácter de las fichas de demostración.
pub fn substances_warning() -> String {
    substances_db().advertencia.clone()
}

/// Carga y devuelve todas las sustancias.
pub fn substances() -> Vec<Substance> {
    substances_db().sustancias.clone()
}

/// Busca una sustancia por identificador.
pub fn substance_by_id(id: &str) -> Option<Substance> {
    substances_db()
        .sustancias
        .iter()
        .find(|s| s.id == id)
        .cloned()
}

/// Busca sustancias por nombre, sinónimo, CAS, CE u ONU (insensible a acentos).
pub fn search_substances(query: &str) -> Vec<Substance> {
    let q = normalize(query);
    if q.is_empty() {
        return Vec::new();
    }
    let numerico = q.chars().any(|c| c.is_ascii_digit());
    substances_db()
        .sustancias
        .iter()
        .filter(|s| {
            let haystack = format!(
                "{} {} {} {} {}",
                s.nombre,
                s.sinonimos.join(" "),
                s.cas,
                s.ce,
                s.onu
            );
            if numerico {
                normalize(&haystack).contains(&q)
            } else {
                fuzzy_match(&q, &haystack)
            }
        })
        .cloned()
        .collect()
}

/// Devuelve el país por su identificador.
pub fn country_by_id(id: &str) -> Option<Country> {
    static CACHE: OnceLock<CountriesDb> = OnceLock::new();
    CACHE
        .get_or_init(|| parse(COUNTRIES_JSON))
        .paises
        .iter()
        .find(|c| c.id == id)
        .cloned()
}

/// Devuelve el tema por su identificador.
pub fn topic_by_id(id: &str) -> Option<Topic> {
    static CACHE: OnceLock<TopicsDb> = OnceLock::new();
    CACHE
        .get_or_init(|| parse(TOPICS_JSON))
        .temas
        .iter()
        .find(|t| t.id == id)
        .cloned()
}

/// Etiqueta legible de un tema.
pub fn topic_label(id: &str) -> String {
    topic_by_id(id)
        .map(|t| t.nombre)
        .unwrap_or_else(|| id.to_string())
}

/// Convierte un estado de verificación en una etiqueta corta para la interfaz.
pub fn verification_label(value: &str) -> &'static str {
    match value {
        "contenido_verificado" => "Verificado",
        "documento_local_verificado" => "Documento local",
        "dominio_snippet" => "Dominio oficial",
        "portal_validar" => "Pendiente",
        "base_local_usuario" => "Base local",
        "referencia_citada" => "Referencia",
        _ => "Sin verificar",
    }
}

/// Indica si una verificación se considera confirmada.
pub fn is_verified(value: &str) -> bool {
    value == "contenido_verificado" || value == "documento_local_verificado"
}

fn parse<T: for<'de> Deserialize<'de>>(json: &str) -> T {
    serde_json::from_str(json).expect("los datos incluidos en el binario son JSON válido")
}

/// Comparación tolerante a acentos y errores tipográficos menores.
/// Cada token de la búsqueda debe coincidir con alguna palabra del texto,
/// bien por substring, bien con distancia de edición 1 o 2.
fn fuzzy_match(query: &str, text: &str) -> bool {
    let stopwords = ["de", "del", "la", "el", "los", "las", "y", "para", "con"];
    let query_tokens: Vec<String> = words(query)
        .into_iter()
        .filter(|w| !stopwords.contains(&w.as_str()))
        .collect();
    if query_tokens.is_empty() {
        return false;
    }
    let text_words = words(text);
    query_tokens.iter().all(|token| {
        text_words.iter().any(|word| {
            if word.contains(token) || token.contains(word) {
                return true;
            }
            let max_dist = if token.len() <= 5 { 1 } else { 2 };
            levenshtein(token, word) <= max_dist
        })
    })
}

/// Extrae palabras normalizadas (sin acentos, solo alfanuméricas).
fn words(value: &str) -> Vec<String> {
    normalize(value)
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(|w| w.to_string())
        .collect()
}

/// Distancia de edición de Levenshtein entre dos cadenas.
fn levenshtein(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    let mut curr = vec![0usize; b.len() + 1];
    for (i, ca) in a.iter().enumerate() {
        curr[0] = i + 1;
        for (j, cb) in b.iter().enumerate() {
            let cost = if ca == cb { 0 } else { 1 };
            curr[j + 1] = (prev[j + 1] + 1)
                .min(curr[j] + 1)
                .min(prev[j] + cost);
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[b.len()]
}

fn normalize(value: &str) -> String {
    value
        .to_lowercase()
        .chars()
        .map(|c| match c {
            'á' | 'à' | 'ä' | 'â' | 'ã' => 'a',
            'é' | 'è' | 'ë' | 'ê' => 'e',
            'í' | 'ì' | 'ï' | 'î' => 'i',
            'ó' | 'ò' | 'ö' | 'ô' | 'õ' => 'o',
            'ú' | 'ù' | 'ü' | 'û' => 'u',
            'ñ' => 'n',
            'ç' => 'c',
            other => other,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hay_paises_y_temas() {
        assert!(!countries().is_empty());
        assert_eq!(topics().len(), 5);
    }

    #[test]
    fn cuba_tiene_normas_locales() {
        let instruments = instruments_for("cuba");
        assert!(!instruments.is_empty());
        assert!(instruments
            .iter()
            .any(|i| i.verificacion == "documento_local_verificado"));
        assert!(instruments.iter().any(|i| i.id == "cu-res-253-2021"));
    }

    #[test]
    fn no_hay_fichas_alojadas_en_el_repositorio() {
        assert!(substances().is_empty());
        assert!(search_substances("acetona").is_empty());
    }

    #[test]
    fn ficha_desde_pubchem_tiene_16_secciones() {
        let compuesto = PubChemCompound {
            cid: 3672,
            title: "Ibuprofen".to_string(),
            molecular_formula: "C13H18O2".to_string(),
            molecular_weight: "206.28".to_string(),
            inchi_key: "HEFNNWSXXWATRW-JTQLQIEISA-N".to_string(),
            x_log_p: serde_json::json!(3.97),
            exact_mass: "206.130679813".to_string(),
            heavy_atom_count: serde_json::json!(17),
            complexity: serde_json::json!(263),
            synonyms: vec!["Ibuprofeno [INN-Spanish]".to_string()],
            lang: "es".to_string(),
        };
        let ficha = substance_from_pubchem(&compuesto);
        assert_eq!(ficha.secciones.len(), 16);
        assert_eq!(ficha.estado, "online");
        assert!(ficha.fuentes[0].url.contains("3672"));
    }

    #[test]
    fn variantes_de_busqueda_multilingues() {
        let v = search_variants("Ácido cítrico");
        assert!(v.contains(&"acido citrico".to_string()));
        assert!(v.contains(&"citric acid".to_string()));
        let v2 = search_variants("acide citrique");
        assert!(v2.contains(&"citric acid".to_string()));
    }

    #[test]
    fn nombre_localizado_por_idioma() {
        let sinonimos = vec![
            "Ibuprofen [INN]".to_string(),
            "Ibuprofeno [INN-Spanish]".to_string(),
            "Ibuprofene [INN-French]".to_string(),
        ];
        assert_eq!(localized_name(&sinonimos, "es").as_deref(), Some("Ibuprofeno"));
        assert_eq!(localized_name(&sinonimos, "fr").as_deref(), Some("Ibuprofene"));
        assert_eq!(localized_name(&sinonimos, "de"), None);
    }

    #[test]
    fn interacciones_usan_nombres_de_linea() {
        let ids = vec!["online-3672".to_string(), "online-1983".to_string()];
        let mut nombres = HashMap::new();
        nombres.insert("online-3672".to_string(), "Ibuprofen".to_string());
        nombres.insert("online-1983".to_string(), "Paracetamol".to_string());
        let resultado = analyze_interactions_named(&ids, &nombres);
        assert_eq!(resultado.len(), 1);
        assert_eq!(resultado[0].nombre_a, "Ibuprofen");
        assert_eq!(resultado[0].nombre_b, "Paracetamol");
    }

    #[test]
    fn interacciones_cubren_todas_las_parejas() {
        let db = interactions_db();
        assert_eq!(db.interacciones.len(), 15);
        assert_eq!(db.niveles.len(), 4);
    }

    #[test]
    fn neutralizacion_es_peligro_alto() {
        let ids = vec![
            "acido-clorhidrico".to_string(),
            "hidroxido-sodio".to_string(),
        ];
        let result = analyze_interactions(&ids);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].nivel, "alto");
        assert_eq!(result[0].etiqueta, "Peligro alto");
        assert!(!result[0].es_regla_general);
    }

    #[test]
    fn pareja_desconocida_usa_regla_general() {
        let ids = vec!["acetona".to_string(), "desconocida".to_string()];
        let result = analyze_interactions(&ids);
        assert_eq!(result.len(), 1);
        assert!(result[0].es_regla_general);
        assert_eq!(result[0].nivel, "nulo");
    }

    #[test]
    fn espana_tiene_rd_97_2014() {
        assert!(instruments_for("espana")
            .iter()
            .any(|i| i.id == "es-rd-97-2014"));
    }
}
