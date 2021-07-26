use diesel::{ExpressionMethods, QueryDsl};
use rocket::{Build, Rocket};
use rocket::serde::{Deserialize,  Serialize};
use rocket_sync_db_pools::database;
use rocket_sync_db_pools::diesel::RunQueryDsl;

use crate::player_states::*;

#[database("player_state")]
pub struct PlayerStateDbConn(diesel::SqliteConnection);

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name = "player_states"]
pub struct PlayerState {
    pub(crate) id: i32,
    pub(crate) playing_file_path: String,
    pub(crate) playing_file_type: String,
    pub(crate) caching_url: String,
    pub(crate) queueing_urls: String,
    #[serde(skip_deserializing)]
    pub(crate) player_playing: bool,
}

table! {
    player_states (id) {
        id -> Integer,
        playing_file_path -> Text,
        playing_file_type -> Text,
        caching_url -> Text,
        queueing_urls -> Text,
        player_playing -> Bool,
    }
}

pub async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    embed_migrations!("migrations");
    let db_connection = PlayerStateDbConn::get_one(&rocket).await.expect("database connection");
    db_connection.run(|c| embedded_migrations::run(c)).await.expect("diesel migrations");
    return rocket;
}

pub async fn create(db_con: &PlayerStateDbConn, state: PlayerState) {
    db_con.run(|conn| {
        diesel::insert_into(player_states::table)
            .values(state)
            .execute(conn)
    }).await;
}

pub async fn write(db_con: &PlayerStateDbConn, state: PlayerState) {
    db_con.run(|conn| {
        diesel::update(dsl::player_states.find(state.id))
            .set((
                playing_file_path.eq(state.playing_file_path),
                playing_file_type.eq(state.playing_file_type),
                caching_url.eq(state.caching_url),
                queueing_urls.eq(state.queueing_urls),
                player_playing.eq(state.player_playing)
            ))
            .execute(conn)
    }).await;
}

pub async fn read(db_con: &PlayerStateDbConn, player_state_id: i32) -> PlayerState {
    db_con.run(move |conn| {
        player_states::table
            .filter(player_states::id.eq(player_state_id))
            .first::<PlayerState>(conn)
    }).await.unwrap()
}