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
    pub playing_file_path: String,
    pub playing_file_type: String,
    pub caching_url: String,
    pub queueing_urls: String,
    pub player_playing: bool,
}

impl Display for PlayerState {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

pub fn prepare(pool: &Pool<SqliteConnectionManager>) {
    let con = pool.get().unwrap();

    kv_store::setup(&con);

    kv_store::set(&con, "playing_file_path", "");
    kv_store::set(&con, "playing_file_type", "");
    kv_store::set(&con, "caching_url", "");
    kv_store::set(&con, "queueing_urls", "https://www.learningcontainer.com/wp-content/uploads/2020/05/sample-mp4-file.mp4
https://filesamples.com/samples/video/mp4/sample_960x400_ocean_with_audio.mp4
https://filesamples.com/samples/video/mp4/sample_640x360.mp4".to_string());
    kv_store::set(&con, "player_playing", false);
}

pub async fn play(pool: &Pool<SqliteConnectionManager>, queue: String) {
    let mut con = pool.get().unwrap();

    let mut url_queue: Vec<String> = split_urls(queue);
    if url_queue.len() < 1 {
        return;
    }

    let first = url_queue.get(0).unwrap().clone();
    url_queue.remove(0);

    kv_store::set(&con, "player_playing", true);
    kv_store::set(&con, "playing_file_path", first.as_str());
    kv_store::set(&con, "playing_file_type", "video/mp4");
    kv_store::set(&con, "queueing_urls", url_queue.join("\n").as_str());
}

pub async fn stop(pool: &Pool<SqliteConnectionManager>) {
    let con = pool.get().unwrap();

    kv_store::set(&con, "player_playing", false);
    kv_store::set(&con, "caching_url", "");
    kv_store::set(&con, "playing_file_path", "");
    kv_store::set(&con, "playing_file_type", "");
}

fn split_urls(queue_string: String) -> Vec<String> {
    return queue_string.trim()
        .split('\n')
        .map(|url| url.to_string())
        .filter(|url| !url.is_empty())
        .collect();
}

pub fn get(con: &PooledConnection<SqliteConnectionManager>) -> PlayerState {
    PlayerState {
        playing_file_path: kv_store::get(con, "playing_file_path"),
        playing_file_type: kv_store::get(con, "playing_file_type"),
        caching_url: kv_store::get(con, "caching_url"),
        queueing_urls: kv_store::get(con, "queueing_urls"),
        player_playing: kv_store::get(con, "player_playing"),
    }
}