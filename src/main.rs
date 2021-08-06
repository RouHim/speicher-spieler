use std::collections::HashMap;

use actix_web::{App, Error, HttpResponse, HttpServer, middleware, Result, web};
use rusqlite::Connection;
use tera::{Context, Tera};

mod player_state;
mod player;
mod kv_store;

// https://www.youtube.com/watch?v=mSOXgy0SRvI
// https://www.mixcloud.com/ManoelCandido/artbat-watergate-open-air-2019-beatport-live/
// https://soundcloud.com/astateoftrance/sets/chicane-an-ocean-apart-ruben
// https://vimeo.com/8877539

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let tera = Tera::new("templates/**/*.html")
            .unwrap();

        let connection: Connection = rusqlite::Connection::open(":memory:").unwrap();
        player_state::prepare(&connection);

        App::new()
            .data(tera)
            .data(connection)
            .wrap(middleware::Logger::default()) // enable logger
            .route("/", web::get().to(web_get_handler))
            .route("/api/play", web::post().to(api_play_handler))
            .route("/api/stop", web::post().to(api_stop_handler))
            // .service(web::resource("/api").route(web::get().to(api_get_handler)))
            .service(web::scope(""))
    })
        .bind("0.0.0.0:2555")?
        .run()
        .await
}

async fn web_get_handler(
    tera: web::Data<tera::Tera>,
    db_connection: web::Data<Connection>,
    query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    let state = player_state::get(db_connection.get_ref(), "1").await;

    println!("GET / {}", state);

    let content = tera.render(
        "index.html",
        &Context::from_serialize(&state).unwrap(),
    ).unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(content))
}


async fn api_get_handler(
    tera: web::Data<tera::Tera>,
    db_connection: web::Data<Connection>,
    query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().content_type("text/html").body("content"))
}

async fn api_play_handler(
    db_connection: web::Data<Connection>,
    payload: web::Bytes,
) -> Result<HttpResponse, Error> {
    let body_as_string = std::str::from_utf8(payload.as_ref()).unwrap().to_string();

    player::play(
        db_connection.as_ref(),
        "1",
        body_as_string,
    ).await;

    Ok(HttpResponse::Ok().finish())
}


async fn api_stop_handler(
    db_connection: web::Data<Connection>,
) -> Result<HttpResponse, Error> {
    player::stop(
        db_connection.as_ref(),
        "1",
    ).await;

    Ok(HttpResponse::Ok().finish())
}
