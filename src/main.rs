#[macro_use]
extern crate lazy_static;

use actix_web::{web, App, HttpServer};
use util::variables::HOST;

pub mod routes;
pub mod structs;
pub mod util;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(routes::info::get))
            .route("/embed", web::get().to(routes::embed::get))
            .route("/proxy", web::get().to(routes::proxy::get))
    })
    .bind(HOST.clone())?
    .run()
    .await
}
