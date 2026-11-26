use dioxus::prelude::*;
use sustancias_core as core;
use sustancias_i18n::Translator;

/// URL canónica del repositorio, reutilizada en feedback, redes y footer.
const GITHUB_URL: &str = "https://github.com/CarlosAntonioAceaJimenezCD/sustancias-peligrosas";
/// Tope de combinaciones calculadas y de tarjetas mostradas (evita explosión con n=10).
const MAX_TRIPLE_CANDIDATES: usize = 120;
const MAX_TRIPLES_SHOWN: usize = 12;

mod flags_extra;

const CSS: &str = r#"
/* ==========================================================================
   Sistema visual — Sustancias Peligrosas
   Capas: primitivos → semánticos → componentes. Los componentes solo usan
   tokens semánticos, de modo que el tema se pueda cambiar en un solo lugar.
   ========================================================================== */
:root{
  color-scheme:dark;

  /* --- Primitivos compartidos --- */
  --gray-950:#090d12; --gray-900:#0d1117; --gray-850:#11161d; --gray-800:#161b22;
  --gray-750:#1c2128; --gray-700:#21262d; --gray-650:#2a313a; --gray-600:#30363d;
  --gray-500:#484f58; --gray-400:#6e7681; --gray-300:#8b949e; --gray-200:#b9c3ce;
  --gray-100:#e6edf3; --gray-50:#f0f6fc;
  --blue-500:#58a6ff; --blue-400:#79c0ff; --blue-700:#1b2c4f; --blue-800:#12213a;
  --green-500:#3fb950; --green-600:#2ea043; --green-700:#238636;
  --amber-500:#d29922; --amber-300:#e3b341; --red-500:#f85149; --purple-500:#a371f7;

  /* --- Semánticos: tema oscuro (instrumental) --- */
  --bg:var(--gray-900); --surface:var(--gray-800); --surface-2:var(--gray-750);
  --input-bg:#0d1117; --code-bg:#0d1117;
  --card:var(--gray-800); --card-alt:var(--gray-750); --raised:var(--gray-700);
  --ink:var(--gray-200); --ink-strong:var(--gray-50); --muted:var(--gray-300);
  --line:var(--gray-600); --line-soft:var(--gray-650);
  --accent:var(--blue-500); --accent-strong:var(--blue-400);
  --accent-soft:rgba(88,166,255,.14); --accent-line:rgba(88,166,255,.38);
  --purple:var(--purple-500); --ok:var(--green-500); --warn:var(--amber-500);
  --danger:var(--red-500); --focus:var(--blue-400);
  --banner-a:#12213a; --banner-b:#1b2c4f; --banner-c:#241d42; --banner-text:#b8c7da;
  --lab-grid:rgba(88,166,255,.045); --lab-line:rgba(88,166,255,.12);

  --sp-1:4px; --sp-2:8px; --sp-3:12px; --sp-4:16px; --sp-5:20px; --sp-6:24px; --sp-8:32px;
  --r-sm:6px; --r-md:8px; --r-lg:12px; --r-pill:2em;
  --shadow-1:0 2px 8px rgba(0,0,0,.24);
  --shadow-2:0 10px 28px rgba(0,0,0,.34);
  --shadow-3:0 24px 60px rgba(0,0,0,.55);

  --font-sans:-apple-system,BlinkMacSystemFont,"Segoe UI","Noto Sans",Helvetica,Arial,sans-serif;
  --font-mono:ui-monospace,SFMono-Regular,"SF Mono",Menlo,Consolas,monospace;
  --step--1:clamp(.78rem,.76rem + .10vw,.84rem);
  --step-0:clamp(.88rem,.85rem + .14vw,1rem);
  --step-1:clamp(1rem,.96rem + .20vw,1.13rem);
  --step-2:clamp(1.18rem,1.10rem + .38vw,1.45rem);
  --step-3:clamp(1.45rem,1.30rem + .70vw,1.95rem);
  --step-4:clamp(1.75rem,1.50rem + 1.10vw,2.45rem);
  --ease:cubic-bezier(.2,.7,.3,1); --dur:150ms;
}

/* --- Tema claro suave: papel de laboratorio, sin blanco deslumbrante --- */
:root[data-theme="light"]{
  color-scheme:light;
  --bg:#eef2f6; --surface:#f6f9fb; --surface-2:#ffffff;
  --input-bg:#ffffff; --code-bg:#f1f5f9;
  --card:#f6f9fb; --card-alt:#ffffff; --raised:#e7edf3;
  --ink:#2a3644; --ink-strong:#152130; --muted:#5b6b7c;
  --line:#ccd6e0; --line-soft:#dde5ed;
  --accent:#0b5d8f; --accent-strong:#08466d;
  --accent-soft:rgba(11,93,143,.10); --accent-line:rgba(11,93,143,.30);
  --purple:#6d4a9c; --ok:#12662b; --warn:#6b4400; --danger:#b42318; --focus:#0b5d8f;
  --banner-a:#dae6f1; --banner-b:#cfdcec; --banner-c:#e3dcf0; --banner-text:#34485d;
  --lab-grid:rgba(11,93,143,.055); --lab-line:rgba(11,93,143,.14);
  --shadow-1:0 2px 8px rgba(21,33,48,.07);
  --shadow-2:0 10px 28px rgba(21,33,48,.10);
  --shadow-3:0 24px 60px rgba(21,33,48,.16);
}
@media (prefers-color-scheme: light){
  :root:not([data-theme="dark"]){
    color-scheme:light;
    --bg:#eef2f6; --surface:#f6f9fb; --surface-2:#ffffff;
  --input-bg:#ffffff; --code-bg:#f1f5f9;
    --card:#f6f9fb; --card-alt:#ffffff; --raised:#e7edf3;
    --ink:#2a3644; --ink-strong:#152130; --muted:#5b6b7c;
    --line:#ccd6e0; --line-soft:#dde5ed;
    --accent:#0b5d8f; --accent-strong:#08466d;
    --accent-soft:rgba(11,93,143,.10); --accent-line:rgba(11,93,143,.30);
    --purple:#6d4a9c; --ok:#12662b; --warn:#6b4400; --danger:#b42318; --focus:#0b5d8f;
    --banner-a:#dae6f1; --banner-b:#cfdcec; --banner-c:#e3dcf0; --banner-text:#34485d;
    --lab-grid:rgba(11,93,143,.055); --lab-line:rgba(11,93,143,.14);
    --shadow-1:0 2px 8px rgba(21,33,48,.07);
    --shadow-2:0 10px 28px rgba(21,33,48,.10);
    --shadow-3:0 24px 60px rgba(21,33,48,.16);
  }
}

*,*::before,*::after{box-sizing:border-box}
html{scroll-behavior:smooth}
body{
  margin:0;font-family:var(--font-sans);font-size:var(--step-0);line-height:1.6;
  color:var(--ink);background:var(--bg);
  background-image:
    linear-gradient(var(--lab-grid) 1px,transparent 1px),
    linear-gradient(90deg,var(--lab-grid) 1px,transparent 1px),
    radial-gradient(1100px 520px at 50% -12%,rgba(88,166,255,.10),transparent 68%);
  background-size:32px 32px,32px 32px,auto;
  -webkit-font-smoothing:antialiased;text-rendering:optimizeLegibility;
  min-height:100dvh;
}
/* Foco siempre visible: requisito WCAG 2.4.7 */
:focus-visible{outline:2px solid var(--focus);outline-offset:2px;border-radius:4px}
.skip-link{position:absolute;left:-9999px;top:0;z-index:100;background:var(--accent);
  color:var(--gray-950);padding:10px 16px;border-radius:0 0 var(--r-sm) 0;font-weight:700}
.skip-link:focus{left:0}
::selection{background:var(--accent-soft);color:var(--ink-strong)}
@supports (scrollbar-width:thin){*{scrollbar-width:thin;scrollbar-color:var(--gray-600) transparent}}

/* --- Iconos: currentColor, sin colores fijos --- */
.ico{width:16px;height:16px;flex:none;fill:none;stroke:currentColor;stroke-width:2;
  stroke-linecap:round;stroke-linejoin:round;vertical-align:middle}
.ico-lg{width:22px;height:22px}.ico-xl{width:44px;height:44px;stroke-width:1.6}

/* --- Cabecera --- */
.cabecera{display:flex;justify-content:space-between;align-items:center;gap:var(--sp-4);
  padding:var(--sp-3) var(--sp-6);background:color-mix(in srgb,var(--surface) 88%,transparent);
  border-bottom:1px solid var(--line);border-top:2px solid var(--accent-line);
  backdrop-filter:blur(12px);position:sticky;top:0;z-index:30}
.cabecera-derecha{display:flex;align-items:center;gap:var(--sp-2)}
.tema-boton{display:grid;place-items:center;width:40px;height:40px;border:1px solid var(--line);
  border-radius:var(--r-sm);background:var(--surface-2);color:var(--accent);cursor:pointer;
  transition:all var(--dur) var(--ease)}
.tema-boton:hover{border-color:var(--accent-line);background:var(--accent-soft);transform:translateY(-1px)}
.estrella-banner{display:flex;align-items:center;gap:var(--sp-2);flex-wrap:wrap;
  border:1px solid var(--accent-line);border-radius:var(--r-sm);background:var(--accent-soft);
  padding:var(--sp-2) var(--sp-3);margin:var(--sp-2) 0;font-size:var(--step--1);color:var(--ink)}
.estrella-banner .ico{color:var(--accent)}
.estrella-texto{flex:1;min-inline-size:180px}
.estrella-enlace{color:var(--accent);font-weight:700;text-decoration:none}
.estrella-enlace:hover{text-decoration:underline}
.estrella-cerrar{flex:none;width:44px;height:44px;display:grid;place-items:center;
  border:1px solid var(--line);border-radius:var(--r-sm);background:var(--surface-2);
  color:var(--muted);cursor:pointer}
.estrella-cerrar:hover{border-color:var(--accent);color:var(--accent)}
.estrella-cerrar .ico{width:20px;height:20px}
.tutorial-cas{margin-top:var(--sp-2);font-size:var(--step--1);color:var(--muted)}
.tutorial-cas summary{cursor:pointer;font-weight:650;color:var(--accent)}
.marca{display:flex;gap:var(--sp-3);align-items:center}
.marca-icono{display:grid;place-items:center;width:38px;height:38px;border:1px solid var(--line-soft);
  border-radius:var(--r-md);color:var(--accent);background:var(--surface-2);
  box-shadow:inset 0 1px 0 rgba(255,255,255,.04)}
.marca strong{display:block;font-size:var(--step-1);color:var(--ink-strong);letter-spacing:-.01em}
.marca small{display:block;color:var(--muted);font-size:var(--step--1)}

/* --- Badges --- */
.badge{display:inline-flex;align-items:center;gap:var(--sp-1);padding:2px 9px;border-radius:var(--r-pill);
  font-family:var(--font-mono);letter-spacing:.03em;text-transform:uppercase;
  font-size:var(--step--1);font-weight:650;line-height:1.5;background:var(--accent-soft);
  color:var(--accent);border:1px solid var(--accent-line);white-space:nowrap}
.badge-suave{background:var(--surface-2);color:var(--muted);border-color:var(--line)}
.badge-ok{background:rgba(63,185,80,.14);color:var(--ok);border-color:rgba(63,185,80,.35)}
.badge-warn{background:rgba(210,153,34,.14);color:var(--amber-300);border-color:rgba(210,153,34,.38)}
.badge-dev{background:rgba(210,153,34,.14);color:var(--amber-300);border-color:rgba(210,153,34,.4)}
.badge-demo{background:rgba(163,113,247,.14);color:var(--purple);border-color:rgba(163,113,247,.38)}
.badge-online{background:var(--accent-soft);color:var(--accent-strong);border-color:var(--accent-line)}
.badge-danger{background:rgba(248,81,73,.14);color:var(--danger);border-color:rgba(248,81,73,.4)}

/* --- Banner del país --- */
.banner{margin:var(--sp-5) auto 0;max-width:1180px;padding:var(--sp-5) var(--sp-6);
  border-radius:var(--r-lg);display:flex;align-items:center;gap:var(--sp-5);
  background:linear-gradient(115deg,var(--banner-a),var(--banner-b) 58%,var(--banner-c));
  border:1px solid var(--accent-line);box-shadow:var(--shadow-2);position:relative;overflow:hidden}
.banner::after{content:"";position:absolute;inset:0;pointer-events:none;
  background:linear-gradient(90deg,transparent 55%,var(--lab-grid));mix-blend-mode:screen}
.flag{width:30px;height:20px;display:block;border-radius:3px;box-shadow:0 0 0 1px rgba(255,255,255,.22)}
.flag-lg{width:56px;height:38px;border-radius:var(--r-sm)}
.banner-pais-grande{display:grid;place-items:center;width:66px;height:66px;border-radius:var(--r-md);
  background:rgba(121,192,255,.14);border:1px solid rgba(121,192,255,.38);
  box-shadow:inset 0 1px 0 rgba(255,255,255,.1);color:var(--accent-strong);
  font-size:var(--step-2);font-weight:750;letter-spacing:.02em;flex:none}
.banner-texto{flex:1;min-inline-size:0}.banner-texto h2{margin:0 0 var(--sp-1);
  font-size:var(--step-3);color:var(--ink-strong);font-weight:700;letter-spacing:-.02em;text-wrap:balance}
.banner-texto p{margin:0;color:var(--banner-text);font-size:var(--step-0);max-width:72ch;line-height:1.6}
.banner-pais{font-size:var(--step--1);font-weight:700;letter-spacing:.06em;padding:5px 13px;
  border-radius:var(--r-pill);color:var(--banner-text);border:1px solid var(--accent-line);white-space:nowrap}

/* --- Contenedor principal: grid con pistas intrínsecas --- */
.contenido{max-width:1180px;margin:var(--sp-5) auto 110px;padding:0 var(--sp-5);
  display:grid;grid-template-columns:minmax(0,1fr);gap:var(--sp-5)}
.panel-sustancia{order:1;min-inline-size:0}
.panel-legal{order:2;min-inline-size:0}
.panel-legal summary{display:flex;align-items:center;gap:var(--sp-3);cursor:pointer;
  font-size:var(--step-1);font-weight:650;color:var(--ink-strong);list-style:none;padding:var(--sp-1) 0}
.panel-legal summary::-webkit-details-marker{display:none}
.panel-legal summary .badge{margin-inline-start:auto}
.panel-legal[open] summary{margin-bottom:var(--sp-3);border-bottom:1px solid var(--line);padding-bottom:var(--sp-3)}

/* --- Paneles --- */
.panel{background:var(--card);border:1px solid var(--line);border-radius:var(--r-md);
  padding:var(--sp-5);box-shadow:var(--shadow-1)}
.panel-cabecera{display:flex;justify-content:space-between;align-items:flex-start;gap:var(--sp-3);
  border-bottom:1px solid var(--line);padding-bottom:var(--sp-3)}
.panel-titulo{display:flex;gap:var(--sp-3);align-items:center}
.panel-icono{display:grid;place-items:center;width:38px;height:38px;border-radius:var(--r-md);
  background:var(--surface-2);border:1px solid var(--line-soft);color:var(--accent);flex:none}
.panel h3{margin:0;font-size:var(--step-1);color:var(--ink-strong);font-weight:650;letter-spacing:-.01em}
.panel-cabecera p{margin:2px 0 0;color:var(--muted);font-size:var(--step--1)}

/* --- Filtros y normas --- */
.filtros-wrap{display:flex;gap:var(--sp-2);align-items:center;margin:var(--sp-3) 0 var(--sp-2);flex-wrap:wrap}
.filtro-icono{color:var(--muted)}.filtros{display:flex;flex-wrap:wrap;gap:var(--sp-2)}
.chip{border:1px solid var(--line);background:var(--surface-2);border-radius:var(--r-pill);
  padding:7px 12px;font-size:var(--step--1);cursor:pointer;color:var(--muted);
  transition:background var(--dur) var(--ease),border-color var(--dur) var(--ease),color var(--dur) var(--ease)}
.chip:hover{border-color:var(--gray-500);color:var(--ink);background:var(--raised)}
.chip.activo{background:var(--accent-soft);color:var(--accent);border-color:var(--accent-line)}
.legal-lista{max-height:560px;overflow:auto;display:flex;flex-direction:column;gap:var(--sp-2);padding-right:var(--sp-1)}
.norma{border:1px solid var(--line);border-radius:var(--r-sm);padding:var(--sp-3);background:var(--surface-2);
  transition:border-color var(--dur) var(--ease),transform var(--dur) var(--ease)}
.norma:hover{border-color:var(--gray-500);transform:translateY(-1px)}
.norma h4{margin:0 0 var(--sp-1);font-size:var(--step-0);line-height:1.5;color:var(--ink-strong);font-weight:650}
.norma-meta{display:flex;gap:var(--sp-2);align-items:center;margin:var(--sp-2) 0;flex-wrap:wrap}
.norma a{color:var(--accent);font-size:var(--step--1);text-decoration:none;word-break:break-word;
  display:flex;align-items:flex-start;gap:var(--sp-1);margin-top:3px}
.norma a:hover{text-decoration:underline}
.vacio{color:var(--muted);font-size:var(--step-0);padding:var(--sp-4) 0}

/* --- Buscador --- */
.resultados-online{max-height:300px;overflow-y:auto;margin:var(--sp-3) 0 var(--sp-4);border-radius:var(--r-sm)}
.buscador{display:flex;align-items:center;gap:var(--sp-3);border:1px solid var(--line);border-radius:var(--r-md);
  padding:var(--sp-3) var(--sp-4);margin:var(--sp-3) 0 var(--sp-2);background:var(--input-bg);
  transition:border-color var(--dur) var(--ease),box-shadow var(--dur) var(--ease)}
.buscador:focus-within{border-color:var(--accent);box-shadow:0 0 0 3px var(--accent-soft)}
.buscador-icono{color:var(--muted)}
.buscador input,.multi input{border:0;outline:0;width:100%;font-size:var(--step-0);background:transparent;color:var(--ink)}
.buscador input::placeholder,.multi input::placeholder{color:var(--gray-400)}
.resultados{display:flex;flex-wrap:wrap;gap:var(--sp-2);margin-bottom:var(--sp-3)}
.resultado{border:1px solid var(--line);background:var(--surface-2);border-radius:var(--r-sm);
  padding:7px 12px;cursor:pointer;font-size:var(--step-0);color:var(--ink);
  transition:all var(--dur) var(--ease)}
.resultado:hover,.resultado.activo{border-color:var(--accent);background:var(--accent-soft);color:var(--accent)}
.aviso{display:flex;gap:var(--sp-2);align-items:flex-start;background:rgba(210,153,34,.10);
  border:1px solid rgba(210,153,34,.36);border-radius:var(--r-sm);padding:var(--sp-2) var(--sp-3);
  font-size:var(--step--1);color:var(--warn);margin-bottom:var(--sp-3)}
.aviso .ico{color:var(--warn);margin-top:2px}

/* --- Multi-sustancia --- */
.multi{border:1px dashed var(--gray-500);border-radius:var(--r-md);padding:var(--sp-3);
  margin:var(--sp-3) 0;background:color-mix(in srgb,var(--surface) 60%,transparent)}
.multi-cabecera{display:flex;justify-content:space-between;align-items:center;font-size:var(--step-0);margin-bottom:var(--sp-3)}
.multi-titulo{display:flex;align-items:center;gap:var(--sp-2);color:var(--ink-strong)}
.multi-titulo .ico{color:var(--purple)}
.multi-anadir{display:flex;gap:var(--sp-2)}
.multi-anadir input{border:1px solid var(--line);border-radius:var(--r-sm);padding:8px 11px;background:var(--input-bg)}
.multi-chips{display:flex;flex-wrap:wrap;gap:var(--sp-2);margin:var(--sp-3) 0 var(--sp-1)}
.multi-chip{background:rgba(163,113,247,.14);color:#d2a8ff;border:1px solid rgba(163,113,247,.38);
  border-radius:var(--r-pill);padding:4px 11px;font-size:var(--step--1);font-weight:650}
.multi-nota{margin:var(--sp-1) 0 0;font-size:var(--step--1);color:var(--muted)}
.aviso-ternario{margin:var(--sp-3) 0 var(--sp-1);border-left:2px solid var(--warn);padding-left:var(--sp-2);font-weight:650}
.multi-tercias{margin-top:var(--sp-3)}
.regla-ternaria{margin:var(--sp-2) 0;padding:var(--sp-2);border:1px solid rgba(210,153,34,.38);border-radius:var(--r-sm);background:rgba(210,153,34,.08)}
.regla-ternaria strong{color:var(--amber-300)}

/* --- Botones --- */
.boton{display:inline-flex;align-items:center;justify-content:center;gap:var(--sp-2);
  background:var(--green-700);color:#fff;border:1px solid var(--green-600);
  border-radius:var(--r-sm);padding:9px 14px;font-size:var(--step-0);font-weight:650;
  cursor:pointer;text-decoration:none;min-height:40px;
  transition:background var(--dur) var(--ease),border-color var(--dur) var(--ease),transform var(--dur) var(--ease)}
.boton:hover{background:var(--green-600);transform:translateY(-1px)}
.boton:active{transform:translateY(0)}
.boton-descargar{background:var(--blue-600);border-color:var(--blue-500)}
.boton-descargar:hover{background:var(--blue-500)}
.boton-secundario{background:var(--surface-2);color:var(--ink);border-color:var(--line)}
.boton-secundario:hover{border-color:var(--gray-500);background:var(--raised)}

/* --- Ficha de 16 secciones --- */
.ficha{margin-top:var(--sp-2)}.ficha-vacia{text-align:center;padding:var(--sp-8) var(--sp-3);color:var(--muted)}
.ficha-vacia .ico{color:var(--gray-500)}.ficha-vacia h4{margin:var(--sp-3) 0 var(--sp-1);color:var(--ink);font-size:var(--step-1)}
.ficha-vacia p{margin:0;font-size:var(--step-0)}
.ficha-titulo{display:flex;justify-content:space-between;align-items:center;gap:var(--sp-3);flex-wrap:wrap}
.ficha-titulo h4{margin:0;font-size:var(--step-3);color:var(--ink-strong);letter-spacing:-.02em;text-wrap:balance}
.identificadores{display:flex;gap:var(--sp-2);flex-wrap:wrap;margin:var(--sp-3) 0 var(--sp-4)}
.ident{background:var(--surface-2);border:1px solid var(--line);border-radius:var(--r-sm);
  padding:4px 9px;font-size:var(--step--1);font-weight:650;color:var(--muted);font-family:var(--font-mono)}
.fds-index{display:flex;flex-wrap:wrap;gap:var(--sp-1);border:1px solid var(--line);border-radius:var(--r-sm);
  padding:var(--sp-2);margin-bottom:var(--sp-4);background:var(--surface-2)}
.fds-index a{font-size:var(--step--1);font-family:var(--font-mono);color:var(--muted);text-decoration:none;
  border:1px solid var(--line);border-radius:var(--r-sm);padding:3px 7px;
  transition:all var(--dur) var(--ease)}
.fds-index a:hover{color:var(--accent);border-color:var(--accent-line);background:var(--accent-soft)}
.seccion{margin:var(--sp-4) 0;border-left:2px solid var(--accent-line);box-shadow:inset 1px 0 0 var(--lab-line);padding-left:var(--sp-3);scroll-margin-top:80px}
.seccion h5{margin:0 0 var(--sp-2);font-size:var(--step--1);text-transform:uppercase;
  letter-spacing:.09em;color:var(--accent);font-weight:750}
.seccion p{margin:0 0 var(--sp-2);font-size:var(--step-0);line-height:1.7;color:var(--gray-200);max-width:72ch}
.prop-grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(min(180px,100%),1fr));gap:var(--sp-2)}
.prop{background:var(--surface-2);border:1px solid var(--line);border-radius:var(--r-sm);padding:9px 11px;font-size:var(--step-0)}
.prop b{display:block;color:var(--accent);font-size:var(--step--1);text-transform:uppercase;
  letter-spacing:.05em;margin-bottom:3px}
.lista{margin:0 0 var(--sp-2);padding-left:20px;font-size:var(--step-0);color:var(--gray-200);
  line-height:1.7;max-width:74ch}.lista li{margin:3px 0}
.lista-titulo{margin:var(--sp-2) 0 3px!important;font-size:var(--step-0)!important;
  font-weight:750;color:var(--ink-strong)!important}
.nota{display:flex;gap:var(--sp-2);align-items:flex-start;font-size:var(--step--1)!important;color:var(--amber-300)!important}
.fuentes a{display:flex;align-items:center;gap:var(--sp-1);font-size:var(--step--1);
  color:var(--accent);text-decoration:none;margin-top:var(--sp-1)}

/* --- Resultados online --- */
.online-titulo{display:flex;align-items:center;gap:var(--sp-2);font-size:var(--step--1);
  font-weight:750;color:var(--accent);margin-bottom:var(--sp-2);text-transform:uppercase;letter-spacing:.06em}
.online-resultado{display:flex;justify-content:space-between;align-items:center;gap:var(--sp-3);
  border:1px solid var(--line);border-radius:var(--r-sm);background:var(--surface-2);
  padding:var(--sp-3);margin-bottom:var(--sp-2);transition:border-color var(--dur) var(--ease)}
.online-resultado:hover{border-color:var(--accent-line)}
.online-resultado strong{display:block;font-size:var(--step-0);color:var(--ink-strong)}
.online-meta{display:block;font-size:var(--step--1);color:var(--muted);margin-top:2px;font-family:var(--font-mono)}
.online-nota{margin:var(--sp-2) 0 0;font-size:var(--step--1);line-height:1.5;
  color:var(--amber-300);border-left:2px solid var(--warn);padding-left:var(--sp-2)}
.online-resultado .boton{white-space:nowrap;font-size:var(--step--1)}

/* --- Feedback y redes --- */
.feedback-panel,.redes-panel{max-width:1180px;width:calc(100% - 40px);margin:0 auto var(--sp-4)}
.feedback-panel summary{display:flex;align-items:center;gap:var(--sp-3);cursor:pointer;
  font-weight:650;color:var(--ink-strong);list-style:none}
.feedback-panel summary::-webkit-details-marker{display:none}
.formulario{display:flex;flex-direction:column;gap:var(--sp-3);margin-top:var(--sp-3)}
.formulario .opcion{display:flex;flex-direction:column;align-items:stretch;gap:var(--sp-1)}
.formulario input,.formulario select,.formulario textarea{background:var(--input-bg);color:var(--ink);
  border:1px solid var(--line);border-radius:var(--r-sm);padding:10px 12px;
  font-size:var(--step-0);font-family:inherit}
.formulario textarea{min-height:100px;resize:vertical}
.redes{display:flex;gap:var(--sp-5);flex-wrap:wrap;margin-top:var(--sp-3)}
.redes a{display:flex;align-items:center;gap:var(--sp-2);color:var(--accent);
  text-decoration:none;font-size:var(--step-0)}
.redes a:hover{text-decoration:underline}
.pie{text-align:center;padding:var(--sp-5);color:var(--muted);font-size:var(--step--1);
  border-top:1px solid var(--line)}
.link-feedback{color:var(--accent);text-decoration:none;border-bottom:1px dashed var(--accent-line)}
.link-feedback:hover{text-decoration:underline}
.pie p{margin:var(--sp-1) 0}

/* --- Selector flotante y modal --- */
.flotante{position:fixed;right:var(--sp-5);bottom:var(--sp-5);z-index:40;
  display:flex;align-items:center;gap:var(--sp-2);background:var(--raised);color:var(--ink);
  border:1px solid var(--gray-500);border-radius:var(--r-md);padding:11px 15px;
  font-size:var(--step-0);font-weight:650;cursor:pointer;box-shadow:var(--shadow-2);
  min-height:44px;transition:background var(--dur) var(--ease),border-color var(--dur) var(--ease),transform var(--dur) var(--ease)}
.flotante:hover{background:var(--gray-600);border-color:var(--gray-400);transform:translateY(-2px)}
.flotante-codigo{background:var(--accent-soft);color:var(--accent);border:1px solid var(--accent-line);
  border-radius:var(--r-sm);padding:2px 7px;font-size:var(--step--1);letter-spacing:.04em;font-family:var(--font-mono)}
.flotante-cambiar{color:var(--muted)}
.overlay{position:fixed;inset:0;z-index:50;display:flex;align-items:center;justify-content:center;
  background:rgba(4,8,14,.84);padding:var(--sp-5);backdrop-filter:blur(6px)}
.overlay-card{background:var(--card);border:1px solid var(--line);border-radius:var(--r-lg);
  padding:var(--sp-8);max-width:860px;width:100%;max-height:calc(100dvh - 40px);
  overflow-y:auto;text-align:center;box-shadow:var(--shadow-3)}
.overlay-card.compacta{max-width:740px;position:relative}.overlay-logo{color:var(--accent)}
.overlay-card h1,.overlay-card h2{margin:var(--sp-3) 0 var(--sp-1);font-size:var(--step-3);
  color:var(--ink-strong);font-weight:700;letter-spacing:-.02em;text-wrap:balance}
.overlay-card p{color:var(--muted);font-size:var(--step-0);margin:0 0 var(--sp-5)}
.pais-grid{display:flex;flex-direction:column;gap:var(--sp-2)}
.continente-carpeta{border:1px solid var(--line);border-radius:var(--r-md);background:var(--surface-2);overflow:hidden}
.continente-titulo{margin:0;padding:12px var(--sp-3);cursor:pointer;list-style:none;
  display:flex;align-items:center;gap:var(--sp-2);font-size:var(--step--1);
  color:var(--accent);text-transform:uppercase;letter-spacing:.09em;font-weight:750;
  transition:background var(--dur) var(--ease)}
.continente-titulo:hover{background:var(--accent-soft)}
.continente-titulo::-webkit-details-marker{display:none}
.continente-titulo .badge{margin-inline-start:auto}
.continente-carpeta[open] .continente-titulo{border-bottom:1px solid var(--line)}
.continente-icono{display:grid;place-items:center;background:var(--gray-50);border-radius:var(--r-sm);padding:3px}
.continente-silueta{width:20px;height:20px;display:block}
.paises-carpeta{display:grid;grid-template-columns:repeat(auto-fit,minmax(min(180px,100%),1fr));
  gap:var(--sp-2);padding:var(--sp-3)}
.pais-boton{display:flex;align-items:center;gap:var(--sp-3);text-align:left;border:1px solid var(--line);
  background:var(--surface-2);border-radius:var(--r-sm);padding:var(--sp-3);cursor:pointer;
  color:var(--ink);min-height:44px;transition:all var(--dur) var(--ease)}
.pais-boton:hover{border-color:var(--accent-line);background:var(--accent-soft);transform:translateY(-1px)}
.pais-boton .bandera{display:grid;place-items:center;width:38px;height:38px;flex:none;
  border:1px solid var(--line);border-radius:var(--r-sm);background:var(--gray-950);
  color:var(--accent);font-size:var(--step-0);font-weight:700;letter-spacing:.03em;overflow:hidden}
.pais-boton .nombre{font-weight:650;font-size:var(--step-0);display:block;color:var(--ink-strong)}
.pais-boton .codigo{color:var(--muted);font-size:var(--step--1);display:block;letter-spacing:.04em;font-family:var(--font-mono)}
.cerrar{position:absolute;right:var(--sp-3);top:var(--sp-3);display:grid;place-items:center;
  border:1px solid var(--line);background:var(--surface-2);color:var(--muted);
  border-radius:var(--r-sm);width:36px;height:36px;cursor:pointer;min-height:44px;min-width:44px}
.cerrar:hover{color:var(--ink-strong);border-color:var(--gray-500)}
.oculto{display:none!important}
.norma-modal{max-width:900px;text-align:left;max-height:88dvh;display:flex;flex-direction:column}
.norma-modal h2{font-size:var(--step-2);padding-right:44px}.norma-modal #norma-meta{margin-bottom:var(--sp-3)}
.norma-texto{flex:1;overflow:auto;white-space:pre-wrap;word-wrap:break-word;background:var(--input-bg);
  border:1px solid var(--line);border-radius:var(--r-sm);padding:var(--sp-4);
  font-family:var(--font-mono);font-size:var(--step--1);line-height:1.65;
  color:var(--gray-200);margin:0 0 var(--sp-4)}
.norma-modal .boton{align-self:flex-start;text-decoration:none}
.leer-norma{margin-top:var(--sp-2);background:var(--surface-2);color:var(--accent);border-color:var(--line)}
.leer-norma:hover{border-color:var(--accent-line);background:var(--accent-soft)}

/* --- Idioma, BOE y capas legales --- */
.idioma-busqueda{display:flex;align-items:center;gap:var(--sp-2);flex-wrap:wrap;
  margin:var(--sp-3) 0;font-size:var(--step--1);color:var(--muted)}
.idioma-busqueda select{background:var(--input-bg);color:var(--ink);border:1px solid var(--line);
  border-radius:var(--r-sm);padding:7px 10px;font-size:var(--step--1);min-height:36px}
.boe-buscar{border:1px solid var(--line);border-radius:var(--r-sm);padding:var(--sp-1) var(--sp-3) var(--sp-3);
  margin:var(--sp-3) 0;background:var(--surface-2)}
.boe-input-wrap{margin:var(--sp-2) 0}
.boe-atribucion{font-size:var(--step--1)!important;color:var(--muted)!important;margin:var(--sp-2) 0 0!important;line-height:1.5!important}
.boe-atribucion a{color:var(--accent)}
.leyenda-grupo{margin:var(--sp-4) 0 var(--sp-1);font-size:var(--step--1);text-transform:uppercase;
  letter-spacing:.09em;color:var(--muted);font-weight:750;display:flex;align-items:center;gap:var(--sp-2);
  font-family:var(--font-mono)}
.leyenda-grupo::after{content:"";flex:1;height:1px;background:var(--line)}
.portales{border:1px solid var(--line);border-radius:var(--r-sm);padding:var(--sp-3);
  background:var(--surface-2);margin-bottom:var(--sp-1)}
.portales a{display:flex;align-items:center;gap:var(--sp-2);color:var(--accent);
  font-size:var(--step--1);text-decoration:none;margin-top:var(--sp-1)}
.portales a:hover{text-decoration:underline}

/* --- Interacciones --- */
.interacciones{display:flex;flex-direction:column;gap:var(--sp-2);margin-top:var(--sp-3)}
.interaccion{border:1px solid var(--line);border-left:3px solid var(--gray-500);border-radius:var(--r-sm);
  padding:var(--sp-3);background:var(--card)}
.interaccion-cabecera{display:flex;justify-content:space-between;align-items:center;gap:var(--sp-2);flex-wrap:wrap}
.interaccion-cabecera strong{font-size:var(--step-0);color:var(--ink-strong)}
.interaccion-tipo{font-size:var(--step--1);color:var(--accent);margin:var(--sp-1) 0 var(--sp-2);font-weight:650}
.interaccion p{margin:0 0 var(--sp-2);font-size:var(--step--1);line-height:1.6;color:var(--gray-200)}
.interaccion .lista{margin-bottom:var(--sp-1)}.interaccion-productos b{color:var(--purple)}
.interaccion.cameo{border-left-color:var(--purple)}
.citas{margin-top:var(--sp-2);font-size:var(--step--1);color:var(--muted)}
.citas summary{cursor:pointer;font-weight:650}
.norma-acciones{display:flex;gap:var(--sp-2);flex-wrap:wrap}
.documentos-acciones{display:flex;gap:var(--sp-2);flex-wrap:wrap;margin-top:var(--sp-3)}
.documentos-acciones .boton{text-decoration:none}

/* --- Opciones nativas --- */
.offline-panel{max-width:1180px;margin:var(--sp-5) auto;width:calc(100% - 40px)}
.opciones{display:flex;flex-direction:column;gap:var(--sp-3);margin:var(--sp-4) 0}
.opcion{display:flex;align-items:center;gap:var(--sp-2);font-size:var(--step-0);color:var(--gray-200);flex-wrap:wrap}
.opcion input[type=checkbox]{width:18px;height:18px;accent-color:var(--green-600)}
.opcion input[type=text],.opcion input[type=password],.opcion select{flex:1;min-width:220px;
  background:var(--input-bg);color:var(--ink);border:1px solid var(--line);
  border-radius:var(--r-sm);padding:8px 11px;font-size:var(--step-0);font-family:inherit}
.estado-descarga{font-size:var(--step--1);color:var(--ok);margin:var(--sp-1) 0}
.doc-guardado{display:flex;justify-content:space-between;align-items:center;gap:var(--sp-3);
  border:1px solid var(--line);border-radius:var(--r-sm);background:var(--surface-2);
  padding:var(--sp-3);margin-bottom:var(--sp-2)}
.doc-guardado strong{font-size:var(--step-0);color:var(--ink-strong);display:block}

/* --- Robustez ante contenido extremo (texto largo, zoom, pantallas estrechas) --- */
.norma,.norma-meta,.panel-cabecera,.norma h4,.banner-texto,.resultados-online,
.online-resultado,.interaccion,.doc-guardado{min-inline-size:0;overflow-wrap:anywhere}
.badge{max-inline-size:100%;overflow-wrap:anywhere;white-space:normal}
.norma h4{overflow-wrap:anywhere;word-break:break-word}
.panel-legal summary{flex-wrap:wrap;min-inline-size:0}
.panel-legal summary .badge{margin-inline-start:auto}
.cabecera{flex-wrap:wrap}
.marca{min-inline-size:0}
.marca strong,.marca small{overflow-wrap:anywhere}
.leyenda-grupo{min-inline-size:0;overflow-wrap:anywhere}

@media(max-width:360px){
  .cabecera{padding:var(--sp-2) var(--sp-3)}
  .banner-pais-grande{width:42px;height:42px}
  .flag-lg{width:40px;height:27px}
  .banner{padding:var(--sp-3);gap:var(--sp-2)}
  .contenido{margin-inline:var(--sp-2)}
  .panel,.offline-panel{padding:var(--sp-3)}
  .online-resultado,.norma{padding:var(--sp-2)}
  .ficha-titulo h4{font-size:var(--step-2)}
}

/* --- Preferencias de movimiento --- */
@media (prefers-reduced-motion:reduce){
  html{scroll-behavior:auto}
  *,*::before,*::after{animation-duration:.01ms!important;transition-duration:.01ms!important}
}

/* --- Impresión: solo la ficha --- */
@media print{body{background:#fff!important;color:#000!important}
  .cabecera,.banner,#panel-legal,.multi,.pie,.flotante,.fds-index,.documentos,.overlay,.skip-link{display:none!important}
  .contenido{display:block;margin:0;padding:0}#panel-sustancia{border:0;box-shadow:none;padding:0}
  .seccion{break-inside:avoid}.seccion h5{color:#0550ae}.prop,.interaccion{background:#fff;border-color:#ccc}}

/* --- Tablet --- */
@media(max-width:900px){
  .cabecera{padding:var(--sp-3) var(--sp-4)}
  .banner{margin:var(--sp-3) var(--sp-3) 0;padding:var(--sp-4);gap:var(--sp-3);flex-wrap:wrap}
  .banner-pais-grande{width:50px;height:50px}.banner-texto h2{font-size:var(--step-2)}
  .banner-texto p{font-size:var(--step-0)}.banner-pais{width:100%;text-align:center}
  .contenido{margin:var(--sp-3) var(--sp-3) 110px;padding:0;gap:var(--sp-3)}
  .panel{padding:var(--sp-4)}
  .prop-grid{grid-template-columns:1fr}.legal-lista{max-height:none}
  .buscador{padding:14px}.buscador input,.multi input{font-size:16px}
  .resultado,.chip,.boton{padding:11px 14px;font-size:var(--step-0);min-height:44px}
  .flotante{bottom:76px;right:var(--sp-3)}
  .pie{padding-bottom:110px}
}
@media(max-width:480px){
  .overlay-card{padding:var(--sp-5) var(--sp-4)}
  .paises-carpeta{grid-template-columns:repeat(auto-fit,minmax(min(150px,100%),1fr))}
  .pais-boton{flex-direction:column;align-items:flex-start;gap:var(--sp-2)}
  .fds-index a{font-size:var(--step--1)}
  .online-resultado{flex-direction:column;align-items:stretch}
  .online-resultado .boton{width:100%}
}
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
        "sun" => ("0 0 24 24", r#"<circle cx="12" cy="12" r="4"/><path d="M12 2v2M12 20v2M4.9 4.9l1.4 1.4M17.7 17.7l1.4 1.4M2 12h2M20 12h2M4.9 19.1l1.4-1.4M17.7 6.3l1.4-1.4"/>"#),
        "moon" => ("0 0 24 24", r#"<path d="M21 12.8A9 9 0 1 1 11.2 3a7 7 0 0 0 9.8 9.8z"/>"#),
        "clock" => ("0 0 24 24", r#"<circle cx="12" cy="12" r="9"/><path d="M12 7v5l3 2"/>"#),
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
    let opciones = sustancias_storage::Options::from(&opciones);
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


/// Tipos neutrales (existen en web y nativo) para no condicionar el RSX.
#[derive(Clone, Debug)]
struct AppOptions {
    offline_cache: bool,
    copy_to_downloads: bool,
    downloads_dir: String,
    auto_report: bool,
    deepl_enabled: bool,
    deepl_key: String,
    deepl_pro: bool,
    cas_api_key: String,
}

impl Default for AppOptions {
    fn default() -> Self {
        Self {
            offline_cache: true,
            copy_to_downloads: true,
            downloads_dir: String::new(),
            auto_report: false,
            deepl_enabled: false,
            deepl_key: String::new(),
            deepl_pro: false,
            cas_api_key: String::new(),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<&AppOptions> for sustancias_storage::Options {
    fn from(o: &AppOptions) -> Self {
        Self {
            offline_cache: o.offline_cache,
            copy_to_downloads: o.copy_to_downloads,
            downloads_dir: if o.downloads_dir.trim().is_empty() {
                None
            } else {
                Some(std::path::PathBuf::from(&o.downloads_dir))
            },
            auto_report: o.auto_report,
            deepl_enabled: o.deepl_enabled,
            deepl_key: o.deepl_key.clone(),
            deepl_pro: o.deepl_pro,
            cas_api_key: o.cas_api_key.clone(),
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
        deepl_enabled: o.deepl_enabled,
        deepl_key: o.deepl_key,
        deepl_pro: o.deepl_pro,
        cas_api_key: o.cas_api_key,
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
    let opciones = sustancias_storage::Options::from(o);
    let _ = sustancias_storage::save_options(&opciones);
}

#[cfg(target_arch = "wasm32")]
fn guardar_opciones(_o: &AppOptions) {}

/// Tipo de documento para la descarga compartida.
#[allow(dead_code)]
#[derive(Clone, Copy)]
enum DocKind {
    Substance,
    Legislation,
}

/// Descarga un documento con el mismo efecto en web y en nativo.
#[allow(unused_mut)]
fn descargar_documento(
    url: String,
    nombre: String,
    titulo: String,
    kind: DocKind,
    opciones: AppOptions,
    mut guardados: Signal<Vec<SavedDocView>>,
    mut estado: Signal<String>,
) {
    #[cfg(target_arch = "wasm32")]
    {
        dioxus::document::eval(&js_descargar(&url, &nombre));
        let _ = (titulo, kind, opciones, guardados, estado);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let opciones = opciones.clone();
        let auto = opciones.auto_report;
        let doc_kind = match kind {
            DocKind::Substance => sustancias_storage::DocumentKind::Substance,
            DocKind::Legislation => sustancias_storage::DocumentKind::Legislation,
        };
        spawn(async move {
            match descargar_nativo(url, titulo, nombre, doc_kind, opciones).await {
                Ok(_) => {
                    guardados.set(cargar_guardados());
                    estado.set("Guardado sin conexión y copiado a Descargas".to_string());
                }
                Err(e) => {
                    if auto {
                        abrir_issue("Error al descargar documento", &e);
                    }
                    estado.set(e);
                }
            }
        });
    }
}

/// Construye la URL de un issue de GitHub ya redactado.
fn url_issue(titulo: &str, cuerpo: &str) -> String {
    format!(
        "{}/issues/new?title={}&body={}",
        GITHUB_URL,
        urlencoding::encode(titulo),
        urlencoding::encode(cuerpo)
    )
}

/// Abre el issue con el reporte ya redactado, en web o en nativo.
fn abrir_issue(titulo: &str, cuerpo: &str) {
    let url = url_issue(titulo, cuerpo);
    #[cfg(target_arch = "wasm32")]
    dioxus::document::eval(&format!("window.open({:?}, '_blank');", url));
    #[cfg(not(target_arch = "wasm32"))]
    let _ = webbrowser::open(&url);
}

/// Análisis CAMEO ya traducido al idioma activo.
#[derive(Clone)]
struct CameoVisible {
    na: String,
    nb: String,
    etiqueta: String,
    compatibilidad: String,
    nivel: String,
    peligros: Vec<String>,
    gases: Vec<String>,
    citas: Vec<String>,
    fuentes: Vec<String>,
    revisado: bool,
}

/// Terna agregada, lista para RSX (nombres ya localizados).
#[derive(Clone)]
struct TripleVisible {
    nombres: Vec<String>,
    reglas: Vec<core::TernaryRule>,
    nivel: String,
    etiqueta: String,
    compatibilidad: String,
    /// Texto compacto "A + B: INCOMPATIBLE" para cada par de la terna.
    parejas: Vec<(String, String)>,
    puente: Option<String>,
    fuentes: Vec<core::Fuente>,
    nota: String,
    cobertura_limitada: bool,
    revisado: bool,
}

/// Solo ordena la presentación; no es una valoración científica nueva.
fn triple_prioridad(t: &core::TripleAnalysis) -> u8 {
    let nivel = match t.nivel.as_str() {
        "alto" => 3,
        "moderado" => 2,
        _ => 1,
    };
    let comp = match t.compatibilidad.as_str() {
        "incompatible" => 3,
        "precaucion" => 2,
        _ => 1,
    };
    nivel * 4
        + comp
        + if t.puente.is_some() { 2 } else { 0 }
        + if t.reglas.is_empty() { 0 } else { 4 }
}

fn combinaciones3(n: usize) -> usize {
    n.saturating_mul(n.saturating_sub(1))
        .saturating_mul(n.saturating_sub(2))
        / 6
}

/// Traduce texto DINÁMICO que llega en inglés (CAMEO/PubChem).
/// `lang == "en"` es un no-op deliberado: la entrada ya está en inglés.
/// Los textos FIJOS usan `sustancias_i18n::texto_fijo` (catálogo offline).
async fn traducir_texto(traductor: &sustancias_i18n::Cadena, lang: &str, texto: String) -> String {
    if lang == "en" {
        return texto;
    }
    traductor
        .translate(&texto, lang)
        .await
        .map(|r| r.texto)
        .unwrap_or(texto)
}

/// Lee de forma síncrona si el usuario cerró ya el aviso de la estrella.
fn estrella_cerrada_guardada() -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window()
            .and_then(|w| w.local_storage().ok().flatten())
            .and_then(|s| s.get_item("estrella-cerrada").ok().flatten())
            .as_deref()
            == Some("1")
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        false
    }
}

/// Cierra el aviso de la estrella y recuerda la preferencia.
fn cerrar_estrella() {
    dioxus::document::eval("try{localStorage.setItem('estrella-cerrada','1')}catch(e){}");
}

/// Nombre a mostrar: sinónimo localizado, traducción automática o título.
fn nombre_visible(c: &core::PubChemCompound, lang: &str) -> String {
    let nombre = core::localized_name(&c.synonyms, lang)
        .or_else(|| {
            if c.nombre_localizado.is_empty() {
                None
            } else {
                Some(c.nombre_localizado.clone())
            }
        })
        .unwrap_or_else(|| c.title.clone());
    if lang == "es" {
        core::corregir_tildes(&nombre)
    } else {
        nombre
    }
}

/// Espera asíncrona portable (WASM y nativo) para el *debounce* de búsqueda.
async fn dormir(ms: u64) {
    #[cfg(target_arch = "wasm32")]
    gloo_timers::future::TimeoutFuture::new(ms as u32).await;
    #[cfg(not(target_arch = "wasm32"))]
    tokio::time::sleep(std::time::Duration::from_millis(ms)).await;
}

/// Consulta CAS Common Chemistry para resolver el CAS de un nombre.
/// Requiere una clave API gratuita solicitada en commonchemistry.cas.org.
async fn cas_from_common_chemistry(nombre: &str, clave: &str) -> Option<String> {
    if clave.trim().is_empty() || nombre.trim().is_empty() {
        return None;
    }
    let url = format!(
        "https://commonchemistry.cas.org/api/search?q={}",
        urlencoding::encode(nombre.trim())
    );
    let valor = sustancias_i18n::HttpClient::default()
        .get_json_with_header(&url, "X-Api-Key", clave.trim())
        .await
        .ok()?;
    valor
        .get("results")?
        .get(0)?
        .get("rn")?
        .as_str()
        .map(str::to_string)
}

/// Hora local del dispositivo (zona horaria del usuario).
fn hora_local() -> u32 {
    #[cfg(target_arch = "wasm32")]
    {
        js_sys::Date::new_0().get_hours()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        chrono::Timelike::hour(&chrono::Local::now())
    }
}

/// Resuelve el tema efectivo: la preferencia manual gana; «auto» usa la hora.
fn tema_efectivo(preferencia: &str) -> &'static str {
    match preferencia {
        "light" => "light",
        "dark" => "dark",
        _ => core::tema_por_hora(hora_local()),
    }
}

/// Siluetas de los continentes en negro (derivadas de un mapa GeoJSON público).
fn continent_svg(name: &str, class: &str) -> String {
    let d = match name {
        "{txt(\"continent_international\", lang_ui)}" => r##"M12,2 A10,10 0 1,0 12,22 A10,10 0 1,0 12,2 Z M2,12 L22,12 M12,2 C7,7 7,17 12,22 M12,2 C17,7 17,17 12,22"##,
        "{txt(\"continent_america\", lang_ui)}" => r##"M18.8,4.9 L18.9,5.1 L18.4,5.5 L18.5,5.6 L17.8,5.5 L17.3,5.0 L17.6,4.9 L17.3,5.0 L17.5,4.8 L17.1,4.7 L17.6,4.6 L17.0,4.6 L17.6,4.5 L17.1,4.5 L17.6,4.5 L17.1,4.4 L17.5,4.2 L16.9,4.1 L17.5,4.1 L17.2,4.0 L17.3,3.9 L16.8,4.0 L17.0,3.8 L16.4,3.4 L15.0,3.3 L14.6,3.2 L15.3,3.1 L14.3,3.0 L16.1,2.5 L19.0,2.4 L18.0,2.4 L18.5,2.4 L18.2,2.3 L21.6,2.4 L20.0,2.5 L21.6,2.5 L21.2,2.7 L23.0,2.6 L21.7,2.7 L22.4,2.7 L21.8,2.8 L21.9,2.9 L21.5,3.1 L22.0,3.2 L21.4,3.2 L21.9,3.5 L20.7,3.7 L21.6,4.1 L20.6,3.9 L21.0,4.0 L20.5,4.1 L21.5,4.2 L20.0,4.4 L19.7,4.7 L18.8,4.9 Z M6.3,6.2 L6.0,6.2 L5.6,5.7 L5.4,5.8 L3.9,5.4 L3.3,5.7 L3.6,5.4 L1.7,6.3 L2.6,5.7 L1.8,5.8 L1.9,5.5 L1.6,5.6 L1.3,5.4 L2.0,4.9 L1.0,4.8 L2.1,4.7 L1.2,4.4 L2.6,4.0 L5.6,4.4 L6.6,4.1 L9.5,4.7 L9.5,4.4 L9.7,4.4 L10.9,4.4 L11.1,4.7 L11.5,4.4 L11.0,4.1 L11.3,3.9 L12.3,4.6 L12.7,4.3 L12.6,4.2 L13.2,4.3 L13.0,4.4 L13.2,4.6 L12.6,4.6 L12.8,4.7 L11.8,4.7 L12.4,4.9 L11.4,5.0 L11.9,5.2 L11.5,5.2 L11.3,5.7 L11.6,6.0 L13.1,6.3 L13.2,6.8 L13.5,6.8 L13.4,6.3 L13.9,6.1 L13.6,5.8 L13.7,5.2 L14.8,5.4 L14.6,5.6 L15.0,5.8 L14.9,5.9 L15.5,5.5 L16.1,6.3 L16.5,6.3 L16.1,6.5 L16.8,6.5 L16.2,6.9 L14.6,7.2 L14.8,7.2 L14.2,7.6 L15.4,7.1 L15.6,7.2 L15.2,7.3 L16.0,7.6 L15.4,7.9 L15.7,7.6 L15.2,7.7 L14.7,7.9 L14.8,8.2 L14.2,8.2 L13.9,8.8 L14.0,8.4 L13.8,8.5 L14.0,9.0 L13.2,9.6 L13.3,10.5 L13.0,9.9 L12.5,9.7 L11.9,9.7 L12.0,9.9 L11.3,9.8 L10.9,10.1 L11.1,11.4 L11.8,11.4 L11.9,11.0 L12.4,11.0 L12.1,11.8 L12.9,11.9 L12.8,12.4 L13.1,12.7 L13.4,12.8 L13.2,13.0 L12.6,12.6 L12.3,12.1 L10.1,11.4 L8.5,9.5 L9.2,10.7 L8.9,10.6 L8.1,9.3 L7.2,8.5 L7.3,7.5 L7.1,7.2 L7.4,7.3 L7.1,6.9 L6.7,6.8 L6.8,6.7 L6.3,6.4 L6.3,6.2 Z M13.7,12.9 L13.8,12.9 L14.5,12.3 L14.5,12.7 L14.8,12.3 L15.0,12.5 L15.9,12.5 L15.8,12.6 L16.4,13.1 L17.0,13.2 L16.9,13.7 L17.4,13.4 L17.6,13.8 L17.2,14.2 L17.5,14.1 L17.6,14.4 L18.0,14.1 L18.3,14.2 L18.3,14.5 L19.0,14.4 L19.7,14.8 L18.8,17.1 L17.8,17.6 L17.8,18.0 L17.0,18.9 L16.5,18.8 L16.6,19.2 L15.8,19.4 L15.8,19.7 L15.5,19.7 L15.7,20.0 L15.1,20.5 L15.4,20.7 L14.9,21.0 L15.0,21.4 L14.9,21.4 L15.5,21.7 L14.5,21.6 L14.9,21.6 L14.7,21.5 L14.8,21.4 L14.3,21.5 L14.6,21.4 L14.3,21.4 L14.4,21.3 L14.1,21.0 L14.3,20.7 L14.0,20.6 L14.4,20.2 L14.3,19.2 L14.8,17.0 L14.7,16.6 L13.9,16.1 L13.2,14.8 L13.4,14.4 L13.2,14.1 L13.8,13.5 L13.7,12.9 Z M17.2,7.3 L16.3,7.3 L16.8,6.8 L16.6,7.0 L17.1,7.1 L17.2,7.3 Z M12.7,4.0 L12.6,3.8 L12.8,3.8 L12.6,3.7 L13.2,3.7 L13.2,3.9 L14.0,3.8 L13.9,3.9 L15.0,4.1 L14.7,4.2 L15.1,4.1 L14.8,4.2 L16.0,4.6 L15.7,4.9 L15.4,4.8 L15.6,4.7 L14.9,4.7 L15.5,5.0 L15.5,5.2 L14.9,5.1 L15.3,5.3 L13.6,4.9 L14.4,4.6 L13.5,4.1 L12.6,4.2 L11.9,3.9 L12.7,3.6 L12.4,3.8 L12.7,4.0 Z M15.9,2.5 L13.6,2.9 L14.1,2.9 L13.5,2.9 L14.1,3.0 L13.1,3.1 L13.7,3.2 L13.2,3.3 L12.0,3.3 L13.0,3.0 L12.4,3.0 L13.2,2.9 L12.5,2.7 L13.3,2.8 L12.9,2.7 L13.8,2.6 L11.7,2.5 L15.9,2.5 Z M13.3,3.5 L11.8,3.5 L11.0,3.2 L13.3,3.5 Z M11.1,2.8 L11.5,2.6 L12.7,2.9 L11.7,3.0 L11.4,2.9 L11.9,2.9 L11.1,2.8 Z M11.1,3.8 L10.7,4.0 L10.2,3.8 L10.9,3.6 L11.1,3.8 Z M10.4,4.2 L8.7,4.4 L8.1,4.2 L9.0,4.1 L7.9,3.9 L8.5,3.7 L9.5,3.9 L9.6,3.7 L10.4,4.2 Z M8.4,3.7 L7.4,4.0 L6.9,3.9 L7.1,3.6 L8.4,3.7 Z"##,
        "{txt(\"continent_europe\", lang_ui)}" => r##"M11.5,13.3 L11.4,12.9 L11.9,12.9 L11.9,12.7 L11.7,12.6 L12.6,12.3 L12.5,12.1 L12.6,12.1 L12.5,12.1 L12.7,12.0 L12.6,12.2 L12.9,12.3 L13.3,12.2 L13.3,12.0 L13.5,12.0 L13.4,11.9 L13.8,11.9 L13.3,11.8 L13.3,11.7 L13.6,11.6 L13.4,11.5 L13.1,11.7 L13.2,11.9 L13.0,12.0 L13.0,12.1 L12.8,12.2 L12.7,11.9 L12.5,12.0 L12.3,12.0 L12.4,11.8 L12.3,11.8 L12.5,11.8 L12.3,11.7 L12.7,11.7 L12.6,11.6 L13.1,11.3 L13.5,11.2 L14.5,11.4 L14.4,11.5 L13.9,11.4 L14.3,11.6 L14.2,11.6 L14.7,11.5 L14.6,11.3 L14.8,11.4 L14.7,11.4 L14.8,11.5 L15.3,11.3 L15.6,11.4 L15.7,11.3 L16.2,11.4 L16.1,11.2 L16.4,11.1 L16.5,11.4 L16.2,11.5 L16.4,11.5 L16.6,11.3 L16.8,11.4 L16.5,11.3 L16.5,11.2 L16.6,11.1 L16.7,11.2 L16.8,11.2 L16.6,11.1 L17.1,11.2 L17.1,11.3 L17.1,11.2 L16.9,11.0 L17.3,11.1 L17.4,11.0 L17.2,11.0 L17.3,11.0 L18.1,10.9 L18.4,10.8 L19.0,10.9 L18.4,11.1 L19.6,11.0 L19.9,11.1 L19.8,11.1 L20.0,11.2 L20.6,11.2 L20.6,11.1 L21.7,11.2 L21.8,11.4 L22.4,11.3 L22.4,11.3 L23.0,11.3 L23.0,11.6 L22.7,11.6 L22.8,11.6 L23.0,11.7 L22.0,11.9 L21.9,12.0 L22.0,12.1 L21.6,12.4 L21.6,12.0 L22.1,11.7 L21.8,11.8 L21.8,11.8 L21.6,11.8 L21.5,11.9 L20.6,12.0 L20.3,12.2 L20.6,12.3 L20.6,12.6 L20.1,12.9 L20.0,12.9 L20.2,12.6 L20.0,12.6 L19.6,12.3 L19.3,12.3 L19.2,12.5 L18.6,12.5 L18.0,12.4 L17.9,12.5 L17.6,12.4 L17.3,12.5 L16.9,12.4 L16.7,12.2 L16.5,12.3 L16.2,12.2 L15.7,12.2 L15.8,12.3 L15.7,12.4 L15.8,12.4 L15.1,12.4 L14.9,12.5 L14.8,12.6 L15.0,12.7 L14.9,12.8 L14.9,13.0 L14.2,12.8 L14.4,12.7 L14.2,12.7 L14.2,12.8 L14.1,12.8 L13.9,12.7 L13.7,13.0 L13.4,13.1 L13.4,13.3 L13.2,13.0 L12.8,12.7 L12.8,12.8 L13.1,13.1 L13.0,13.1 L13.0,13.2 L13.0,13.1 L12.5,12.8 L12.2,12.9 L11.9,13.3 L11.5,13.3 Z M11.2,11.6 L10.9,11.7 L10.5,11.5 L11.2,11.6 Z M1.0,11.3 L1.6,11.5 L1.4,11.6 L1.1,11.5 L1.0,11.6 L1.0,11.3 Z M11.8,12.4 L11.7,12.4 L11.8,12.3 L11.7,12.1 L11.6,12.2 L11.7,12.0 L11.9,12.0 L11.8,12.1 L12.1,12.3 L12.1,12.4 L11.7,12.5 L11.8,12.4 Z M13.0,10.9 L12.9,10.8 L13.0,10.8 L12.8,10.8 L13.1,10.8 L12.7,10.7 L13.3,10.7 L13.0,10.9 Z M20.7,12.2 L20.8,12.6 L20.8,12.5 L20.8,12.7 L20.7,12.7 L20.7,12.2 Z"##,
        "{txt(\"continent_africa\", lang_ui)}" => r##"M1.5,7.9 L2.0,6.9 L1.7,5.7 L2.4,4.4 L5.0,1.5 L9.8,1.0 L10.2,1.1 L10.0,2.1 L12.6,3.2 L13.5,2.3 L17.3,2.8 L17.3,3.9 L16.7,3.4 L18.9,7.8 L20.0,8.6 L20.1,9.5 L21.5,9.9 L21.8,9.0 L22.5,8.8 L22.5,9.2 L18.8,13.8 L19.3,16.9 L17.4,18.4 L17.6,19.8 L15.3,22.5 L12.9,23.0 L12.4,22.8 L10.4,17.9 L11.1,15.7 L10.9,14.2 L9.5,12.6 L9.8,11.1 L8.6,11.1 L8.0,10.4 L4.5,11.1 L1.5,7.9 Z M20.4,17.3 L21.5,16.9 L21.9,16.0 L22.2,17.1 L20.7,20.2 L20.0,19.2 L20.4,17.3 Z"##,
        "{txt(\"continent_asia\", lang_ui)}" => r##"M23.0,8.3 L21.9,8.5 L22.3,7.9 L23.0,8.3 Z M20.3,10.0 L21.7,9.4 L22.2,8.6 L22.0,9.8 L20.3,10.0 Z M22.1,17.9 L21.6,17.3 L20.8,16.8 L20.6,17.0 L20.5,16.8 L20.8,16.7 L20.3,16.5 L20.8,16.4 L21.1,16.9 L21.6,16.5 L22.1,16.7 L22.1,17.9 Z M4.2,13.9 L4.0,13.1 L2.5,11.1 L2.8,9.5 L1.2,9.5 L1.0,8.9 L2.3,8.5 L3.6,8.7 L3.9,8.6 L3.7,8.2 L5.1,8.6 L5.5,8.9 L5.2,9.3 L6.1,9.5 L5.9,8.8 L6.3,8.7 L5.4,8.1 L6.0,7.7 L5.1,7.7 L4.8,7.1 L5.5,6.7 L7.5,6.9 L7.2,6.7 L7.6,6.5 L7.4,6.3 L8.9,6.1 L9.7,6.4 L10.3,6.2 L10.9,6.9 L12.3,7.2 L13.2,6.9 L14.1,7.1 L14.4,6.7 L16.2,7.2 L17.9,7.1 L18.3,6.6 L18.9,6.4 L20.3,7.5 L21.0,7.4 L19.6,9.0 L19.9,9.8 L19.5,10.0 L19.5,9.3 L19.1,9.2 L19.1,8.9 L18.5,9.1 L18.6,8.7 L17.8,9.1 L18.1,9.4 L18.8,9.4 L18.1,9.8 L18.6,10.4 L18.2,10.3 L18.6,10.6 L18.3,10.7 L18.6,10.9 L17.6,12.0 L17.1,12.0 L16.5,12.5 L16.3,12.3 L15.8,12.4 L15.6,12.8 L16.2,13.5 L16.2,14.2 L15.5,14.7 L15.5,14.4 L14.6,13.8 L14.4,14.6 L15.4,16.0 L14.8,15.7 L14.3,14.7 L14.2,13.2 L13.5,13.3 L13.5,12.7 L12.9,11.9 L11.0,13.4 L10.8,14.6 L10.4,14.8 L9.7,13.3 L9.6,12.2 L9.2,12.5 L8.9,12.2 L9.2,12.0 L8.4,11.5 L6.1,11.3 L5.2,10.6 L5.0,10.9 L5.6,11.8 L6.6,11.4 L7.2,12.1 L6.8,12.8 L4.2,13.9 Z M17.8,15.5 L18.1,16.1 L17.5,16.9 L16.5,16.8 L16.3,15.9 L16.7,16.0 L17.7,15.0 L18.1,15.3 L17.8,15.5 Z M15.7,16.8 L15.6,17.3 L15.0,17.0 L13.7,15.2 L15.7,16.8 Z"##,
        "{txt(\"continent_oceania\", lang_ui)}" => r##"M21.3,20.0 L20.1,19.8 L22.5,17.8 L23.0,18.1 L21.3,20.0 Z M2.5,15.8 L1.4,15.6 L1.0,11.0 L3.6,10.1 L5.4,8.1 L6.7,8.6 L7.6,7.1 L9.2,7.4 L8.8,8.5 L10.7,9.5 L11.4,6.9 L12.8,9.9 L15.3,12.5 L15.3,14.4 L14.1,16.7 L12.8,17.3 L11.0,17.0 L9.8,15.5 L9.3,15.9 L9.7,14.9 L9.0,15.8 L8.2,14.7 L2.5,15.8 Z M10.9,4.0 L13.2,5.2 L14.4,6.9 L12.1,5.8 L10.8,6.4 L10.9,4.0 Z"##,
        _ => "",
    };
    format!("<svg class=\"{}\" viewBox=\"0 0 24 24\" fill='black'><path d=\"{}\"/></svg>", class, d)
}

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
        _ => {
            if let Some(uri) = flags_extra::flag_img(id) {
                return format!("<img class=\"{}\" src=\"{}\" alt=\"\">", class, uri);
            }
            r##"<rect width="30" height="20" rx="3" fill="#0d419d"/><circle cx="15" cy="10" r="6" fill="none" stroke="#fff" stroke-width="1"/><path d="M9 10h12M15 4v12" stroke="#fff" stroke-width="1"/>"##
        }
    };
    format!("<svg class=\"{}\" viewBox=\"0 0 30 20\">{}</svg>", class, body)
}

fn txt(key: &str, lang: &str) -> &'static str {
    sustancias_i18n::texto_fijo(key, lang)
}

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
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
    let mut multi_ids: Signal<Vec<(String, String, String)>> = use_signal(Vec::new);
    let mut norma: Signal<Option<(String, String, String, String, String)>> = use_signal(|| None);
    let mut online_resultados: Signal<Vec<core::PubChemCompound>> = use_signal(Vec::new);
    let mut online_ficha: Signal<Option<core::Substance>> = use_signal(|| None);
    let mut online_cargando: Signal<bool> = use_signal(|| false);
    let mut online_error_red: Signal<bool> = use_signal(|| false);
    let mut busqueda_id: Signal<u64> = use_signal(|| 0);
    let mut boe_resultados: Signal<Vec<core::BoeNorm>> = use_signal(Vec::new);
    let mut boe_cargando: Signal<bool> = use_signal(|| false);
    let mut boe_termino: Signal<String> = use_signal(String::new);
    let mut output_lang: Signal<String> = use_signal(|| "es".to_string());
    let mut theme: Signal<String> = use_signal(|| "auto".to_string());
    let mut tema_actual: Signal<String> = use_signal(|| core::tema_por_hora(hora_local()).to_string());
    let mut feedback_tipo: Signal<String> = use_signal(|| "Sugerencia".to_string());
    let mut feedback_titulo: Signal<String> = use_signal(String::new);
    let mut feedback_texto: Signal<String> = use_signal(String::new);
    let mut busquedas: Signal<u32> = use_signal(|| 0);
    let mut mostrar_estrella: Signal<bool> = use_signal(|| false);
    let mut cameo_visible: Signal<Vec<CameoVisible>> = use_signal(Vec::new);
    let mut triple_visible: Signal<Vec<TripleVisible>> = use_signal(Vec::new);
    let mut estrella_cerrada: Signal<bool> = use_signal(estrella_cerrada_guardada);
    let mut opciones: Signal<AppOptions> = use_signal(cargar_opciones);
    let mut guardados: Signal<Vec<SavedDocView>> = use_signal(cargar_guardados);
    #[allow(unused_mut)]
    let mut estado_descarga: Signal<String> = use_signal(String::new);

    let country = selected_id()
        .and_then(|id| core::country_by_id(&id))
        .unwrap_or_else(|| countries[0].clone());
    let lang = if country.idioma == "en" { "en" } else { "es" };
    let lang_ui = sustancias_i18n::idioma_efectivo(&output_lang(), lang);
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
    // Organizaciones regionales del país: solo se incluyen si tienen normativa
    // oficial verificada (p. ej. UE o Comunidad Andina).
    let org_blocks: Vec<(core::Organization, Vec<core::Source>)> = core::organizations_for(&country.codigo)
        .into_iter()
        .map(|o| {
            let normas = core::organization_sources(&o.fuente_pais)
                .into_iter()
                .filter(|s| filter() == "todos" || s.tema == filter())
                .collect();
            (o, normas)
        })
        .collect();
    let org_count: usize = org_blocks.iter().map(|(_, normas)| normas.len()).sum();
    let visible_instruments: Vec<&core::Instrument> = instruments
        .iter()
        .filter(|i| filter() == "todos" || i.tema == filter())
        .collect();
    let n_normas = (visible_instruments.len() + international_sources.len() + org_count).to_string();
    let multi_items = multi_ids();

    // Límites de exposición de NIOSH para la sustancia mostrada.
    let limites = online_ficha()
        .or_else(|| active_substance.clone())
        .and_then(|s| core::niosh_limit_for_cas(&s.cas).cloned());

    // Perfiles de reactividad de PubChem para sustancias sin cobertura CAMEO.
    let perfiles: Vec<(String, &'static core::PubChemReactivity)> = multi_items
        .iter()
        .filter_map(|(id, nombre, _)| {
            let cid = id.strip_prefix("online-")?.parse::<u64>().ok()?;
            core::pubchem_reactivity_for_cid(cid).map(|p| (nombre.clone(), p))
        })
        .collect();

    let mut buscar_pubchem = move |texto: String, lang: String| {
        if texto.trim().len() < 2 || !core::es_consulta_valida(&texto) {
            online_resultados.set(Vec::new());
            online_error_red.set(false);
            online_cargando.set(false);
            return;
        }
        busquedas += 1;
        if busquedas() >= 10 && !estrella_cerrada() {
            mostrar_estrella.set(true);
        }
        let id = {
            busqueda_id += 1;
            busqueda_id()
        };
        online_cargando.set(true);
        online_error_red.set(true);
        spawn(async move {
            // Debounce: escribir rápido no lanza una avalancha de peticiones.
            dormir(320).await;
            if id != busqueda_id() {
                return;
            }
            let base = "https://pubchem.ncbi.nlm.nih.gov/rest/pug/compound";
            fn cids_de(v: serde_json::Value) -> Vec<u64> {
                serde_json::from_value(
                    v.get("IdentifierList")
                        .and_then(|x| x.get("CID"))
                        .cloned()
                        .unwrap_or_default(),
                )
                .unwrap_or_default()
            }

            // Traductor compartido (web + nativo): DeepL solo si el usuario
            // configuró su key; después Google; después diccionario local.
            let op = opciones();
            let traductor = sustancias_i18n::Cadena::con_configuracion(
                if op.deepl_enabled && !op.deepl_key.trim().is_empty() {
                    Some(op.deepl_key.clone())
                } else {
                    None
                },
                if op.deepl_pro {
                    sustancias_i18n::DeeplApi::Pro
                } else {
                    sustancias_i18n::DeeplApi::Free
                },
            );

            // La capa `query` decide si la consulta es CAS, fórmula, código de
            // producto o nombre; los nombres se traducen al inglés.
            let clase = core::classify(&texto);
            let alias = core::alias_for(&texto);
            let mut cids: Vec<u64> = alias.map(|a| a.cids.clone()).unwrap_or_default();
            let mut nota = alias.map(|a| a.nota.clone()).unwrap_or_default();

            if cids.is_empty() {
                match &clase {
                    core::QueryKind::Cas(cas) => {
                        for modo in ["?language=all", "?name_type=word"] {
                            let url = format!(
                                "{}/name/{}/cids/JSON{}",
                                base,
                                urlencoding::encode(cas),
                                modo
                            );
                            if let Ok(v) = http_get_json(&url).await {
                                online_error_red.set(false);
                                cids = cids_de(v);
                                if !cids.is_empty() {
                                    break;
                                }
                            }
                        }
                        if !cids.is_empty() {
                            nota = format!("Búsqueda por número CAS {cas}.");
                        }
                    }
                    core::QueryKind::Formula(formula) => {
                        let url = format!(
                            "{}/fastformula/{}/cids/JSON",
                            base,
                            urlencoding::encode(formula)
                        );
                        if let Ok(v) = http_get_json(&url).await {
                                online_error_red.set(false);
                            cids = cids_de(v);
                        }
                        if !cids.is_empty() {
                            nota = format!(
                                "Búsqueda por fórmula molecular {formula}: se muestran los primeros compuestos con esa fórmula."
                            );
                        }
                    }
                    _ => {
                        let mut variantes = core::name_variants(&texto);
                        if let Some(traduccion) = traductor.translate(&texto, "en").await {
                            variantes.insert(0, traduccion.texto);
                        }
                        variantes.dedup();
                        for variante in variantes {
                            for modo in ["?language=all", "?name_type=word"] {
                                let url = format!(
                                    "{}/name/{}/cids/JSON{}",
                                    base,
                                    urlencoding::encode(&variante),
                                    modo
                                );
                                if let Ok(v) = http_get_json(&url).await {
                                online_error_red.set(false);
                                    cids = cids_de(v);
                                    if !cids.is_empty() {
                                        break;
                                    }
                                }
                            }
                            if !cids.is_empty() {
                                break;
                            }
                        }
                    }
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
                    c.cas = core::extraer_cas(&sinonimos).unwrap_or_default();
                    if c.cas.is_empty() {
                        let clave = opciones().cas_api_key.clone();
                        if !clave.trim().is_empty() {
                            if let Some(rn) = cas_from_common_chemistry(&c.title, &clave).await {
                                c.cas = rn;
                            }
                        }
                    }
                    c.synonyms = sinonimos;
                    c.lang = lang.clone();
                    c.nota = nota.clone();
                    if let Some(traduccion) = traductor.translate(&c.title, &lang).await {
                        c.nombre_localizado = traduccion.texto;
                    }
                    resultados.push(c);
                }
            }
            if id != busqueda_id() {
                return;
            }
            online_resultados.set(resultados);
            online_cargando.set(false);
        });
    };

    // Restaura la preferencia guardada (auto/claro/oscuro) y aplica el tema
    // efectivo. «auto» se resuelve con la hora local de la zona horaria.
    // Traduce el análisis de compatibilidad al idioma elegido por el usuario.
    use_effect(move || {
        let lang = output_lang();
        let items = multi_ids();
        let clave = opciones().deepl_key.clone();
        let pro = opciones().deepl_pro;
        spawn(async move {
            let traductor = sustancias_i18n::Cadena::con_configuracion(
                if clave.trim().is_empty() { None } else { Some(clave) },
                if pro {
                    sustancias_i18n::DeeplApi::Pro
                } else {
                    sustancias_i18n::DeeplApi::Free
                },
            );
            let mut salida = Vec::new();
            for i in 0..items.len() {
                for j in (i + 1)..items.len() {
                    if let Some(a) = core::cameo_analyze(&items[i].2, &items[j].2) {
                        let mut peligros = Vec::new();
                        for x in &a.peligros { peligros.push(traducir_texto(&traductor, &lang, x.clone()).await); }
                        let mut gases = Vec::new();
                        for x in &a.gases { gases.push(traducir_texto(&traductor, &lang, x.clone()).await); }
                        salida.push(CameoVisible {
                            na: items[i].1.clone(), nb: items[j].1.clone(),
                            etiqueta: traducir_texto(&traductor, &lang, a.etiqueta.clone()).await,
                            compatibilidad: a.compatibilidad.clone(),
                            nivel: a.nivel.clone(),
                            peligros, gases,
                            citas: a.citas.clone(), fuentes: a.fuentes.clone(),
                            revisado: a.revisado,
                        });
                    }
                }
            }
            cameo_visible.set(salida);
        });
    });

    // Resumen de ternas: agrega los TRES pares CAMEO reales sin inventar química.
    use_effect(move || {
        let sel = output_lang();
        let lang = sustancias_i18n::idioma_efectivo(&sel, lang);
        let items = multi_ids();
        let clave = opciones().deepl_key.clone();
        let pro = opciones().deepl_pro;

        // Un mismo CAS con distinto CID no cuenta como sustancia distinta.
        let mut vistos = std::collections::HashSet::new();
        let unicos: Vec<_> = items
            .iter()
            .filter(|item| {
                let cas = item.2.trim().to_lowercase();
                !cas.is_empty() && vistos.insert(cas)
            })
            .collect();

        let mut candidatas: Vec<(u8, usize, core::TripleAnalysis)> = Vec::new();
        'ternas: for i in 0..unicos.len() {
            for j in (i + 1)..unicos.len() {
                for k in (j + 1)..unicos.len() {
                    if candidatas.len() >= MAX_TRIPLE_CANDIDATES {
                        break 'ternas;
                    }
                    if let Some(t) = core::cameo_analyze_triple_named(
                        [unicos[i].1.as_str(), unicos[j].1.as_str(), unicos[k].1.as_str()],
                        [unicos[i].2.as_str(), unicos[j].2.as_str(), unicos[k].2.as_str()],
                    ) {
                        candidatas.push((triple_prioridad(&t), i * 10000 + j * 100 + k, t));
                    }
                }
            }
        }
        candidatas.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
        candidatas.truncate(MAX_TRIPLES_SHOWN);

        spawn(async move {
            let traductor = sustancias_i18n::Cadena::con_configuracion(
                if clave.trim().is_empty() { None } else { Some(clave) },
                if pro {
                    sustancias_i18n::DeeplApi::Pro
                } else {
                    sustancias_i18n::DeeplApi::Free
                },
            );
            let mut salida = Vec::new();
            for (_, _, t) in candidatas {
                let parejas = t
                    .parejas
                    .iter()
                    .map(|p| {
                        (
                            format!("{} + {}", p.nombre_a, p.nombre_b),
                            p.compatibilidad.clone(),
                        )
                    })
                    .collect();
                salida.push(TripleVisible {
                    nombres: t.nombres.clone(),
                    reglas: t.reglas.clone(),
                    nivel: t.nivel.clone(),
                    etiqueta: traducir_texto(&traductor, lang, t.etiqueta.clone()).await,
                    compatibilidad: t.compatibilidad.clone(),
                    parejas,
                    puente: t.puente.as_ref().map(|b| b.nombre.clone()),
                    fuentes: t.fuentes.clone(),
                    nota: core::cameo_triple_nota(lang),
                    cobertura_limitada: t.cobertura_limitada,
                    revisado: t.revisado,
                });
            }
            triple_visible.set(salida);
        });
    });

    use_effect(move || {
        // Se aplica de forma directa: `Eval::recv` no es fiable en el primer
        // render y dejaría el tema sin aplicar. La preferencia se sincroniza
        // después, solo para el icono del botón.
        dioxus::document::eval(
            "try{var p=localStorage.getItem('sustancias-theme')||'auto';var h=new Date().getHours();var t=p==='auto'?(h>=7&&h<19?'light':'dark'):p;document.documentElement.dataset.theme=t;}catch(e){document.documentElement.dataset.theme='dark';}",
        );
        spawn(async move {
            let mut evaluar = dioxus::document::eval(
                "(function(){try{return localStorage.getItem('sustancias-theme')||'auto'}catch(e){return 'auto'}})()",
            );
            if let Ok(valor) = evaluar.recv::<serde_json::Value>().await {
                if let Some(preferencia) = valor.as_str() {
                    theme.set(preferencia.to_string());
                }
            }
        });
    });

    let continentes = vec![
        "{txt(\"continent_international\", lang_ui)}", "{txt(\"continent_america\", lang_ui)}", "{txt(\"continent_europe\", lang_ui)}", "{txt(\"continent_africa\", lang_ui)}", "{txt(\"continent_asia\", lang_ui)}", "{txt(\"continent_oceania\", lang_ui)}",
    ];
    let rsx_countries = |_modal: bool| -> Element {
        rsx! {
            for cont in continentes.iter() {
                details { class: "continente-carpeta",
                    summary { class: "continente-titulo",
                        span { class: "continente-icono", dangerous_inner_html: "{continent_svg(*cont, \"continente-silueta\")}" }
                        span { "{cont}" }
                        span { class: "badge badge-suave", "{countries.iter().filter(|c| c.continente.as_str() == *cont).count()}" }
                    }
                    div { class: "paises-carpeta",
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
            }
        }
    };

    let tema_icono = match theme().as_str() {
        "auto" => "clock",
        "light" => "sun",
        _ => "moon",
    };
    let tema_titulo = match theme().as_str() {
        "auto" => txt("theme_title_auto", lang_ui),
        "light" => txt("theme_title_light", lang_ui),
        _ => txt("theme_title_dark", lang_ui),
    };

    rsx! {
        style { {CSS} }
        a { class: "skip-link", href: "#panel-sustancia", "{txt(\"skip_to_search\", lang_ui)}" }

        if !chosen {
            div { class: "overlay",
                div { class: "overlay-card",
                    div { class: "overlay-logo", dangerous_inner_html: "{icon(\"flask\", \"ico ico-xl\")}" }
                    h1 { "{txt(\"select_country\", lang_ui)}" }
                    p { "{txt(\"select_sub\", lang_ui)}" }
                    div { class: "pais-grid", {rsx_countries(false)} }
                }
            }
        }

        if modal_open() {
            div { class: "overlay",
                div { class: "overlay-card compacta",
                    button { class: "cerrar", onclick: move |_| modal_open.set(false),
                        dangerous_inner_html: "{icon(\"x\", \"ico\")}" }
                    h2 { "{txt(\"change_country\", lang_ui)}" }
                    p { "{txt(\"change_sub\", lang_ui)}" }
                    div { class: "pais-grid", {rsx_countries(true)} }
                }
            }
        }

        header { class: "cabecera",
            div { class: "marca",
                span { class: "marca-icono", dangerous_inner_html: "{icon(\"flask\", \"ico ico-lg\")}" }
                span {
                    strong { "{txt(\"brand_title\", lang_ui)}" }
                    small { "{txt(\"brand_subtitle\", lang_ui)}" }
                }
            }
            div { class: "cabecera-derecha",
                button {
                    class: "tema-boton",
                    aria_label: "{tema_titulo}",
                    title: "{tema_titulo}",
                    onclick: move |_| {
                        let nuevo = match theme().as_str() {
                            "auto" => "light",
                            "light" => "dark",
                            _ => "auto",
                        };
                        let efectivo = tema_efectivo(nuevo);
                        theme.set(nuevo.to_string());
                        tema_actual.set(efectivo.to_string());
                        dioxus::document::eval(&format!(
                            "try{{document.documentElement.dataset.theme={:?};localStorage.setItem('sustancias-theme',{:?});}}catch(e){{}}",
                            efectivo, nuevo
                        ));
                    },
                    dangerous_inner_html: "{icon(tema_icono, \"ico\")}"
                }
                span { class: "badge badge-suave", "{country.codigo}" }
            }
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
                    span { "{txt(\"legal_details_summary\", lang_ui)}" }
                    span { class: "badge", "{sustancias_i18n::texto_fijo_fmt(\"norms_count\", lang_ui, &[(\"n\", &n_normas)])}" }
                }
                div { class: "panel-cabecera",
                    div { class: "panel-titulo",
                        span { class: "panel-icono", dangerous_inner_html: "{icon(\"scale\", \"ico ico-lg\")}" }
                        div {
                            h3 { "{txt(\"legal_title\", lang_ui)}" }
                            p { "{txt(\"legal_sub\", lang_ui)}" }
                        }
                    }
                    span { class: "badge", "{sustancias_i18n::texto_fijo_fmt(\"norms_count\", lang_ui, &[(\"n\", &n_normas)])}" }
                }
                div { class: "filtros-wrap",
                    span { class: "filtro-icono", dangerous_inner_html: "{icon(\"filter\", \"ico\")}" }
                    div { class: "filtros",
                        button {
                            class: if filter() == "todos" { "chip activo" } else { "chip" },
                            onclick: move |_| filter.set("todos".to_string()),
                            "{txt(\"filter_all\", lang_ui)}"
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
                                placeholder: "{txt(\"boe_search_placeholder\", lang_ui)}",
                                oninput: move |e| {
                                    let value = e.value();
                                    if value.trim().len() < 3 {
                                        boe_resultados.set(Vec::new());
                                        return;
                                    }
                                    // Solo caracteres alfanuméricos: evita inyectar
                                    // sintaxis Lucene y agrupa varias palabras con AND.
                                    let palabras: Vec<String> = value
                                        .trim()
                                        .split_whitespace()
                                        .map(|w| w.chars().filter(|c| c.is_alphanumeric()).collect())
                                        .filter(|w: &String| !w.is_empty())
                                        .collect();
                                    if palabras.is_empty() {
                                        boe_resultados.set(Vec::new());
                                        boe_termino.set(String::new());
                                        return;
                                    }
                                    let consulta = if palabras.len() > 1 {
                                        format!("titulo:({})", palabras.join(" AND "))
                                    } else {
                                        format!("titulo:{}", palabras[0])
                                    };
                                    boe_termino.set(value.trim().to_string());
                                    boe_cargando.set(true);
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
                        div { class: "boe-resultados", aria_live: "polite",
                            if boe_cargando() {
                                p { class: "vacio", "{txt(\"boe_loading\", lang_ui)}" }
                            } else if !boe_resultados().is_empty() {
                                for n in boe_resultados().iter() {
                                    div { class: "norma",
                                        h4 { "{n.titulo}" }
                                        div { class: "norma-meta",
                                            span { class: "badge", "{n.identificador}" }
                                            if let Some(dep) = &n.departamento {
                                                span { class: "badge badge-suave", "{dep.texto}" }
                                            }
                                            span { class: "badge badge-warn", "{txt(\"boe_consolidated_badge\", lang_ui)}" }
                                        }
                                        a { href: "{n.url()}", target: "_blank",
                                            dangerous_inner_html: "{icon(\"link\", \"ico\")}" }
                                        a { href: "{n.url()}", target: "_blank", "{txt(\"boe_view_link\", lang_ui)}" }
                                    }
                                }
                                p { class: "boe-atribucion",
                                    "{txt(\"boe_source_prefix\", lang_ui)}"
                                    a { href: "https://www.boe.es", target: "_blank", "Agencia Estatal Boletín Oficial del Estado" }
                                    "{txt(\"boe_disclaimer\", lang_ui)}"
                                    if !boe_resultados()[0].fecha_formateada().is_empty() {
                                        " Última actualización: {boe_resultados()[0].fecha_formateada()}."
                                    }
                                }
                            } else if !boe_termino().is_empty() {
                                p { class: "vacio", "No se encontraron normas en el BOE para «{boe_termino()}»." }
                            }
                        }
                    }
                }
                div { class: "legal-lista",
                    if !portals.is_empty() {
                        div { class: "portales",
                            p { class: "lista-titulo", "{sustancias_i18n::texto_fijo_fmt(\"portals_title\", lang_ui, &[(\"pais\", country.nombre.as_str())])}" }
                            for p in portals.iter() {
                                a { href: "{p.url}", target: "_blank",
                                    dangerous_inner_html: "{icon(\"link\", \"ico\")}" }
                                a { href: "{p.url}", target: "_blank", "{p.institucion}" }
                            }
                        }
                    }
                    h4 { class: "leyenda-grupo", "{txt(\"international_section_title\", lang_ui)}" }
                    for s in international_sources.iter().filter(|s| filter() == "todos" || s.tema == filter()) {
                        div { class: "norma",
                            h4 { "{s.titulo}" }
                            div { class: "norma-meta",
                                span { class: "badge", "{core::topic_label(&s.tema)}" }
                                span { class: "badge badge-suave", "{txt(\"continent_international\", lang_ui)}" }
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
                    for bloque in org_blocks.iter() {
                        if !bloque.1.is_empty() {
                            h4 { class: "leyenda-grupo", "{bloque.0.nombre}" }
                            for u in bloque.1.iter() {
                                div { class: "norma",
                                    h4 { "{u.titulo}" }
                                    div { class: "norma-meta",
                                        span { class: "badge", "{core::topic_label(&u.tema)}" }
                                        span { class: "badge badge-suave", "{bloque.0.nombre}" }
                                        span { class: "badge badge-ok", "{txt(\"verified_badge\", lang_ui)}" }
                                    }
                                    a { href: "{u.url}", target: "_blank",
                                        dangerous_inner_html: "{icon(\"link\", \"ico\")}" }
                                    a { href: "{u.url}", target: "_blank", "{u.url}" }
                                }
                            }
                        }
                    }
                    h4 { class: "leyenda-grupo", "{sustancias_i18n::texto_fijo_fmt(\"national_legislation_title\", lang_ui, &[(\"pais\", country.nombre.as_str())])}" }
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
                                    "{txt(\"read_full_norm\", lang_ui)}"
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
                            h3 { "{txt(\"sub_title\", lang_ui)}" }
                            p { "{txt(\"sub_sub\", lang_ui)}" }
                        }
                    }
                }

                div { class: "aviso",
                    span { dangerous_inner_html: "{icon(\"alert\", \"ico\")}" }
                    span { "{txt(\"substances_warning\", lang_ui)}" }
                }

                div { class: "idioma-busqueda",
                    span { "{txt(\"language_search_hint\", lang_ui)}" }
                    select {
                        aria_label: "{txt(\"output_language_label\", lang_ui)}",
                        value: "{output_lang()}",
                        onchange: move |e| {
                            let lang = e.value();
                            output_lang.set(lang.clone());
                            buscar_pubchem(query(), lang);
                        },
                        option { value: "auto", "{txt(\"lang_option_auto\", lang_ui)}" }
                        option { value: "es", "{txt(\"lang_option_es\", lang_ui)}" }
                        option { value: "en", "{txt(\"lang_option_en\", lang_ui)}" }
                        option { value: "fr", "{txt(\"lang_option_fr\", lang_ui)}" }
                        option { value: "de", "{txt(\"lang_option_de\", lang_ui)}" }
                        option { value: "it", "{txt(\"lang_option_it\", lang_ui)}" }
                    }
                }
                div { class: "buscador",
                    span { class: "buscador-icono", dangerous_inner_html: "{icon(\"search\", \"ico\")}" }
                    input {
                        placeholder: "{txt(\"search_placeholder\", lang_ui)}",
                        value: "{query()}",
                        oninput: move |e| {
                            let value = e.value();
                            query.set(value.clone());
                            buscar_pubchem(value, if output_lang() == "auto" { lang.to_string() } else { output_lang() });
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

                if mostrar_estrella() {
                    div { class: "estrella-banner",
                        span { dangerous_inner_html: "{icon(\"flask\", \"ico\")}" }
                        span { class: "estrella-texto", "{txt(\"star_banner_text\", lang_ui)}" }
                        a { class: "estrella-enlace", href: GITHUB_URL, target: "_blank", "{txt(\"star_banner_link\", lang_ui)}" }
                        button { class: "estrella-cerrar", aria_label: "{txt(\"star_dismiss\", lang_ui)}", title: "{txt(\"star_dismiss\", lang_ui)}",
                            onclick: move |_| {
                                mostrar_estrella.set(false);
                                estrella_cerrada.set(true);
                                cerrar_estrella();
                            },
                            dangerous_inner_html: "{icon(\"x\", \"ico\")}"
                        }
                    }
                }
                div { class: "resultados-online", aria_live: "polite",
                    if online_cargando() {
                        p { class: "vacio", "{txt(\"searching_official_db\", lang_ui)}" }
                    } else if !query().trim().is_empty() && online_resultados().is_empty() {
                        if online_error_red() {
                            p { class: "vacio", "{txt(\"network_error_hint\", lang_ui)}" }
                        } else if !core::es_consulta_valida(&query()) {
                            p { class: "vacio", "{txt(\"invalid_query_hint\", lang_ui)}" }
                        } else {
                            p { class: "vacio", "{txt(\"no_substances_hint\", lang_ui)}" }
                        }
                    } else if !online_resultados().is_empty() {
                        div { class: "online-titulo",
                            span { dangerous_inner_html: "{icon(\"globe\", \"ico\")}" }
                            "{txt(\"online_results_title\", lang_ui)}"
                        }
                        for p in online_resultados().iter() {
                            div { class: "online-resultado",
                                div {
                                    strong { "{nombre_visible(p, &output_lang())}" }
                                    span { class: "online-meta", "CID {p.cid} · {p.molecular_formula} · {p.molecular_weight} g/mol" }
                                    if !p.nota.is_empty() {
                                        p { class: "online-nota", "{p.nota}" }
                                    }
                                }
                                div { class: "online-acciones",
                                    button {
                                        class: "boton",
                                        onclick: {
                                            let compuesto = p.clone();
                                            let lang = output_lang();
                                            move |_| {
                                                let c = compuesto.clone();
                                                let l = lang.clone();
                                                online_ficha.set(Some(core::substance_from_pubchem(&c)));
                                                spawn(async move {
                                                    let url = format!(
                                                        "https://pubchem.ncbi.nlm.nih.gov/rest/pug_view/data/compound/{}/JSON",
                                                        c.cid
                                                    );
                                                    if let Ok(view) = http_get_json(&url).await {
                                                        online_ficha.set(Some(
                                                            core::substance_from_pubchem_view(&c, &view, &l),
                                                        ));
                                                    }
                                                });
                                            }
                                        },
                                        "{txt(\"view_online_sheet\", lang_ui)}"
                                    }
                                    button {
                                        class: "boton boton-secundario",
                                        onclick: {
                                            let cid = p.cid;
                                            let nombre = nombre_visible(&p, &output_lang());
                                            let cas = p.cas.clone();
                                            move |_| {
                                                let mut items = multi_ids();
                                                let id = format!("online-{}", cid);
                                                if !items.iter().any(|(x, _, _)| x == &id) {
                                                    items.push((id, nombre.clone(), cas.clone()));
                                                }
                                                multi_ids.set(items);
                                            }
                                        },
                                        "{txt(\"add_to_compare\", lang_ui)}"
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
                            strong { "{txt(\"compare_title\", lang_ui)}" }
                        }
                        if cameo_visible().is_empty() && triple_visible().is_empty() {
                            span { class: "badge badge-suave", "{txt(\"add_substances_badge\", lang_ui)}" }
                        } else {
                            span { class: "badge badge-ok", "{sustancias_i18n::texto_fijo_fmt(\"compare_counts\", lang_ui, &[(\"n\", &cameo_visible().len().to_string()), (\"m\", &triple_visible().len().to_string()), (\"t\", &combinaciones3(multi_items.len()).to_string())])}" }
                        }
                    }
                    div { class: "multi-anadir",
                        input {
                            placeholder: "{txt(\"add_another_placeholder\", lang_ui)}",
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
                                    if !items.iter().any(|(x, _, _)| x == &id) {
                                        items.push((id, p.title.clone(), p.cas.clone()));
                                    }
                                    multi_ids.set(items);
                                    multi_input.set(String::new());
                                }
                            },
                            span { dangerous_inner_html: "{icon(\"plus\", \"ico\")}" }
                            "{txt(\"add_button\", lang_ui)}"
                        }
                    }
                    div { class: "multi-chips",
                        for (_, nombre, _) in multi_items.iter() {
                            span { class: "multi-chip", "{nombre}" }
                        }
                    }
                    div { class: "interacciones",
                    for (nombre, perfil) in perfiles.iter() {
                        if let Some(textos) = perfil.secciones.get("Reactivity Profile") {
                            div { class: "interaccion cameo",
                                div { class: "interaccion-cabecera",
                                    strong { "{sustancias_i18n::texto_fijo_fmt(\"reactivity_prefix\", lang_ui, &[(\"nombre\", nombre.as_str())])}" }
                                    span { class: "badge badge-suave", "PubChem" }
                                }
                                p { class: "multi-nota", "{txt(\"pubchem_source_note\", lang_ui)}" }
                                for txt in textos.iter() { p { "{txt}" } }
                            }
                        }
                    }
                    for a in cameo_visible.iter() {
                        div { class: "interaccion cameo",
                            div { class: "interaccion-cabecera",
                                strong { "{a.na} + {a.nb}" }
                                span {
                                    class: match a.nivel.as_str() {
                                        "alto" => "badge badge-danger",
                                        "moderado" => "badge badge-warn",
                                        _ => "badge badge-suave",
                                    },
                                    "{a.etiqueta}"
                                }
                            }
                            p { class: "multi-nota", "{txt(\"cameo_compatibility_note\", lang_ui)}" }
                            if a.revisado { span { class: "badge badge-ok", "{txt(\"reviewed_badge\", lang_ui)}" } }
                            span {
                                class: match a.compatibilidad.as_str() {
                                    "incompatible" => "badge badge-danger",
                                    "precaucion" => "badge badge-warn",
                                    _ => "badge badge-ok",
                                },
                                match a.compatibilidad.as_str() {
                                    "incompatible" => "{txt(\"status_incompatible\", lang_ui)}",
                                    "precaucion" => "{txt(\"status_precaution\", lang_ui)}",
                                    _ => "{txt(\"status_compatible\", lang_ui)}",
                                }
                            }
                            if !a.peligros.is_empty() {
                                p { class: "lista-titulo", "{txt(\"predicted_hazards\", lang_ui)}" }
                                ul { class: "lista",
                                    for x in a.peligros.iter() { li { "{x}" } }
                                }
                            }
                            if !a.gases.is_empty() {
                                p { class: "lista-titulo", "{txt(\"possible_gases\", lang_ui)}" }
                                p { class: "interaccion-productos", "{a.gases.join(\", \")}" }
                            }
                            if !a.citas.is_empty() {
                                details { class: "citas",
                                    summary { "{txt(\"pair_references_summary\", lang_ui)}" }
                                    ul { class: "lista",
                                        for c in a.citas.iter() { li { "{c}" } }
                                    }
                                }
                            }
                            for u in a.fuentes.iter() {
                                a { href: "{u}", target: "_blank", class: "multi-nota", "{txt(\"view_pair_docs_cameo\", lang_ui)}" }
                            }
                        }
                    }
                    }
                    if !triple_visible().is_empty() {
                        div { class: "multi-tercias",
                            p { class: "lista-titulo", "{txt(\"triple_summary_title\", lang_ui)}" }
                            p { class: "multi-nota", "{txt(\"triple_summary_note\", lang_ui)}" }
                            for t in triple_visible.iter() {
                                div { class: "interaccion cameo",
                                    div { class: "interaccion-cabecera",
                                        strong { "{t.nombres.join(\" + \")}" }
                                        span {
                                            class: match t.nivel.as_str() {
                                                "alto" => "badge badge-danger",
                                                "moderado" => "badge badge-warn",
                                                _ => "badge badge-suave",
                                            },
                                            "{t.etiqueta}"
                                        }
                                    }
                                    span {
                                        class: match t.compatibilidad.as_str() {
                                            "incompatible" => "badge badge-danger",
                                            "precaucion" => "badge badge-warn",
                                            _ => "badge badge-ok",
                                        },
                                        match (t.compatibilidad.as_str(), lang_ui) {
                                            ("incompatible", _) => "{txt(\"status_incompatible_pairs\", lang_ui)}",
                                            ("precaucion", _) => "{txt(\"status_precaution_pairs\", lang_ui)}",
                                            (_, "en") => "NO HAZARD REPORTED FOR ANY PAIR",
                                            _ => "{txt(\"status_no_hazard_pairs\", lang_ui)}",
                                        }
                                    }
                                    div { class: "multi-chips",
                                        for (par, comp) in t.parejas.iter() {
                                            span {
                                                class: match comp.as_str() {
                                                    "incompatible" => "badge badge-danger",
                                                    "precaucion" => "badge badge-warn",
                                                    _ => "badge badge-suave",
                                                },
                                                "{par}: {comp}"
                                            }
                                        }
                                    }
                                    if let Some(puente) = &t.puente {
                                        p { class: "multi-nota",
                                            "{sustancias_i18n::texto_fijo_fmt(\"bridge_pattern\", lang_ui, &[(\"puente\", puente.as_str())])}"
                                        }
                                    }
                                    if t.cobertura_limitada {
                                        span { class: "badge badge-warn", "{txt(\"limited_coverage_badge\", lang_ui)}" }
                                    }
                                    if t.revisado {
                                        span { class: "badge badge-ok", "{txt(\"reviewed_badge\", lang_ui)}" }
                                    }
                                    for r in t.reglas.iter() {
                                        div { class: "regla-ternaria",
                                            strong { "{sustancias_i18n::texto_fijo_fmt(\"rule_badge\", lang_ui, &[(\"titulo\", r.titulo.as_str())])}" }
                                            p { class: "multi-nota", "{r.descripcion}" }
                                            p { class: "multi-nota", "{sustancias_i18n::texto_fijo_fmt(\"rule_conditions\", lang_ui, &[(\"condiciones\", r.condiciones.as_str())])}" }
                                            p { class: "multi-nota", "{sustancias_i18n::texto_fijo_fmt(\"rule_hazards\", lang_ui, &[(\"peligros\", &r.peligros.join(\", \"))])}" }
                                            details { class: "citas",
                                                summary { "{txt(\"rule_source_summary\", lang_ui)}" }
                                                ul { class: "lista",
                                                    for f in r.fuentes.iter() {
                                                        li { a { href: "{f.url}", target: "_blank", "{f.titulo}" } }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    details { class: "citas",
                                        summary { "{txt(\"triple_references_summary\", lang_ui)}" }
                                        ul { class: "lista",
                                            for f in t.fuentes.iter() {
                                                li { a { href: "{f.url}", target: "_blank", "{f.titulo}" } }
                                            }
                                        }
                                    }
                                    p { class: "multi-nota", "{t.nota}" }
                                }
                            }
                        }
                    }
                    if multi_items.len() >= 3 {
                        div {
                            class: "aviso aviso-ternario",
                            aria_label: "{txt(\"ternary_warning_aria\", lang_ui)}",
                            span { dangerous_inner_html: "{icon(\"alert\", \"ico\")}" }
                            span { "{sustancias_i18n::texto_fijo_count(\"multi_substance_warning\", lang_ui, multi_items.len())}" }
                        }
                    }
                    p { class: "multi-nota", "{txt(\"interactions_warning\", lang_ui)}" }
                }

                div { class: "ficha",
                    if let Some(s) = online_ficha().or(active_substance) {
                        div { class: "ficha-titulo",
                            h4 { "{s.nombre}" }
                            if s.estado == "online" {
                                span { class: "badge badge-online", "{txt(\"badge_online_data\", lang_ui)}" }
                            } else if core::substance_is_verified(&s) {
                                span { class: "badge badge-ok", "{txt(\"badge_verified_sheet\", lang_ui)}" }
                            } else {
                                span { class: "badge badge-demo", "{txt(\"badge_demo_sheet\", lang_ui)}" }
                            }
                        }
                        div { class: "identificadores",
                            span { class: "ident", "CAS {s.cas}" }
                            span { class: "ident", "CE {s.ce}" }
                            span { class: "ident", "ONU {s.onu}" }
                            span { class: "ident", "{s.palabra_advertencia}" }
                            if !s.fecha_verificacion.is_empty() {
                                span { class: "ident", "{sustancias_i18n::texto_fijo_fmt(\"verified_date\", lang_ui, &[(\"fecha\", s.fecha_verificacion.as_str())])}" }
                            }
                            if !s.verificado_por.is_empty() {
                                span { class: "ident", "{sustancias_i18n::texto_fijo_fmt(\"verified_by\", lang_ui, &[(\"quien\", s.verificado_por.as_str())])}" }
                            }
                        }
                        if s.estado == "online" {
                            div { class: "aviso",
                                span { dangerous_inner_html: "{icon(\"alert\", \"ico\")}" }
                                span { "{txt(\"partial_sheet_notice\", lang_ui)}" }
                            }
                        }
                        if let Some(lim) = limites {
                            div { class: "seccion limites",
                                h5 { "{txt(\"exposure_limits_title\", lang_ui)}" }
                                div { class: "prop-grid",
                                    if !lim.rel.is_empty() {
                                        div { class: "prop", b { "NIOSH REL" } "{lim.rel}" }
                                    }
                                    if !lim.pel.is_empty() {
                                        div { class: "prop", b { "OSHA PEL" } "{lim.pel}" }
                                    }
                                    if !lim.idlh.is_empty() {
                                        div { class: "prop", b { "IDLH" } "{lim.idlh}" }
                                    }
                                }
                                p { class: "multi-nota",
                                    a { href: "{lim.url}", target: "_blank", "{txt(\"niosh_source\", lang_ui)}" }
                                }
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
                            h5 { "{txt(\"sources_title\", lang_ui)}" }
                            for f in s.fuentes.iter() {
                                a { href: "{f.url}", target: "_blank", "{f.titulo}" }
                            }
                        }
                        div { class: "seccion documentos",
                            h5 { "{txt(\"documents_title\", lang_ui)}" }
                            p { "{txt(\"documents_intro\", lang_ui)}" }
                            div { class: "documentos-acciones",
                                if !s.pdf_url.is_empty() {
                                    button {
                                        class: "boton boton-descargar",
                                        onclick: move |_| {
                                            descargar_documento(
                                                s.pdf_url.clone(),
                                                core::pdf_filename(&s.pdf_url, &s.pdf_nombre),
                                                s.nombre.clone(),
                                                DocKind::Substance,
                                                opciones(),
                                                guardados,
                                                estado_descarga,
                                            );
                                        },
                                        span { dangerous_inner_html: "{icon(\"download\", \"ico\")}" }
                                        "{txt(\"download_sds_pdf\", lang_ui)}"
                                    }
                                }
                                button {
                                    class: "boton boton-secundario",
                                    onclick: move |_| {
                                        dioxus::document::eval("window.print()");
                                    },
                                    span { dangerous_inner_html: "{icon(\"file\", \"ico\")}" }
                                    "{txt(\"print_sheet_pdf\", lang_ui)}"
                                }
                            }
                        }
                    } else {
                        div { class: "ficha-vacia",
                            span { dangerous_inner_html: "{icon(\"file\", \"ico ico-xl\")}" }
                            h4 { "{txt(\"pick_substance\", lang_ui)}" }
                            p { "{txt(\"pick_substance_sub\", lang_ui)}" }
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
                            "{txt(\"open_original_pdf\", lang_ui)}"
                        }
                        button {
                            class: "boton boton-descargar",
                            onclick: move |_| {
                                descargar_documento(
                                    url.clone(),
                                    nombre_pdf.clone(),
                                    titulo.clone(),
                                    DocKind::Legislation,
                                    opciones(),
                                    guardados,
                                    estado_descarga,
                                );
                            },
                            span { dangerous_inner_html: "{icon(\"download\", \"ico\")}" }
                            "{txt(\"download_pdf\", lang_ui)}"
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
                            h3 { "{txt(\"offline_docs_title\", lang_ui)}" }
                            p { "{txt(\"offline_docs_sub\", lang_ui)}" }
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
                        "{txt(\"opt_save_offline\", lang_ui)}"
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
                        "{txt(\"opt_copy_downloads\", lang_ui)}"
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
                        "{txt(\"opt_auto_report\", lang_ui)}"
                    }
                    label { class: "opcion",
                        span { "{txt(\"translation_provider\", lang_ui)}" }
                        select {
                            aria_label: "{txt(\"translation_provider\", lang_ui)}",
                            value: if opciones().deepl_enabled { "deepl" } else { "google" },
                            onchange: move |e| {
                                let mut o = opciones();
                                o.deepl_enabled = e.value() == "deepl";
                                guardar_opciones(&o);
                                opciones.set(o);
                            },
                            option { value: "google", "{txt(\"provider_google\", lang_ui)}" }
                            option { value: "deepl", "{txt(\"provider_deepl\", lang_ui)}" }
                        }
                    }
                    if opciones().deepl_enabled {
                        label { class: "opcion",
                            span { "{txt(\"deepl_key_label\", lang_ui)}" }
                            input {
                                r#type: "password",
                                value: opciones().deepl_key.clone(),
                                placeholder: "{txt(\"deepl_key_placeholder\", lang_ui)}",
                                onchange: move |e| {
                                    let mut o = opciones();
                                    o.deepl_key = e.value();
                                    guardar_opciones(&o);
                                    opciones.set(o);
                                },
                            }
                        }
                        label { class: "opcion",
                            input {
                                r#type: "checkbox",
                                checked: opciones().deepl_pro,
                                onchange: move |e| {
                                    let mut o = opciones();
                                    o.deepl_pro = e.checked();
                                    guardar_opciones(&o);
                                    opciones.set(o);
                                },
                            }
                            "{txt(\"deepl_pro_option\", lang_ui)}"
                        }
                    }
                    if !cfg!(target_arch = "wasm32") {
                        button {
                            class: "boton boton-secundario",
                            onclick: move |_| {
                                #[cfg(not(target_arch = "wasm32"))]
                                abrir_issue("Feedback sobre Sustancias Peligrosas", "Cuéntanos qué funciona mal o qué te gustaría mejorar:");
                            },
                            "{txt(\"feedback_button\", lang_ui)}"
                        }
                    }
                    label { class: "opcion",
                        span { "{txt(\"cas_key_label\", lang_ui)}" }
                        input {
                            r#type: "password",
                            value: opciones().cas_api_key.clone(),
                            placeholder: "X-Api-Key",
                            onchange: move |e| {
                                let mut o = opciones();
                                o.cas_api_key = e.value();
                                guardar_opciones(&o);
                                opciones.set(o);
                            },
                        }
                    }
                    details { class: "tutorial-cas",
                        summary { "{txt(\"cas_tutorial_summary\", lang_ui)}" }
                        ol { class: "lista",
                            li { "Abre " a { href: "https://www.cas.org/services/commonchemistry-api", target: "_blank", "{txt(\"cas_step_form_link\", lang_ui)}" } "." }
                            li { "{txt(\"cas_step_fill\", lang_ui)}" }
                            li { "{txt(\"cas_step_token\", lang_ui)}" }
                            li { "{txt(\"cas_step_paste\", lang_ui)}" }
                        }
                        p { class: "multi-nota", "{txt(\"cas_tutorial_note\", lang_ui)}" }
                    }
                    label { class: "opcion",
                        span { "{txt(\"support_creator\", lang_ui)}" }
                        a { class: "estrella-enlace", href: GITHUB_URL, target: "_blank",
                            "{txt(\"star_support_link\", lang_ui)}" }
                    }
                    label { class: "opcion",
                        "{txt(\"downloads_dir_label\", lang_ui)}"
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
                h4 { "{txt(\"saved_docs_title\", lang_ui)}" }
                if guardados().is_empty() {
                    p { class: "vacio", "{txt(\"no_saved_docs\", lang_ui)}" }
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
                                "{txt(\"delete_button\", lang_ui)}"
                            }
                        }
                    }
                }
            }
        }

        details { class: "panel feedback-panel",
            summary {
                span { class: "panel-icono", dangerous_inner_html: "{icon(\"info\", \"ico ico-lg\")}" }
                span { "{txt(\"feedback_button\", lang_ui)}" }
            }
            p { class: "multi-nota", "{txt(\"feedback_intro\", lang_ui)}" }
            div { class: "formulario",
                label { class: "opcion",
                    "{txt(\"feedback_type_label\", lang_ui)}"
                    select {
                        aria_label: "{txt(\"feedback_type_aria\", lang_ui)}",
                        value: "{feedback_tipo()}",
                        onchange: move |e| feedback_tipo.set(e.value()),
                        option { value: "Sugerencia", "{txt(\"feedback_type_suggestion\", lang_ui)}" }
                        option { value: "Bug", "{txt(\"feedback_type_bug\", lang_ui)}" }
                        option { value: "Datos incorrectos", "{txt(\"feedback_type_data\", lang_ui)}" }
                    }
                }
                label { class: "opcion",
                    "{txt(\"feedback_title_label\", lang_ui)}"
                    input {
                        r#type: "text",
                        value: "{feedback_titulo()}",
                        placeholder: "{txt(\"feedback_title_placeholder\", lang_ui)}",
                        oninput: move |e| feedback_titulo.set(e.value()),
                    }
                }
                label { class: "opcion",
                    "{txt(\"feedback_details_label\", lang_ui)}"
                    textarea {
                        value: "{feedback_texto()}",
                        placeholder: "{txt(\"feedback_details_placeholder\", lang_ui)}",
                        oninput: move |e| feedback_texto.set(e.value()),
                    }
                }
                button {
                    class: "boton",
                    onclick: move |_| {
                        let tipo = feedback_tipo();
                        let titulo = feedback_titulo().trim().to_string();
                        let detalles = feedback_texto().trim().to_string();
                        if titulo.is_empty() || detalles.is_empty() { return; }
                        let cuerpo = format!(
                            "## Tipo\n{}\n\n## Descripción\n{}\n\n## Contexto automatico\n- Pais: {}\n- Idioma interfaz: {}\n- Busqueda actual: {}\n- Plataforma: {}\n\n---\nReporte generado automaticamente por Sustancias Peligrosas.",
                            tipo, detalles, country.nombre, lang, query(),
                            if cfg!(target_arch = "wasm32") { "Web" } else { "Nativa" }
                        );
                        abrir_issue(&format!("[{}] {}", tipo, titulo), &cuerpo);
                        feedback_titulo.set(String::new());
                        feedback_texto.set(String::new());
                    },
                    "{txt(\"send_report\", lang_ui)}"
                }
            }
        }

        section { class: "panel redes-panel",
            h3 { "{txt(\"social_title\", lang_ui)}" }
            div { class: "redes",
                a { href: "{GITHUB_URL}", target: "_blank", aria_label: "GitHub",
                    dangerous_inner_html: "{icon(\"link\", \"ico\")}" }
                    "GitHub"
                a { href: "{GITHUB_URL}/issues", target: "_blank", aria_label: "{txt(\"github_issues_aria\", lang_ui)}",
                    dangerous_inner_html: "{icon(\"info\", \"ico\")}" }
                    "{txt(\"github_issues_link\", lang_ui)}"
                a { href: "https://www.linkedin.com/", target: "_blank", aria_label: "LinkedIn",
                    dangerous_inner_html: "{icon(\"globe\", \"ico\")}" }
                    "LinkedIn"
            }
        }

        footer { class: "pie",
            p { "{txt(\"substances_warning\", lang_ui)}" }
            p { "{txt(\"footer_sources\", lang_ui)}" }
            p {
                a {
                    href: "{GITHUB_URL}/issues/new",
                    target: "_blank",
                    class: "link-feedback",
                    "{txt(\"feedback_button\", lang_ui)}"
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
