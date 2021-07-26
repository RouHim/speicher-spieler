use crate::storage;
use crate::storage::PlayerStateDbConn;

pub async fn play(queue: String, conn: &PlayerStateDbConn) {
    let mut state = storage::read(conn, 1).await;
    state.player_playing = true;
    state.queueing_urls = queue;
    storage::write(conn, &state).await;
}

pub async fn stop(conn: &PlayerStateDbConn) {
    let mut state = storage::read(conn, 1).await;
    state.player_playing = false;
    storage::write(conn, &state).await;
}