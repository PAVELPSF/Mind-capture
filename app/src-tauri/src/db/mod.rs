pub mod models;
pub mod schema;

use rusqlite::{Connection, Result};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

pub struct Database {
    pub conn: Mutex<Connection>,
    pub app_data_dir: PathBuf,
}

impl Database {
    pub fn new(app_data_dir: PathBuf) -> Result<Self> {
        fs::create_dir_all(&app_data_dir)
            .expect("failed to create app data directory");

        let db_path = app_data_dir.join("mindcapture.db");
        let conn = Connection::open(db_path)?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
        conn.execute_batch(schema::CREATE_TABLES)?;

        for idx in schema::CREATE_INDEXES {
            conn.execute_batch(idx)?;
        }

        Ok(Self {
            conn: Mutex::new(conn),
            app_data_dir,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn temp_dir() -> PathBuf {
        let mut dir = env::temp_dir();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        dir.push(format!("mindcapture-test-{}-{:x}", std::process::id(), nanos));
        dir
    }

    #[test]
    fn creates_database_file() {
        let dir = temp_dir();
        let db_path = dir.join("mindcapture.db");
        let _ = fs::remove_file(&db_path);

        let db = Database::new(dir.clone()).expect("should create database");

        let conn = db.conn.lock().unwrap();
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert!(count >= 6, "expected at least 6 tables, got {}", count);

        let _ = fs::remove_file(&db_path);
    }

    #[test]
    fn creates_all_required_tables() {
        let dir = temp_dir();
        let db_path = dir.join("mindcapture.db");
        let _ = fs::remove_file(&db_path);

        let db = Database::new(dir.clone()).expect("should create database");
        let conn = db.conn.lock().unwrap();

        let expected = vec!["tabs", "notes", "collections", "tab_collections", "reviews", "sync_log"];
        for table in &expected {
            let exists: bool = conn
                .query_row(
                    "SELECT COUNT(*) > 0 FROM sqlite_master WHERE type='table' AND name=?1",
                    [table],
                    |row| row.get(0),
                )
                .unwrap();
            assert!(exists, "table '{}' should exist", table);
        }

        let _ = fs::remove_file(&db_path);
    }

    #[test]
    fn enforces_foreign_keys() {
        let dir = temp_dir();
        let db_path = dir.join("mindcapture.db");
        let _ = fs::remove_file(&db_path);

        let db = Database::new(dir.clone()).expect("should create database");
        let conn = db.conn.lock().unwrap();

        let fk: bool = conn
            .query_row("PRAGMA foreign_keys", [], |row| row.get(0))
            .unwrap();
        assert!(fk, "foreign_keys should be ON");

        let _ = fs::remove_file(&db_path);
    }
}
