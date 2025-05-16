mod config;
mod db;
mod models;
mod handlers;
mod routes;
mod schema;

use actix_web::{App, HttpServer, web};
use crate::{
    config::init, 
    db::establish_connection, 
    routes::{
        health::health_check, 
        users::user_routes
    }
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init();
    let pool = establish_connection();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // <-- inject DbPool
            .route("/health", web::get().to(health_check)) // health check
            .configure(user_routes) // <-- daftarkan routes user
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
