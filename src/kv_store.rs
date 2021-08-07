// REQUIRES:
// rusqlite = "0.25.3"
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"
//
// REQUIRES: struct that implements #[derive(Serialize, Deserialize)]

use serde::de::DeserializeOwned;
use serde::Serialize;

use rusqlite::{Connection, params, Result};
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;

// TODO: herausfinden warum im json das bool feld zu 0/1 konvertiert wird

pub fn setup(con: &PooledConnection<SqliteConnectionManager>) {
    con.execute_batch("
                PRAGMA journal_mode = WAL;          -- better write-concurrency
                PRAGMA synchronous = NORMAL;        -- fsync only in critical moments
            "
    ).unwrap();

    con.execute(
        "CREATE TABLE IF NOT EXISTS kv (
                  key             TEXT PRIMARY KEY NOT NULL,
                  value           TEXT
            );", [],
    ).unwrap();
}

pub fn set<T: Serialize>(con: &PooledConnection<SqliteConnectionManager>, key: &str, value: T) {
    let json_value = serde_json::to_string(&value).expect("convertable to json");
    con.execute(
        "REPLACE INTO kv (key, value) VALUES (?1, ?2)",
        params![key, json_value],
    ).unwrap();
}

pub fn set_string(con: &PooledConnection<SqliteConnectionManager>, key: &str, field: &str, value: &str) {
    let update_query = format!("UPDATE kv
                SET value = (select json_set(kv.value, '$.{}', '{}') from kv)
                WHERE key == ?1;
            ", field, value);
    con.execute(
        update_query.as_str(),
        params![key],
    ).unwrap();
}

pub fn set_bool(con: &PooledConnection<SqliteConnectionManager>, key: &str, field: &str, value: bool) {
    let update_query = format!("UPDATE kv
                SET value = (select json_set(kv.value, '$.{}', {}) from kv)
                WHERE key == ?1;
            ", field, value);
    con.execute(
        update_query.as_str(),
        params![key],
    ).unwrap();
}

pub fn get<T: DeserializeOwned>(con: &PooledConnection<SqliteConnectionManager>, key: &str) -> T {
    let mut stmt = con.prepare("SELECT value FROM kv WHERE key = ?1").unwrap();

    let result: Result<String, _> = stmt.query_row(
        params![key.clone()],
        |row| row.get(0),
    );

    let result_data = result.unwrap();
    return serde_json::from_str::<T>(result_data.as_str()).unwrap();
}