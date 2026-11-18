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
            normalize(&haystack).contains(&q)
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
    fn espana_tiene_rd_97_2014() {
        assert!(instruments_for("espana")
            .iter()
            .any(|i| i.id == "es-rd-97-2014"));
    }
}
