use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web::error::ErrorUnauthorized;
use actix_web_lab::middleware::Next;
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::models::auth::Claims;
use crate::config::get_jwt_secret;

pub async fn jwt_middleware(
    req: ServiceRequest,
    next: Next<impl actix_web::dev::ServiceFactory>,
) -> Result<actix_web::dev::ServiceResponse, Error> {
    // Ambil Authorization header
    let token_opt = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|header| {
            if header.starts_with("Bearer ") {
                Some(header.trim_start_matches("Bearer ").to_owned())
            } else {
                None
            }
        });

    // Jika tidak ada atau format salah
    let token = match token_opt {
        Some(t) => t,
        None => return Err(ErrorUnauthorized("No valid Authorization header found")),
    };

    // Decode token
    let decoded = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(get_jwt_secret().as_ref()),
        &Validation::default(),
    )
    .map_err(|_| ErrorUnauthorized("Invalid token"))?;

    // Masukkan user_id ke dalam extensions untuk digunakan handler
    req.extensions_mut().insert(decoded.claims.sub);

    // Lanjutkan ke handler berikutnya
    next.call(req).await
}
