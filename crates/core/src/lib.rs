//! Núcleo de la aplicación de sustancias peligrosas.
//!
//! Toda la lógica (países, marco legal, búsqueda de sustancias) vive aquí y es
//! independiente de la interfaz. Los datos se incluyen en el binario con
//! `include_str!`, de modo que la misma librería compila tanto para nativo como
//! para WebAssembly sin depender de un sistema de gestión de bases de datos.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

/// Carga y devuelve todos los países disponibles.
pub fn countries() -> Vec<Country> {
    parse::<CountriesDb>(COUNTRIES_JSON).paises
}

/// Carga y devuelve todos los apartados/temas.
pub fn topics() -> Vec<Topic> {
    parse::<TopicsDb>(TOPICS_JSON).temas
}

/// Carga y devuelve todas las fuentes.
pub fn sources() -> Vec<Source> {
    parse::<SourcesDb>(SOURCES_JSON).fuentes
}

/// Devuelve las fuentes aplicables a un país (o todas si es "internacional").
pub fn sources_for(country_id: &str) -> Vec<Source> {
    sources()
        .into_iter()
        .filter(|s| {
            country_id == "internacional"
                || s.pais == country_id
                || (country_id == "espana" && s.pais == "espana")
                || (country_id == "eeuu" && s.pais == "eeuu")
        })
        .collect()
}

/// Carga y devuelve la base legislativa completa.
pub fn legislation_db() -> LegislationDb {
    parse::<LegislationDb>(LEGISLATION_JSON)
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

/// Base de interacciones entre sustancias.
pub fn interactions_db() -> InteractionsDb {
    parse::<InteractionsDb>(INTERACTIONS_JSON)
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
    interactions_db().advertencia
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
    let db = interactions_db();
    let all = substances();
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

            let nombre_a = all
                .iter()
                .find(|s| &s.id == a)
                .map(|s| s.nombre.clone())
                .unwrap_or_else(|| a.clone());
            let nombre_b = all
                .iter()
                .find(|s| &s.id == b)
                .map(|s| s.nombre.clone())
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

/// Advertencia sobre el carácter de las fichas de demostración.
pub fn substances_warning() -> String {
    parse::<SubstancesDb>(SUBSTANCES_JSON).advertencia
}

/// Carga y devuelve todas las sustancias.
pub fn substances() -> Vec<Substance> {
    parse::<SubstancesDb>(SUBSTANCES_JSON).sustancias
}

/// Busca una sustancia por identificador.
pub fn substance_by_id(id: &str) -> Option<Substance> {
    substances().into_iter().find(|s| s.id == id)
}

/// Busca sustancias por nombre, sinónimo, CAS, CE u ONU. La búsqueda ignora
/// mayúsculas y acentos para que "acido" encuentre "Ácido".
pub fn search_substances(query: &str) -> Vec<Substance> {
    let q = normalize(query);
    if q.is_empty() {
        return Vec::new();
    }
    substances()
        .into_iter()
        .filter(|s| {
            let haystack = [
                s.nombre.clone(),
                s.sinonimos.join(" "),
                s.cas.clone(),
                s.ce.clone(),
                s.onu.clone(),
            ]
            .join(" ");
            if q.chars().any(|c| c.is_ascii_digit()) {
                normalize(&haystack).contains(&q)
            } else {
                fuzzy_match(&q, &haystack)
            }
        })
        .collect()
}

/// Devuelve el país por su identificador.
pub fn country_by_id(id: &str) -> Option<Country> {
    countries().into_iter().find(|c| c.id == id)
}

/// Devuelve el tema por su identificador.
pub fn topic_by_id(id: &str) -> Option<Topic> {
    topics().into_iter().find(|t| t.id == id)
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
    fn busca_por_cas_y_acentos() {
        assert_eq!(search_substances("67-64-1").len(), 1);
        assert_eq!(search_substances("acido clorhidrico").len(), 1);
        assert_eq!(search_substances("ACETONA").len(), 1);
    }

    #[test]
    fn busca_con_errores_y_sin_acentos() {
        assert_eq!(search_substances("acetna").len(), 1);
        assert_eq!(search_substances("acido clorhidrco").len(), 1);
        assert_eq!(search_substances("hidroxido sodio").len(), 1);
        assert_eq!(search_substances("amoniaco").len(), 1);
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
