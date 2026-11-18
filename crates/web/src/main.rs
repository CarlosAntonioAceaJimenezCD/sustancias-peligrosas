use dioxus::prelude::*;
use sustancias_core as core;

const CSS: &str = r#"
:root{--bg:#0d1117;--card:#161b22;--card-alt:#1c2128;--ink:#c9d1d9;--muted:#8b949e;
--line:#30363d;--accent:#58a6ff;--accent-soft:rgba(56,139,253,.15);--purple:#a371f7;
--ok:#3fb950;--warn:#d29922;}
*{box-sizing:border-box}
body{margin:0;font-family:-apple-system,BlinkMacSystemFont,"Segoe UI","Noto Sans",Helvetica,Arial,sans-serif;
background:var(--bg);background-image:radial-gradient(1000px 500px at 50% -10%,rgba(56,139,253,.10),transparent 70%);
color:var(--ink);font-size:14px;line-height:1.5}
.ico{width:16px;height:16px;flex:none;fill:none;stroke:currentColor;stroke-width:2;
stroke-linecap:round;stroke-linejoin:round;vertical-align:middle}
.ico-lg{width:22px;height:22px}.ico-xl{width:44px;height:44px;stroke-width:1.6}
.cabecera{display:flex;justify-content:space-between;align-items:center;padding:14px 24px;
background:var(--card);border-bottom:1px solid var(--line)}
.marca{display:flex;gap:12px;align-items:center}
.marca-icono{display:grid;place-items:center;width:36px;height:36px;border:1px solid var(--line);
border-radius:8px;color:var(--accent);background:var(--card-alt)}
.marca strong{display:block;font-size:15px;color:#f0f6fc}.marca small{display:block;color:var(--muted);font-size:11px}
.badge{display:inline-flex;align-items:center;gap:4px;padding:2px 8px;border-radius:2em;font-size:11px;
font-weight:600;line-height:18px;background:var(--accent-soft);color:var(--accent);
border:1px solid rgba(56,139,253,.3);white-space:nowrap}
.badge-suave{background:var(--card-alt);color:var(--muted);border-color:var(--line)}
.badge-ok{background:rgba(46,160,67,.15);color:var(--ok);border-color:rgba(63,185,80,.3)}
.badge-warn{background:rgba(187,128,9,.15);color:var(--warn);border-color:rgba(210,153,34,.3)}
.badge-dev{background:rgba(187,128,9,.15);color:var(--warn);border-color:rgba(210,153,34,.35)}
.badge-demo{background:rgba(163,113,247,.12);color:var(--purple);border-color:rgba(163,113,247,.3)}
.banner{margin:20px auto 0;max-width:1180px;padding:22px 26px;border-radius:8px;display:flex;
align-items:center;gap:20px;background:linear-gradient(110deg,#12213a,#1b2140 70%,#241d42);
border:1px solid #2d4373;box-shadow:0 8px 24px rgba(0,0,0,.25)}
.banner-pais-grande{display:grid;place-items:center;width:62px;height:62px;border-radius:8px;
background:rgba(88,166,255,.12);border:1px solid rgba(88,166,255,.35);color:#79c0ff;
font-size:21px;font-weight:700;letter-spacing:1px;flex:none}
.banner-texto{flex:1}.banner-texto h2{margin:0 0 4px;font-size:22px;color:#f0f6fc;font-weight:600}
.banner-texto p{margin:0;color:#a9b6c6;font-size:13px;max-width:760px}
.banner-pais{font-size:11px;font-weight:600;letter-spacing:.8px;padding:5px 12px;border-radius:2em;
color:#a9b6c6;border:1px solid #3b4a63;white-space:nowrap}
.contenido{max-width:1180px;margin:20px auto 90px;padding:0 20px;display:grid;
grid-template-columns:1fr 1fr;gap:20px}
.panel{background:var(--card);border:1px solid var(--line);border-radius:8px;padding:18px;
box-shadow:0 3px 12px rgba(0,0,0,.18)}
.panel-cabecera{display:flex;justify-content:space-between;align-items:flex-start;gap:12px;
border-bottom:1px solid var(--line);padding-bottom:12px}
.panel-titulo{display:flex;gap:11px;align-items:center}
.panel-icono{display:grid;place-items:center;width:36px;height:36px;border-radius:8px;
background:var(--card-alt);border:1px solid var(--line);color:var(--accent)}
.panel h3{margin:0;font-size:15px;color:#f0f6fc;font-weight:600}
.panel-cabecera p{margin:2px 0 0;color:var(--muted);font-size:12px}
.filtros-wrap{display:flex;gap:8px;align-items:center;margin:14px 0 10px}
.filtro-icono{color:var(--muted)}.filtros{display:flex;flex-wrap:wrap;gap:6px}
.chip{border:1px solid var(--line);background:var(--card-alt);border-radius:2em;padding:4px 10px;
font-size:11px;cursor:pointer;color:var(--muted)}
.chip:hover{border-color:#4b5563;color:var(--ink)}
.chip.activo{background:var(--accent-soft);color:var(--accent);border-color:rgba(56,139,253,.5)}
.legal-lista{max-height:560px;overflow:auto;display:flex;flex-direction:column;gap:8px;padding-right:4px}
.norma{border:1px solid var(--line);border-radius:6px;padding:10px 12px;background:var(--card-alt)}
.norma:hover{border-color:#444c56}.norma h4{margin:0 0 6px;font-size:12px;line-height:1.45;color:#e6edf3;font-weight:600}
.norma-meta{display:flex;gap:6px;align-items:center;margin:8px 0 6px;flex-wrap:wrap}
.norma a{color:var(--accent);font-size:11px;text-decoration:none;word-break:break-all;
display:flex;align-items:flex-start;gap:5px;margin-top:3px}.norma a:hover{text-decoration:underline}
.vacio{color:var(--muted);font-size:13px;padding:16px 0}
.buscador{display:flex;align-items:center;gap:10px;border:1px solid var(--line);border-radius:6px;
padding:8px 12px;margin:12px 0 10px;background:#0d1117}
.buscador:focus-within{border-color:var(--accent);box-shadow:0 0 0 2px var(--accent-soft)}
.buscador-icono{color:var(--muted)}
.buscador input,.multi input{border:0;outline:0;width:100%;font-size:13px;background:transparent;color:var(--ink)}
.buscador input::placeholder,.multi input::placeholder{color:#6e7681}
.resultados{display:flex;flex-wrap:wrap;gap:6px;margin-bottom:12px}
.resultado{border:1px solid var(--line);background:var(--card-alt);border-radius:6px;padding:5px 10px;
cursor:pointer;font-size:12px;color:var(--ink)}
.resultado:hover,.resultado.activo{border-color:var(--accent);background:var(--accent-soft);color:var(--accent)}
.aviso{display:flex;gap:8px;align-items:flex-start;background:rgba(187,128,9,.08);
border:1px solid rgba(187,128,9,.35);border-radius:6px;padding:9px 11px;font-size:11px;
color:#e3b341;margin-bottom:12px}.aviso .ico{color:var(--warn);margin-top:1px}
.multi{border:1px dashed #484f58;border-radius:6px;padding:12px;margin:12px 0;background:rgba(22,27,34,.6)}
.multi-cabecera{display:flex;justify-content:space-between;align-items:center;font-size:12px;margin-bottom:10px}
.multi-titulo{display:flex;align-items:center;gap:7px;color:#e6edf3}.multi-titulo .ico{color:var(--purple)}
.multi-anadir{display:flex;gap:8px}.multi-anadir input{border:1px solid var(--line);border-radius:6px;padding:7px 10px;background:#0d1117}
.boton{display:inline-flex;align-items:center;gap:6px;background:#238636;color:#fff;border:1px solid #2ea043;
border-radius:6px;padding:7px 12px;font-size:12px;font-weight:600;cursor:pointer}.boton:hover{background:#2ea043}
.multi-chips{display:flex;flex-wrap:wrap;gap:6px;margin:10px 0 6px}
.multi-chip{background:rgba(163,113,247,.12);color:#d2a8ff;border:1px solid rgba(163,113,247,.3);
border-radius:2em;padding:3px 10px;font-size:11px;font-weight:600}
.multi-nota{margin:6px 0 0;font-size:11px;color:var(--muted)}
.ficha{margin-top:8px}.ficha-vacia{text-align:center;padding:30px 10px;color:var(--muted)}
.ficha-vacia .ico{color:#484f58}.ficha-vacia h4{margin:10px 0 4px;color:var(--ink);font-size:14px}
.ficha-vacia p{margin:0;font-size:12px}
.ficha-titulo{display:flex;justify-content:space-between;align-items:center;gap:10px}
.ficha-titulo h4{margin:0;font-size:19px;color:#f0f6fc}
.identificadores{display:flex;gap:6px;flex-wrap:wrap;margin:10px 0 14px}
.ident{background:var(--card-alt);border:1px solid var(--line);border-radius:6px;padding:3px 8px;
font-size:10px;font-weight:600;color:var(--muted);font-family:ui-monospace,SFMono-Regular,Menlo,monospace}
.fds-index{display:flex;flex-wrap:wrap;gap:5px;border:1px solid var(--line);border-radius:6px;padding:9px;margin-bottom:14px;background:var(--card-alt)}
.fds-index a{font-size:10px;color:var(--muted);text-decoration:none;border:1px solid var(--line);border-radius:4px;padding:2px 6px}
.fds-index a:hover{color:var(--accent);border-color:var(--accent)}
.seccion{margin:14px 0;border-left:2px solid #30475f;padding-left:12px;scroll-margin-top:16px}
.seccion h5{margin:0 0 6px;font-size:11px;text-transform:uppercase;letter-spacing:.7px;color:var(--accent);font-weight:700}
.seccion p{margin:0 0 7px;font-size:12px;line-height:1.6;color:#b9c3ce}
.prop-grid{display:grid;grid-template-columns:1fr 1fr;gap:6px}
.prop{background:var(--card-alt);border:1px solid var(--line);border-radius:6px;padding:7px 9px;font-size:11px}
.prop b{display:block;color:var(--accent);font-size:9px;text-transform:uppercase;letter-spacing:.4px;margin-bottom:2px}
.lista{margin:0 0 8px;padding-left:18px;font-size:12px;color:#b9c3ce;line-height:1.6}
.lista li{margin:2px 0}.lista-titulo{margin:8px 0 3px!important;font-size:11px!important;font-weight:700;color:#d0d7de!important}
.nota{display:flex;gap:6px;align-items:flex-start;font-size:11px!important;color:#d29922!important}
.fuentes a{display:flex;align-items:center;gap:5px;font-size:11px;color:var(--accent);text-decoration:none;margin-top:5px}
.pie{text-align:center;padding:20px;color:var(--muted);font-size:11px;border-top:1px solid var(--line)}
.pie p{margin:4px 0}
.flotante{position:fixed;right:20px;bottom:20px;z-index:40;display:flex;align-items:center;gap:9px;
background:#21262d;color:var(--ink);border:1px solid #444c56;border-radius:6px;padding:9px 13px;
font-size:12px;font-weight:600;cursor:pointer;box-shadow:0 8px 24px rgba(0,0,0,.4)}
.flotante:hover{background:#30363d;border-color:#6e7681}
.flotante-codigo{background:var(--accent-soft);color:var(--accent);border:1px solid rgba(56,139,253,.35);
border-radius:4px;padding:2px 6px;font-size:10px;letter-spacing:.5px}.flotante-cambiar{color:var(--muted)}
.overlay{position:fixed;inset:0;z-index:50;display:flex;align-items:center;justify-content:center;
background:rgba(1,4,9,.82);padding:20px}
.overlay-card{background:var(--card);border:1px solid var(--line);border-radius:8px;padding:30px;
max-width:860px;width:100%;text-align:center;box-shadow:0 24px 60px rgba(0,0,0,.6)}
.overlay-card.compacta{max-width:740px;position:relative}.overlay-logo{color:var(--accent)}
.overlay-card h1,.overlay-card h2{margin:10px 0 6px;font-size:20px;color:#f0f6fc;font-weight:600}
.overlay-card p{color:var(--muted);font-size:13px;margin:0 0 22px}
.pais-grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(160px,1fr));gap:9px}
.pais-boton{display:flex;align-items:center;gap:10px;text-align:left;border:1px solid var(--line);
background:var(--card-alt);border-radius:6px;padding:12px;cursor:pointer;color:var(--ink);transition:.12s}
.pais-boton:hover{border-color:var(--accent);background:var(--accent-soft)}
.pais-boton .bandera{display:grid;place-items:center;width:36px;height:36px;flex:none;border:1px solid var(--line);
border-radius:6px;background:#0d1117;color:var(--accent);font-size:12px;font-weight:700;letter-spacing:.5px}
.pais-boton .nombre{font-weight:600;font-size:12px;display:block}
.pais-boton .codigo{color:var(--muted);font-size:10px;display:block;letter-spacing:.5px}
.cerrar{position:absolute;right:12px;top:12px;display:grid;place-items:center;border:1px solid var(--line);
background:var(--card-alt);color:var(--muted);border-radius:6px;width:30px;height:30px;cursor:pointer}
.oculto{display:none!important}
.norma-modal{max-width:900px;text-align:left;max-height:88vh;display:flex;flex-direction:column}
.norma-modal h2{font-size:17px;padding-right:36px}.norma-modal #norma-meta{margin-bottom:12px}
.norma-texto{flex:1;overflow:auto;white-space:pre-wrap;word-wrap:break-word;background:#0d1117;
border:1px solid var(--line);border-radius:6px;padding:14px;font-family:ui-monospace,SFMono-Regular,Menlo,monospace;
font-size:11px;line-height:1.55;color:#b9c3ce;margin:0 0 14px}
.norma-modal .boton{align-self:flex-start;text-decoration:none}
.leer-norma{margin-top:8px;background:var(--card-alt);color:var(--accent);border:1px solid var(--line)}
.leer-norma:hover{border-color:var(--accent)}
.boe-buscar{border:1px solid var(--line);border-radius:6px;padding:4px 10px 10px;margin:10px 0;background:var(--card-alt)}
.boe-input-wrap{margin:8px 0}
.boe-atribucion{font-size:10px!important;color:var(--muted)!important;margin:8px 0 0!important;line-height:1.5!important}
.boe-atribucion a{color:var(--accent)}
.portales{border:1px solid var(--line);border-radius:6px;padding:10px 12px;background:var(--card-alt);margin-bottom:4px}
.portales a{display:flex;align-items:center;gap:6px;color:var(--accent);font-size:11px;text-decoration:none;margin-top:5px}
.portales a:hover{text-decoration:underline}
.resultados-online{margin:10px 0 14px}
.online-titulo{display:flex;align-items:center;gap:6px;font-size:11px;font-weight:700;color:var(--accent);margin-bottom:8px}
.online-resultado{display:flex;justify-content:space-between;align-items:center;gap:10px;border:1px solid var(--line);border-radius:6px;background:var(--card-alt);padding:9px 11px;margin-bottom:6px}
.online-resultado strong{display:block;font-size:12px;color:#e6edf3}
.online-meta{display:block;font-size:10px;color:var(--muted);margin-top:2px}
.online-resultado .boton{white-space:nowrap;font-size:11px}
.badge-online{background:rgba(56,139,253,.15);color:#79c0ff;border-color:rgba(56,139,253,.45)}
.badge-danger{background:rgba(248,81,73,.12);color:#ff7b72;border-color:rgba(248,81,73,.35)}
.interacciones{display:flex;flex-direction:column;gap:9px;margin-top:12px}
.interaccion{border:1px solid var(--line);border-left:3px solid #484f58;border-radius:6px;padding:10px 12px;background:var(--card)}
.interaccion-cabecera{display:flex;justify-content:space-between;align-items:center;gap:8px;flex-wrap:wrap}
.interaccion-cabecera strong{font-size:12px;color:#e6edf3}
.interaccion-tipo{font-size:11px;color:var(--accent);margin:5px 0 7px;font-weight:600}
.interaccion p{margin:0 0 7px;font-size:11px;line-height:1.55;color:#b9c3ce}
.interaccion .lista{margin-bottom:6px}.interaccion-productos b{color:var(--purple)}
.norma-acciones{display:flex;gap:8px;flex-wrap:wrap}
.norma-modal .boton{text-decoration:none}
.boton-descargar{background:#1f6feb;border-color:#388bfd}.boton-descargar:hover{background:#388bfd}
.boton-secundario{background:var(--card-alt);color:var(--ink);border:1px solid var(--line)}
.boton-secundario:hover{border-color:#6e7681}
.documentos-acciones{display:flex;gap:8px;flex-wrap:wrap;margin-top:10px}
.documentos-acciones .boton{text-decoration:none}
@media print{body{background:#fff!important;color:#000!important}
.cabecera,.banner,#panel-legal,.multi,.movil-tabs,.pie,.flotante,.fds-index,.documentos,.overlay{display:none!important}
.contenido{display:block;margin:0;padding:0}#panel-sustancia{border:0;box-shadow:none;padding:0}
.seccion{break-inside:avoid}.seccion h5{color:#0550ae}.prop,.interaccion{background:#fff;border-color:#ccc}}
.movil-tabs{display:none}
@media(max-width:900px){
.cabecera{padding:12px 16px}.marca-icono{width:32px;height:32px}.marca strong{font-size:14px}.marca small{font-size:10px}
.banner{margin:12px 12px 0;padding:15px 16px;gap:12px;flex-wrap:wrap}
.banner-pais-grande{width:46px;height:46px;font-size:16px}.banner-texto h2{font-size:18px}
.banner-texto p{font-size:12px}.banner-pais{width:100%;text-align:center}
.contenido{grid-template-columns:1fr;margin:14px 12px 140px;padding:0;gap:14px}
.panel{display:none;padding:14px}.panel.movil-activo{display:block}
.panel h3{font-size:14px}.prop-grid{grid-template-columns:1fr}.legal-lista{max-height:none}
.buscador{padding:12px}.buscador input,.multi input{font-size:16px}
.resultado,.chip,.boton{padding:10px 14px;font-size:13px}
.flotante{bottom:74px;right:12px;padding:10px 12px}
.movil-tabs{display:flex;position:fixed;left:0;right:0;bottom:0;z-index:45;background:var(--card);
border-top:1px solid var(--line);padding:6px 8px calc(6px + env(safe-area-inset-bottom))}
.movil-tabs .tab{flex:1;display:flex;flex-direction:column;align-items:center;gap:3px;background:transparent;
border:0;border-radius:6px;color:var(--muted);font-size:11px;font-weight:600;padding:8px 4px;cursor:pointer}
.movil-tabs .tab.activo{color:var(--accent);background:var(--accent-soft)}
.movil-tabs .tab .ico{width:20px;height:20px}
.pie{padding-bottom:100px}
}
@media(min-width:901px){.panel{display:block!important}}
@media(max-width:480px){.overlay-card{padding:20px 14px}.pais-grid{grid-template-columns:1fr 1fr}
.pais-boton{flex-direction:column;align-items:flex-start;gap:6px}.fds-index a{font-size:9px}}
"#;

fn icon(name: &str, class: &str) -> String {
    let (view, body) = match name {
        "flask" => ("0 0 24 24", r#"<path d="M9 3h6M10 3v6l-5 9a2 2 0 0 0 1.8 3h10.4a2 2 0 0 0 1.8-3l-5-9V3M7 15h10"/>"#),
        "globe" => ("0 0 24 24", r#"<circle cx="12" cy="12" r="10"/><path d="M2 12h20M12 2a15 15 0 0 1 0 20M12 2a15 15 0 0 0 0 20"/>"#),
        "scale" => ("0 0 24 24", r#"<path d="M12 3v18M5 21h14M4 7h16M6 7l-3 6h6L6 7zM18 7l-3 6h6l-3-6z"/>"#),
        "search" => ("0 0 24 24", r#"<circle cx="11" cy="11" r="8"/><path d="M21 21l-4.35-4.35"/>"#),
        "layers" => ("0 0 24 24", r#"<polygon points="12 2 2 7 12 12 22 7 12 2"/><polyline points="2 17 12 22 22 17"/><polyline points="2 12 12 17 22 12"/>"#),
        "x" => ("0 0 24 24", r#"<path d="M18 6L6 18M6 6l12 12"/>"#),
        "refresh" => ("0 0 24 24", r#"<polyline points="23 4 23 10 17 10"/><path d="M20.49 15a9 9 0 1 1-2.12-9.36L23 10"/>"#),
        "alert" => ("0 0 24 24", r#"<path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/><path d="M12 9v5M12 17h.01"/>"#),
        "file" => ("0 0 24 24", r#"<path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/><path d="M16 13H8M16 17H8M10 9H8"/>"#),
        "link" => ("0 0 24 24", r#"<path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/><polyline points="15 3 21 3 21 9"/><path d="M10 14L21 3"/>"#),
        "download" => ("0 0 24 24", r#"<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><path d="M12 15V3"/>"#),
        "book" => ("0 0 24 24", r#"<path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"/><path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"/>"#),
        "filter" => ("0 0 24 24", r#"<polygon points="22 3 2 3 10 12.46 10 19 14 21 14 12.46 22 3"/>"#),
        "check" => ("0 0 24 24", r#"<path d="M20 6L9 17l-5-5"/>"#),
        "info" => ("0 0 24 24", r#"<circle cx="12" cy="12" r="10"/><path d="M12 16v-5M12 8h.01"/>"#),
        "plus" => ("0 0 24 24", r#"<path d="M12 5v14M5 12h14"/>"#),
        _ => ("0 0 24 24", ""),
    };
    format!("<svg class=\"{}\" viewBox=\"{}\">{}</svg>", class, view, body)
}

/// Codificación mínima para incluir un término en una URL.
fn url_encode(value: &str) -> String {
    value
        .replace(' ', "%20")
        .replace('á', "%C3%A1")
        .replace('é', "%C3%A9")
        .replace('í', "%C3%AD")
        .replace('ó', "%C3%B3")
        .replace('ú', "%C3%BA")
        .replace('ñ', "%C3%B1")
        .replace('&', "%26")
}

/// Genera JavaScript para descargar un PDF mediante fetch + blob.
fn js_descargar(url: &str, nombre: &str) -> String {
    format!(
        r#"(async()=>{{const r=await fetch({url:?});const b=await r.blob();const a=document.createElement('a');a.href=URL.createObjectURL(b);a.download={nombre:?};document.body.appendChild(a);a.click();a.remove();setTimeout(()=>URL.revokeObjectURL(a.href),2000);}})();"#
    )
}

fn txt(key: &str, lang: &str) -> &'static str {
    match (lang, key) {
        ("en", "select_country") => "Select your country",
        ("en", "select_sub") => "The legal framework and language will adapt to your choice.",
        ("en", "change_country") => "Change country",
        ("en", "change_sub") => "The banner and legal framework update instantly.",
        ("en", "legal_title") => "Legal framework verification",
        ("en", "legal_sub") => "Regulations applicable to the selected country",
        ("en", "sub_title") => "Substance technical sheet",
        ("en", "sub_sub") => "Search by name, CAS, EC or UN number",
        ("en", "pick_substance") => "Select a substance",
        ("en", "pick_substance_sub") => "Type in the search box and click a result to see the full 16-section sheet.",
        ("es", "select_country") => "Selecciona tu país",
        ("es", "select_sub") => "El marco legal y el idioma del sitio se adaptarán a tu elección.",
        ("es", "change_country") => "Cambiar de país",
        ("es", "change_sub") => "El banner y el marco legal se actualizarán al instante.",
        ("es", "legal_title") => "Verificación del marco legal",
        ("es", "legal_sub") => "Normativa aplicable al país seleccionado",
        ("es", "sub_title") => "Carta técnica de la sustancia",
        ("es", "sub_sub") => "Busca por nombre, CAS, CE o número ONU",
        ("es", "pick_substance") => "Selecciona una sustancia",
        ("es", "pick_substance_sub") => "Escribe en el buscador y haz clic en un resultado para ver la ficha de 16 secciones.",
        _ => "",
    }
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let countries = core::countries();
    let topics = core::topics();
    let all_sources = core::sources();

    let mut selected_id: Signal<Option<String>> = use_signal(|| None);
    let mut modal_open: Signal<bool> = use_signal(|| false);
    let mut query: Signal<String> = use_signal(String::new);
    let mut filter: Signal<String> = use_signal(|| "todos".to_string());
    let mut selected_substance: Signal<Option<String>> = use_signal(|| None);
    let mut multi_input: Signal<String> = use_signal(String::new);
    let mut multi_ids: Signal<Vec<(String, String)>> = use_signal(Vec::new);
    let mut movil_tab: Signal<&'static str> = use_signal(|| "legal");
    let mut norma: Signal<Option<(String, String, String, String, String)>> = use_signal(|| None);
    let mut online_resultados: Signal<Vec<core::PubChemCompound>> = use_signal(Vec::new);
    let mut online_ficha: Signal<Option<core::Substance>> = use_signal(|| None);
    let mut online_cargando: Signal<bool> = use_signal(|| false);
    let mut boe_resultados: Signal<Vec<core::BoeNorm>> = use_signal(Vec::new);
    let mut boe_cargando: Signal<bool> = use_signal(|| false);

    let country = selected_id()
        .and_then(|id| core::country_by_id(&id))
        .unwrap_or_else(|| countries[0].clone());
    let lang = if country.idioma == "en" { "en" } else { "es" };
    let chosen = selected_id().is_some();

    let results = if query().trim().is_empty() {
        Vec::new()
    } else {
        core::search_substances(&query())
    };
    let active_substance = selected_substance().and_then(|id| core::substance_by_id(&id));

    let instruments: Vec<core::Instrument> = if country.id == "internacional" {
        Vec::new()
    } else {
        core::instruments_for(&country.id)
    };
    let international_sources: Vec<core::Source> = all_sources
        .iter()
        .filter(|s| s.oficial && s.pais == "internacional")
        .cloned()
        .collect();
    let portals = core::portals_for(&country.id);
    let visible_instruments: Vec<&core::Instrument> = instruments
        .iter()
        .filter(|i| filter() == "todos" || i.tema == filter())
        .collect();
    let multi_items = multi_ids();
    let multi_names: std::collections::HashMap<String, String> = multi_items
        .iter()
        .cloned()
        .collect();
    let multi_id_list: Vec<String> = multi_items.iter().map(|(id, _)| id.clone()).collect();
    let interactions = core::analyze_interactions_named(&multi_id_list, &multi_names);

    let rsx_countries = |_modal: bool| {
        countries.iter().map(move |c| {
            let id = c.id.clone();
            let codigo = c.codigo.clone();
            let nombre = c.nombre.clone();
            rsx! {
                button {
                    class: "pais-boton",
                    onclick: move |_| {
                        selected_id.set(Some(id.clone()));
                        modal_open.set(false);
                        selected_substance.set(None);
                        query.set(String::new());
                        filter.set("todos".to_string());
                        boe_resultados.set(Vec::new());
                        online_resultados.set(Vec::new());
                        online_ficha.set(None);
                        multi_ids.set(Vec::new());
                    },
                    span { class: "bandera", "{codigo}" }
                    span {
                        span { class: "nombre", "{nombre}" }
                        span { class: "codigo", "{codigo}" }
                    }
                }
            }
        })
    };

    rsx! {
        style { {CSS} }

        if !chosen {
            div { class: "overlay",
                div { class: "overlay-card",
                    div { class: "overlay-logo", dangerous_inner_html: "{icon(\"flask\", \"ico ico-xl\")}" }
                    h1 { "{txt(\"select_country\", lang)}" }
                    p { "{txt(\"select_sub\", lang)}" }
                    div { class: "pais-grid", {rsx_countries(false)} }
                }
            }
        }

        if modal_open() {
            div { class: "overlay",
                div { class: "overlay-card compacta",
                    button { class: "cerrar", onclick: move |_| modal_open.set(false),
                        dangerous_inner_html: "{icon(\"x\", \"ico\")}" }
                    h2 { "{txt(\"change_country\", lang)}" }
                    p { "{txt(\"change_sub\", lang)}" }
                    div { class: "pais-grid", {rsx_countries(true)} }
                }
            }
        }

        header { class: "cabecera",
            div { class: "marca",
                span { class: "marca-icono", dangerous_inner_html: "{icon(\"flask\", \"ico ico-lg\")}" }
                span {
                    strong { "Sustancias Peligrosas" }
                    small { "Marco legal y fichas técnicas" }
                }
            }
            span { class: "badge badge-suave", "{country.codigo}" }
        }

        section { class: "banner",
            div { class: "banner-pais-grande", "{country.codigo}" }
            div { class: "banner-texto",
                h2 { "{country.saludo}" }
                p { "{country.descripcion}" }
            }
            div { class: "banner-pais", "{country.nombre} · {lang.to_uppercase()}" }
        }

        main { class: "contenido",

            section {
                class: if movil_tab() == "legal" { "panel movil-activo" } else { "panel" },
                id: "panel-legal",
                div { class: "panel-cabecera",
                    div { class: "panel-titulo",
                        span { class: "panel-icono", dangerous_inner_html: "{icon(\"scale\", \"ico ico-lg\")}" }
                        div {
                            h3 { "{txt(\"legal_title\", lang)}" }
                            p { "{txt(\"legal_sub\", lang)}" }
                        }
                    }
                    span { class: "badge", "{visible_instruments.len() + international_sources.len()} normas" }
                }
                div { class: "filtros-wrap",
                    span { class: "filtro-icono", dangerous_inner_html: "{icon(\"filter\", \"ico\")}" }
                    div { class: "filtros",
                        button {
                            class: if filter() == "todos" { "chip activo" } else { "chip" },
                            onclick: move |_| filter.set("todos".to_string()),
                            "Todos"
                        }
                        for t in topics.iter() {
                            button {
                                class: if filter() == t.id { "chip activo" } else { "chip" },
                                onclick: { let id = t.id.clone(); move |_| filter.set(id.clone()) },
                                "{t.nombre}"
                            }
                        }
                    }
                }
                if country.id == "espana" {
                    div { class: "boe-buscar",
                        div { class: "buscador boe-input-wrap",
                            span { class: "buscador-icono", dangerous_inner_html: "{icon(\"search\", \"ico\")}" }
                            input {
                                placeholder: "Buscar normas en el BOE (España)…",
                                oninput: move |e| {
                                    let value = e.value();
                                    if value.trim().len() < 3 {
                                        boe_resultados.set(Vec::new());
                                        return;
                                    }
                                    boe_cargando.set(true);
                                    let js = format!(
                                        r#"(async()=>{{try{{const q={{query:{{query_string:{{query:{q:?}}}}},sort:[]}};const r=await fetch('https://www.boe.es/datosabiertos/api/legislacion-consolidada?query='+encodeURIComponent(JSON.stringify(q))+'&limit=8',{{headers:{{Accept:'application/json'}}}});const j=await r.json();return JSON.stringify(j.data||[]);}}catch(e){{return '[]';}}}})()"#,
                                        q = format!("titulo:{}", value.trim())
                                    );
                                    let mut ev = dioxus::document::eval(&js);
                                    spawn(async move {
                                        if let Ok(texto) = ev.recv::<String>().await {
                                            let parsed: Vec<core::BoeNorm> =
                                                serde_json::from_str(&texto).unwrap_or_default();
                                            boe_resultados.set(parsed);
                                            boe_cargando.set(false);
                                        }
                                    });
                                },
                            }
                        }
                        div { class: "boe-resultados",
                            if boe_cargando() {
                                p { class: "vacio", "Consultando la API de datos abiertos del BOE…" }
                            } else if !boe_resultados().is_empty() {
                                for n in boe_resultados().iter() {
                                    div { class: "norma",
                                        h4 { "{n.titulo}" }
                                        div { class: "norma-meta",
                                            span { class: "badge", "{n.identificador}" }
                                            if let Some(dep) = &n.departamento {
                                                span { class: "badge badge-suave", "{dep.texto}" }
                                            }
                                            span { class: "badge badge-warn", "Texto consolidado informativo" }
                                        }
                                        a { href: "{n.url()}", target: "_blank",
                                            dangerous_inner_html: "{icon(\"link\", \"ico\")}" }
                                        a { href: "{n.url()}", target: "_blank", "Ver en el BOE" }
                                    }
                                }
                                p { class: "boe-atribucion",
                                    "Fuente de los datos: "
                                    a { href: "https://www.boe.es", target: "_blank", "Agencia Estatal Boletín Oficial del Estado" }
                                    ". Texto consolidado de carácter meramente informativo."
                                    if !boe_resultados()[0].fecha_formateada().is_empty() {
                                        " Última actualización: {boe_resultados()[0].fecha_formateada()}."
                                    }
                                }
                            }
                        }
                    }
                }
                div { class: "legal-lista",
                    if !portals.is_empty() {
                        div { class: "portales",
                            p { class: "lista-titulo", "Portales oficiales de {country.nombre}" }
                            for p in portals.iter() {
                                a { href: "{p.url}", target: "_blank",
                                    dangerous_inner_html: "{icon(\"link\", \"ico\")}" }
                                a { href: "{p.url}", target: "_blank", "{p.institucion}" }
                            }
                        }
                    }
                    for s in international_sources.iter().filter(|s| filter() == "todos" || s.tema == filter()) {
                        div { class: "norma",
                            h4 { "{s.titulo}" }
                            div { class: "norma-meta",
                                span { class: "badge", "{core::topic_label(&s.tema)}" }
                                span { class: "badge badge-suave", "Marco internacional" }
                                span {
                                    class: if core::is_verified(&s.verificacion) { "badge badge-ok" } else { "badge badge-warn" },
                                    "{core::verification_label(&s.verificacion)}"
                                }
                            }
                            a { href: "{s.url}", target: "_blank",
                                dangerous_inner_html: "{icon(\"link\", \"ico\")}" }
                            a { href: "{s.url}", target: "_blank", "{s.url}" }
                        }
                    }
                    for i in visible_instruments.iter() {
                        div { class: "norma",
                            h4 { "{i.titulo}" }
                            div { class: "norma-meta",
                                span { class: "badge", "{core::topic_label(&i.tema)}" }
                                span { class: "badge badge-suave", "{country.nombre}" }
                                span {
                                    class: if core::is_verified(&i.verificacion) { "badge badge-ok" } else { "badge badge-warn" },
                                    "{core::verification_label(&i.verificacion)}"
                                }
                            }
                            if !i.url.is_empty() {
                                a { href: "{i.url}", target: "_blank",
                                    dangerous_inner_html: "{icon(\"link\", \"ico\")}" }
                                a { href: "{i.url}", target: "_blank", "{i.url}" }
                            }
                            for alt in i.alternativas.iter() {
                                a { href: "{alt}", target: "_blank", "Alternativa: {alt}" }
                            }
                            if !i.nota.is_empty() {
                                p { class: "multi-nota", "{i.nota}" }
                            }
                            if !i.texto.is_empty() {
                                button {
                                    class: "boton leer-norma",
                                    onclick: {
                                        let titulo = i.titulo.clone();
                                        let meta = format!("Páginas {} de la Gaceta Oficial", i.paginas);
                                        let texto = i.texto.clone();
                                        let url = i.url.clone();
                                        let nombre = core::pdf_filename(&i.url, &i.pdf_nombre);
                                        move |_| norma.set(Some((titulo.clone(), meta.clone(), texto.clone(), url.clone(), nombre.clone())))
                                    },
                                    dangerous_inner_html: "{icon(\"book\", \"ico\")}",
                                    "Leer norma íntegra"
                                }
                            }
                        }
                    }
                }            }

            section {
                class: if movil_tab() == "ficha" { "panel movil-activo" } else { "panel" },
                id: "panel-sustancia",
                div { class: "panel-cabecera",
                    div { class: "panel-titulo",
                        span { class: "panel-icono", dangerous_inner_html: "{icon(\"flask\", \"ico ico-lg\")}" }
                        div {
                            h3 { "{txt(\"sub_title\", lang)}" }
                            p { "{txt(\"sub_sub\", lang)}" }
                        }
                    }
                }

                div { class: "aviso",
                    dangerous_inner_html: "{icon(\"alert\", \"ico\")}",
                    span { "{core::substances_warning()}" }
                }

                div { class: "buscador",
                    span { class: "buscador-icono", dangerous_inner_html: "{icon(\"search\", \"ico\")}" }
                    input {
                        placeholder: "Ej. Acetona, 67-64-1, ácido clorhídrico…",
                        value: "{query()}",
                        oninput: move |e| {
                            let value = e.value();
                            query.set(value.clone());
                            if value.trim().len() < 2 {
                                online_resultados.set(Vec::new());
                                return;
                            }
                            online_cargando.set(true);
                            let url = format!(
                                "https://pubchem.ncbi.nlm.nih.gov/rest/pug/compound/name/{}/property/Title,MolecularFormula,MolecularWeight,InChIKey,XLogP,ExactMass,HeavyAtomCount,Complexity/JSON",
                                url_encode(value.trim())
                            );
                            let js = format!(r#"(async()=>{{try{{const r=await fetch({url:?});if(!r.ok)return '[]';const j=await r.json();return JSON.stringify(j.PropertyTable?.Properties||[]);}}catch(e){{return '[]';}}}})()"#);
                            let mut ev = dioxus::document::eval(&js);
                            spawn(async move {
                                if let Ok(texto) = ev.recv::<String>().await {
                                    let parsed: Vec<core::PubChemCompound> =
                                        serde_json::from_str(&texto).unwrap_or_default();
                                    online_resultados.set(parsed);
                                    online_cargando.set(false);
                                }
                            });
                        },
                    }
                }

                div { class: "resultados",
                    for s in results.iter() {
                        button {
                            class: if selected_substance() == Some(s.id.clone()) { "resultado activo" } else { "resultado" },
                            onclick: { let id = s.id.clone(); move |_| selected_substance.set(Some(id.clone())) },
                            "{s.nombre} · {s.cas}"
                        }
                    }
                }

                div { class: "resultados-online",
                    if online_cargando() {
                        p { class: "vacio", "Consultando bases de datos oficiales…" }
                    } else if !online_resultados().is_empty() {
                        div { class: "online-titulo",
                            dangerous_inner_html: "{icon(\"globe\", \"ico\")}",
                            "Resultados en bases de datos oficiales · PubChem (NIH)"
                        }
                        for p in online_resultados().iter() {
                            div { class: "online-resultado",
                                div {
                                    strong { "{p.title}" }
                                    span { class: "online-meta", "CID {p.cid} · {p.molecular_formula} · {p.molecular_weight} g/mol" }
                                }
                                div { class: "online-acciones",
                                    button {
                                        class: "boton",
                                        onclick: {
                                            let compuesto = p.clone();
                                            move |_| {
                                                online_ficha.set(Some(core::substance_from_pubchem(&compuesto)));
                                            }
                                        },
                                        "Ver ficha en línea"
                                    }
                                    button {
                                        class: "boton boton-secundario",
                                        onclick: {
                                            let cid = p.cid;
                                            let nombre = p.title.clone();
                                            move |_| {
                                                let mut items = multi_ids();
                                                let id = format!("online-{}", cid);
                                                if !items.iter().any(|(x, _)| x == &id) {
                                                    items.push((id, nombre.clone()));
                                                }
                                                multi_ids.set(items);
                                            }
                                        },
                                        "Añadir a comparación"
                                    }
                                }
                            }
                        }
                    }
                }

                div { class: "multi",
                    div { class: "multi-cabecera",
                        span { class: "multi-titulo",
                            dangerous_inner_html: "{icon(\"layers\", \"ico\")}",
                            strong { "Comparación de varias sustancias" }
                        }
                        if interactions.is_empty() {
                            span { class: "badge badge-ok", "ACTIVO" }
                        } else {
                            span { class: "badge badge-ok", "{interactions.len()} parejas" }
                        }
                    }
                    div { class: "multi-anadir",
                        input {
                            placeholder: "Añadir otra sustancia…",
                            value: "{multi_input()}",
                            oninput: move |e| multi_input.set(e.value()),
                        }
                        button {
                            class: "boton",
                            onclick: move |_| {
                                let texto = multi_input().trim().to_lowercase();
                                let encontrado = online_resultados()
                                    .into_iter()
                                    .find(|p| p.title.to_lowercase().contains(&texto));
                                if let Some(p) = encontrado {
                                    let mut items = multi_ids();
                                    let id = format!("online-{}", p.cid);
                                    if !items.iter().any(|(x, _)| x == &id) {
                                        items.push((id, p.title.clone()));
                                    }
                                    multi_ids.set(items);
                                    multi_input.set(String::new());
                                }
                            },
                            dangerous_inner_html: "{icon(\"plus\", \"ico\")}",
                            "Añadir"
                        }
                    }
                    div { class: "multi-chips",
                        for (_, nombre) in multi_items.iter() {
                            span { class: "multi-chip", "{nombre}" }
                        }
                    }
                    div { class: "interacciones",
                        for r in interactions.iter() {
                            div { class: "interaccion",
                                div { class: "interaccion-cabecera",
                                    strong { "{r.nombre_a} + {r.nombre_b}" }
                                    span {
                                        class: match r.nivel.as_str() {
                                            "alto" => "badge badge-danger",
                                            "moderado" => "badge badge-warn",
                                            "bajo" => "badge badge-ok",
                                            _ => "badge badge-suave",
                                        },
                                        "{r.etiqueta}"
                                    }
                                }
                                div { class: "interaccion-tipo", "{r.tipo}" }
                                p { "{r.descripcion}" }
                                if !r.productos.is_empty() {
                                    p { class: "interaccion-productos",
                                        b { "Productos: " }
                                        "{r.productos.join(\", \")}"
                                    }
                                }
                                if !r.consecuencias.is_empty() {
                                    p { class: "lista-titulo", "Consecuencias" }
                                    ul { class: "lista",
                                        for x in r.consecuencias.iter() { li { "{x}" } }
                                    }
                                }
                                if !r.acciones.is_empty() {
                                    p { class: "lista-titulo", "Acciones recomendadas" }
                                    ul { class: "lista",
                                        for x in r.acciones.iter() { li { "{x}" } }
                                    }
                                }
                                if r.es_regla_general {
                                    p { class: "multi-nota", "Regla general: verificar compatibilidad en fuentes oficiales." }
                                }
                            }
                        }
                    }
                    p { class: "multi-nota", "{core::interactions_warning()}" }
                }

                div { class: "ficha",
                    if let Some(s) = online_ficha().or(active_substance) {
                        div { class: "ficha-titulo",
                            h4 { "{s.nombre}" }
                            if s.estado == "online" {
                                span { class: "badge badge-online", "DATOS EN LÍNEA · PUBCHEM" }
                            } else if core::substance_is_verified(&s) {
                                span { class: "badge badge-ok", "FDS VERIFICADA" }
                            } else {
                                span { class: "badge badge-demo", "DEMO · 16 SECCIONES" }
                            }
                        }
                        div { class: "identificadores",
                            span { class: "ident", "CAS {s.cas}" }
                            span { class: "ident", "CE {s.ce}" }
                            span { class: "ident", "ONU {s.onu}" }
                            span { class: "ident", "{s.palabra_advertencia}" }
                            if !s.fecha_verificacion.is_empty() {
                                span { class: "ident", "Verificada: {s.fecha_verificacion}" }
                            }
                            if !s.verificado_por.is_empty() {
                                span { class: "ident", "Por: {s.verificado_por}" }
                            }
                        }
                        if s.estado == "online" {
                            div { class: "aviso",
                                dangerous_inner_html: "{icon(\"alert\", \"ico\")}",
                                span { "Ficha parcial generada con datos públicos de PubChem. No sustituye la ficha de datos de seguridad del fabricante." }
                            }
                        }
                        div { class: "fds-index",
                            for sec in s.secciones.iter() {
                                a { href: "#sec-{s.id}-{sec.id}", "{sec.numero}. {sec.titulo}" }
                            }
                        }
                        for sec in s.secciones.iter() {
                            div { class: "seccion", id: "sec-{s.id}-{sec.id}",
                                h5 { "{sec.numero}. {sec.titulo}" }
                                if !sec.texto.is_empty() {
                                    p { "{sec.texto}" }
                                }
                                if !sec.campos.is_empty() {
                                    div { class: "prop-grid",
                                        for campo in sec.campos.iter() {
                                            div { class: "prop",
                                                b { "{campo.k}" }
                                                "{campo.v}"
                                            }
                                        }
                                    }
                                }
                                for lista in sec.listas.iter() {
                                    p { class: "lista-titulo", "{lista.titulo}" }
                                    ul { class: "lista",
                                        for item in lista.items.iter() {
                                            li { "{item}" }
                                        }
                                    }
                                }
                                for nota in sec.notas.iter() {
                                    p { class: "nota", "{nota}" }
                                }
                            }
                        }
                        div { class: "seccion fuentes",
                            h5 { "Fuentes" }
                            for f in s.fuentes.iter() {
                                a { href: "{f.url}", target: "_blank", "{f.titulo}" }
                            }
                        }
                        div { class: "seccion documentos",
                            h5 { "Documentos y descarga" }
                            p { "Guarda esta ficha o descarga el documento PDF original cuando esté disponible." }
                            div { class: "documentos-acciones",
                                if !s.pdf_url.is_empty() {
                                    button {
                                        class: "boton boton-descargar",
                                        onclick: {
                                            let url = s.pdf_url.clone();
                                            let nombre = core::pdf_filename(&s.pdf_url, &s.pdf_nombre);
                                            move |_| {
                                                let js = js_descargar(&url, &nombre);
                                                dioxus::document::eval(&js);
                                            }
                                        },
                                        dangerous_inner_html: "{icon(\"download\", \"ico\")}",
                                        "Descargar FDS en PDF"
                                    }
                                }
                                button {
                                    class: "boton boton-secundario",
                                    onclick: move |_| {
                                        dioxus::document::eval("window.print()");
                                    },
                                    dangerous_inner_html: "{icon(\"file\", \"ico\")}",
                                    "Guardar ficha como PDF (imprimir)"
                                }
                            }
                        }
                    } else {
                        div { class: "ficha-vacia",
                            span { dangerous_inner_html: "{icon(\"file\", \"ico ico-xl\")}" }
                            h4 { "{txt(\"pick_substance\", lang)}" }
                            p { "{txt(\"pick_substance_sub\", lang)}" }
                        }
                    }
                }
            }
        }

        if let Some((titulo, meta, texto, url, nombre_pdf)) = norma() {
            div { class: "overlay",
                div { class: "overlay-card norma-modal",
                    button {
                        class: "cerrar",
                        onclick: move |_| norma.set(None),
                        dangerous_inner_html: "{icon(\"x\", \"ico\")}",
                    }
                    h2 { "{titulo}" }
                    p { "{meta}" }
                    pre { class: "norma-texto", "{texto}" }
                    div { class: "norma-acciones",
                        a {
                            class: "boton",
                            href: "{url}",
                            target: "_blank",
                            dangerous_inner_html: "{icon(\"link\", \"ico\")}",
                            "Abrir PDF original"
                        }
                        button {
                            class: "boton boton-descargar",
                            onclick: move |_| {
                                let js = js_descargar(&url, &nombre_pdf);
                                dioxus::document::eval(&js);
                            },
                            dangerous_inner_html: "{icon(\"download\", \"ico\")}",
                            "Descargar PDF"
                        }
                    }
                }
            }
        }

        nav { class: "movil-tabs",
            button {
                class: if movil_tab() == "legal" { "tab activo" } else { "tab" },
                onclick: move |_| movil_tab.set("legal"),
                dangerous_inner_html: "{icon(\"scale\", \"ico\")}",
                span { "Marco legal" }

            }
            button {
                class: if movil_tab() == "ficha" { "tab activo" } else { "tab" },
                onclick: move |_| movil_tab.set("ficha"),
                dangerous_inner_html: "{icon(\"flask\", \"ico\")}",
                span { "Ficha técnica" }
            }
        }

        footer { class: "pie",
            p { "{core::substances_warning()}" }
            p { "Fuentes normativas: UNECE · OMI · OIT · INSST · BOE · DOF · SINIA · Gaceta Oficial de Cuba" }
        }

        if chosen {
            button { class: "flotante", onclick: move |_| modal_open.set(true),
                span { class: "flotante-codigo", "{country.codigo}" }
                span { "{country.nombre}" }
                span { class: "flotante-cambiar", dangerous_inner_html: "{icon(\"refresh\", \"ico\")}" }
            }
        }
    }
}