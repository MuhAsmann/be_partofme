use std::env;

// use std::env;
use dotenvy::dotenv;

pub fn init() {
    dotenv().ok();
}

pub fn get_jwt_secret() -> String {
    env::var("JWT_SECRET").expect("JWT_SECRET must be set")
}