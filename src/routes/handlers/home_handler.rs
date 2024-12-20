use crate::utils::api_response;
use actix_web::{get, Responder};

#[get("/")]
async fn index() -> impl Responder {
    api_response::ApiResponse::new(200, "hello rustacean! 🦀".to_string())
}
