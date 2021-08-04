use std::fmt::{Display, Formatter};
use std::fmt;

use rusqlite::{Connection};
use serde::{Deserialize, Serialize};

use crate::kv_store;

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

pub fn prepare(connection: &Connection) {
    kv_store::setup(connection);

    let initial_state = PlayerState {
        id: 1,
        playing_file_path: "".to_string(),
        playing_file_type: "".to_string(),
        caching_url: "".to_string(),
        queueing_urls: "".to_string(),
        player_playing: false,
    };

    kv_store::put(
        connection,
        "1",
        initial_state,
    );
}

pub async fn set(db_connection: &Connection, key: &str, value: PlayerState) {
    kv_store::put(db_connection, key, value);
}

pub async fn get(db_connection: &Connection, key: &str) -> PlayerState {
    kv_store::get(db_connection, key)
}