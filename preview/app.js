/* Vista previa interactiva. Toda la lógica es JS para poder editarla sin compilar. */

const $ = (id) => document.getElementById(id);
const icon = (name, cls = "ico") => `<svg class="${cls}"><use href="#i-${name}"/></svg>`;

const I18N = {
  es: {
    select_country: "Selecciona tu país",
    select_sub: "El marco legal y el idioma del sitio se adaptarán a tu elección.",
    change_country: "Cambiar de país",
    change_sub: "El banner y el marco legal se actualizarán al instante.",
    legal_title: "Verificación del marco legal",
    legal_sub: "Normativa aplicable al país seleccionado",
    sub_title: "Carta técnica de la sustancia",
    sub_sub: "Busca por nombre, CAS, CE o número ONU",
    no_country: "Selecciona un país para ver su marco legal.",
    pick_substance: "Selecciona una sustancia",
    pick_substance_sub: "Escribe en el buscador y haz clic en un resultado para ver su ficha completa.",
  },
  en: {
    select_country: "Select your country",
    select_sub: "The legal framework and language will adapt to your choice.",
    change_country: "Change country",
    change_sub: "The banner and legal framework update instantly.",
    legal_title: "Legal framework verification",
    legal_sub: "Regulations applicable to the selected country",
    sub_title: "Substance technical sheet",
    sub_sub: "Search by name, CAS, EC or UN number",
    no_country: "Select a country to see its legal framework.",
    pick_substance: "Select a substance",
    pick_substance_sub: "Type in the search box and click a result to see the full sheet.",
  },
};

const state = {
  countryId: null,
  filter: "todos",
  query: "",
  substanceId: null,
  multi: [],
};

function t(key) {
  const country = getCountry();
  const lang = country && country.idioma === "en" ? "en" : "es";
  return (I18N[lang] && I18N[lang][key]) || I18N.es[key] || key;
}

function esc(value) {
  return String(value ?? "")
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;")
    .replaceAll("'", "&#39;");
}

function normalize(value) {
  return String(value ?? "")
    .toLowerCase()
    .replace(/[áàäâã]/g, "a")
    .replace(/[éèëê]/g, "e")
    .replace(/[íìïî]/g, "i")
    .replace(/[óòöôõ]/g, "o")
    .replace(/[úùüû]/g, "u")
    .replace(/ñ/g, "n")
    .replace(/ç/g, "c");
}

function getCountry() {
  return window.DATA.countries.find((c) => c.id === state.countryId) || null;
}

function topicLabel(id) {
  const topic = window.DATA.topics.find((t) => t.id === id);
  return topic ? topic.nombre : id;
}

function verificationBadge(value) {
  const verified = value === "contenido_verificado" || value === "documento_local_verificado";
  const labels = {
    contenido_verificado: "Verificado",
    documento_local_verificado: "Documento local",
    dominio_snippet: "Dominio oficial",
    portal_validar: "Pendiente",
    base_local_usuario: "Base local",
    referencia_citada: "Referencia",
  };
  const cls = verified ? "badge badge-ok" : "badge badge-warn";
  const ico = verified ? icon("check") : icon("info");
  return `<span class="${cls}">${ico}${esc(labels[value] || "Sin verificar")}</span>`;
}

/* --- País --------------------------------------------------------------- */

function renderCountryGrids() {
  const html = window.DATA.countries
    .map(
      (c) => `
      <button class="pais-boton" data-pais="${esc(c.id)}">
        <span class="bandera">${esc(c.codigo)}</span>
        <span class="nombre">${esc(c.nombre)}</span>
        <span class="codigo">${esc(c.codigo)}</span>
      </button>`
    )
    .join("");

  for (const id of ["pais-grid-inicial", "pais-grid-modal"]) {
    $(id).innerHTML = html;
    $(id).querySelectorAll("[data-pais]").forEach((btn) => {
      btn.addEventListener("click", () => selectCountry(btn.dataset.pais));
    });
  }
}

function selectCountry(id) {
  state.countryId = id;
  state.filter = "todos";
  state.query = "";
  state.substanceId = null;
  state.multi = [];

  $("pais-overlay").classList.add("oculto");
  $("pais-modal").classList.add("oculto");
  $("banner").classList.remove("oculto");
  $("contenido").classList.remove("oculto");
  $("flotante-pais").classList.remove("oculto");

  $("buscador-sustancias").value = "";
  $("multi-input").value = "";

  renderAll();
}

/* --- Banner e idioma ---------------------------------------------------- */

function renderBanner() {
  const c = getCountry();
  if (!c) return;
  document.documentElement.lang = c.idioma || "es";
  $("banner-bandera").textContent = c.codigo;
  $("banner-saludo").textContent = c.saludo;
  $("banner-descripcion").textContent = c.descripcion;
  $("banner-pais").textContent = `${c.nombre} · ${(c.idioma || "es").toUpperCase()}`;
  $("idioma-badge").textContent = (c.idioma || "es").toUpperCase();
  $("flotante-bandera").textContent = c.codigo;
  $("flotante-nombre").textContent = c.nombre;

  $("overlay-titulo").textContent = t("select_country");
  $("overlay-subtitulo").textContent = t("select_sub");
  $("modal-titulo").textContent = t("change_country");
  $("modal-subtitulo").textContent = t("change_sub");
  $("legal-titulo").textContent = t("legal_title");
  $("legal-subtitulo").textContent = t("legal_sub");
  $("sustancia-titulo").textContent = t("sub_title");
  $("sustancia-subtitulo").textContent = t("sub_sub");
}

/* --- Marco legal -------------------------------------------------------- */

function renderFilters() {
  const topics = [{ id: "todos", nombre: "Todos" }, ...window.DATA.topics];
  $("legal-filtros").innerHTML = topics
    .map(
      (t) =>
        `<button class="chip ${state.filter === t.id ? "activo" : ""}" data-filtro="${esc(t.id)}">${esc(t.nombre)}</button>`
    )
    .join("");

  $("legal-filtros").querySelectorAll("[data-filtro]").forEach((btn) => {
    btn.addEventListener("click", () => {
      state.filter = btn.dataset.filtro;
      renderFilters();
      renderLegal();
    });
  });
}

function renderLegal() {
  const c = getCountry();
  if (!c) return;

  let items = [];
  if (c.id === "internacional") {
    items = window.DATA.sources
      .filter((s) => s.oficial && s.pais === "internacional")
      .filter((s) => state.filter === "todos" || s.tema === state.filter)
      .map((s) => ({
        titulo: s.titulo,
        tema: s.tema,
        url: s.url,
        alternativas: [],
        verificacion: s.verificacion,
        nota: s.nota,
      }));
  } else {
    const data = window.DATA.legislation[c.id] || { instrumentos: [] };
    items = data.instrumentos.filter(
      (i) => state.filter === "todos" || i.tema === state.filter
    );
  }

  $("legal-contador").textContent = `${items.length} normas`;

  if (!items.length) {
    $("legal-lista").innerHTML = `<p class="vacio">${esc(t("no_country"))}</p>`;
    return;
  }

  $("legal-lista").innerHTML = items
    .map(
      (i) => `
      <div class="norma">
        <h4>${esc(i.titulo)}</h4>
        <div class="norma-meta">
          <span class="badge">${esc(topicLabel(i.tema))}</span>
          ${verificationBadge(i.verificacion)}
        </div>
        ${i.url ? `<a href="${esc(i.url)}" target="_blank" rel="noopener">${icon("link")}<span>${esc(i.url)}</span></a>` : ""}
        ${(i.alternativas || [])
          .map(
            (alt) =>
              `<a href="${esc(alt)}" target="_blank" rel="noopener">${icon("link")}<span>Alternativa: ${esc(alt)}</span></a>`
          )
          .join("")}
        ${i.nota ? `<p class="multi-nota">${esc(i.nota)}</p>` : ""}
        ${i.texto ? `<button class="boton leer-norma" data-norma="${esc(i.id)}">${icon("book")}Leer norma íntegra</button>` : ""}
      </div>`
    )
    .join("");

  $("legal-lista").querySelectorAll("[data-norma]").forEach((btn) => {
    btn.addEventListener("click", () => abrirNorma(btn.dataset.norma));
  });
}

function abrirNorma(id) {
  const c = getCountry();
  const data = window.DATA.legislation[c.id] || { instrumentos: [] };
  const norma = data.instrumentos.find((i) => i.id === id);
  if (!norma || !norma.texto) return;
  $("norma-titulo").textContent = norma.titulo;
  $("norma-meta").textContent = [
    norma.paginas ? `Páginas ${norma.paginas} de la Gaceta Oficial` : "Gaceta Oficial",
    norma.archivo_local || "",
  ].filter(Boolean).join(" · ");
  $("norma-texto").textContent = norma.texto;
  const pdf = $("norma-pdf");
  if (norma.url) {
    pdf.href = norma.url;
    pdf.classList.remove("oculto");
  } else {
    pdf.classList.add("oculto");
  }
  $("norma-modal").classList.remove("oculto");
}

/* --- Sustancias --------------------------------------------------------- */

function levenshtein(a, b) {
  const prev = Array.from({ length: b.length + 1 }, (_, i) => i);
  const curr = new Array(b.length + 1).fill(0);
  for (let i = 0; i < a.length; i++) {
    curr[0] = i + 1;
    for (let j = 0; j < b.length; j++) {
      const cost = a[i] === b[j] ? 0 : 1;
      curr[j + 1] = Math.min(prev[j + 1] + 1, curr[j] + 1, prev[j] + cost);
    }
    for (let j = 0; j <= b.length; j++) prev[j] = curr[j];
  }
  return prev[b.length];
}

function fuzzyTokens(text) {
  return normalize(text)
    .split(/[^a-z0-9]+/)
    .filter(Boolean);
}

function fuzzyMatch(query, text) {
  const stopwords = new Set(["de", "del", "la", "el", "los", "las", "y", "para", "con"]);
  const q = fuzzyTokens(query).filter((w) => !stopwords.has(w));
  if (!q.length) return false;
  const hay = fuzzyTokens(text);
  return q.every((token) =>
    hay.some((word) => {
      if (word.includes(token) || token.includes(word)) return true;
      const max = token.length <= 5 ? 1 : 2;
      return levenshtein(token, word) <= max;
    })
  );
}

function matchingSubstances(query) {
  if (!query.trim()) return [];
  const q = normalize(query.trim());
  const numerico = /[0-9]/.test(q);
  return window.DATA.substances.filter((s) => {
    const fields = [s.nombre, (s.sinonimos || []).join(" "), s.cas, s.ce, s.onu];
    const texto = fields.join(" ");
    return numerico ? normalize(texto).includes(q) : fuzzyMatch(query, texto);
  });
}

function renderResults() {
  const results = matchingSubstances(state.query);
  $("resultados").innerHTML = results
    .map(
      (s) =>
        `<button class="resultado ${state.substanceId === s.id ? "activo" : ""}" data-sustancia="${esc(s.id)}">${esc(s.nombre)} · ${esc(s.cas)}</button>`
    )
    .join("");

  $("resultados").querySelectorAll("[data-sustancia]").forEach((btn) => {
    btn.addEventListener("click", () => {
      state.substanceId = btn.dataset.sustancia;
      renderResults();
      renderFicha();
    });
  });
}

function prop(label, value) {
  if (!value) return "";
  return `<div class="prop"><b>${esc(label)}</b>${esc(value)}</div>`;
}

function renderFicha() {
  const s = window.DATA.substances.find((x) => x.id === state.substanceId);
  if (!s) {
    $("ficha").innerHTML = `
      <div class="ficha-vacia">
        <div class="ficha-vacia-icono">${icon("file", "ico ico-xl")}</div>
        <h4>${esc(t("pick_substance"))}</h4>
        <p>${esc(t("pick_substance_sub"))}</p>
      </div>`;
    return;
  }

  const secciones = s.secciones || [];
  const index = secciones
    .map((sec) => `<a href="#sec-${esc(s.id)}-${esc(sec.id)}">${sec.numero}. ${esc(sec.titulo)}</a>`)
    .join("");

  const verificado = s.estado === "verificado";
  const estadoBadge = verificado
    ? `<span class="badge badge-ok">${icon("check")}FDS VERIFICADA</span>`
    : `<span class="badge badge-demo">${icon("info")}DEMO · 16 SECCIONES</span>`;

  $("ficha").innerHTML = `
    <div class="ficha-titulo">
      <h4>${esc(s.nombre)}</h4>
      ${estadoBadge}
    </div>
    <div class="identificadores">
      <span class="ident">CAS ${esc(s.cas)}</span>
      <span class="ident">CE ${esc(s.ce)}</span>
      <span class="ident">ONU ${esc(s.onu)}</span>
      <span class="ident">${esc(s.palabra_advertencia || "Peligro")}</span>
      ${verificado && s.fecha_verificacion ? `<span class="ident">Verificada: ${esc(s.fecha_verificacion)}</span>` : ""}
      ${verificado && s.verificado_por ? `<span class="ident">Por: ${esc(s.verificado_por)}</span>` : ""}
    </div>
    <div class="fds-index">${index}</div>
    ${secciones.map((sec) => `
      <div class="seccion" id="sec-${esc(s.id)}-${esc(sec.id)}">
        <h5>${sec.numero}. ${esc(sec.titulo)}</h5>
        ${sec.texto ? `<p>${esc(sec.texto)}</p>` : ""}
        ${sec.campos && sec.campos.length ? `<div class="prop-grid">${sec.campos.map((c) => `<div class="prop"><b>${esc(c.k)}</b>${esc(c.v)}</div>`).join("")}</div>` : ""}
        ${(sec.listas || []).map((l) => `<p class="lista-titulo">${esc(l.titulo)}</p><ul class="lista">${l.items.map((i) => `<li>${esc(i)}</li>`).join("")}</ul>`).join("")}
        ${(sec.notas || []).map((n) => `<p class="nota">${icon("info")}${esc(n)}</p>`).join("")}
      </div>`).join("")}
  `;
}

/* --- Multi sustancia: análisis de reacciones ---------------------------- */

function nivelInfo(nivel) {
  const db = window.DATA.interactions || { niveles: {} };
  const obj = db.niveles[nivel] || { etiqueta: nivel, descripcion: "" };
  const cls = { alto: "badge-danger", moderado: "badge-warn", bajo: "badge-ok", nulo: "badge-suave" }[nivel] || "badge-suave";
  return { ...obj, cls };
}

function analyzeInteractionsJS(ids) {
  const db = window.DATA.interactions || { interacciones: [] };
  const sustancias = window.DATA.substances;
  const results = [];
  for (let i = 0; i < ids.length; i++) {
    for (let j = i + 1; j < ids.length; j++) {
      const a = ids[i], b = ids[j];
      if (a === b) continue;
      const pair = [a, b].sort();
      const found = (db.interacciones || []).find((it) => {
        const s = [...it.sustancias].sort();
        return s[0] === pair[0] && s[1] === pair[1];
      });
      const na = sustancias.find((s) => s.id === a)?.nombre || a;
      const nb = sustancias.find((s) => s.id === b)?.nombre || b;
      if (found) {
        results.push({ ...found, nombre_a: na, nombre_b: nb, es_regla_general: false });
      } else {
        results.push({
          nivel: "nulo",
          tipo: "Sin reacción registrada",
          descripcion: `No hay una interacción registrada entre ${na} y ${nb} en la base actual. No se debe asumir que son compatibles: verificar en las fichas de datos de seguridad antes de cualquier mezcla.`,
          productos: [],
          consecuencias: [],
          acciones: ["No mezclar sin verificar la compatibilidad en fuentes oficiales"],
          nombre_a: na,
          nombre_b: nb,
          es_regla_general: true,
        });
      }
    }
  }
  return results;
}

function renderInteractions() {
  const box = $("interacciones");
  if (state.multi.length < 2) {
    box.innerHTML = "";
    $("multi-nota").textContent = "Selecciona al menos dos sustancias para analizar las reacciones entre todas las parejas.";
    $("multi-badge").textContent = "ACTIVO";
    return;
  }
  const results = analyzeInteractionsJS(state.multi);
  $("multi-badge").textContent = `${results.length} parejas`;
  $("multi-nota").textContent = window.DATA.interactions.advertencia;
  box.innerHTML = results.map((r) => {
    const info = nivelInfo(r.nivel);
    return `
      <div class="interaccion">
        <div class="interaccion-cabecera">
          <strong>${esc(r.nombre_a)} + ${esc(r.nombre_b)}</strong>
          <span class="badge ${info.cls}">${esc(info.etiqueta)}</span>
        </div>
        <div class="interaccion-tipo">${esc(r.tipo)}</div>
        <p>${esc(r.descripcion)}</p>
        ${r.productos && r.productos.length ? `<p class="interaccion-productos"><b>Productos:</b> ${r.productos.map((x) => esc(x)).join(", ")}</p>` : ""}
        ${r.consecuencias && r.consecuencias.length ? `<p class="lista-titulo">Consecuencias</p><ul class="lista">${r.consecuencias.map((x) => `<li>${esc(x)}</li>`).join("")}</ul>` : ""}
        ${r.acciones && r.acciones.length ? `<p class="lista-titulo">Acciones recomendadas</p><ul class="lista">${r.acciones.map((x) => `<li>${esc(x)}</li>`).join("")}</ul>` : ""}
        ${r.es_regla_general ? `<p class="multi-nota">Regla general: la compatibilidad debe verificarse en fuentes oficiales.</p>` : ""}
      </div>`;
  }).join("");
}

function renderMulti() {
  $("multi-chips").innerHTML = state.multi
    .map((id) => {
      const s = window.DATA.substances.find((x) => x.id === id);
      return s ? `<span class="multi-chip">${esc(s.nombre)}</span>` : "";
    })
    .join("");
  renderInteractions();
}

/* --- Arranque ----------------------------------------------------------- */

function renderAll() {
  renderBanner();
  renderFilters();
  renderLegal();
  renderResults();
  renderFicha();
  renderMulti();
}

function setMovilTab(tab) {
  $("panel-legal").classList.toggle("movil-activo", tab === "legal");
  $("panel-sustancia").classList.toggle("movil-activo", tab === "ficha");
  $("tab-legal").classList.toggle("activo", tab === "legal");
  $("tab-ficha").classList.toggle("activo", tab === "ficha");
  window.scrollTo({ top: 0, behavior: "smooth" });
}

function init() {
  setMovilTab("legal");
  $("tab-legal").addEventListener("click", () => setMovilTab("legal"));
  $("tab-ficha").addEventListener("click", () => setMovilTab("ficha"));
  renderCountryGrids();
  $("aviso-demo").innerHTML = `${icon("alert")}<span>${esc(window.DATA.warning)}</span>`;
  $("pie-advertencia").textContent = window.DATA.warning;

  $("cerrar-modal").addEventListener("click", () => {
    $("pais-modal").classList.add("oculto");
  });

  $("cerrar-norma").addEventListener("click", () => {
    $("norma-modal").classList.add("oculto");
  });

  $("norma-modal").addEventListener("click", (e) => {
    if (e.target === $("norma-modal")) $("norma-modal").classList.add("oculto");
  });

  $("flotante-pais").addEventListener("click", () => {
    $("pais-modal").classList.remove("oculto");
  });

  $("buscador-sustancias").addEventListener("input", (e) => {
    state.query = e.target.value;
    renderResults();
  });

  $("multi-boton").addEventListener("click", () => {
    const found = matchingSubstances($("multi-input").value)[0];
    if (found && !state.multi.includes(found.id)) {
      state.multi.push(found.id);
      $("multi-input").value = "";
      renderMulti();
    }
  });

  $("multi-input").addEventListener("keydown", (e) => {
    if (e.key === "Enter") $("multi-boton").click();
  });

  document.addEventListener("keydown", (e) => {
    if (e.key === "Escape") {
      $("pais-modal").classList.add("oculto");
      $("norma-modal").classList.add("oculto");
    }
  });

  renderAll();
}

init();