use diesel::{ExpressionMethods, QueryDsl, update};
use rocket::{Build, Rocket};
use rocket::serde::{Deserialize, Serialize};
use rocket_sync_db_pools::database;
use rocket_sync_db_pools::diesel::RunQueryDsl;

use crate::player_states::*;
use rocket::serde::json::serde_json;

#[database("player_state")]
pub struct PlayerStateDbConn(diesel::SqliteConnection);

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, Identifiable)]
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

    // create table structure
    let db_connection = PlayerStateDbConn::get_one(&rocket).await.expect("database connection");

    // insert initial data
    db_connection.run(|c| embedded_migrations::run(c)).await.expect("diesel migrations");
    let mut state = PlayerState {
        id: 1,
        playing_file_path: "".to_string(),
        playing_file_type: "".to_string(),
        caching_url: "".to_string(),
        queueing_urls: "".to_string(),
        player_playing: false,
    };
    create(&db_connection, &state).await;

    return rocket;
}

pub async fn create(db_con: &PlayerStateDbConn, state: &PlayerState) {
    let cloned_state = state.clone();
    db_con.run(|conn| {
        diesel::insert_into(player_states::table)
            .values(cloned_state)
            .execute(conn)
    }).await;
}

pub async fn write(db_con: &PlayerStateDbConn, state: &PlayerState) {
    let cloned_state = state.clone();
    db_con.run(|conn| {
        diesel::update(player_states::dsl::player_states)
            .set((
                playing_file_path.eq(cloned_state.playing_file_path),
                playing_file_type.eq(cloned_state.playing_file_type),
                caching_url.eq(cloned_state.caching_url),
                queueing_urls.eq(cloned_state.queueing_urls),
                player_playing.eq(cloned_state.player_playing)
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