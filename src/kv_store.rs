// REQUIRES:
// rusqlite = "0.25.3"
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"
//
// REQUIRES: struct that implements #[derive(Serialize, Deserialize)]

use serde::de::DeserializeOwned;
use serde::Serialize;

use rusqlite::{Connection, params, Result};

pub fn setup(con: &Connection) {
    con.execute_batch("
                PRAGMA journal_mode = WAL;          -- better write-concurrency
                PRAGMA synchronous = NORMAL;        -- fsync only in critical moments
            ").unwrap();

    con.execute(
        "CREATE TABLE IF NOT EXISTS kv (
                  key             TEXT PRIMARY KEY NOT NULL,
                  value           TEXT
            );", [],
    ).unwrap();
}

pub fn put<T: Serialize>(con: &Connection, key: &str, value: T) {
    let json_value = serde_json::to_string(&value).expect("convertable to json");
    con.execute(
        "REPLACE INTO kv (key, value) VALUES (?1, ?2)",
        params![key, json_value],
    ).unwrap();
}

pub fn get<T: DeserializeOwned>(con: &Connection, key: &str) -> T {
    let mut stmt = con.prepare("SELECT value FROM kv WHERE key = ?1").unwrap();

    let result: Result<String, _> = stmt.query_row(
        params![key.clone()],
        |row| row.get(0),
    );

    let result_data = result.unwrap();
    return serde_json::from_str::<T>(result_data.as_str()).unwrap();
}