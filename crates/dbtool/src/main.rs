//! Herramienta de base de datos del proyecto.
//!
//! Convierte los JSON versionados en una base SQLite embebida (un solo archivo,
//! sin servidor) con índices para búsqueda rápida. SQLite se compila incluida
//! (`bundled`), por lo que no depende de librerías del sistema.

use rusqlite::{params, Connection, Result};
use std::path::PathBuf;
use sustancias_core as core;

fn db_path() -> PathBuf {
    std::env::var_os("SUSTANCIAS_DB")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../data/sustancias.db")
        })
}

fn create_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        PRAGMA journal_mode = WAL;
        PRAGMA foreign_keys = ON;

        CREATE TABLE IF NOT EXISTS countries (
            id TEXT PRIMARY KEY,
            nombre TEXT NOT NULL,
            codigo TEXT NOT NULL,
            idioma TEXT NOT NULL,
            bandera TEXT NOT NULL,
            data TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS topics (
            id TEXT PRIMARY KEY,
            nombre TEXT NOT NULL,
            data TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS sources (
            id TEXT PRIMARY KEY,
            pais TEXT NOT NULL,
            tema TEXT NOT NULL,
            categoria TEXT NOT NULL,
            titulo TEXT NOT NULL,
            url TEXT NOT NULL,
            data TEXT NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_sources_pais ON sources(pais);
        CREATE INDEX IF NOT EXISTS idx_sources_tema ON sources(tema);

        CREATE TABLE IF NOT EXISTS instruments (
            id TEXT PRIMARY KEY,
            pais TEXT NOT NULL,
            tema TEXT NOT NULL,
            titulo TEXT NOT NULL,
            url TEXT NOT NULL,
            verificacion TEXT NOT NULL,
            data TEXT NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_instruments_pais ON instruments(pais);
        CREATE INDEX IF NOT EXISTS idx_instruments_tema ON instruments(tema);

        CREATE TABLE IF NOT EXISTS portals (
            id TEXT PRIMARY KEY,
            pais TEXT NOT NULL,
            institucion TEXT NOT NULL,
            url TEXT NOT NULL,
            data TEXT NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_portals_pais ON portals(pais);

        CREATE TABLE IF NOT EXISTS substances (
            id TEXT PRIMARY KEY,
            nombre TEXT NOT NULL,
            cas TEXT NOT NULL,
            ce TEXT NOT NULL,
            onu TEXT NOT NULL,
            data TEXT NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_substances_cas ON substances(cas);
        CREATE VIRTUAL TABLE IF NOT EXISTS substances_fts USING fts5(
            nombre, sinonimos, cas, ce, onu, entity_id UNINDEXED
        );
        ",
    )
}

fn clear(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "DELETE FROM substances_fts;
         DELETE FROM substances;
         DELETE FROM portals;
         DELETE FROM instruments;
         DELETE FROM sources;
         DELETE FROM topics;
         DELETE FROM countries;",
    )
}

fn build(conn: &Connection) -> Result<()> {
    create_schema(conn)?;
    clear(conn)?;

    let mut stmt = conn.prepare(
        "INSERT INTO countries (id, nombre, codigo, idioma, bandera, data)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
    )?;
    for c in core::countries() {
        let data = serde_json::to_string(&c).unwrap();
        stmt.execute(params![c.id, c.nombre, c.codigo, c.idioma, c.bandera, data])?;
    }

    let mut stmt = conn.prepare("INSERT INTO topics (id, nombre, data) VALUES (?1, ?2, ?3)")?;
    for t in core::topics() {
        let data = serde_json::to_string(&t).unwrap();
        stmt.execute(params![t.id, t.nombre, data])?;
    }

    let mut stmt = conn.prepare(
        "INSERT INTO sources (id, pais, tema, categoria, titulo, url, data)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
    )?;
    for s in core::sources() {
        let data = serde_json::to_string(&s).unwrap();
        stmt.execute(params![
            s.id, s.pais, s.tema, s.categoria, s.titulo, s.url, data
        ])?;
    }

    let db = core::legislation_db();
    let mut ins_stmt = conn.prepare(
        "INSERT INTO instruments (id, pais, tema, titulo, url, verificacion, data)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
    )?;
    let mut port_stmt = conn.prepare(
        "INSERT INTO portals (id, pais, institucion, url, data) VALUES (?1, ?2, ?3, ?4, ?5)",
    )?;
    for (pais, country) in &db.paises {
        for p in &country.portales_oficiales {
            let data = serde_json::to_string(&p).unwrap();
            port_stmt.execute(params![p.id, pais, p.institucion, p.url, data])?;
        }
        for i in &country.instrumentos {
            let data = serde_json::to_string(&i).unwrap();
            ins_stmt.execute(params![
                i.id, pais, i.tema, i.titulo, i.url, i.verificacion, data
            ])?;
        }
    }

    let mut stmt = conn.prepare(
        "INSERT INTO substances (id, nombre, cas, ce, onu, data)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
    )?;
    let mut fts_stmt = conn.prepare(
        "INSERT INTO substances_fts (nombre, sinonimos, cas, ce, onu, entity_id)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
    )?;
    for s in core::substances() {
        let data = serde_json::to_string(&s).unwrap();
        stmt.execute(params![s.id, s.nombre, s.cas, s.ce, s.onu, data])?;
        fts_stmt.execute(params![
            s.nombre,
            s.sinonimos.join(" "),
            s.cas,
            s.ce,
            s.onu,
            s.id
        ])?;
    }

    Ok(())
}

fn stats(conn: &Connection) -> Result<()> {
    for table in [
        "countries",
        "topics",
        "sources",
        "instruments",
        "portals",
        "substances",
    ] {
        let count: i64 = conn.query_row(&format!("SELECT COUNT(*) FROM {table}"), [], |r| {
            r.get(0)
        })?;
        println!("{table:<12} {count:>4}");
    }
    Ok(())
}

fn search(conn: &Connection, query: &str) -> Result<()> {
    let mut stmt = conn.prepare(
        "SELECT s.nombre, s.cas FROM substances_fts f
         JOIN substances s ON s.id = f.entity_id
         WHERE substances_fts MATCH ?1 ORDER BY s.nombre",
    )?;
    let pattern = format!("{query}*");
    let rows = stmt.query_map(params![pattern], |r| {
        Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?))
    })?;
    for row in rows {
        let (name, cas) = row?;
        println!("{name:<28} {cas}");
    }
    Ok(())
}

fn main() -> Result<()> {
    let path = db_path();
    let conn = Connection::open(&path)?;
    let command = std::env::args().nth(1).unwrap_or_else(|| "stats".into());

    match command.as_str() {
        "build" => {
            build(&conn)?;
            println!("Base SQLite creada en {}", path.display());
        }
        "stats" => stats(&conn)?,
        "search" => {
            let query = std::env::args().nth(2).unwrap_or_default();
            if query.is_empty() {
                eprintln!("Uso: sustancias-dbtool search <texto>");
            } else {
                search(&conn, &query)?;
            }
        }
        "path" => println!("{}", path.display()),
        other => {
            eprintln!("Comando desconocido: {other}");
            eprintln!("Uso: sustancias-dbtool <build|stats|search|path>");
        }
    }

    Ok(())
}
