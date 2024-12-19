use actix_web::{get, web};

use crate::utils::{api_response, app_state};



#[get("")]
async fn user( 
    app_state: web::Data<app_state::AppState>
) -> impl actix_web::Responder {
   api_response::ApiResponse::new(200, "user verified".to_string())
}