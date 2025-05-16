use actix_web::web;
use crate::handlers::user_handler::{get_users, create_user, get_user_by_id, delete_user, login_user};

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users);
    cfg.service(create_user);
    cfg.service(get_user_by_id); // Uncomment if you want to use this route
    cfg.service(delete_user); // Uncomment if you want to use this route
    cfg.service(
        web::scope("/users")
            .route("/login", web::post().to(login_user))
    );
}
