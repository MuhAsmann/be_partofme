// use std::env;
use dotenvy::dotenv;

pub fn init() {
    dotenv().ok();
}
