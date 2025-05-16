mod config;
mod db;
mod handlers;
mod models;
mod routes;
mod schema;

use handlers::user_handler::get_current_user;
use tracing::info;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::EnvFilter;
// use tracing_actix_web::TracingLogger;
use actix_web_lab::middleware::from_fn;
use middleware::auth::jwt_middleware;


use crate::{
    config::init,
    db::establish_connection,
    routes::{health::health_check, users::user_routes},
};

use actix_web::{web, App, HttpServer, middleware::Logger};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env()
            .add_directive("tracing_actix_web=trace".parse().unwrap())
            .add_directive("actix_web=info".parse().unwrap()),
        )
        .with_target(true)
        .init();

    init();

    let pool = establish_connection();

    info!("Server starting...");

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default()) // <- aktifkan middleware log
            .app_data(web::Data::new(pool.clone())) // <-- inject DbPool
            .route("/health", web::get().to(health_check)) // health check
            .configure(user_routes) // <-- daftarkan routes user
            .service(
                web::scope("/api")
                    .wrap(from_fn(jwt_middleware)) // ⬅️ wrap scope dengan middleware
                    .route("/me", web::get().to(get_current_user)) // contoh endpoint dengan auth
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
