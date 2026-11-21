//! Capa de traducción compartida por **todas** las plataformas.
//!
//! La web es solo la vista previa; el producto real es nativo (Windows, macOS,
//! Linux, Android e iOS). Por eso esta capa vive en su propio crate y no en
//! `crates/web`: el mismo código se compila para WASM y para nativo.
//!
//! Proveedores, en orden de preferencia:
//! 1. `DeeplTranslator`: opcional, solo si el usuario configura su propia key.
//! 2. `GoogleTranslator`: por defecto, sin key, detecta el idioma con `sl=auto`.
//! 3. `LocalDictionary`: respaldo offline del diccionario químico del core.

use async_trait::async_trait;
use sustancias_core as core;

/// Resultado de una traducción, con el idioma de origen cuando se detecta.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Traduccion {
    pub texto: String,
    pub idioma_detectado: Option<String>,
}

/// Contrato común a todos los proveedores de traducción.
///
/// En WASM los tipos de `gloo-net` no son `Send`, así que el futuro no puede
/// exigirlo; en nativo sí se exige para poder usar el traductor desde hilos.
#[cfg(target_arch = "wasm32")]
#[async_trait(?Send)]
pub trait Translator: Sync {
    async fn translate(&self, texto: &str, destino: &str) -> Option<Traduccion>;

    /// Nombre legible del proveedor, para la interfaz de opciones.
    fn nombre(&self) -> &'static str {
        "desconocido"
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[async_trait]
pub trait Translator: Send + Sync {
    async fn translate(&self, texto: &str, destino: &str) -> Option<Traduccion>;

    /// Nombre legible del proveedor, para la interfaz de opciones.
    fn nombre(&self) -> &'static str {
        "desconocido"
    }
}

/// Cliente HTTP único para ambas plataformas.
///
/// Centraliza la diferencia `gloo-net` / `reqwest` que antes estaba duplicada
/// en la aplicación. Los proveedores solo conocen este tipo.
#[derive(Clone, Default)]
pub struct HttpClient;

impl HttpClient {
    pub async fn get_json(&self, url: &str) -> Result<serde_json::Value, String> {
        #[cfg(target_arch = "wasm32")]
        {
            gloo_net::http::Request::get(url)
                .header("Accept", "application/json")
                .send()
                .await
                .map_err(|e| e.to_string())?
                .json::<serde_json::Value>()
                .await
                .map_err(|e| e.to_string())
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            reqwest::get(url)
                .await
                .map_err(|e| e.to_string())?
                .json::<serde_json::Value>()
                .await
                .map_err(|e| e.to_string())
        }
    }

    pub async fn post_form(
        &self,
        url: &str,
        headers: &[(&str, String)],
        body: &str,
    ) -> Result<serde_json::Value, String> {
        #[cfg(target_arch = "wasm32")]
        {
            let mut peticion = gloo_net::http::Request::post(url);
            for (clave, valor) in headers {
                peticion = peticion.header(clave, valor.as_str());
            }
            peticion
                .body(body)
                .map_err(|e| e.to_string())?
                .send()
                .await
                .map_err(|e| e.to_string())?
                .json::<serde_json::Value>()
                .await
                .map_err(|e| e.to_string())
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut peticion = reqwest::Client::new().post(url);
            for (clave, valor) in headers {
                peticion = peticion.header(*clave, valor.as_str());
            }
            peticion
                .body(body.to_string())
                .send()
                .await
                .map_err(|e| e.to_string())?
                .json::<serde_json::Value>()
                .await
                .map_err(|e| e.to_string())
        }
    }
}

/// Proveedor por defecto: Google Translate, sin clave y con detección automática.
pub struct GoogleTranslator {
    http: HttpClient,
}

impl Default for GoogleTranslator {
    fn default() -> Self {
        Self {
            http: HttpClient::default(),
        }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl Translator for GoogleTranslator {
    fn nombre(&self) -> &'static str {
        "Google Translate"
    }

    async fn translate(&self, texto: &str, destino: &str) -> Option<Traduccion> {
        let url = format!(
            "https://translate.googleapis.com/translate_a/single?client=gtx&sl=auto&tl={}&dt=t&q={}",
            urlencoding::encode(destino),
            urlencoding::encode(texto.trim())
        );
        let datos = self.http.get_json(&url).await.ok()?;
        let texto_traducido = datos.get(0)?.get(0)?.get(0)?.as_str()?.trim();
        if texto_traducido.is_empty() {
            return None;
        }
        let idioma = datos
            .get(2)
            .and_then(|v| v.as_str())
            .map(str::to_string);
        Some(Traduccion {
            texto: texto_traducido.to_string(),
            idioma_detectado: idioma,
        })
    }
}

/// Respaldo sin red: diccionario químico del core (209 términos) y alias.
pub struct LocalDictionary;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl Translator for LocalDictionary {
    fn nombre(&self) -> &'static str {
        "Diccionario local"
    }

    async fn translate(&self, texto: &str, destino: &str) -> Option<Traduccion> {
        // El diccionario del core solo traduce al inglés.
        if destino != "en" {
            return None;
        }
        let traducido = core::english(texto);
        if traducido.is_empty() || traducido.eq_ignore_ascii_case(texto.trim()) {
            None
        } else {
            Some(Traduccion {
                texto: traducido,
                idioma_detectado: None,
            })
        }
    }
}

/// Endpoint de la API de DeepL elegido por el usuario.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeeplApi {
    Free,
    Pro,
}

impl DeeplApi {
    pub fn endpoint(self) -> &'static str {
        match self {
            DeeplApi::Free => "https://api-free.deepl.com/v2/translate",
            DeeplApi::Pro => "https://api.deepl.com/v2/translate",
        }
    }
}

/// Proveedor opcional: solo se activa si el usuario aporta su propia key.
pub struct DeeplTranslator {
    key: String,
    api: DeeplApi,
    http: HttpClient,
}

impl DeeplTranslator {
    pub fn new(key: String, api: DeeplApi) -> Self {
        Self {
            key,
            api,
            http: HttpClient::default(),
        }
    }

    pub fn configurada(&self) -> bool {
        !self.key.trim().is_empty()
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl Translator for DeeplTranslator {
    fn nombre(&self) -> &'static str {
        "DeepL (key del usuario)"
    }

    async fn translate(&self, texto: &str, destino: &str) -> Option<Traduccion> {
        if !self.configurada() {
            return None;
        }
        // DeepL no acepta `auto` como idioma de destino.
        let objetivo = if destino == "auto" {
            "EN"
        } else {
            destino
        };
        let cuerpo = format!(
            "text={}&target_lang={}",
            urlencoding::encode(texto.trim()),
            urlencoding::encode(&objetivo.to_uppercase())
        );
        let cabeceras = vec![
            (
                "Authorization",
                format!("DeepL-Auth-Key {}", self.key.trim()),
            ),
            ("Content-Type", "application/x-www-form-urlencoded".to_string()),
        ];
        let datos = self
            .http
            .post_form(self.api.endpoint(), &cabeceras, &cuerpo)
            .await
            .ok()?;
        let traduccion = datos.get("translations")?.get(0)?;
        let texto_traducido = traduccion.get("text")?.as_str()?.trim();
        if texto_traducido.is_empty() {
            return None;
        }
        let idioma = traduccion
            .get("detected_source_language")
            .and_then(|v| v.as_str())
            .map(str::to_string);
        Some(Traduccion {
            texto: texto_traducido.to_string(),
            idioma_detectado: idioma,
        })
    }
}

/// Cadena de proveedores: devuelve el primero que responde.
pub struct Cadena {
    proveedores: Vec<Box<dyn Translator>>,
}

impl Cadena {
    /// Configuración por defecto: Google Translate + diccionario local.
    pub fn por_defecto() -> Self {
        Self {
            proveedores: vec![
                Box::new(GoogleTranslator::default()),
                Box::new(LocalDictionary),
            ],
        }
    }

    /// Si el usuario aporta una key de DeepL, se intenta primero.
    pub fn con_configuracion(key: Option<String>, api: DeeplApi) -> Self {
        let mut proveedores: Vec<Box<dyn Translator>> = Vec::new();
        if let Some(clave) = key.filter(|k| !k.trim().is_empty()) {
            proveedores.push(Box::new(DeeplTranslator::new(clave, api)));
        }
        proveedores.push(Box::new(GoogleTranslator::default()));
        proveedores.push(Box::new(LocalDictionary));
        Self { proveedores }
    }

    pub fn nombres(&self) -> Vec<&'static str> {
        self.proveedores.iter().map(|p| p.nombre()).collect()
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl Translator for Cadena {
    fn nombre(&self) -> &'static str {
        "Cadena de proveedores"
    }

    async fn translate(&self, texto: &str, destino: &str) -> Option<Traduccion> {
        for proveedor in &self.proveedores {
            if let Some(resultado) = proveedor.translate(texto, destino).await {
                return Some(resultado);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Falla;

    #[async_trait]
    impl Translator for Falla {
        async fn translate(&self, _texto: &str, _destino: &str) -> Option<Traduccion> {
            None
        }
    }

    #[tokio::test]
    async fn diccionario_local_traduce_al_ingles() {
        let t = LocalDictionary.translate("Ácido sulfúrico", "en").await.unwrap();
        assert_eq!(t.texto, "sulfuric acid");
        assert!(LocalDictionary.translate("acetone", "en").await.is_none());
        assert!(LocalDictionary.translate("ácido sulfúrico", "fr").await.is_none());
    }

    #[tokio::test]
    async fn cadena_cae_al_respaldo() {
        let cadena = Cadena {
            proveedores: vec![Box::new(Falla), Box::new(LocalDictionary)],
        };
        let t = cadena.translate("átomo de cloro", "en").await;
        assert!(t.is_some());
        assert_eq!(cadena.nombres(), vec!["desconocido", "Diccionario local"]);
    }

    #[tokio::test]
    async fn deepl_sin_key_no_se_activa() {
        let d = DeeplTranslator::new(String::new(), DeeplApi::Free);
        assert!(!d.configurada());
        assert!(d.translate("ácido sulfúrico", "en").await.is_none());
        assert_eq!(DeeplApi::Free.endpoint(), "https://api-free.deepl.com/v2/translate");
        assert_eq!(DeeplApi::Pro.endpoint(), "https://api.deepl.com/v2/translate");
    }
}
