#[macro_use]
extern crate diesel;
use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::Logger, web::{Data, get}, HttpRequest, Result};
use std::path::PathBuf;
use actix_files::NamedFile;
mod mysql;
mod schema;
mod models;
mod api;

async fn covers(req: HttpRequest) -> Result<NamedFile> {
    let mut path = PathBuf::from(format!("static/covers/{}", req.match_info().query("filename")));
     match path.metadata() {
        Ok(data) => {
            if data.file_type().is_dir() {
                path = PathBuf::from("static/private/nofile.txt");
            } 
        },
        Err(_) => ()
    } 
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    HttpServer::new( || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .wrap(Cors::permissive())
            .route("/covers/{filename:.*}", get().to(covers))
            .app_data(Data::new(mysql::establish_connection().unwrap()))
            .service(api::test::search_movie_empty)
            .service(api::test::search_movie)
            .service(api::test::route_function_example)
            .service(api::test::upload_episodes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
