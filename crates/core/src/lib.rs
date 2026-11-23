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
const ORGANIZATIONS_JSON: &str = include_str!("../../../data/organizations.json");
const ALIASES_JSON: &str = include_str!("../../../data/aliases.json");
const CAMEO_SUBSTANCES_JSON: &str = include_str!("../../../data/cameo_substances.json");
const CAMEO_REACTIONS_JSON: &str = include_str!("../../../data/cameo_reactions_min.json");
const PUBCHEM_REACTIVITY_JSON: &str = include_str!("../../../data/pubchem_reactivity.json");
const CAMEO_REVIEWED_JSON: &str = include_str!("../../../data/cameo_reviewed.json");
const NIOSH_LIMITS_JSON: &str = include_str!("../../../data/niosh_limits.json");

mod query;

pub use query::{
    classify, corregir_tildes, detect_cas, detect_formula, english, es_consulta_valida,
    name_variants, QueryKind,
};

// ---------------------------------------------------------------------------
// Modelos
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Country {
    pub id: String,
    pub nombre: String,
    pub codigo: String,
    #[serde(default)]
    pub idioma: String,
    #[serde(default)]
    pub saludo: String,
    #[serde(default)]
    pub descripcion: String,
    #[serde(default)]
    pub continente: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CountriesDb {
    pub paises: Vec<Country>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Topic {
    pub id: String,
    pub nombre: String,
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
    pub tema: String,
    pub titulo: String,
    pub url: String,
    #[serde(default)]
    pub verificacion: String,
    #[serde(default)]
    pub categoria: String,
    #[serde(default)]
    pub oficial: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SourcesDb {
    pub fuentes: Vec<Source>,
}

/// Término industrial o ambiguo resuelto a uno o más CIDs concretos de PubChem.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Alias {
    pub id: String,
    #[serde(default)]
    pub terminos: Vec<String>,
    #[serde(default)]
    pub cids: Vec<u64>,
    #[serde(default)]
    pub nota: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AliasesDb {
    pub alias: Vec<Alias>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Organization {
    pub id: String,
    pub nombre: String,
    #[serde(default)]
    pub miembros: Vec<String>,
    #[serde(default)]
    pub fuente_pais: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrganizationsDb {
    pub organizaciones: Vec<Organization>,
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
struct CameoReviewedDb {
    #[serde(default)]
    pub revisiones: Vec<serde_json::Value>,
}

/// Indica si un par ha sido revisado por un profesional.
pub fn par_revisado(id: &str) -> bool {
    static CACHE: OnceLock<CameoReviewedDb> = OnceLock::new();
    let db = CACHE.get_or_init(|| parse(CAMEO_REVIEWED_JSON));
    db.revisiones.iter().any(|r| {
        r.get("id").and_then(|v| v.as_str()).map(|s| s == id).unwrap_or(false)
    })
}

/// Límites de exposición de la Guía de Bolsillo de NIOSH.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NioshLimit {
    pub cas: String,
    pub nombre: String,
    #[serde(default)]
    pub rel: String,
    #[serde(default)]
    pub pel: String,
    #[serde(default)]
    pub idlh: String,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct NioshLimitsDb {
    pub limites: Vec<NioshLimit>,
}

/// Perfil de reactividad de PubChem para sustancias sin clasificación CAMEO.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PubChemReactivity {
    pub cid: u64,
    pub nombre: String,
    pub secciones: std::collections::HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct PubChemReactivityDb {
    pub sustancias: Vec<PubChemReactivity>,
}

/// Sustancia mapeada a sus grupos reactivos de CAMEO.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CameoSubstance {
    pub cas: String,
    pub nombre: String,
    pub cameo_id: u64,
    pub url: String,
    pub grupos: Vec<CameoGrupo>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CameoGrupo {
    pub id: String,
    pub numero: u32,
    pub nombre: String,
}

/// Par de grupos reactivos con predicciones documentadas.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CameoPair {
    pub id: String,
    pub a: u32,
    pub b: u32,
    #[serde(default)]
    pub peligros: Vec<String>,
    #[serde(default)]
    pub gases: Vec<String>,
    #[serde(default)]
    pub citas: Vec<String>,
}

impl CameoPair {
    /// URL de la documentación del par (se deriva de los identificadores).
    pub fn url(&self) -> String {
        format!(
            "https://cameochemicals.noaa.gov/reactivity/documentation/RG{}-RG{}",
            self.a, self.b
        )
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct CameoSubstancesDb {
    pub sustancias: Vec<CameoSubstance>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct CameoReactionsDb {
    pub pares: Vec<CameoPair>,
}

/// Resultado agregado de la compatibilidad entre dos sustancias.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CameoAnalysis {
    pub nivel: String,
    pub etiqueta: String,
    /// Clasificación de la mezcla según el criterio de CAMEO.
    pub compatibilidad: String,
    /// Indica si un profesional ha revisado y confirmado este par.
    pub revisado: bool,
    pub peligros: Vec<String>,
    pub gases: Vec<String>,
    pub citas: Vec<String>,
    pub fuentes: Vec<String>,
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

/// Extrae el primer número CAS válido de una lista de sinónimos.
pub fn extraer_cas(sinonimos: &[String]) -> Option<String> {
    sinonimos.iter().find_map(|s| {
        let texto = s.trim();
        if texto.len() > 30 { return None; }
        let partes: Vec<&str> = texto.split('-').collect();
        if partes.len() != 3 { return None; }
        let (a, b, c) = (partes[0], partes[1], partes[2]);
        if !(2..=7).contains(&a.len()) || !a.bytes().all(|x| x.is_ascii_digit()) { return None; }
        if b.len() != 2 || !b.bytes().all(|x| x.is_ascii_digit()) { return None; }
        if c.len() != 1 || !c.bytes().all(|x| x.is_ascii_digit()) { return None; }
        Some(texto.to_string())
    })
}

/// Límites de exposición de NIOSH (cacheados).
pub fn niosh_limits() -> &'static [NioshLimit] {
    static CACHE: OnceLock<NioshLimitsDb> = OnceLock::new();
    &CACHE.get_or_init(|| parse(NIOSH_LIMITS_JSON)).limites
}

/// Busca los límites de exposición de NIOSH por CAS.
pub fn niosh_limit_for_cas(cas: &str) -> Option<&'static NioshLimit> {
    let buscado = normalize(cas.trim());
    niosh_limits().iter().find(|l| normalize(&l.cas) == buscado)
}

/// Perfiles de reactividad de PubChem (cacheados).
pub fn pubchem_reactivity() -> &'static [PubChemReactivity] {
    static CACHE: OnceLock<PubChemReactivityDb> = OnceLock::new();
    &CACHE.get_or_init(|| parse(PUBCHEM_REACTIVITY_JSON)).sustancias
}

/// Devuelve el perfil de reactividad de PubChem para un CID.
pub fn pubchem_reactivity_for_cid(cid: u64) -> Option<&'static PubChemReactivity> {
    pubchem_reactivity().iter().find(|r| r.cid == cid)
}

/// Base de sustancias mapeadas a grupos reactivos (cacheada).
pub fn cameo_substances() -> &'static [CameoSubstance] {
    static CACHE: OnceLock<CameoSubstancesDb> = OnceLock::new();
    &CACHE.get_or_init(|| parse(CAMEO_SUBSTANCES_JSON)).sustancias
}

/// Busca una sustancia de CAMEO por número CAS normalizado.
pub fn cameo_substance_for_cas(cas: &str) -> Option<&'static CameoSubstance> {
    let buscado = normalize(cas.trim());
    cameo_substances().iter().find(|s| normalize(&s.cas) == buscado)
}

/// Base de pares reactivos (cacheada).
pub fn cameo_pairs() -> &'static [CameoPair] {
    static CACHE: OnceLock<CameoReactionsDb> = OnceLock::new();
    &CACHE.get_or_init(|| parse(CAMEO_REACTIONS_JSON)).pares
}

/// Analiza la compatibilidad entre dos CAS con datos de NOAA/EPA CAMEO.
///
/// Una sustancia puede pertenecer a varios grupos reactivos; se agregan todos
/// los pares resultantes y se deduce el nivel más grave.
pub fn cameo_analyze(cas_a: &str, cas_b: &str) -> Option<CameoAnalysis> {
    let a = cameo_substance_for_cas(cas_a)?;
    let b = cameo_substance_for_cas(cas_b)?;
    let pares = cameo_pairs();
    let alto = ["Explosive", "Intense or explosive reaction", "Toxic", "Corrosive"];
    let mut peligros: Vec<String> = Vec::new();
    let mut gases: Vec<String> = Vec::new();
    let mut citas: Vec<String> = Vec::new();
    let mut fuentes: Vec<String> = Vec::new();
    let mut revisado = false;
    for ga in &a.grupos {
        for gb in &b.grupos {
            let (x, y) = if ga.numero <= gb.numero {
                (ga.numero, gb.numero)
            } else {
                (gb.numero, ga.numero)
            };
            if let Some(par) = pares.iter().find(|p| p.a == x && p.b == y) {
                if par_revisado(&par.id) { revisado = true; }
                for d in &par.peligros {
                    if !peligros.iter().any(|q| q == d) { peligros.push(d.clone()); }
                }
                for g in &par.gases {
                    if !gases.iter().any(|q| q == g) { gases.push(g.clone()); }
                }
                for c in &par.citas {
                    if !citas.iter().any(|q| q == c) { citas.push(c.clone()); }
                }
                if !fuentes.iter().any(|q| q == &par.url()) { fuentes.push(par.url()); }
            }
        }
    }
    let nivel = if peligros.iter().any(|p| alto.iter().any(|a| p == a)) {
        "alto"
    } else if peligros.iter().any(|p| p != "No known hazardous reaction") {
        "moderado"
    } else {
        "nulo"
    };
    let etiqueta = match nivel {
        "alto" => "Peligro alto",
        "moderado" => "Precaución",
        _ => "Sin reacción registrada",
    };
    // Criterio publicado por CAMEO: incompatible si hay productos peligrosos
    // o calor excesivo; cautela si hay inestabilidad o reacción lenta;
    // compatible si no hay reacción conocida.
    let incompatibles = [
        "Explosive", "Intense or explosive reaction", "Toxic", "Corrosive", "Flammable",
    ];
    let compatibilidad = if peligros.iter().any(|p| incompatibles.iter().any(|i| p == i)) {
        "incompatible"
    } else if peligros.iter().any(|p| p != "No known hazardous reaction") {
        "precaucion"
    } else {
        "compatible"
    };
    Some(CameoAnalysis {
        nivel: nivel.to_string(),
        etiqueta: etiqueta.to_string(),
        compatibilidad: compatibilidad.to_string(),
        revisado,
        peligros, gases, citas, fuentes,
    })
}

// ---------------------------------------------------------------------------
// Carga de datos
// ---------------------------------------------------------------------------

/// Devuelve el tema adecuado según la hora local (modo «automático»).
///
/// De 07:00 a 19:00 se usa el tema claro; el resto del día, el oscuro, para
/// no deslumbrar al usuario cuando ya es de noche en su zona horaria.
pub fn tema_por_hora(hora: u32) -> &'static str {
    if (7..19).contains(&hora) { "light" } else { "dark" }
}

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

/// Carga y devuelve todas las organizaciones regionales (datos cacheados).
pub fn organizations() -> Vec<Organization> {
    static CACHE: OnceLock<OrganizationsDb> = OnceLock::new();
    CACHE
        .get_or_init(|| parse(ORGANIZATIONS_JSON))
        .organizaciones
        .clone()
}

/// Indica si una fuente es normativa oficial, verificada y con enlace directo.
fn es_normativa_oficial(s: &Source) -> bool {
    s.oficial && s.categoria == "normativa" && is_verified(&s.verificacion) && !s.url.is_empty()
}

/// Devuelve las organizaciones regionales aplicables al código ISO de un país.
///
/// Solo se incluyen organizaciones con marco legal relevante y verificado:
/// una organización sin normativa oficial comprobable no se tiene en cuenta.
pub fn organizations_for(codigo_iso: &str) -> Vec<Organization> {
    let fuentes = sources();
    organizations()
        .into_iter()
        .filter(|o| {
            o.miembros.iter().any(|m| m == codigo_iso)
                && fuentes
                    .iter()
                    .any(|s| s.pais == o.fuente_pais && es_normativa_oficial(s))
        })
        .collect()
}

/// Normativa verificada de una organización regional, para el país seleccionado.
pub fn organization_sources(fuente_pais: &str) -> Vec<Source> {
    sources()
        .into_iter()
        .filter(|s| s.pais == fuente_pais && es_normativa_oficial(s))
        .collect()
}

/// Resuelve nombres industriales o ambiguos («AEO7», «éter», «alcohol
/// etoxilado») a los CIDs de PubChem que realmente los representan.
pub fn alias_for(query: &str) -> Option<&'static Alias> {
    static CACHE: OnceLock<AliasesDb> = OnceLock::new();
    let db = CACHE.get_or_init(|| parse(ALIASES_JSON));
    let busqueda = normalize(query.trim());
    db.alias.iter().find(|a| {
        a.terminos
            .iter()
            .any(|t| normalize(t.trim()) == busqueda)
    })
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
    /// Aclaración cuando el resultado procede de un término industrial o ambiguo.
    #[serde(default)]
    pub nota: String,
    /// Nombre traducido automáticamente al idioma de salida elegido.
    #[serde(default)]
    pub nombre_localizado: String,
    /// Número CAS extraído de los sinónimos de PubChem, cuando existe.
    #[serde(default)]
    pub cas: String,
}

/// Devuelve un sinónimo localizado a partir de las etiquetas que PubChem
/// añade a algunos nombres, por ejemplo `[INN-Spanish]` o `[French]`.
pub fn localized_name(synonyms: &[String], lang: &str) -> Option<String> {
    let marcas: &[&str] = match lang {
        "es" => &["[inn-spanish]", "[spanish]", "[es]"],
        "fr" => &["[inn-french]", "[french]", "[fr]"],
        "de" => &["[inn-german]", "[german]", "[de]"],
        "it" => &["[inn-italian]", "[italian]", "[it]"],
        "pt" => &["[inn-portuguese]", "[portuguese]", "[pt]"],
        "ru" => &["[russian]", "[ru]"],
        "zh" => &["[chinese]", "[zh]"],
        "ja" => &["[japanese]", "[ja]"],
        "ar" => &["[arabic]", "[ar]"],
        "hi" => &["[hindi]", "[hi]"],
        "nl" => &["[dutch]", "[nl]"],
        "sv" => &["[swedish]", "[sv]"],
        "pl" => &["[polish]", "[pl]"],
        "tr" => &["[turkish]", "[tr]"],
        "ko" => &["[korean]", "[ko]"],
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
        cas: p.cas.clone(),
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

/// Elimina líneas repetidas manteniendo el orden.
fn dedup_lineas(lineas: Vec<String>) -> Vec<String> {
    let mut vistas = std::collections::HashSet::new();
    lineas
        .into_iter()
        .filter(|l| vistas.insert(l.trim().to_string()))
        .collect()
}

/// Busca una sección por su encabezado dentro del árbol de PUG View.
fn encontrar_seccion<'a>(secciones: &'a [serde_json::Value], titulo: &str) -> Option<&'a serde_json::Value> {
    for sec in secciones {
        if sec.get("TOCHeading").and_then(|v| v.as_str()) == Some(titulo) {
            return Some(sec);
        }
        if let Some(hijos) = sec.get("Section").and_then(|v| v.as_array()) {
            if let Some(hallado) = encontrar_seccion(hijos, titulo) {
                return Some(hallado);
            }
        }
    }
    None
}

/// Extrae líneas legibles de una sección de PUG View (incluye subsecciones).
fn lineas_seccion(sec: Option<&serde_json::Value>, maximo: usize) -> Vec<String> {
    let Some(sec) = sec else { return Vec::new() };
    let mut lineas = Vec::new();
    if let Some(info) = sec.get("Information").and_then(|v| v.as_array()) {
        for item in info.iter().take(maximo) {
            let nombre = item.get("Name").and_then(|v| v.as_str()).unwrap_or("");
            let valor = item.get("Value");
            let mut textos = Vec::new();
            if let Some(cadenas) = valor
                .and_then(|v| v.get("StringWithMarkup"))
                .and_then(|v| v.as_array())
            {
                for entrada in cadenas {
                    if let Some(s) = entrada.get("String").and_then(|v| v.as_str()) {
                        textos.push(s.to_string());
                    }
                }
            }
            if let Some(numero) = valor.and_then(|v| v.get("Number")) {
                textos.push(numero.to_string());
            }
            if textos.is_empty() {
                continue;
            }
            let mut texto = textos.join(" ");
            if texto.len() > 600 {
                texto = format!("{}…", &texto[..600]);
            }
            if nombre.is_empty() {
                lineas.push(texto);
            } else {
                lineas.push(format!("{}: {}", nombre, texto));
            }
        }
    }
    if let Some(hijos) = sec.get("Section").and_then(|v| v.as_array()) {
        for hijo in hijos {
            lineas.extend(lineas_seccion(Some(hijo), maximo));
        }
    }
    lineas
}

/// Construye una ficha técnica completa a partir de los datos PUG View de PubChem.
pub fn substance_from_pubchem_view(
    base: &PubChemCompound,
    view: &serde_json::Value,
    lang: &str,
) -> Substance {
    let vacias: Vec<serde_json::Value> = Vec::new();
    let secciones_root = view
        .get("Record")
        .and_then(|r| r.get("Section"))
        .and_then(|s| s.as_array())
        .map_or(vacias.as_slice(), |v| v);
    let nombre_mostrado = localized_name(&base.synonyms, lang).unwrap_or_else(|| base.title.clone());

    let buscar = |titulo: &str| encontrar_seccion(secciones_root, titulo);
    let items = |titulo: &str| lineas_seccion(buscar(titulo), 8);

    let safety = buscar("Safety and Hazards");
    let propiedades = buscar("Chemical and Physical Properties");
    let fabricacion = buscar("Use and Manufacturing");

    let seccion = |numero: u32, id: &str, titulo: &str, items_: Vec<String>| FdsSection {
        numero,
        id: id.to_string(),
        titulo: titulo.to_string(),
        texto: if items_.is_empty() {
            "No hay datos publicados para esta sección en la fuente consultada.".to_string()
        } else {
            String::new()
        },
        campos: Vec::new(),
        listas: if items_.is_empty() {
            Vec::new()
        } else {
            vec![FdsList {
                titulo: "Datos de PubChem (NIH)".to_string(),
                items: items_,
            }]
        },
        notas: Vec::new(),
    };

    let mut secciones = vec![
        seccion(1, "identificacion", "Identificación del producto y del proveedor", {
            let mut v = vec![
                format!("Nombre: {}", nombre_mostrado),
                format!("CID de PubChem: {}", base.cid),
            ];
            v.extend(items("Names and Identifiers"));
            v
        }),
        seccion(2, "peligros", "Identificación de los peligros", {
            let mut v = lineas_seccion(buscar("GHS Classification"), 60);
            v.extend(items("Hazards Identification"));
            v.extend(lineas_seccion(safety, 8));
            dedup_lineas(v)
        }),
        seccion(3, "composicion", "Composición / información sobre los componentes", {
            let mut v = vec![format!("Fórmula molecular: {}", base.molecular_formula)];
            v.extend(items("CAS"));
            v
        }),
        seccion(4, "primeros_auxilios", "Primeros auxilios", items("First Aid Measures")),
        seccion(5, "incendio", "Medidas de lucha contra incendios", items("Fire Fighting")),
        seccion(6, "derrame", "Medidas en caso de vertido accidental", items("Accidental Release Measures")),
        seccion(7, "manipulacion", "Manipulación y almacenamiento", items("Handling and Storage")),
        seccion(8, "exposicion", "Controles de exposición / protección personal",
            items("Exposure Control and Personal Protection")),
        seccion(9, "propiedades", "Propiedades físicas y químicas",
            lineas_seccion(propiedades, 10)),
        seccion(10, "estabilidad", "Estabilidad y reactividad", items("Stability and Reactivity")),
        seccion(11, "toxicologica", "Información toxicológica", items("Toxicological Information")),
        seccion(12, "ecotoxicologica", "Información ecotoxicológica", items("Ecological Information")),
        seccion(13, "eliminacion", "Consideraciones sobre eliminación", {
            let mut v = items("Disposal Methods");
            v.extend(items("Spillage Disposal"));
            v
        }),
        seccion(14, "transporte", "Información relativa al transporte", {
            let mut v = items("Transport Information");
            if v.is_empty() {
                v.push("Consultar el número ONU, clase y grupo de embalaje en la ficha del fabricante o en ADR/RID/IMDG/IATA.".to_string());
            }
            v
        }),
        seccion(15, "reglamentacion", "Información sobre la reglamentación", {
            let mut v = items("Regulatory Information");
            v.extend(lineas_seccion(fabricacion, 3));
            v
        }),
        seccion(16, "otras", "Otras informaciones", vec![
            "Fuente: PubChem (NIH), vista PUG View.".to_string(),
            format!("CID: {}", base.cid),
            "Ficha generada automáticamente; verificar siempre con la FDS del fabricante.".to_string(),
        ]),
    ];

    // Evitar secciones duplicadas por nombres repetidos (p. ej. "CAS" dentro de nombres)
    for s in secciones.iter_mut() {
        s.listas.iter_mut().for_each(|l| {
            l.items = dedup_lineas(std::mem::take(&mut l.items));
        });
    }

    let _ = safety;

    Substance {
        id: format!("online-{}", base.cid),
        nombre: nombre_mostrado,
        sinonimos: base.synonyms.iter().take(25).cloned().collect(),
        cas: base.cas.clone(),
        ce: String::new(),
        onu: String::new(),
        peligrosidad: Peligrosidad::default(),
        almacenamiento: String::new(),
        derrame: String::new(),
        propiedades: Propiedades::default(),
        interacciones: Interacciones::default(),
        fuentes: vec![
            Fuente { titulo: format!("PubChem - CID {}", base.cid), url: base.url() },
            Fuente { titulo: "ICSC - OIT/OMS".to_string(), url: "https://chemicalsafety.ilo.org/dyn/icsc/showcard.listCards3?p_lang=es".to_string() },
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

/// Variante que recibe los nombres de las sustancias, necesaria cuando estas
/// proceden de una base de datos en línea y no están en el repositorio.
pub fn analyze_interactions_named(
    ids: &[String],
    nombres: &HashMap<String, String>,
) -> Vec<PairAnalysis> {
    let db = interactions_db();
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
                .unwrap_or_else(|| a.clone());
            let nombre_b = nombres
                .get(b)
                .cloned()
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

pub(crate) fn normalize(value: &str) -> String {
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
            nota: String::new(),
            nombre_localizado: String::new(),
            cas: String::new(),
        };
        let ficha = substance_from_pubchem(&compuesto);
        assert_eq!(ficha.secciones.len(), 16);
        assert_eq!(ficha.estado, "online");
        assert!(ficha.fuentes[0].url.contains("3672"));
    }

    #[test]
    fn variantes_de_busqueda_multilingues() {
        let v = name_variants("Ácido cítrico");
        assert!(v.contains(&"acido citrico".to_string()));
        assert!(v.contains(&"citric acid".to_string()));
        let v2 = name_variants("acide citrique");
        assert!(v2.contains(&"citric acid".to_string()));
        let v3 = name_variants("ácido estearico");
        assert!(v3.contains(&"stearic acid".to_string()));
    }

    #[test]
    fn nombre_localizado_por_idioma() {
        let mut sinonimos = vec![
            "Ibuprofen [INN]".to_string(),
            "Ibuprofeno [INN-Spanish]".to_string(),
            "Ibuprofene [INN-French]".to_string(),
        ];
        assert_eq!(localized_name(&sinonimos, "es").as_deref(), Some("Ibuprofeno"));
        assert_eq!(localized_name(&sinonimos, "fr").as_deref(), Some("Ibuprofene"));
        assert_eq!(localized_name(&sinonimos, "de"), None);
        sinonimos.push("Ibuprofeno [INN-Portuguese]".to_string());
        assert_eq!(localized_name(&sinonimos, "pt").as_deref(), Some("Ibuprofeno"));
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
        let result = analyze_interactions_named(&ids, &HashMap::new());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].nivel, "alto");
        assert_eq!(result[0].etiqueta, "Peligro alto");
        assert!(!result[0].es_regla_general);
    }

    #[test]
    fn pareja_desconocida_usa_regla_general() {
        let ids = vec!["acetona".to_string(), "desconocida".to_string()];
        let result = analyze_interactions_named(&ids, &HashMap::new());
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

    #[test]
    fn organizaciones_solo_con_marco_legal_verificado() {
        let alemania = organizations_for("DE");
        assert_eq!(alemania.len(), 1);
        assert_eq!(alemania[0].id, "ue");

        let colombia = organizations_for("CO");
        assert_eq!(colombia.len(), 1);
        assert_eq!(colombia[0].id, "comunidad_andina");

        // MERCOSUR se descartó: sus normas están tras Cloudflare y no hay
        // enlaces directos verificables al texto legal.
        assert!(organizations_for("BR").is_empty());
        assert!(organizations_for("US").is_empty());
    }

    #[test]
    fn capa_query_clasifica_y_traduce() {
        use QueryKind::*;
        assert_eq!(classify("7664-93-9"), Cas("7664-93-9".to_string()));
        assert_eq!(classify("H2SO4"), Formula("H2SO4".to_string()));
        assert_eq!(classify("NaCl"), Formula("NaCl".to_string()));
        assert_eq!(classify("CH3COOH"), Formula("CH3COOH".to_string()));
        assert_eq!(classify("AEO7"), Product("alcohol-etoxilado".to_string()));
        assert_eq!(classify("TX-100"), Product("triton-x-100".to_string()));
        assert_eq!(classify("ácido sulfúrico"), Name);

        assert_eq!(english("Ácido sulfúrico"), "sulfuric acid");
        assert_eq!(english("ácido cítrico"), "citric acid");
        assert_eq!(english("alcohol etílico"), "ethanol");
        assert_eq!(english("alcohol etoxilado"), "ethoxylated alcohol");
        assert_eq!(english("acetona"), "acetone");
        let variantes = name_variants("Ácido sulfúrico");
        assert_eq!(variantes[0], "sulfuric acid");
        assert_eq!(variantes[1], "acido sulfurico");
    }

    #[test]
    fn extrae_cas_de_sinonimos() {
        let sin = vec!["Sulfuric acid".to_string(), "7664-93-9".to_string(), "CID 1118".to_string()];
        assert_eq!(extraer_cas(&sin).as_deref(), Some("7664-93-9"));
        assert!(extraer_cas(&["no CAS aqui".to_string()]).is_none());
    }

    #[test]
    fn cameo_analiza_pares_por_cas() {
        let r = cameo_analyze("7664-93-9", "67-64-1").unwrap();
        assert_eq!(r.nivel, "alto");
        assert_eq!(r.compatibilidad, "incompatible");
        assert!(r.peligros.iter().any(|p| p.contains("Toxic") || p.contains("Explosive") || p.contains("Generates")));
        assert!(r.fuentes.iter().any(|u| u.contains("cameochemicals.noaa.gov")));
        let nulo = cameo_analyze("67-64-1", "67-64-1").unwrap();
        assert_eq!(nulo.nivel, "nulo");
        assert_eq!(nulo.compatibilidad, "compatible");
        assert!(cameo_analyze("000-00-0", "67-64-1").is_none());
    }

    #[test]
    fn tildes_en_nombre_mostrado() {
        assert_eq!(corregir_tildes("Acido sulfurico"), "Ácido sulfúrico");
        assert_eq!(corregir_tildes("Acido sulfurico, H2SO4"), "Ácido sulfúrico, H2SO4");
        assert_eq!(corregir_tildes("Triton X 100"), "Tritón X 100");
        assert_eq!(corregir_tildes("Ácido sulfúrico"), "Ácido sulfúrico");
    }

    #[test]
    fn tema_segun_hora_local() {
        assert_eq!(tema_por_hora(6), "dark");
        assert_eq!(tema_por_hora(7), "light");
        assert_eq!(tema_por_hora(13), "light");
        assert_eq!(tema_por_hora(18), "light");
        assert_eq!(tema_por_hora(19), "dark");
        assert_eq!(tema_por_hora(23), "dark");
    }

    #[test]
    fn valida_consultas_con_ruido() {
        assert!(es_consulta_valida("ácido sulfúrico"));
        assert!(es_consulta_valida("7664-93-9"));
        assert!(es_consulta_valida("H2SO4"));
        assert!(es_consulta_valida("AEO7"));
        assert!(!es_consulta_valida(""));
        assert!(!es_consulta_valida("😀"));
        assert!(!es_consulta_valida("123"));
        assert!(!es_consulta_valida("!!!"));
    }

    #[test]
    fn detecta_cas_y_formula_con_casos_limite() {
        assert!(detect_cas("67-56-1").is_some());
        assert!(detect_cas("7664-93-9").is_some());
        assert!(detect_cas("123-45").is_none());
        assert!(detect_cas("abc-56-1").is_none());
        assert!(detect_formula("NaCl").is_some());
        assert!(detect_formula("H2SO4").is_some());
        assert!(detect_formula("CH3COOH").is_some());
        assert!(detect_formula("Acetona").is_none());
        assert!(detect_formula("H2 SO4").is_none());
    }

    #[test]
    fn alias_resuelve_terminos_industriales_y_ambiguos() {
        let aeo = alias_for("AEO7").expect("AEO7 debe resolverse");
        assert_eq!(aeo.cids, vec![76459, 76457]);
        assert!(!aeo.nota.is_empty());
        assert_eq!(alias_for("alcohol etoxilado").unwrap().id, aeo.id);
        assert_eq!(alias_for("Éter").unwrap().cids, vec![3283]);
        assert_eq!(alias_for("ETER").unwrap().cids, vec![3283]);
        assert_eq!(alias_for("dietil eter").unwrap().cids, vec![3283]);
        assert!(alias_for("acetona").is_none());
    }

    #[test]
    fn normativa_de_organizaciones_tiene_enlace_directo_verificado() {
        for org in organizations() {
            let normas = organization_sources(&org.fuente_pais);
            assert!(!normas.is_empty(), "{} sin normativa verificada", org.nombre);
            for s in normas {
                assert!(
                    s.url.starts_with("https://"),
                    "{}: enlace no directo",
                    s.id
                );
                assert!(is_verified(&s.verificacion), "{}: sin verificar", s.id);
            }
        }
    }
}
