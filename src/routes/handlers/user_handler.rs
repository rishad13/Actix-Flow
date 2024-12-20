use crate::utils::{api_response, app_state, jwt::Claims};
use actix_web::{get, post, web};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, Set};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct UpdateUserJsonModel {
    name: String,
}

#[get("/test")]
async fn user(
    app_state: web::Data<app_state::AppState>,
    claims_data: Claims,
) -> Result<api_response::ApiResponse, api_response::ApiResponse> {
    let user_model = entity::user::Entity::find_by_id(claims_data.id)
        .one(&app_state.db)
        .await
        .map_err(|e| api_response::ApiResponse::new(500, e.to_string()))?
        .ok_or(api_response::ApiResponse::new(
            401,
            "user not found".to_owned(),
        ))?;
    Ok(api_response::ApiResponse::new(
        200,
        format!(
            "{{'name':'{}','email':'{}'}}",
            user_model.name, user_model.email
        ),
    ))
}

#[post("/update")]
async fn update_user(
    app_state: web::Data<app_state::AppState>,
    claims_data: Claims,
    update_user_json: web::Json<UpdateUserJsonModel>,
) -> Result<api_response::ApiResponse, api_response::ApiResponse> {
    let mut user_model = entity::user::Entity::find_by_id(claims_data.id)
        .one(&app_state.db)
        .await
        .map_err(|e| api_response::ApiResponse::new(500, e.to_string()))?
        .ok_or(api_response::ApiResponse::new(
            401,
            "user not found".to_owned(),
        ))?
        .into_active_model();
    user_model.name = Set(update_user_json.name.clone());
    user_model
        .update(&app_state.db)
        .await
        .map_err(|e| api_response::ApiResponse::new(500, e.to_string()))?;
    Ok(api_response::ApiResponse::new(200, "success".to_string()))
}
