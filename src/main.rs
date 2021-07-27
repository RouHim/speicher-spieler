#[macro_use]
extern crate rocket;

use rocket::fairing::AdHoc;
use rocket::http::Method;
use rocket::serde::json::serde_json;
use rocket::serde::Serialize;
use rocket_dyn_templates::Template;

use crate::storage::{create, read, write};
use crate::storage::PlayerState;

mod storage;
mod player;

// https://www.youtube.com/watch?v=mSOXgy0SRvI
// https://www.mixcloud.com/ManoelCandido/artbat-watergate-open-air-2019-beatport-live/
// https://soundcloud.com/astateoftrance/sets/chicane-an-ocean-apart-ruben
// https://vimeo.com/8877539

#https://github.com/actix/examples/blob/master/template_engines/tera/Cargo.toml

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/api", routes![api_root,  play_route, stop_route])
        .attach(Template::fairing())
}

#[post("/play", data = "<queue>")]
async fn play_route(queue: String) {
    player::play(queue).await;
}

#[post("/stop")]
async fn stop_route() {
    player::stop().await;
}

#[get("/")]
async fn api_root() -> String {
    format!("{}", "Welcome to the API")
}

#[get("/")]
async fn index() -> Template {
    let state = read(1).await;
    return Template::render("index", &state);
}