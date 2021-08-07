use rusqlite::Connection;

use std::fmt::{Display, Formatter};
use std::fmt;
use crate::{kv_store};
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use serde::{Deserialize, Serialize, Deserializer, Serializer};
use serde::de::Unexpected;
use serde::__private::de;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlayerState {
    pub(crate) id: i32,
    pub(crate) playing_file_path: String,
    pub(crate) playing_file_type: String,
    pub(crate) caching_url: String,
    pub(crate) queueing_urls: String,
    #[serde(deserialize_with = "int_to_bool")]
    // #[serde(serialize_with = "bool_to_int")]
    pub(crate) player_playing: bool,
}

fn int_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error> where D: Deserializer<'de> {
    match u8::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        _ => Ok(false)
    }
}

fn bool_to_int<'se, S>(serializer: S) -> Result<i32, S::Error> where S: Serializer {
    match bool::serialize(serializer)? {
        true => Ok(1),
        false => Ok(0),
    }
}

impl Display for PlayerState {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

pub fn prepare(pool: &Pool<SqliteConnectionManager>) {
    let con = pool.get().unwrap();

    kv_store::setup(&con);

    let initial_state = PlayerState {
        id: 1,
        playing_file_path: "".to_string(),
        playing_file_type: "".to_string(),
        caching_url: "".to_string(),
        queueing_urls: "https://www.learningcontainer.com/wp-content/uploads/2020/05/sample-mp4-file.mp4
https://filesamples.com/samples/video/mp4/sample_960x400_ocean_with_audio.mp4
https://filesamples.com/samples/video/mp4/sample_640x360.mp4".to_string(),
        player_playing: false,
    };

    kv_store::set(
        &con,
        "1",
        initial_state,
    );
}


pub async fn play(pool: &Pool<SqliteConnectionManager>, key: &str, queue: String) {
    let mut con = pool.get().unwrap();

    let mut url_queue: Vec<String> = split_urls(queue);
    if url_queue.len() < 1 {
        return;
    }

    let first = url_queue.get(0).unwrap().clone();
    url_queue.remove(0);

    kv_store::set_bool(&con, key, "player_playing", true);
    kv_store::set_string(&con, key, "playing_file_path", first.as_str());
    kv_store::set_string(&con, key, "playing_file_type", "video/mp4");
    kv_store::set_string(&con, key, "queueing_urls", url_queue.join("\n").as_str());
}

pub async fn stop(pool: &Pool<SqliteConnectionManager>, key: &str) {
    let mut con = pool.get().unwrap();

    kv_store::set_bool(&con, key, "player_playing", false);
    kv_store::set_string(&con, key, "caching_url", "");
    kv_store::set_string(&con, key, "playing_file_path", "");
    kv_store::set_string(&con, key, "playing_file_type", "");
}

fn split_urls(queue_string: String) -> Vec<String> {
    return queue_string.trim()
        .split('\n')
        .map(|url| url.to_string())
        .filter(|url| !url.is_empty())
        .collect();
}

pub fn get(con: &PooledConnection<SqliteConnectionManager>, key: &str) -> PlayerState {
    return kv_store::get(con, key);
}