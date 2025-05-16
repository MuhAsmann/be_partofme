use actix_web::{delete, get, post, web, HttpResponse, Responder};
use diesel::prelude::*;
use crate::{models::user::*, schema::users::dsl::*, db::DbPool};

#[tracing::instrument(skip(pool))]
#[get("/users")]
pub async fn get_users(pool: web::Data<DbPool>) -> impl Responder {
    let pool = pool.clone(); // clone for thread safety

    tracing::info!("Fetching all users");

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

#[tracing::instrument(skip(pool))]
#[get("/user/{id}")]
pub async fn get_user_by_id(
    pool: web::Data<DbPool>,
    user_id: web::Path<i32>,
) -> impl Responder {
    let pool = pool.clone();

    let user_id_value = user_id.into_inner();

    let result = web::block(move || {
        let mut conn = pool.get().expect("Failed to get DB connection");
        users.filter(id.eq(user_id_value)).first::<User>(&mut conn)
    })
    .await;

    match result {
        Ok(Ok(user)) => HttpResponse::Ok().json(user),
        Ok(Err(_diesel_err)) => HttpResponse::NotFound().body("User not found"),
        Err(_blocking_err) => HttpResponse::InternalServerError().body("Thread pool error"),
    }
}

#[tracing::instrument(skip(pool, user))]
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

// delete_user
#[delete("/user/{id}")]
pub async fn delete_user(
    pool: web::Data<DbPool>,
    user_id: web::Path<i32>,
) -> impl Responder {
    let pool = pool.clone();
    let user_id_value = user_id.into_inner();

    let result = web::block(move || {
        let mut conn = pool.get().expect("Failed to get DB connection");
        diesel::delete(users.filter(id.eq(user_id_value))).execute(&mut conn)
    })
    .await;

    match result {
        Ok(Ok(_)) => HttpResponse::Ok().body("User deleted"),
        Ok(Err(_diesel_err)) => HttpResponse::NotFound().body("User not found"),
        Err(_blocking_err) => HttpResponse::InternalServerError().body("Thread pool error"),
    }
}

