//! Capa de normalización de consultas antes de llamar a PubChem.
//!
//! Todo nombre se traduce al inglés (idioma en el que PubChem indexa mejor),
//! y se reconocen los códigos CAS, las fórmulas químicas y los códigos de
//! producto industrial para usar el endpoint adecuado en cada caso.

use crate::{alias_for, normalize};

/// Tipo de consulta detectado; determina el endpoint de PubChem a utilizar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryKind {
    /// Número CAS con formato `NN-NNNN-N` (p. ej. `7664-93-9`).
    Cas(String),
    /// Fórmula molecular (p. ej. `H2SO4`, `NaCl`, `CH3COOH`).
    Formula(String),
    /// Código de producto o término industrial (p. ej. `AEO7`, `TX-100`).
    Product(String),
    /// Nombre químico en cualquier idioma; se traduce al inglés.
    Name,
}

const ACID_WORDS: &[&str] = &["acido", "acide", "acid", "saure", "saur", "acidos", "acides"];

/// Vocabulario químico multilingüe → inglés. Se conserva el inglés como
/// destino de las palabras que ya lo están para que la tabla sea total.
const WORDS: &[(&str, &str)] = &[
    // Ácidos
    ("citrico", "citric"), ("citrique", "citric"), ("zitronensaure", "citric"), ("citrico", "citric"),
    ("clorhidrico", "hydrochloric"), ("chlorhydrique", "hydrochloric"), ("chlorwasserstoffsaure", "hydrochloric"),
    ("sulfurico", "sulfuric"), ("sulfurique", "sulfuric"), ("schwefelsaure", "sulfuric"), ("solfurico", "sulfuric"),
    ("nitrico", "nitric"), ("nitrique", "nitric"), ("salpetersaure", "nitric"), ("nitrico", "nitric"),
    ("acetico", "acetic"), ("acetique", "acetic"), ("essigsaure", "acetic"), ("acetico", "acetic"),
    ("ascorbico", "ascorbic"), ("ascorbique", "ascorbic"), ("askorbinsaure", "ascorbic"),
    ("lactico", "lactic"), ("lactique", "lactic"), ("milchsaure", "lactic"), ("latico", "lactic"),
    ("benzoico", "benzoic"), ("benzoique", "benzoic"), ("benzoesaure", "benzoic"),
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
    // Bases, sales y aniones
    ("amoniaco", "ammonia"), ("ammoniaque", "ammonia"), ("ammoniak", "ammonia"),
    ("hidroxido", "hydroxide"), ("hydroxyde", "hydroxide"), ("hydroxid", "hydroxide"),
    ("sodio", "sodium"), ("natrium", "sodium"), ("sodium", "sodium"),
    ("potasio", "potassium"), ("kalium", "potassium"), ("potassium", "potassium"),
    ("calcio", "calcium"), ("calcium", "calcium"), ("magnesio", "magnesium"), ("magnesium", "magnesium"),
    ("amonio", "ammonium"), ("ammonium", "ammonium"),
    ("bicarbonato", "bicarbonate"), ("carbonato", "carbonate"), ("carbonate", "carbonate"),
    ("sulfato", "sulfate"), ("sulfate", "sulfate"), ("cloruro", "chloride"), ("chlorure", "chloride"),
    ("nitrato", "nitrate"), ("nitrate", "nitrate"), ("fosfato", "phosphate"), ("phosphate", "phosphate"),
    ("acetato", "acetate"), ("cianuro", "cyanide"), ("fluoruro", "fluoride"),
    ("bromuro", "bromide"), ("yoduro", "iodide"), ("peroxido", "peroxide"), ("peroxyde", "peroxide"),
    ("clorhidrato", "hydrochloride"),
    // Disolventes y compuestos comunes
    ("etanol", "ethanol"), ("ethanol", "ethanol"),
    ("metanol", "methanol"), ("methanol", "methanol"), ("acetona", "acetone"), ("acetone", "acetone"),
    ("cafeina", "caffeine"), ("cafeine", "caffeine"), ("koffein", "caffeine"), ("caffeina", "caffeine"),
    ("ibuprofeno", "ibuprofen"), ("ibuprofene", "ibuprofen"), ("ibuprofen", "ibuprofen"),
    ("paracetamol", "paracetamol"), ("acetaminofen", "acetaminophen"), ("acetaminophene", "acetaminophen"),
    ("estearico", "stearic"), ("stearique", "stearic"), ("stearinsaure", "stearic"),
    ("palmitico", "palmitic"), ("palmitique", "palmitic"), ("palmitinsaure", "palmitic"),
    ("oleico", "oleic"), ("oleique", "oleic"), ("olsaure", "oleic"),
    ("linoleico", "linoleic"), ("linoleique", "linoleic"),
    ("dipirona", "metamizole"), ("novalgina", "metamizole"),
    ("agua", "water"), ("eau", "water"), ("wasser", "water"), ("acqua", "water"),
    ("cloro", "chlorine"), ("chlore", "chlorine"), ("chlor", "chlorine"),
    ("hidrogeno", "hydrogen"), ("hydrogene", "hydrogen"), ("wasserstoff", "hydrogen"),
    ("oxigeno", "oxygen"), ("oxygene", "oxygen"), ("sauerstoff", "oxygen"),
    ("nitrogeno", "nitrogen"), ("azote", "nitrogen"), ("stickstoff", "nitrogen"),
    ("glucosa", "glucose"), ("sacarosa", "sucrose"), ("saccharose", "sucrose"),
    ("almidon", "starch"), ("amidon", "starch"), ("staerke", "starch"),
    ("aceite", "oil"), ("huile", "oil"), ("oel", "oil"),
    // Tensioactivos, etoxilatos y términos industriales
    ("etoxilado", "ethoxylated"), ("etoxilados", "ethoxylated"), ("ethoxylated", "ethoxylated"),
    ("etoxilato", "ethoxylate"), ("etoxilatos", "ethoxylates"), ("ethoxylate", "ethoxylate"),
    ("eter", "ether"), ("etere", "ether"), ("aether", "ether"),
    ("dietil", "diethyl"), ("dietileter", "diethyl ether"), ("etilico", "ethyl"),
    ("alcohol", "alcohol"), ("alcoholes", "alcohols"), ("graso", "fatty"), ("grasos", "fatty"),
    ("lauril", "lauryl"), ("laureth", "laureth"),
    ("tensioactivo", "surfactant"), ("tensioactivos", "surfactants"), ("detergente", "detergent"),
    ("disolvente", "solvent"), ("solvente", "solvent"), ("sal", "salt"),
    ("corrosivo", "corrosive"), ("inflamable", "flammable"), ("toxico", "toxic"),
    ("benceno", "benzene"), ("tolueno", "toluene"), ("xileno", "xylene"),
    ("hexano", "hexane"), ("heptano", "heptane"), ("pentano", "pentane"),
];

/// Frases completas que deben traducirse como una unidad.
const PHRASES: &[(&str, &str)] = &[
    ("alcohol etilico", "ethanol"),
    ("alcohol metilico", "methanol"),
    ("alcohol etoxilado", "ethoxylated alcohol"),
    ("alcoholes etoxilados", "ethoxylated alcohols"),
    ("eter de petroleo", "petroleum ether"),
    ("agua oxigenada", "hydrogen peroxide"),
    ("sal comun", "sodium chloride"),
    ("acido muriatico", "hydrochloric acid"),
];

/// Clasifica la consulta para elegir el endpoint de PubChem adecuado.
///
/// El orden importa: los códigos de producto se comprueban antes que las
/// fórmulas, porque muchos códigos (AEO7, BHT, EDTA) parecen fórmulas.
pub fn classify(query: &str) -> QueryKind {
    let q = query.trim();
    if let Some(cas) = detect_cas(q) {
        return QueryKind::Cas(cas);
    }
    if let Some(codigo) = alias_for(q) {
        return QueryKind::Product(codigo.id.clone());
    }
    if let Some(formula) = detect_formula(q) {
        return QueryKind::Formula(formula);
    }
    QueryKind::Name
}

/// Detecta un número CAS: `NN-NNNN-N` (entre 2 y 7 dígitos en el primer bloque).
pub fn detect_cas(query: &str) -> Option<String> {
    let q = query.trim();
    let partes: Vec<&str> = q.split('-').collect();
    if partes.len() != 3 {
        return None;
    }
    let (a, b, c) = (partes[0], partes[1], partes[2]);
    let valido = (2..=7).contains(&a.len())
        && a.bytes().all(|x| x.is_ascii_digit())
        && b.len() == 2
        && b.bytes().all(|x| x.is_ascii_digit())
        && c.len() == 1
        && c.bytes().all(|x| x.is_ascii_digit());
    if valido { Some(q.to_string()) } else { None }
}

/// Detecta una fórmula molecular escrita sin espacios (H2SO4, NaCl, CH3COOH).
pub fn detect_formula(query: &str) -> Option<String> {
    let q = query.trim();
    if q.is_empty() || q.len() > 40 || q.chars().any(char::is_whitespace) {
        return None;
    }
    let b = q.as_bytes();
    if !b[0].is_ascii_uppercase() {
        return None;
    }
    let mut i = 0;
    let mut simbolos = 0;
    while i < b.len() {
        match b[i] {
            c if c.is_ascii_uppercase() => {
                simbolos += 1;
                i += 1;
                if i < b.len() && b[i].is_ascii_lowercase() {
                    i += 1;
                }
                while i < b.len() && b[i].is_ascii_digit() {
                    i += 1;
                }
            }
            b'(' | b')' => {
                i += 1;
                while i < b.len() && b[i].is_ascii_digit() {
                    i += 1;
                }
            }
            _ => return None,
        }
    }
    if simbolos > 0 { Some(q.to_string()) } else { None }
}

/// Traduce la consulta al inglés, idioma en el que PubChem indexa los nombres.
pub fn english(query: &str) -> String {
    let base = normalize(query.trim());
    if let Some((_, destino)) = PHRASES.iter().find(|(origen, _)| normalize(origen) == base) {
        return destino.to_string();
    }

    let mut es_acid = false;
    let mut palabras: Vec<String> = Vec::new();
    for token in base.split_whitespace() {
        if ACID_WORDS.contains(&token) {
            es_acid = true;
            continue;
        }
        let convertido = WORDS
            .iter()
            .find(|(origen, _)| *origen == token)
            .map(|(_, destino)| destino.to_string())
            .unwrap_or_else(|| token.to_string());
        palabras.push(convertido);
    }
    let mut frase = palabras.join(" ");
    if es_acid && !frase.split_whitespace().any(|p| p == "acid") {
        frase = format!("{} acid", frase);
    }
    frase
}

/// Variantes de búsqueda: primero la traducción al inglés, luego el original.
pub fn name_variants(query: &str) -> Vec<String> {
    let mut variantes = vec![english(query), normalize(query.trim())];
    variantes.dedup();
    variantes
}
