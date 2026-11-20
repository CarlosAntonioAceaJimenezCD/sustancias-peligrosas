//! Almacenamiento local para las versiones nativas (escritorio/móvil).
//!
//! Los documentos descargados se guardan primero en el directorio de datos de
//! la aplicación (para poder consultarlos sin internet) y, según las opciones,
//! se copian al directorio de descargas del sistema.
//!
//! Este crate **no se compila para WebAssembly**: la versión web no dispone de
//! sistema de archivos y descarga directamente al navegador.

use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const APP_DIR: &str = "sustancias-peligrosas";
/// Carpeta propia que se crea dentro del directorio de descargas.
pub const EXPORT_DIR: &str = "Sustancias Peligrosas";
const DOCS_DIR: &str = "documents";
const INDEX_FILE: &str = "index.json";
const OPTIONS_FILE: &str = "options.json";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DocumentKind {
    Legislation,
    Substance,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SavedDocument {
    pub id: String,
    pub titulo: String,
    pub kind: DocumentKind,
    pub filename: String,
    pub saved_at: String,
    pub size: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Options {
    /// Guardar los documentos en la app para consultarlos sin conexión.
    pub offline_cache: bool,
    /// Copiar además el archivo al directorio de descargas del sistema.
    pub copy_to_downloads: bool,
    /// Directorio de descargas personalizado (None = el del sistema).
    pub downloads_dir: Option<PathBuf>,
    /// Abrir automáticamente un issue de GitHub al detectar un error.
    pub auto_report: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            offline_cache: true,
            copy_to_downloads: true,
            downloads_dir: None,
            auto_report: false,
        }
    }
}

/// Directorio de datos de la aplicación dentro del directorio de datos del usuario.
pub fn app_data_dir() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(APP_DIR)
}

pub fn documents_dir() -> PathBuf {
    app_data_dir().join(DOCS_DIR)
}

fn index_path() -> PathBuf {
    app_data_dir().join(INDEX_FILE)
}

fn options_path() -> PathBuf {
    app_data_dir().join(OPTIONS_FILE)
}

fn ensure_dirs() -> io::Result<()> {
    fs::create_dir_all(documents_dir())
}

/// Carga las opciones, con los valores por defecto si no existen o son inválidas.
pub fn load_options() -> Options {
    fs::read_to_string(options_path())
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

/// Guarda las opciones en el directorio de datos de la aplicación.
pub fn save_options(options: &Options) -> io::Result<()> {
    ensure_dirs()?;
    fs::write(
        options_path(),
        serde_json::to_string_pretty(options).unwrap_or_else(|_| "{}".to_string()),
    )
}

/// Lista los documentos guardados para uso sin conexión.
pub fn list_documents() -> Vec<SavedDocument> {
    fs::read_to_string(index_path())
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

fn write_index(docs: &[SavedDocument]) -> io::Result<()> {
    ensure_dirs()?;
    fs::write(
        index_path(),
        serde_json::to_string_pretty(docs).unwrap_or_else(|_| "[]".to_string()),
    )
}

/// Normaliza un nombre de archivo para evitar caracteres inválidos.
fn safe_filename(name: &str) -> String {
    let base = Path::new(name)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("documento.pdf");
    base.chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            other => other,
        })
        .collect()
}

/// Guarda un documento en el directorio de datos de la aplicación.
pub fn save_document(doc: &SavedDocument, bytes: &[u8]) -> io::Result<PathBuf> {
    ensure_dirs()?;
    let nombre = safe_filename(&doc.filename);
    let destino = documents_dir().join(&nombre);
    fs::write(&destino, bytes)?;

    let mut docs = list_documents();
    docs.retain(|d| d.id != doc.id);
    docs.push(doc.clone());
    write_index(&docs)?;
    Ok(destino)
}

/// Devuelve un documento guardado y sus bytes.
pub fn read_document(id: &str) -> Option<(SavedDocument, Vec<u8>)> {
    let doc = list_documents().into_iter().find(|d| d.id == id)?;
    let ruta = documents_dir().join(safe_filename(&doc.filename));
    let bytes = fs::read(ruta).ok()?;
    Some((doc, bytes))
}

/// Elimina un documento guardado del índice y del disco.
pub fn delete_document(id: &str) -> io::Result<()> {
    let docs = list_documents();
    if let Some(doc) = docs.iter().find(|d| d.id == id) {
        let ruta = documents_dir().join(safe_filename(&doc.filename));
        let _ = fs::remove_file(ruta);
    }
    let restantes: Vec<SavedDocument> = docs.into_iter().filter(|d| d.id != id).collect();
    write_index(&restantes)
}

/// Directorio base de descargas según las opciones (sin la carpeta propia).
pub fn downloads_base(options: &Options) -> PathBuf {
    options
        .downloads_dir
        .clone()
        .or_else(dirs::download_dir)
        .unwrap_or_else(|| PathBuf::from("."))
}

/// Directorio de exportación: `<Descargas>/Sustancias Peligrosas`.
pub fn downloads_target(options: &Options) -> PathBuf {
    downloads_base(options).join(EXPORT_DIR)
}

/// Copia un documento guardado a la carpeta propia dentro de Descargas.
pub fn copy_to_downloads(id: &str, options: &Options) -> io::Result<PathBuf> {
    let (doc, bytes) = read_document(id).ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, "el documento no está en la caché local")
    })?;
    let directorio = downloads_target(options);
    fs::create_dir_all(&directorio)?;
    let destino = directorio.join(safe_filename(&doc.filename));
    fs::write(&destino, bytes)?;
    Ok(destino)
}

/// Flujo completo en nativo: guardar en la app y copiar a Descargas si está activado.
pub fn store_and_export(
    doc: &SavedDocument,
    bytes: &[u8],
    options: &Options,
) -> io::Result<PathBuf> {
    ensure_dirs()?;
    if options.offline_cache {
        save_document(doc, bytes)?;
    }
    if options.copy_to_downloads {
        copy_to_downloads(&doc.id, options)?;
    }
    Ok(documents_dir().join(safe_filename(&doc.filename)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opciones_por_defecto_habilitan_cache_y_copia() {
        let o = Options::default();
        assert!(o.offline_cache);
        assert!(o.copy_to_downloads);
        assert!(o.downloads_dir.is_none());
    }

    #[test]
    fn nombre_de_archivo_se_sanitiza() {
        assert_eq!(safe_filename("a/b:c.pdf"), "b_c.pdf");
    }

    #[test]
    fn descargas_usa_carpeta_propia() {
        let opciones = Options {
            downloads_dir: Some(PathBuf::from("/tmp/descargas")),
            ..Options::default()
        };
        assert_eq!(
            downloads_target(&opciones),
            PathBuf::from("/tmp/descargas").join(EXPORT_DIR)
        );
    }
}
