use crate::utils::api_response::ApiResponse;
use crate::utils::jwt::encode_token;
use crate::utils::{api_response, app_state};
use actix_web::{post, web};
use entity::user;
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::Condition;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::Set;
use serde::Deserialize;
use serde::Serialize;
use sha256::digest;

#[derive(Deserialize, Serialize)]
struct RegisterJsonModel {
    username: String,
    password: String,
    email: String,
}

#[derive(Deserialize, Serialize)]
struct LoginJsonModel {
    email: String,
    password: String,
}

/// Handles a registration request
///
/// # Example
/// Request:
///
/// {
///     "username": "user",
///     "password": "userpassword",
///     "email": "user@example.com"
/// }
///
/// Response:
///
/// {
///     "status_code": 200,
///     "body": "1"
/// }
///
/// Returns a 500 status code if the user could not be inserted into the database.
#[post("/register")]
pub async fn register(
    app_state: web::Data<app_state::AppState>,
    register_json: web::Json<RegisterJsonModel>,
) -> Result<ApiResponse, ApiResponse> {
    let user_model = user::ActiveModel {
        email: Set(register_json.email.clone()),
        name: Set(register_json.username.clone()),
        password: Set(digest(&register_json.password)),
        ..Default::default()
    }
    .insert(&app_state.db)
    .await
    .map_err(|e| api_response::ApiResponse::new(500, e.to_string()))?;

    Ok(api_response::ApiResponse::new(
        200,
        format!("{:?}", user_model.id),
    ))
}

/// Handles a login request
///
/// # Example
/// Request:
///
/// {
///     "email": "user@example.com",
///     "password": "userpassword"
/// }
///
/// Response:
///
/// {
///     "status_code": 200,
///     "body": "{'token': 'your_jwt_token'}"
/// }
///
/// Returns a 401 status code if the user is not found or the password is incorrect.
/// Returns a 500 status code if there is a database error or token encoding fails.

#[post("/login")]
pub async fn login(
    app_state: web::Data<app_state::AppState>,
    login_json: web::Json<LoginJsonModel>,
) -> Result<ApiResponse, ApiResponse> {
    let user_data = entity::user::Entity::find()
        .filter(
            Condition::all()
                .add(entity::user::Column::Email.eq(&login_json.email))
                .add(entity::user::Column::Password.eq(digest(&login_json.password))),
        )
        .one(&app_state.db)
        .await
        .map_err(|e| api_response::ApiResponse::new(500, e.to_string()))?
        .ok_or(api_response::ApiResponse::new(
            401,
            "User not found".to_string(),
        ))?;
    let token = encode_token(user_data.email, user_data.id)
        .map_err(|e| api_response::ApiResponse::new(500, e.to_string()))?;

    Ok(api_response::ApiResponse::new(
        200,
        format!("{{'token': '{}'}}", token),
    ))
}
