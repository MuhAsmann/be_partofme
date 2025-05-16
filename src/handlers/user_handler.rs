use actix_web::{get, post, web, HttpResponse, Responder};
use diesel::prelude::*;
use crate::{models::user::*, schema::users::dsl::*, db::DbPool};

#[get("/users")]
pub async fn get_users(pool: web::Data<DbPool>) -> impl Responder {
    let pool = pool.clone(); // clone for thread safety

    let result = web::block(move || {
        let mut conn = pool.get().expect("Failed to get DB connection");
        users.load::<User>(&mut conn)
    })
    .await;

    match result {
        Ok(Ok(user_list)) => HttpResponse::Ok().json(user_list),
        Ok(Err(_diesel_err)) => HttpResponse::InternalServerError().body("Database error"),
        Err(_blocking_err) => HttpResponse::InternalServerError().body("Thread pool error"),
    }
}


#[post("/users")]
pub async fn create_user(
    pool: web::Data<DbPool>,
    user: web::Json<NewUser>,
) -> impl Responder {
    let pool = pool.clone();
    let new_user = user.into_inner();

    let result = web::block(move || {
        let mut conn = pool.get().expect("Failed to get DB connection");
        diesel::insert_into(users)
            .values(&new_user)
            .get_result::<User>(&mut conn)
    })
    .await;

    match result {
        Ok(Ok(inserted_user)) => HttpResponse::Ok().json(inserted_user),
        Ok(Err(_diesel_err)) => HttpResponse::InternalServerError().body("Database error"),
        Err(_blocking_err) => HttpResponse::InternalServerError().body("Thread pool error"),
    }
}

