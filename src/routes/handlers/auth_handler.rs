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

/// Handles a register request
///
/// # Example
/// Request:
///
/// {
///     "username": "username",
///     "password": "password",
///     "email": "email"
/// }
///
/// Response:
///
/// {
///     "status_code": 200,
///     "body": "id"
/// }
#[post("/register")]
pub async fn register(
    app_state: web::Data<app_state::AppState>,
    register_json: web::Json<RegisterJsonModel>,
) -> impl actix_web::Responder {
    let user_model = user::ActiveModel {
        email: Set(register_json.email.clone()),
        name: Set(register_json.username.clone()),
        password: Set(digest(&register_json.password)),
        ..Default::default()
    }
    .insert(&app_state.db)
    .await
    .unwrap();

    api_response::ApiResponse::new(200, format!("{:?}", user_model.id))
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
///     "body": "username"
/// }
///
/// Returns a 401 status code if the user is not found.

#[post("/login")]
pub async fn login(
    app_state: web::Data<app_state::AppState>,
    login_json: web::Json<LoginJsonModel>,
) -> impl actix_web::Responder {
    let user = entity::user::Entity::find()
        .filter(
            Condition::all()
                .add(entity::user::Column::Email.eq(&login_json.email))
                .add(entity::user::Column::Password.eq(digest(&login_json.password))),
        )
        .one(&app_state.db)
        .await
        .unwrap();

    if user.is_none() {
        return api_response::ApiResponse::new(401, "user not found".to_string());
    }
    let user_data = user.unwrap();
    let token = encode_token(user_data.email, user_data.id).unwrap();

    api_response::ApiResponse::new(200, format!("{{'token': '{}'}}", token))
}
