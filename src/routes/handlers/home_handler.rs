use actix_web::{get, Responder};
use crate::utils::api_response;

#[get("/")]
async fn index() -> impl Responder {
    api_response::ApiResponse::new(200, "hello rustacean! 🦀".to_string())
}

#[get("/test")]
async fn test() -> impl Responder {
    api_response::ApiResponse::new(200, "hello rustacean! test 🦀".to_string())
}
