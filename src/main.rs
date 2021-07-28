use std::borrow::BorrowMut;
use std::collections::HashMap;

use actix_http::{body::Body, Response};
use actix_web::{App, error, Error, HttpResponse, HttpServer, middleware, Result, web};
use actix_web::dev::ServiceResponse;
use actix_web::http::StatusCode;
use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use tera::{Context, Tera};

use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};

use crate::storage::{create, read, write};
use crate::storage::PlayerState;

mod storage;
mod player;

// https://www.youtube.com/watch?v=mSOXgy0SRvI
// https://www.mixcloud.com/ManoelCandido/artbat-watergate-open-air-2019-beatport-live/
// https://soundcloud.com/astateoftrance/sets/chicane-an-ocean-apart-ruben
// https://vimeo.com/8877539

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let tera = Tera::new("templates/**/*.html")
            .unwrap();

        let db_manager = SqliteConnectionManager::memory();
        let db_pool = Pool::new(db_manager).unwrap();
        storage::init_database(&db_pool);

        App::new()
            .data(tera)
            .data(db_pool.clone())
            .wrap(middleware::Logger::default()) // enable logger
            .service(web::resource("/").route(web::get().to(index_handler)))
            .service(web::scope(""))
    })
        .bind("0.0.0.0:2555")?
        .run()
        .await
}

// store tera template in application state
async fn index_handler(
    tera: web::Data<tera::Tera>,
    db: web::Data<Pool<SqliteConnectionManager>>,
    query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    let connection = db.get().unwrap();

    let row = connection.query_row(
        "SELECT * FROM player_states WHERE id = :id;",
        &[(":id", 1)],
        |row| row.get(0),
    );

    // db.set("1", &PlayerState {
    //     id: 1,
    //     playing_file_path: "".to_string(),
    //     playing_file_type: "".to_string(),
    //     caching_url: "".to_string(),
    //     queueing_urls: "".to_string(),
    //     player_playing: false,
    // });
    //
    // let state: PlayerState = db.get("1").unwrap();

    let content = tera.render(
        "index.html",
        &Context::from_serialize(&state).unwrap(),
    ).unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(content))
}
