use actix_web::{delete, get, post, web, HttpResponse, Responder, HttpRequest, Result, HttpMessage};
use diesel::prelude::*;
use bcrypt::verify;
use jsonwebtoken::{encode, Header, EncodingKey};
use crate::{models::user::*, schema::users::dsl::*, db::DbPool, models::auth::Claims,config::get_jwt_secret};


pub async fn get_current_user(req: HttpRequest) -> Result<HttpResponse> {
    let user_id = req.extensions().get::<i32>().cloned();

    match user_id {
        Some(uid) => Ok(HttpResponse::Ok().body(format!("User ID from token: {}", uid))),
        None => Ok(HttpResponse::Unauthorized().finish()),
    }
}


pub async fn login_user(
    pool: web::Data<DbPool>,
    payload: web::Json<LoginRequest>,
) -> Result<HttpResponse> {
    use crate::schema::users::dsl::*;

    let conn = &mut pool.get().unwrap();
    let user = users
        .filter(email.eq(&payload.email))
        .first::<User>(conn)
        .optional()
        .map_err(|_| actix_web::error::ErrorInternalServerError("Database query error"))?;

    if let Some(user) = user {
        let is_valid = verify(&payload.password, &user.password_hash)
            .map_err(|_| actix_web::error::ErrorInternalServerError("Password verification error"))?;
        if is_valid {
            let claims = Claims::new(user.id);
            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(get_jwt_secret().as_ref()),
            ).map_err(|_| actix_web::error::ErrorInternalServerError("Token encoding error"))?;
            return Ok(HttpResponse::Ok().json(LoginResponse { token }));
        }
    }

    Ok(HttpResponse::Unauthorized().body("Invalid email or password"))
}


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

