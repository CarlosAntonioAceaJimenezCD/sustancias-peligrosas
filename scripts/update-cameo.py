#!/usr/bin/env python3
"""Comprueba mensualmente si CAMEO ha cambiado y avisa abriendo un issue.

No modifica los datos automáticamente: solo detecta diferencias para que una
persona las revise antes de actualizar el dataset de la aplicación.
"""
import hashlib
import json
import re
import urllib.request
from pathlib import Path

BASE = "https://cameochemicals.noaa.gov"
ROOT = Path(__file__).resolve().parent.parent
DATA = ROOT / "data"
CHECK = DATA / "cameo_check.json"


def get(url):
    req = urllib.request.Request(url, headers={"User-Agent": "sustancias-peligrosas/1.0"})
    with urllib.request.urlopen(req, timeout=40) as r:
        return r.read().decode("utf-8", "ignore")


def estado_actual():
    """Huella de los archivos de datos que dependen de CAMEO."""
    resultado = {}
    for nombre in ("cameo_groups.json", "cameo_reactions.json",
                   "cameo_substances.json", "cameo_reactions_min.json"):
        ruta = DATA / nombre
        if ruta.exists():
            resultado[nombre] = hashlib.sha256(ruta.read_bytes()).hexdigest()[:16]
    return resultado


def huella_remota():
    """Huella ligera de la versión y el índice de grupos de CAMEO."""
    try:
        inicio = get(f"{BASE}/")
        version = re.search(r"CAMEO Chemicals version ([^<\s]+)", inicio)
        grupos = get(f"{BASE}/help/reactivity/reactive_groups.htm")
        ids = sorted(set(re.findall(r"/react/(\d+)", grupos)), key=int)
        return {
            "version": version.group(1) if version else "desconocida",
            "grupos": len(ids),
            "ids": ",".join(ids),
        }
    except Exception as exc:  # pragma: no cover - solo en ejecución programada
        return {"error": str(exc)[:200]}


def main():
    remoto = huella_remota()
    local = estado_actual()
    anterior = json.loads(CHECK.read_text(encoding="utf-8")) if CHECK.exists() else {}

    if "error" in remoto:
        print(f"No se pudo comprobar CAMEO: {remoto['error']}")
        return 1

    cambios = []
    if remoto.get("version") != anterior.get("version"):
        cambios.append(f"Versión: {anterior.get('version')} → {remoto.get('version')}")
    if remoto.get("grupos") != anterior.get("grupos"):
        cambios.append(f"Grupos: {anterior.get('grupos')} → {remoto.get('grupos')}")
    if remoto.get("ids") != anterior.get("ids"):
        cambios.append("Han cambiado los identificadores de grupos reactivos")
    for nombre, huella in local.items():
        if anterior.get(nombre) and anterior[nombre] != huella:
            cambios.append(f"El archivo local {nombre} ha cambiado")

    CHECK.write_text(json.dumps({**remoto, **local}, ensure_ascii=False, indent=2) + "\n",
                     encoding="utf-8")

    if cambios:
        print("CAMBIOS DETECTADOS")
        for c in cambios:
            print(f"- {c}")
        print("\nIssue sugerido:")
        print("Título: [cameo] Revisar cambios en la base de reactividad")
        print("Cuerpo:\n" + "\n".join(f"- {c}" for c in cambios))
        return 2

    print("Sin cambios: la base de CAMEO sigue igual")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
