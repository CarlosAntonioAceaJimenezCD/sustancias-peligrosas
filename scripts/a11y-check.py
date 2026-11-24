#!/usr/bin/env python3
"""Escaneo de accesibilidad WCAG 2.1 AA con axe-core sobre la app servida.

Uso: python3 scripts/a11y-check.py http://127.0.0.1:8765/ [ruta_a_axe.min.js]

Devuelve 0 si no hay violaciones detectables y 1 si las hay.
"""
import json
import os
import sys
from pathlib import Path
from urllib.request import urlretrieve

from playwright.sync_api import sync_playwright

URL = sys.argv[1] if len(sys.argv) > 1 else "http://127.0.0.1:8765/"
AXE = sys.argv[2] if len(sys.argv) > 2 else "/tmp/axe.min.js"
if AXE.startswith("http"):
    AXE_URL = AXE
else:
    if not Path(AXE).exists():
        urlretrieve("https://cdnjs.cloudflare.com/ajax/libs/axe-core/4.10.2/axe.min.js", AXE)
    AXE_URL = None

TAGS = ["wcag2a", "wcag2aa", "wcag21a", "wcag21aa"]


def escanear(page, estado, esperar_selector=True):
    if esperar_selector:
        page.wait_for_selector(".continente-carpeta", timeout=60000)
    page.wait_for_timeout(500)
    resultado = page.evaluate(
        """async (tags) => {
            const r = await axe.run(document, { runOnly: { type: 'tag', values: tags } });
            return r.violations.map(v => ({
                id: v.id, impact: v.impact, help: v.help,
                nodes: v.nodes.slice(0, 4).map(n => ({
                    target: n.target, summary: n.failureSummary || ''
                }))
            }));
        }""",
        TAGS,
    )
    print(f"[{estado}] violaciones: {len(resultado)}")
    for v in resultado:
        print(f"  {v['id']} [{v['impact']}] {v['help']}")
        for n in v["nodes"]:
            print(f"    {n['target']}: {n['summary'][:120]}")
    return resultado


with sync_playwright() as p:
    navegador = getattr(p, os.environ.get("BROWSER", "chromium")).launch(headless=True)
    pagina = navegador.new_page(viewport={"width": 1440, "height": 900})
    pagina.goto(URL, wait_until="domcontentloaded", timeout=90000)
    if AXE_URL:
        pagina.add_script_tag(url=AXE_URL)
    else:
        pagina.add_script_tag(path=AXE)
    total = escanear(pagina, "selector de pais")

    # Estado con pais seleccionado
    europa = pagina.locator(".continente-carpeta", has_text="EUROPA")
    if europa.count():
        europa.locator("summary").click()
        europa.locator(".pais-boton", has_text="España").first.click()
        pagina.wait_for_timeout(600)
        total += escanear(pagina, "con pais seleccionado", esperar_selector=False)

    navegador.close()

if total:
    print(f"\nTotal: {len(total)} violaciones detectables")
    sys.exit(1)
print("\nSin violaciones detectables")
sys.exit(0)
