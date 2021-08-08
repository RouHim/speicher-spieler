use std::collections::HashMap;

use actix_web::{App, Error, HttpResponse, HttpServer, middleware, Result, web};
use rusqlite::Connection;
use tera::{Context, Tera};
use r2d2_sqlite::SqliteConnectionManager;
use r2d2::{PooledConnection, Pool};

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

        let manager = SqliteConnectionManager::file("data.db");
        let pool = r2d2::Pool::new(manager).unwrap();

        player::prepare(&pool);

        App::new()
            .data(tera)
            .data(pool)
            .wrap(middleware::Logger::default()) // enable logger
            .route("/", web::get().to(web_get_handler))
            .route("/api/play", web::post().to(api_play_handler))
            .route("/api/stop", web::post().to(api_stop_handler))
            .service(web::scope(""))
    })
        .bind("0.0.0.0:2555")?
        .run()
        .await
}

async fn web_get_handler(
    tera: web::Data<tera::Tera>,
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, Error> {
    let state = player::get(&pool.get().unwrap());

    println!("GET / {}", &state);

    let content = tera.render(
        "index.html",
        &Context::from_serialize(&state).unwrap(),
    ).unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(content))
}

async fn api_play_handler(
    pool: web::Data<Pool<SqliteConnectionManager>>,
    payload: web::Bytes,
) -> Result<HttpResponse, Error> {
    let body_as_string = std::str::from_utf8(payload.as_ref()).unwrap().to_string();

    println!("PUT /api/play {}", &body_as_string);

    player::play(
        pool.as_ref(),
        body_as_string,
    ).await;

    Ok(HttpResponse::Ok().finish())
}

async fn api_stop_handler(
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, Error> {
    println!("PUT /api/stop");

    player::stop(pool.as_ref()).await;

    Ok(HttpResponse::Ok().finish())
}
