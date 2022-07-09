#[macro_use]
extern crate diesel;
use actix_cors::Cors;
use actix_files::NamedFile;
use actix_web::{
    middleware::Logger,
    web::{get, Data},
    App, HttpRequest, HttpServer, Result,
};
use std::path::PathBuf;

mod api;
mod handle_field;
mod models;
mod mysql;
mod schema;
mod services;
mod errors;

async fn covers(req: HttpRequest) -> Result<NamedFile> {
    let mut path = PathBuf::from(format!(
        "static/covers/{}",
        req.match_info().query("filename")
    ));
    match path.metadata() {
        Ok(data) => {
            if data.file_type().is_dir() {
                path = PathBuf::from("static/private/nofile.txt");
            }
        }
        Err(_) => (),
    }
    Ok(NamedFile::open(path)?)
}

async fn movies(req: HttpRequest) -> Result<NamedFile> {
    let mut path = PathBuf::from(format!(
        "static/movies/{}",
        req.match_info().query("filename")
    ));
    match path.metadata() {
        Ok(data) => {
            if data.file_type().is_dir() {
                path = PathBuf::from("static/private/nofile.txt");
            }
        }
        Err(_) => (),
    }
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(|| {
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .wrap(Cors::permissive())
            .route("/covers/{filename:.*}", get().to(covers))
            .route("/movies/{filename:.*}", get().to(movies))
            .app_data(Data::new(services::Services::new(mysql::establish_connection().unwrap())))
            // .service(services::check_hashmap)
            .service(api::movie_db::create_movie)
            .service(api::movie_db::search_movie_empty)
            .service(api::movie_db::search_movie)
            .service(api::movie_loc_db::upload_episodes)
            .service(api::movie_info_db::movie_detail)
            .service(api::movie_loc_db::epi_data)
    })
    .bind(("127.0.0.1", 8080))?
    .bind(("192.168.178.5", 8080))?
    .run()
    .await
}
