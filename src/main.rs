mod config;
mod db;
mod models;
mod handlers;
mod routes;
mod schema;

use actix_web::{App, HttpServer, web};
use crate::{config::init, db::establish_connection, routes::{health::health_check, users::user_routes}};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init();
    let pool = establish_connection();

    HttpServer::new(move || {
        App::new()
            .route("/health", web::get().to(routes::health::health_check))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
