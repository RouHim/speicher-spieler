#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate rocket;

use rocket::fairing::AdHoc;
use rocket::serde::json::serde_json;
use rocket::serde::Serialize;
use rocket_dyn_templates::Template;

use crate::storage::{create, player_states, read, write};
use crate::storage::PlayerState;
use crate::storage::PlayerStateDbConn;

mod storage;

// https://www.youtube.com/watch?v=mSOXgy0SRvI
// https://www.mixcloud.com/ManoelCandido/artbat-watergate-open-air-2019-beatport-live/
// https://soundcloud.com/astateoftrance/sets/chicane-an-ocean-apart-ruben
// https://vimeo.com/8877539

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/api", routes![api_root,  play_route, stop_route])
        .attach(PlayerStateDbConn::fairing())
        .attach(AdHoc::on_ignite("Diesel Migrations", storage::run_migrations))
        .attach(Template::fairing())
}

#[post("/play", data = "<queue>")]
async fn play_route(queue: String, conn: PlayerStateDbConn) {
    let mut state = read(&conn, 1).await;
    state.queueing_urls = queue;
    state.player_playing = true;
    write(&conn, &state).await;
}

#[post("/stop")]
async fn stop_route(conn: PlayerStateDbConn) {
    let mut state = read(&conn, 1).await;
    state.player_playing = false;
    write(&conn, &state).await;
}

#[get("/")]
async fn api_root() -> String {
    format!("{}", "Welcome to the API")
}

#[get("/")]
async fn index(conn: PlayerStateDbConn) -> Template {
    let state = read(&conn, 1).await;
    return Template::render("index", &state);
}