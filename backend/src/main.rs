#[macro_use]
extern crate diesel;
use actix_web::{App, HttpServer, middleware::Logger};

mod mysql;
mod schema;
mod models;
mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    HttpServer::new( || {
        let logger = Logger::default(); 
        App::new()
            .wrap(logger)
            //.app_data(models::MovieService{conn: &conn})
            .service(api::test::search_movie_empty)
            .service(api::test::search_movie)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
