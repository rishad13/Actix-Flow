use crate::utils::{api_response, app_state};
use actix_web::{get, web, Responder};
use sea_orm::{ConnectionTrait, Statement};

#[get("/")]
async fn index() -> impl Responder {
    api_response::ApiResponse::new(200, "hello rustacean! 🦀".to_string())
}

#[get("/test")]
async fn test(app_state: web::Data<app_state::AppState>) -> impl Responder {
    let db = &app_state.db;
    let res = db
        .query_all(Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            "SELECT * FROM user",
        ))
        .await
        .unwrap();
    api_response::ApiResponse::new(200, "hello rustacean! test 🦀".to_string())
}
