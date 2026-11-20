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
.flag{width:30px;height:20px;display:block;border-radius:2px;box-shadow:0 0 0 1px rgba(255,255,255,.15)}
.flag-lg{width:54px;height:36px;border-radius:4px}
.banner-pais-grande{display:grid;place-items:center;width:62px;height:62px;border-radius:8px;
background:rgba(88,166,255,.12);border:1px solid rgba(88,166,255,.35);color:#79c0ff;
font-size:21px;font-weight:700;letter-spacing:1px;flex:none}
.banner-texto{flex:1}.banner-texto h2{margin:0 0 4px;font-size:22px;color:#f0f6fc;font-weight:600}
.banner-texto p{margin:0;color:#a9b6c6;font-size:13px;max-width:760px}
.banner-pais{font-size:11px;font-weight:600;letter-spacing:.8px;padding:5px 12px;border-radius:2em;
color:#a9b6c6;border:1px solid #3b4a63;white-space:nowrap}
.contenido{max-width:1180px;margin:20px auto 90px;padding:0 20px;display:flex;
flex-direction:column;gap:20px}
.panel-sustancia{order:1}
.panel-legal{order:2}
.panel-legal summary{display:flex;align-items:center;gap:10px;cursor:pointer;font-size:15px;
font-weight:600;color:#f0f6fc;list-style:none;padding:4px 0}
.panel-legal summary::-webkit-details-marker{display:none}
.panel-legal summary .badge{margin-left:auto}
.panel-legal[open] summary{margin-bottom:14px;border-bottom:1px solid var(--line);padding-bottom:12px}
.resultados-online{max-height:260px;overflow-y:auto;scrollbar-width:thin;scrollbar-color:#30363d transparent}
.resultados-online::-webkit-scrollbar{width:8px}
.resultados-online::-webkit-scrollbar-thumb{background:#30363d;border-radius:8px}
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
.link-feedback{color:var(--accent);text-decoration:none;border-bottom:1px dashed rgba(88,166,255,.5)}
.link-feedback:hover{text-decoration:underline}
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
.continente-titulo{grid-column:1/-1;margin:12px 0 0;font-size:11px;color:var(--accent);
text-transform:uppercase;letter-spacing:.9px;font-weight:700}
.pais-grid .continente-titulo:first-child{margin-top:0}
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
.idioma-busqueda{display:flex;align-items:center;gap:8px;flex-wrap:wrap;margin:10px 0;font-size:11px;color:var(--muted)}
.idioma-busqueda select{background:#0d1117;color:var(--ink);border:1px solid var(--line);border-radius:6px;padding:5px 8px;font-size:11px}
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
.offline-panel{max-width:1180px;margin:20px auto;width:calc(100% - 40px)}
.opciones{display:flex;flex-direction:column;gap:10px;margin:14px 0}
.opcion{display:flex;align-items:center;gap:9px;font-size:12px;color:#b9c3ce;flex-wrap:wrap}
.opcion input[type=checkbox]{width:16px;height:16px;accent-color:#238636}
.opcion input[type=text]{flex:1;min-width:220px;background:#0d1117;color:var(--ink);border:1px solid var(--line);border-radius:6px;padding:7px 10px;font-size:12px}
.estado-descarga{font-size:11px;color:var(--ok);margin:6px 0}
.doc-guardado{display:flex;justify-content:space-between;align-items:center;gap:10px;border:1px solid var(--line);border-radius:6px;background:var(--card-alt);padding:9px 11px;margin-bottom:6px}
.doc-guardado strong{font-size:12px;color:#e6edf3;display:block}
@media print{body{background:#fff!important;color:#000!important}
.cabecera,.banner,#panel-legal,.multi,.pie,.flotante,.fds-index,.documentos,.overlay{display:none!important}
.contenido{display:block;margin:0;padding:0}#panel-sustancia{border:0;box-shadow:none;padding:0}
.seccion{break-inside:avoid}.seccion h5{color:#0550ae}.prop,.interaccion{background:#fff;border-color:#ccc}}
@media(max-width:900px){
.cabecera{padding:12px 16px}.marca-icono{width:32px;height:32px}.marca strong{font-size:14px}.marca small{font-size:10px}
.banner{margin:12px 12px 0;padding:15px 16px;gap:12px;flex-wrap:wrap}
.banner-pais-grande{width:46px;height:46px;font-size:16px}.banner-texto h2{font-size:18px}
.banner-texto p{font-size:12px}.banner-pais{width:100%;text-align:center}
.contenido{margin:14px 12px 90px;padding:0;gap:14px}
.panel{padding:14px}
.panel h3{font-size:14px}.prop-grid{grid-template-columns:1fr}.legal-lista{max-height:none}
.buscador{padding:12px}.buscador input,.multi input{font-size:16px}
.resultado,.chip,.boton{padding:10px 14px;font-size:13px}
.flotante{bottom:74px;right:12px;padding:10px 12px}
.pie{padding-bottom:100px}
}
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

/// Genera JavaScript para descargar un PDF mediante fetch + blob (solo web).
#[cfg(target_arch = "wasm32")]
fn js_descargar(url: &str, nombre: &str) -> String {
    format!(
        r#"(async()=>{{const r=await fetch({url:?});const b=await r.blob();const a=document.createElement('a');a.href=URL.createObjectURL(b);a.download={nombre:?};document.body.appendChild(a);a.click();a.remove();setTimeout(()=>URL.revokeObjectURL(a.href),2000);}})();"#
    )
}

#[cfg(not(target_arch = "wasm32"))]
fn ahora() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs().to_string())
        .unwrap_or_default()
}

/// En nativo: descarga los bytes, los guarda en la app y los copia a Descargas.
#[cfg(not(target_arch = "wasm32"))]
async fn descargar_nativo(
    url: String,
    titulo: String,
    filename: String,
    kind: sustancias_storage::DocumentKind,
    opciones: AppOptions,
) -> Result<(), String> {
    let opciones = sustancias_storage::Options {
        offline_cache: opciones.offline_cache,
        copy_to_downloads: opciones.copy_to_downloads,
        downloads_dir: if opciones.downloads_dir.trim().is_empty() {
            None
        } else {
            Some(std::path::PathBuf::from(&opciones.downloads_dir))
        },
        auto_report: opciones.auto_report,
    };
    let respuesta = reqwest::get(&url)
        .await
        .map_err(|e| format!("Error de red: {e}"))?;
    let bytes = respuesta
        .bytes()
        .await
        .map_err(|e| format!("Error al leer el archivo: {e}"))?;
    let documento = sustancias_storage::SavedDocument {
        id: url,
        titulo,
        kind,
        filename,
        saved_at: ahora(),
        size: bytes.len() as u64,
    };
    sustancias_storage::store_and_export(&documento, &bytes, &opciones)
        .map(|_| ())
        .map_err(|e| format!("Error al guardar: {e}"))
}

/// Abre GitHub Issues con un reporte precargado (solo en nativo).
#[cfg(not(target_arch = "wasm32"))]
fn abrir_feedback(titulo: &str, cuerpo: &str) {
    let url = format!(
        "https://github.com/CarlosAntonioAceaJimenezCD/sustancias-peligrosas/issues/new?title={}&body={}",
        urlencoding::encode(titulo),
        urlencoding::encode(cuerpo)
    );
    let _ = webbrowser::open(&url);
}

/// Tipos neutrales (existen en web y nativo) para no condicionar el RSX.
#[derive(Clone, Debug)]
struct AppOptions {
    offline_cache: bool,
    copy_to_downloads: bool,
    downloads_dir: String,
    auto_report: bool,
}

impl Default for AppOptions {
    fn default() -> Self {
        Self {
            offline_cache: true,
            copy_to_downloads: true,
            downloads_dir: String::new(),
            auto_report: false,
        }
    }
}

#[derive(Clone, Debug)]
struct SavedDocView {
    id: String,
    titulo: String,
    kind: String,
    filename: String,
    size: u64,
    saved_at: String,
}

#[cfg(not(target_arch = "wasm32"))]
fn cargar_opciones() -> AppOptions {
    let o = sustancias_storage::load_options();
    AppOptions {
        offline_cache: o.offline_cache,
        copy_to_downloads: o.copy_to_downloads,
        downloads_dir: o
            .downloads_dir
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default(),
        auto_report: o.auto_report,
    }
}

#[cfg(target_arch = "wasm32")]
fn cargar_opciones() -> AppOptions {
    AppOptions::default()
}

#[cfg(not(target_arch = "wasm32"))]
fn cargar_guardados() -> Vec<SavedDocView> {
    sustancias_storage::list_documents()
        .into_iter()
        .map(|d| SavedDocView {
            id: d.id,
            titulo: d.titulo,
            kind: format!("{:?}", d.kind),
            filename: d.filename,
            size: d.size,
            saved_at: d.saved_at,
        })
        .collect()
}

#[cfg(target_arch = "wasm32")]
fn cargar_guardados() -> Vec<SavedDocView> {
    Vec::new()
}

#[cfg(not(target_arch = "wasm32"))]
fn guardar_opciones(o: &AppOptions) {
    let opciones = sustancias_storage::Options {
        offline_cache: o.offline_cache,
        copy_to_downloads: o.copy_to_downloads,
        downloads_dir: if o.downloads_dir.trim().is_empty() {
            None
        } else {
            Some(std::path::PathBuf::from(&o.downloads_dir))
        },
        auto_report: o.auto_report,
    };
    let _ = sustancias_storage::save_options(&opciones);
}

#[cfg(target_arch = "wasm32")]
fn guardar_opciones(_o: &AppOptions) {}

/// Cliente HTTP común: gloo-net en WASM, reqwest en nativo.
#[cfg(target_arch = "wasm32")]
async fn http_get_json(url: &str) -> Result<serde_json::Value, String> {
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
async fn http_get_json(url: &str) -> Result<serde_json::Value, String> {
    reqwest::get(url)
        .await
        .map_err(|e| e.to_string())?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| e.to_string())
}

/// Banderas SVG (sin emojis) para el selector de países y el banner.
fn flag_svg(id: &str, class: &str) -> String {
    let body = match id {
        "colombia" => r##"<rect width="30" height="10" fill="#fcd116"/><rect y="10" width="30" height="5" fill="#003893"/><rect y="15" width="30" height="5" fill="#ce1126"/>"##,
        "espana" => r##"<rect width="30" height="20" fill="#c60b1e"/><rect y="5" width="30" height="10" fill="#ffc400"/>"##,
        "mexico" => r##"<rect width="10" height="20" fill="#006847"/><rect x="10" width="10" height="20" fill="#fff"/><rect x="20" width="10" height="20" fill="#ce1126"/>"##,
        "peru" => r##"<rect width="10" height="20" fill="#d91023"/><rect x="10" width="10" height="20" fill="#fff"/><rect x="20" width="10" height="20" fill="#d91023"/>"##,
        "venezuela" => r##"<rect width="30" height="10" fill="#ffcc00"/><rect y="10" width="30" height="5" fill="#002b7f"/><rect y="15" width="30" height="5" fill="#c8102e"/><circle cx="6" cy="5" r="1" fill="#002b7f"/><circle cx="12" cy="5" r="1" fill="#002b7f"/><circle cx="18" cy="5" r="1" fill="#002b7f"/>"##,
        "cuba" => r##"<rect width="30" height="20" fill="#002590"/><rect y="5" width="30" height="3" fill="#fff"/><rect y="12" width="30" height="3" fill="#fff"/><polygon points="0,0 14,10 0,20" fill="#c8102e"/><circle cx="4" cy="10" r="1.6" fill="#fff"/>"##,
        "eeuu" => r##"<rect width="30" height="20" fill="#fff"/><rect y="3" width="30" height="3" fill="#b22234"/><rect y="9" width="30" height="3" fill="#b22234"/><rect y="15" width="30" height="3" fill="#b22234"/><rect width="14" height="11" fill="#3c3b6e"/><circle cx="3" cy="3" r="0.7" fill="#fff"/><circle cx="7" cy="3" r="0.7" fill="#fff"/><circle cx="11" cy="3" r="0.7" fill="#fff"/><circle cx="3" cy="8" r="0.7" fill="#fff"/><circle cx="7" cy="8" r="0.7" fill="#fff"/><circle cx="11" cy="8" r="0.7" fill="#fff"/>"##,
        _ => r##"<rect width="30" height="20" rx="3" fill="#0d419d"/><circle cx="15" cy="10" r="6" fill="none" stroke="#fff" stroke-width="1"/><path d="M9 10h12M15 4v12" stroke="#fff" stroke-width="1"/>"##,
    };
    format!("<svg class=\"{}\" viewBox=\"0 0 30 20\">{}</svg>", class, body)
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
    let mut norma: Signal<Option<(String, String, String, String, String)>> = use_signal(|| None);
    let mut online_resultados: Signal<Vec<core::PubChemCompound>> = use_signal(Vec::new);
    let mut online_ficha: Signal<Option<core::Substance>> = use_signal(|| None);
    let mut online_cargando: Signal<bool> = use_signal(|| false);
    let mut boe_resultados: Signal<Vec<core::BoeNorm>> = use_signal(Vec::new);
    let mut boe_cargando: Signal<bool> = use_signal(|| false);
    let mut output_lang: Signal<String> = use_signal(|| "es".to_string());
    let mut opciones: Signal<AppOptions> = use_signal(cargar_opciones);
    let mut guardados: Signal<Vec<SavedDocView>> = use_signal(cargar_guardados);
    #[allow(unused_mut)]
    let mut estado_descarga: Signal<String> = use_signal(String::new);

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

    let mut buscar_pubchem = move |texto: String, lang: String| {
        if texto.trim().len() < 2 {
            online_resultados.set(Vec::new());
            return;
        }
        online_cargando.set(true);
        spawn(async move {
            let base = "https://pubchem.ncbi.nlm.nih.gov/rest/pug/compound";
            let mut cids: Vec<u64> = Vec::new();
            for variante in core::search_variants(&texto) {
                for modo in ["?language=all", "?name_type=word"] {
                    let cid_url = format!(
                        "{}/name/{}/cids/JSON{}",
                        base,
                        urlencoding::encode(&variante),
                        modo
                    );
                    if let Ok(v) = http_get_json(&cid_url).await {
                        let encontrados: Vec<u64> = serde_json::from_value(
                            v.get("IdentifierList")
                                .and_then(|x| x.get("CID"))
                                .cloned()
                                .unwrap_or_default(),
                        )
                        .unwrap_or_default();
                        if !encontrados.is_empty() {
                            cids = encontrados;
                            break;
                        }
                    }
                }
                if !cids.is_empty() {
                    break;
                }
            }

            let mut resultados = Vec::new();
            for cid in cids.into_iter().take(3) {
                let prop_url = format!(
                    "{}/cid/{}/property/Title,MolecularFormula,MolecularWeight,InChIKey,XLogP,ExactMass,HeavyAtomCount,Complexity/JSON?language=all",
                    base, cid
                );
                let syn_url = format!("{}/cid/{}/synonyms/JSON?language=all", base, cid);

                let compuesto: Option<core::PubChemCompound> =
                    match http_get_json(&prop_url).await {
                        Ok(v) => serde_json::from_value(
                            v.get("PropertyTable")
                                .and_then(|t| t.get("Properties"))
                                .and_then(|p| p.get(0))
                                .cloned()
                                .unwrap_or_default(),
                        )
                        .ok(),
                        Err(_) => None,
                    };

                let sinonimos: Vec<String> = match http_get_json(&syn_url).await {
                    Ok(v) => serde_json::from_value(
                        v.get("InformationList")
                            .and_then(|x| x.get("Information"))
                            .and_then(|x| x.get(0))
                            .and_then(|x| x.get("Synonym"))
                            .cloned()
                            .unwrap_or_default(),
                    )
                    .unwrap_or_default(),
                    Err(_) => Vec::new(),
                };

                if let Some(mut c) = compuesto {
                    c.synonyms = sinonimos;
                    c.lang = lang.clone();
                    resultados.push(c);
                }
            }
            online_resultados.set(resultados);
            online_cargando.set(false);
        });
    };

    let continentes = vec![
        "Marco internacional", "America", "Europa", "Africa", "Asia", "Oceania",
    ];
    let rsx_countries = |_modal: bool| -> Element {
        rsx! {
            for cont in continentes.iter() {
                h4 { class: "continente-titulo", "{cont}" }
                for c in countries.iter().filter(|c| c.continente.as_str() == *cont) {
                    button {
                        class: "pais-boton",
                        onclick: {
                            let id = c.id.clone();
                            move |_| {
                                selected_id.set(Some(id.clone()));
                                modal_open.set(false);
                                selected_substance.set(None);
                                query.set(String::new());
                                filter.set("todos".to_string());
                                boe_resultados.set(Vec::new());
                                online_resultados.set(Vec::new());
                                online_ficha.set(None);
                                multi_ids.set(Vec::new());
                            }
                        },
                        span { class: "bandera", dangerous_inner_html: "{flag_svg(&c.id, \"flag\")}" }
                        span {
                            span { class: "nombre", "{c.nombre}" }
                            span { class: "codigo", "{c.codigo}" }
                        }
                    }
                }
            }
        }
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
            div { class: "banner-pais-grande", dangerous_inner_html: "{flag_svg(&country.id, \"flag flag-lg\")}" }
            div { class: "banner-texto",
                h2 { "{country.saludo}" }
                p { "{country.descripcion}" }
            }
            div { class: "banner-pais", "{country.nombre} · {lang.to_uppercase()}" }
        }

        main { class: "contenido",

            details {
                class: "panel panel-legal",
                id: "panel-legal",
                summary {
                    span { class: "panel-icono", dangerous_inner_html: "{icon(\"book\", \"ico ico-lg\")}" }
                    span { "Documentación legal y marco normativo" }
                    span { class: "badge", "{visible_instruments.len() + international_sources.len()} normas" }
                }
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
                                    let consulta = format!("titulo:{}", value.trim());
                                    spawn(async move {
                                        let query = serde_json::json!({
                                            "query": { "query_string": { "query": consulta } },
                                            "sort": []
                                        });
                                        let url = format!(
                                            "https://www.boe.es/datosabiertos/api/legislacion-consolidada?query={}&limit=8",
                                            urlencoding::encode(&query.to_string())
                                        );
                                        let parsed: Vec<core::BoeNorm> =
                                            match http_get_json(&url).await {
                                                Ok(v) => serde_json::from_value(
                                                    v.get("data").cloned().unwrap_or_default()
                                                ).unwrap_or_default(),
                                                Err(_) => Vec::new(),
                                            };
                                        boe_resultados.set(parsed);
                                        boe_cargando.set(false);
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
                                    span { dangerous_inner_html: "{icon(\"book\", \"ico\")}" }
                                    "Leer norma íntegra"
                                }
                            }
                        }
                    }
                }            }

            section {
                class: "panel panel-sustancia",
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
                    span { dangerous_inner_html: "{icon(\"alert\", \"ico\")}" }
                    span { "{core::substances_warning()}" }
                }

                div { class: "idioma-busqueda",
                    span { "Buscar en cualquier idioma · mostrar nombre en:" }
                    select {
                        value: "{output_lang()}",
                        onchange: move |e| {
                            let lang = e.value();
                            output_lang.set(lang.clone());
                            buscar_pubchem(query(), lang);
                        },
                        option { value: "auto", "Autor" }
                        option { value: "es", "Español" }
                        option { value: "en", "English" }
                        option { value: "fr", "Français" }
                        option { value: "de", "Deutsch" }
                        option { value: "it", "Italiano" }
                    }
                }
                div { class: "buscador",
                    span { class: "buscador-icono", dangerous_inner_html: "{icon(\"search\", \"ico\")}" }
                    input {
                        placeholder: "Ej. Acetona, ibuprofeno, 67-64-1…",
                        value: "{query()}",
                        oninput: move |e| {
                            let value = e.value();
                            query.set(value.clone());
                            buscar_pubchem(value, output_lang());
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
                            span { dangerous_inner_html: "{icon(\"globe\", \"ico\")}" }
                            "Resultados en bases de datos oficiales · PubChem (NIH)"
                        }
                        for p in online_resultados().iter() {
                            div { class: "online-resultado",
                                div {
                                    strong { "{core::localized_name(&p.synonyms, &output_lang()).unwrap_or_else(|| p.title.clone())}" }
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
                                            let nombre = core::localized_name(&p.synonyms, &output_lang())
                                                .unwrap_or_else(|| p.title.clone());
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
                            span { dangerous_inner_html: "{icon(\"layers\", \"ico\")}" }
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
                            span { dangerous_inner_html: "{icon(\"plus\", \"ico\")}" }
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
                                span { dangerous_inner_html: "{icon(\"alert\", \"ico\")}" }
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
                                            #[cfg(target_arch = "wasm32")]
                                            let handler = move |_| {
                                                let js = js_descargar(&url, &nombre);
                                                dioxus::document::eval(&js);
                                            };
                                            #[cfg(not(target_arch = "wasm32"))]
                                            let handler = move |_| {
                                                let u = url.clone();
                                                let n = nombre.clone();
                                                let t = s.nombre.clone();
                                                let o = opciones();
                                                let auto = o.auto_report;
                                                spawn(async move {
                                                    match descargar_nativo(
                                                        u, t, n,
                                                        sustancias_storage::DocumentKind::Substance,
                                                        o,
                                                    ).await {
                                                        Ok(_) => {
                                                            guardados.set(cargar_guardados());
                                                            estado_descarga.set("Guardado sin conexión y copiado a Descargas".to_string());
                                                        }
                                                        Err(e) => {
                                                            if auto {
                                                                abrir_feedback("Error al descargar documento", &e);
                                                            }
                                                            estado_descarga.set(e);
                                                        }
                                                    }
                                                });
                                            };
                                            handler
                                        },
                                        span { dangerous_inner_html: "{icon(\"download\", \"ico\")}" }
                                        "Descargar FDS en PDF"
                                    }
                                }
                                button {
                                    class: "boton boton-secundario",
                                    onclick: move |_| {
                                        dioxus::document::eval("window.print()");
                                    },
                                    span { dangerous_inner_html: "{icon(\"file\", \"ico\")}" }
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
                            span { dangerous_inner_html: "{icon(\"link\", \"ico\")}" }
                            "Abrir PDF original"
                        }
                        button {
                            class: "boton boton-descargar",
                            onclick: {
                                #[cfg(target_arch = "wasm32")]
                                let handler = move |_| {
                                    let js = js_descargar(&url, &nombre_pdf);
                                    dioxus::document::eval(&js);
                                };
                                #[cfg(not(target_arch = "wasm32"))]
                                let handler = move |_| {
                                    let u = url.clone();
                                    let n = nombre_pdf.clone();
                                    let t = titulo.clone();
                                    let o = opciones();
                                    let auto = o.auto_report;
                                    spawn(async move {
                                        match descargar_nativo(
                                            u, t, n,
                                            sustancias_storage::DocumentKind::Legislation,
                                            o,
                                        ).await {
                                            Ok(_) => {
                                                guardados.set(cargar_guardados());
                                                estado_descarga.set("Guardado sin conexión y copiado a Descargas".to_string());
                                            }
                                            Err(e) => {
                                                if auto {
                                                    abrir_feedback("Error al descargar documento", &e);
                                                }
                                                estado_descarga.set(e);
                                            }
                                        }
                                    });
                                };
                                handler
                            },
                            span { dangerous_inner_html: "{icon(\"download\", \"ico\")}" }
                            "Descargar PDF"
                        }
                    }
                }
            }
        }

        if !cfg!(target_arch = "wasm32") {
            section { class: "panel offline-panel",
                div { class: "panel-cabecera",
                    div { class: "panel-titulo",
                        span { class: "panel-icono", dangerous_inner_html: "{icon(\"download\", \"ico ico-lg\")}" }
                        div {
                            h3 { "Documentos sin conexión" }
                            p { "Solo en la aplicación nativa. La versión web no almacena archivos." }
                        }
                    }
                }
                div { class: "opciones",
                    label { class: "opcion",
                        input {
                            r#type: "checkbox",
                            checked: opciones().offline_cache,
                            onchange: move |e| {
                                let mut o = opciones();
                                o.offline_cache = e.checked();
                                guardar_opciones(&o);
                                opciones.set(o);
                            },
                        }
                        "Guardar documentos en la app para consultarlos sin internet"
                    }
                    label { class: "opcion",
                        input {
                            r#type: "checkbox",
                            checked: opciones().copy_to_downloads,
                            onchange: move |e| {
                                let mut o = opciones();
                                o.copy_to_downloads = e.checked();
                                guardar_opciones(&o);
                                opciones.set(o);
                            },
                        }
                        "Copiar automáticamente al directorio de descargas del sistema"
                    }
                    label { class: "opcion",
                        input {
                            r#type: "checkbox",
                            checked: opciones().auto_report,
                            onchange: move |e| {
                                let mut o = opciones();
                                o.auto_report = e.checked();
                                guardar_opciones(&o);
                                opciones.set(o);
                            },
                        }
                        "Enviar automáticamente feedback y reportes de errores"
                    }
                    if !cfg!(target_arch = "wasm32") {
                        button {
                            class: "boton boton-secundario",
                            onclick: move |_| {
                                #[cfg(not(target_arch = "wasm32"))]
                                abrir_feedback("Feedback sobre Sustancias Peligrosas", "Cuéntanos qué funciona mal o qué te gustaría mejorar:");
                            },
                            "Enviar feedback / reportar un bug"
                        }
                    }
                    label { class: "opcion",
                        "Directorio de descargas personalizado (vacío = el del sistema)"
                        input {
                            r#type: "text",
                            value: opciones().downloads_dir.clone(),
                            onchange: move |e| {
                                let mut o = opciones();
                                o.downloads_dir = e.value();
                                guardar_opciones(&o);
                                opciones.set(o);
                            },
                        }
                    }
                }
                if !estado_descarga().is_empty() {
                    p { class: "estado-descarga", "{estado_descarga()}" }
                }
                h4 { "Documentos guardados" }
                if guardados().is_empty() {
                    p { class: "vacio", "Todavía no has guardado ningún documento." }
                } else {
                    for d in guardados().iter() {
                        div { class: "doc-guardado",
                            div {
                                strong { "{d.titulo}" }
                                span { class: "online-meta", "{d.kind} · {d.filename} · {d.size} bytes · {d.saved_at}" }
                            }
                            button {
                                class: "boton boton-secundario",
                                onclick: {
                                    #[allow(unused_variables)]
                                    let id = d.id.clone();
                                    move |_| {
                                        #[cfg(not(target_arch = "wasm32"))]
                                        {
                                            let _ = sustancias_storage::delete_document(&id);
                                        }
                                        guardados.set(cargar_guardados());
                                    }
                                },
                                "Eliminar"
                            }
                        }
                    }
                }
            }
        }

        footer { class: "pie",
            p { "{core::substances_warning()}" }
            p { "Fuentes normativas: UNECE · OMI · OIT · INSST · BOE · DOF · SINIA · Gaceta Oficial de Cuba" }
            p {
                a {
                    href: "https://github.com/CarlosAntonioAceaJimenezCD/sustancias-peligrosas/issues/new",
                    target: "_blank",
                    class: "link-feedback",
                    "Enviar feedback / reportar un bug"
                }
            }
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
