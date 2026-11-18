use dioxus::prelude::*;
use sustancias_core as core;

const CSS: &str = r#"
:root { --bg:#0f172a; --card:#ffffff; --ink:#0f172a; --muted:#64748b;
        --accent:#0ea5e9; --accent2:#6366f1; --line:#e2e8f0; --ok:#16a34a;
        --warn:#f59e0b; }
* { box-sizing:border-box; }
body { margin:0; font-family:system-ui,-apple-system,"Segoe UI",Roboto,sans-serif;
       background:linear-gradient(160deg,#e0f2fe 0%,#eef2ff 45%,#f8fafc 100%);
       color:var(--ink); min-height:100vh; }
.cabecera { display:flex; justify-content:space-between; align-items:center;
            padding:16px 28px; background:#0f172a; color:#fff; }
.marca { display:flex; gap:12px; align-items:center; }
.marca-icono { font-size:30px; }
.marca strong { display:block; font-size:18px; letter-spacing:.3px; }
.marca small { display:block; color:#94a3b8; font-size:12px; }
.badge { display:inline-block; padding:4px 10px; border-radius:999px; font-size:11px;
         font-weight:700; background:#e0f2fe; color:#0369a1; letter-spacing:.4px; }
.badge-suave { background:#1e293b; color:#7dd3fc; }
.badge-ok { background:#dcfce7; color:#15803d; }
.badge-warn { background:#fef3c7; color:#b45309; }
.badge-dev { background:#fef3c7; color:#b45309; }
.banner { margin:22px auto 0; max-width:1180px; padding:26px 30px; border-radius:20px;
          display:flex; align-items:center; gap:22px; color:#fff;
          background:linear-gradient(120deg,#0369a1,#4f46e5);
          box-shadow:0 18px 40px rgba(2,132,199,.25); }
.banner-bandera { font-size:52px; }
.banner-texto { flex:1; }
.banner-texto h2 { margin:0 0 6px; font-size:27px; }
.banner-texto p { margin:0; opacity:.88; font-size:14px; max-width:760px; }
.banner-pais { font-size:12px; font-weight:800; letter-spacing:1.5px;
               border:1px solid rgba(255,255,255,.5); padding:8px 14px; border-radius:999px; }
.contenido { max-width:1180px; margin:24px auto 90px; padding:0 20px;
             display:grid; grid-template-columns:1fr 1fr; gap:22px; }
.panel { background:var(--card); border:1px solid var(--line); border-radius:18px;
         padding:22px; box-shadow:0 10px 30px rgba(15,23,42,.06); }
.panel-cabecera { display:flex; justify-content:space-between; align-items:flex-start;
                  gap:12px; border-bottom:1px solid var(--line); padding-bottom:14px; }
.panel-titulo { display:flex; gap:12px; align-items:center; }
.panel-icono { font-size:26px; }
.panel-titulo h3 { margin:0; font-size:18px; }
.panel-titulo p { margin:3px 0 0; color:var(--muted); font-size:12px; }
.filtros { display:flex; flex-wrap:wrap; gap:7px; margin:14px 0; }
.chip { border:1px solid var(--line); background:#f8fafc; border-radius:999px;
        padding:6px 11px; font-size:12px; cursor:pointer; color:var(--muted); }
.chip.activo { background:var(--accent); color:#fff; border-color:var(--accent); }
.legal-lista { max-height:520px; overflow:auto; display:flex; flex-direction:column; gap:10px; }
.norma { border:1px solid var(--line); border-radius:12px; padding:12px 14px; }
.norma h4 { margin:0 0 6px; font-size:13px; line-height:1.35; }
.norma a { color:#0369a1; font-size:12px; text-decoration:none; word-break:break-all; }
.norma a:hover { text-decoration:underline; }
.norma-meta { display:flex; gap:8px; align-items:center; margin-top:8px; flex-wrap:wrap; }
.buscador { display:flex; align-items:center; gap:10px; border:2px solid var(--line);
            border-radius:12px; padding:10px 14px; margin:16px 0 10px; }
.buscador:focus-within { border-color:var(--accent); }
.buscador-icono { font-size:18px; }
.buscador input, .multi input { border:0; outline:0; width:100%; font-size:14px; }
.resultados { display:flex; flex-wrap:wrap; gap:8px; margin-bottom:14px; }
.resultado { border:1px solid var(--line); background:#f8fafc; border-radius:10px;
             padding:8px 12px; cursor:pointer; font-size:13px; }
.resultado:hover { background:#e0f2fe; border-color:var(--accent); }
.vacio { color:var(--muted); font-size:13px; }
.multi { border:1px dashed #cbd5e1; border-radius:14px; padding:14px; margin:14px 0;
         background:#f8fafc; }
.multi-cabecera { display:flex; justify-content:space-between; align-items:center;
                  font-size:13px; margin-bottom:10px; }
.multi-anadir { display:flex; gap:8px; }
.multi-anadir input { border:1px solid var(--line); border-radius:9px; padding:8px 10px; }
.boton { background:var(--accent2); color:#fff; border:0; border-radius:9px;
         padding:8px 14px; font-weight:700; cursor:pointer; }
.multi-chips { display:flex; flex-wrap:wrap; gap:7px; margin:11px 0 6px; }
.multi-chip { background:#e0e7ff; color:#3730a3; border-radius:999px;
              padding:5px 11px; font-size:12px; font-weight:600; }
.multi-nota { margin:6px 0 0; font-size:11px; color:var(--muted); }
.ficha { margin-top:8px; }
.ficha-vacia { text-align:center; padding:36px 10px; color:var(--muted); }
.ficha-vacia-icono { font-size:34px; }
.ficha-vacia h4 { margin:10px 0 6px; color:var(--ink); }
.ficha-titulo { display:flex; justify-content:space-between; align-items:center; gap:10px; }
.ficha-titulo h4 { margin:0; font-size:20px; }
.identificadores { display:flex; gap:8px; flex-wrap:wrap; margin:10px 0 16px; }
.ident { background:#f1f5f9; border-radius:8px; padding:5px 9px; font-size:11px;
         font-weight:700; color:#475569; }
.seccion { margin:14px 0; border-left:3px solid var(--accent); padding-left:12px; }
.seccion h5 { margin:0 0 6px; font-size:13px; text-transform:uppercase;
              letter-spacing:.6px; color:#0369a1; }
.seccion p { margin:0; font-size:13px; line-height:1.55; color:#334155; }
.picto { display:inline-flex; align-items:center; justify-content:center; width:32px;
         height:32px; margin-right:5px; border-radius:6px; font-size:10px; font-weight:800;
         color:#fff; background:#dc2626; }
.frase { font-size:12px; margin:4px 0; color:#334155; }
.frase b { color:#b91c1c; }
.prop-grid { display:grid; grid-template-columns:1fr 1fr; gap:6px; }
.prop { background:#f8fafc; border-radius:8px; padding:7px 9px; font-size:12px; }
.prop b { display:block; color:#0369a1; font-size:10px; text-transform:uppercase; }
.lista { margin:0; padding-left:18px; font-size:13px; color:#334155; line-height:1.5; }
.aviso { background:#fffbeb; border:1px solid #fde68a; border-radius:10px;
         padding:10px 12px; font-size:11px; color:#92400e; margin-bottom:12px; }
.fuentes a { display:block; font-size:11px; color:#0369a1; margin-top:4px; }
.pie { text-align:center; padding:22px; color:var(--muted); font-size:11px; }
.overlay { position:fixed; inset:0; z-index:50; display:flex; align-items:center;
           justify-content:center; background:rgba(15,23,42,.72); padding:20px; }
.overlay-card { background:#fff; border-radius:22px; padding:34px; max-width:840px;
                width:100%; text-align:center; box-shadow:0 30px 70px rgba(0,0,0,.35); }
.overlay-card.compacta { max-width:720px; position:relative; }
.overlay-logo { font-size:46px; }
.overlay-card h1, .overlay-card h2 { margin:8px 0 6px; font-size:24px; }
.overlay-card p { color:var(--muted); font-size:13px; margin:0 0 22px; }
.pais-grid { display:grid; grid-template-columns:repeat(auto-fit,minmax(170px,1fr)); gap:11px; }
.pais-boton { border:2px solid var(--line); background:#f8fafc; border-radius:14px;
              padding:14px 10px; cursor:pointer; transition:.15s; }
.pais-boton:hover { border-color:var(--accent); transform:translateY(-2px); }
.pais-boton .bandera { font-size:28px; display:block; }
.pais-boton .nombre { font-weight:700; font-size:13px; display:block; margin-top:6px; }
.pais-boton .codigo { color:var(--muted); font-size:10px; letter-spacing:1px; }
.cerrar { position:absolute; right:14px; top:12px; border:0; background:#f1f5f9;
          border-radius:8px; width:30px; height:30px; cursor:pointer; }
.flotante { position:fixed; right:22px; bottom:22px; z-index:40; display:flex;
            align-items:center; gap:9px; background:#0f172a; color:#fff; border:0;
            border-radius:999px; padding:11px 17px; font-size:13px; font-weight:700;
            cursor:pointer; box-shadow:0 12px 30px rgba(15,23,42,.35); }
.flotante:hover { background:#1e293b; }
.flotante-cambiar { color:#7dd3fc; font-size:16px; }
.oculto { display:none !important; }
@media (max-width:900px) { .contenido { grid-template-columns:1fr; } }
"#;

fn txt(key: &str, lang: &str) -> &'static str {
    match (lang, key) {
        ("en", "select_country") => "Select your country",
        ("en", "select_sub") => "The legal framework and language will adapt to your choice.",
        ("en", "change_country") => "Change country",
        ("en", "change_sub") => "The banner and legal framework update instantly.",
        ("en", "welcome") => "Welcome",
        ("en", "legal_title") => "Legal framework verification",
        ("en", "legal_sub") => "Regulations applicable to the selected country",
        ("en", "sub_title") => "Substance technical sheet",
        ("en", "sub_sub") => "Search by name, CAS, EC or UN number",
        ("en", "no_country") => "Select a country to see its legal framework.",
        ("en", "pick_substance") => "Select a substance",
        ("en", "pick_substance_sub") => "Type in the search box and click a result to see the full sheet.",
        ("en", "warning") => "DEMO SHEETS. Verify the data in the official sources before real use.",
        ("es", "select_country") => "Selecciona tu país",
        ("es", "select_sub") => "El marco legal y el idioma del sitio se adaptarán a tu elección.",
        ("es", "change_country") => "Cambiar de país",
        ("es", "change_sub") => "El banner y el marco legal se actualizarán al instante.",
        ("es", "welcome") => "Bienvenido",
        ("es", "legal_title") => "Verificación del marco legal",
        ("es", "legal_sub") => "Normativa aplicable al país seleccionado",
        ("es", "sub_title") => "Carta técnica de la sustancia",
        ("es", "sub_sub") => "Busca por nombre, CAS, CE o número ONU",
        ("es", "no_country") => "Selecciona un país para ver su marco legal.",
        ("es", "pick_substance") => "Selecciona una sustancia",
        ("es", "pick_substance_sub") => "Escribe en el buscador y haz clic en un resultado para ver su ficha completa.",
        ("es", "warning") => "FICHAS DE DEMOSTRACIÓN. Verifica los datos en las fuentes oficiales antes de usarlos.",
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
    let mut multi_ids: Signal<Vec<String>> = use_signal(Vec::new);

    let country = selected_id()
        .and_then(|id| core::country_by_id(&id))
        .unwrap_or_else(|| countries[0].clone());
    let lang = if country.idioma == "en" { "en" } else { "es" };
    let chosen = selected_id().is_some();
    let displayed_country = if chosen { &country } else { &countries[0] };

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

    let international_sources: Vec<core::Source> = if country.id == "internacional" {
        all_sources
            .iter()
            .filter(|s| s.oficial && s.pais == "internacional")
            .cloned()
            .collect()
    } else {
        Vec::new()
    };

    let visible_instruments: Vec<&core::Instrument> = instruments
        .iter()
        .filter(|i| filter() == "todos" || i.tema == filter())
        .collect();

    let rsx_countries = |_modal: bool| {
        countries.iter().map(move |c| {
            let id = c.id.clone();
            let bandera = c.bandera.clone();
            let nombre = c.nombre.clone();
            let codigo = c.codigo.clone();
            rsx! {
                button {
                    class: "pais-boton",
                    onclick: move |_| {
                        selected_id.set(Some(id.clone()));
                        modal_open.set(false);
                        selected_substance.set(None);
                        query.set(String::new());
                        filter.set("todos".to_string());
                    },
                    span { class: "bandera", "{bandera}" }
                    span { class: "nombre", "{nombre}" }
                    span { class: "codigo", "{codigo}" }
                }
            }
        })
    };

    rsx! {
        style { {CSS} }

        // Selección inicial
        if !chosen {
            div { class: "overlay",
                div { class: "overlay-card",
                    div { class: "overlay-logo", "⚗️" }
                    h1 { "{txt(\"select_country\", lang)}" }
                    p { "{txt(\"select_sub\", lang)}" }
                    div { class: "pais-grid", {rsx_countries(false)} }
                }
            }
        }

        // Modal de cambio de país
        if modal_open() {
            div { class: "overlay",
                div { class: "overlay-card compacta",
                    button { class: "cerrar", onclick: move |_| modal_open.set(false), "✕" }
                    h2 { "{txt(\"change_country\", lang)}" }
                    p { "{txt(\"change_sub\", lang)}" }
                    div { class: "pais-grid", {rsx_countries(true)} }
                }
            }
        }

        header { class: "cabecera",
            div { class: "marca",
                span { class: "marca-icono", "⚗️" }
                span {
                    strong { "Sustancias Peligrosas" }
                    small { "Marco legal y fichas técnicas" }
                }
            }
            span { class: "badge badge-suave", "{country.codigo}" }
        }

        section { class: "banner",
            div { class: "banner-bandera", "{displayed_country.bandera}" }
            div { class: "banner-texto",
                h2 { "{displayed_country.saludo}" }
                p { "{displayed_country.descripcion}" }
            }
            div { class: "banner-pais", "{displayed_country.nombre} · {lang.to_uppercase()}" }
        }

        main { class: "contenido",

            // Apartado 1: marco legal
            section { class: "panel", id: "panel-legal",
                div { class: "panel-cabecera",
                    div { class: "panel-titulo",
                        span { class: "panel-icono", "⚖️" }
                        div {
                            h3 { "{txt(\"legal_title\", lang)}" }
                            p { "{txt(\"legal_sub\", lang)}" }
                        }
                    }
                    span { class: "badge", "{visible_instruments.len() + international_sources.len()} normas" }
                }

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

                div { class: "legal-lista",
                    if !chosen {
                        p { class: "vacio", "{txt(\"no_country\", lang)}" }
                    } else if country.id == "internacional" {
                        for s in international_sources.iter().filter(|s| filter() == "todos" || s.tema == filter()) {
                            div { class: "norma",
                                h4 { "{s.titulo}" }
                                div { class: "norma-meta",
                                    span { class: "badge", "{core::topic_label(&s.tema)}" }
                                    span {
                                        class: if core::is_verified(&s.verificacion) { "badge badge-ok" } else { "badge badge-warn" },
                                        "{core::verification_label(&s.verificacion)}"
                                    }
                                }
                                a { href: "{s.url}", target: "_blank", "{s.url}" }
                            }
                        }
                    } else {
                        for i in visible_instruments.iter() {
                            div { class: "norma",
                                h4 { "{i.titulo}" }
                                div { class: "norma-meta",
                                    span { class: "badge", "{core::topic_label(&i.tema)}" }
                                    span {
                                        class: if core::is_verified(&i.verificacion) { "badge badge-ok" } else { "badge badge-warn" },
                                        "{core::verification_label(&i.verificacion)}"
                                    }
                                }
                                a { href: "{i.url}", target: "_blank", "{i.url}" }
                                for alt in i.alternativas.iter() {
                                    a { href: "{alt}", target: "_blank", "Alternativa: {alt}" }
                                }
                                if !i.nota.is_empty() {
                                    p { class: "multi-nota", "{i.nota}" }
                                }
                            }
                        }
                    }
                }
            }

            // Apartado 2: carta técnica
            section { class: "panel", id: "panel-sustancia",
                div { class: "panel-cabecera",
                    div { class: "panel-titulo",
                        span { class: "panel-icono", "🧪" }
                        div {
                            h3 { "{txt(\"sub_title\", lang)}" }
                            p { "{txt(\"sub_sub\", lang)}" }
                        }
                    }
                }

                div { class: "aviso", "{txt(\"warning\", lang)}" }

                div { class: "buscador",
                    span { class: "buscador-icono", "🔍" }
                    input {
                        id: "buscador-sustancias",
                        placeholder: "Ej. Acetona, 67-64-1, ácido clorhídrico…",
                        value: "{query()}",
                        oninput: move |e| query.set(e.value()),
                    }
                }

                div { class: "resultados",
                    for s in results.iter() {
                        button {
                            class: "resultado",
                            onclick: { let id = s.id.clone(); move |_| selected_substance.set(Some(id.clone())) },
                            "{s.nombre} · {s.cas}"
                        }
                    }
                }

                div { class: "multi",
                    div { class: "multi-cabecera",
                        span { "⚗️ " strong { "Comparación de varias sustancias" } }
                        span { class: "badge badge-dev", "EN DESARROLLO" }
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
                                let found = core::search_substances(&multi_input()).into_iter().next();
                                if let Some(s) = found {
                                    let mut ids = multi_ids();
                                    if !ids.contains(&s.id) { ids.push(s.id.clone()); }
                                    multi_ids.set(ids);
                                    multi_input.set(String::new());
                                }
                            },
                            "Añadir"
                        }
                    }
                    div { class: "multi-chips",
                        for id in multi_ids().iter() {
                            if let Some(s) = core::substance_by_id(id) {
                                span { class: "multi-chip", "{s.nombre}" }
                            }
                        }
                    }
                    p { class: "multi-nota",
                        "Esta función permitirá analizar las interacciones entre sustancias. Actualmente solo muestra la lista de elementos seleccionados."
                    }
                }

                div { class: "ficha",
                    if let Some(s) = active_substance {
                        div { class: "ficha-titulo",
                            h4 { "{s.nombre}" }
                            span { class: "badge badge-warn", "DEMO" }
                        }
                        div { class: "identificadores",
                            span { class: "ident", "CAS {s.cas}" }
                            span { class: "ident", "CE {s.ce}" }
                            span { class: "ident", "ONU {s.onu}" }
                        }

                        div { class: "seccion",
                            h5 { "Peligrosidad y significado" }
                            div {
                                for p in s.peligrosidad.pictogramas.iter() {
                                    span { class: "picto", "{p}" }
                                }
                            }
                            for c in s.peligrosidad.clasificacion.iter() {
                                p { class: "frase", "• {c}" }
                            }
                            for h in s.peligrosidad.frases_h.iter() {
                                p { class: "frase", b { "{h.codigo}" } " — {h.texto}" }
                            }
                            p { "{s.peligrosidad.significado}" }
                        }

                        div { class: "seccion",
                            h5 { "Condiciones de almacenamiento" }
                            p { "{s.almacenamiento}" }
                        }

                        div { class: "seccion",
                            h5 { "Acciones en caso de derrame o contaminación" }
                            p { "{s.derrame}" }
                        }

                        div { class: "seccion",
                            h5 { "Propiedades químicas y físicas" }
                            div { class: "prop-grid",
                                if !s.propiedades.estado_fisico.is_empty() {
                                    div { class: "prop", b { "Estado físico" } "{s.propiedades.estado_fisico}" }
                                }
                                if !s.propiedades.olor.is_empty() {
                                    div { class: "prop", b { "Olor" } "{s.propiedades.olor}" }
                                }
                                if !s.propiedades.punto_de_inflamacion.is_empty() {
                                    div { class: "prop", b { "Punto de inflamación" } "{s.propiedades.punto_de_inflamacion}" }
                                }
                                if !s.propiedades.punto_de_ebullicion.is_empty() {
                                    div { class: "prop", b { "Punto de ebullición" } "{s.propiedades.punto_de_ebullicion}" }
                                }
                                if !s.propiedades.punto_de_fusion.is_empty() {
                                    div { class: "prop", b { "Punto de fusión" } "{s.propiedades.punto_de_fusion}" }
                                }
                                if !s.propiedades.densidad.is_empty() {
                                    div { class: "prop", b { "Densidad" } "{s.propiedades.densidad}" }
                                }
                                if !s.propiedades.densidad_de_gas.is_empty() {
                                    div { class: "prop", b { "Densidad de gas" } "{s.propiedades.densidad_de_gas}" }
                                }
                                if !s.propiedades.solubilidad_en_agua.is_empty() {
                                    div { class: "prop", b { "Solubilidad en agua" } "{s.propiedades.solubilidad_en_agua}" }
                                }
                                if !s.propiedades.presion_de_vapor.is_empty() {
                                    div { class: "prop", b { "Presión de vapor" } "{s.propiedades.presion_de_vapor}" }
                                }
                                if !s.propiedades.ph.is_empty() {
                                    div { class: "prop", b { "pH" } "{s.propiedades.ph}" }
                                }
                            }
                        }

                        div { class: "seccion",
                            h5 { "Interacción con otros elementos" }
                            ul { class: "lista",
                                for x in s.interacciones.incompatibilidades.iter() {
                                    li { "{x}" }
                                }
                            }
                            p { "{s.interacciones.nota}" }
                        }

                        div { class: "seccion fuentes",
                            h5 { "Fuentes" }
                            for f in s.fuentes.iter() {
                                a { href: "{f.url}", target: "_blank", "{f.titulo}" }
                            }
                        }
                    } else {
                        div { class: "ficha-vacia",
                            div { class: "ficha-vacia-icono", "📄" }
                            h4 { "{txt(\"pick_substance\", lang)}" }
                            p { "{txt(\"pick_substance_sub\", lang)}" }
                        }
                    }
                }
            }
        }

        footer { class: "pie",
            p { "{core::substances_warning()}" }
            p { "Fuentes normativas: UNECE · OMI · OIT · INSST · BOE · DOF · SINIA · Asamblea Nacional" }
        }

        if chosen {
            button {
                class: "flotante",
                onclick: move |_| modal_open.set(true),
                span { id: "flotante-bandera", "{displayed_country.bandera}" }
                span { id: "flotante-nombre", "{displayed_country.nombre}" }
                span { class: "flotante-cambiar", "↻" }
            }
        }
    }
}
