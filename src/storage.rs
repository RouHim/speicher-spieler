use std::fmt::{Display, Formatter};
use std::fmt;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rocket::{Build, Rocket};
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::serde_json;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlayerState {
    pub(crate) id: i32,
    pub(crate) playing_file_path: String,
    pub(crate) playing_file_type: String,
    pub(crate) caching_url: String,
    pub(crate) queueing_urls: String,
    pub(crate) player_playing: bool,
}

impl Display for PlayerState {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

pub async fn create(state: &PlayerState) {}

pub async fn write(state: &PlayerState) {
    let cloned_state = state.clone();
}

pub async fn read(player_state_id: i32) -> PlayerState {
    return PlayerState {
        id: 0,
        playing_file_path: "".to_string(),
        playing_file_type: "".to_string(),
        caching_url: "".to_string(),
        queueing_urls: "".to_string(),
        player_playing: false,
    };
}

pub fn init_database(pool: &Pool<SqliteConnectionManager>) {
    let connection = pool.get().unwrap();

    // CREATE TABLE kv (key TEXT PRIMARY KEY, value TEXT)
    // https://github.com/the-lean-crate/criner/issues/1

    connection.execute("
    CREATE TABLE player_states (
        id                INTEGER PRIMARY KEY,
        playing_file_path TEXT    NOT NULL,
        playing_file_type TEXT    NOT NULL,
        caching_url       TEXT    NOT NULL,
        queueing_urls     TEXT    NOT NULL,
        player_playing    BOOLEAN NOT NULL DEFAULT 0
    );", []);

    connection.execute(
        "INSERT INTO player_states VALUES(1, '', '', '', 'abc\ncd', 0));",
        [],
    );
}