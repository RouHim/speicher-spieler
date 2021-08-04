use rusqlite::Connection;

use crate::player_state;

// pub async fn play(queue: String) {
//     let mut state = player_state::get( 1).await;
//     let mut url_queue: Vec<String> = split_urls(queue);
//
//     if url_queue.len() < 1 {
//         return;
//     }
//
//     let first = url_queue.get(0).unwrap().clone();
//     url_queue.remove(0);
//
//     state.player_playing = true;
//     state.playing_file_path = first.to_string();
//     state.playing_file_type = "video/mp4".to_string();
//     state.queueing_urls = url_queue.join("\n");
//
//     player_state::set( &state).await;
// }
//
fn split_urls(queue_string: String) -> Vec<String> {
    return queue_string.trim()
        .split('\n')
        .map(|url| url.to_string())
        .filter(|url| !url.is_empty())
        .collect();
}
//
// pub async fn stop() {
//     let mut state = player_state::get( 1).await;
//     state.player_playing = false;
//     state.caching_url = "".to_string();
//     state.playing_file_path = "".to_string();
//     player_state::set( &state).await;
// }

pub async fn play(db_connection: &Connection, key: &str, queue: String) {
    let mut state = player_state::get(db_connection, key).await;

    let mut url_queue: Vec<String> = split_urls(queue);
    if url_queue.len() < 1 {
        return;
    }

    let first = url_queue.get(0).unwrap().clone();
    url_queue.remove(0);

    state.player_playing = true;
    state.playing_file_path = first.to_string();
    state.playing_file_type = "video/mp4".to_string();
    state.queueing_urls = url_queue.join("\n");

    player_state::set(db_connection, key, state);
}