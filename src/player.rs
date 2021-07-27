use crate::storage;
use crate::storage::PlayerStateDbConn;

pub async fn play(queue: String, conn: &PlayerStateDbConn) {
    let mut state = storage::read(conn, 1).await;
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

    storage::write(conn, &state).await;
}

fn split_urls(queue_string: String) -> Vec<String> {
    return queue_string.trim()
        .split('\n')
        .map(|url| url.to_string())
        .filter(|url| !url.is_empty())
        .collect();
}

pub async fn stop(conn: &PlayerStateDbConn) {
    let mut state = storage::read(conn, 1).await;
    state.player_playing = false;
    state.caching_url = "".to_string();
    state.playing_file_path = "".to_string();
    storage::write(conn, &state).await;
}