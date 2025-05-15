use actix_web::web;
use crate::handlers::user_handler::{get_users, create_user};

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users);
    cfg.service(create_user);
}
