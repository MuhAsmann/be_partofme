use actix_web::{get, post, web, HttpResponse, Responder};
use diesel::prelude::*;
use crate::{models::user::*, schema::users::dsl::*, db::DbPool};

#[get("/users")]
pub async fn get_users(pool: web::Data<DbPool>) -> impl Responder {
    let conn = pool.get().expect("DB pool error");
    let result = web::block(move || users.load::<User>(&conn)).await;

    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/users")]
pub async fn create_user(
    pool: web::Data<DbPool>,
    user: web::Json<NewUser>,
) -> impl Responder {
    let conn = pool.get().expect("DB pool error");
    let new_user = user.into_inner();

    let inserted = web::block(move || {
        diesel::insert_into(users)
            .values(&new_user)
            .get_result::<User>(&conn)
    })
    .await;

    match inserted {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
