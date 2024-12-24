use std::path::PathBuf;

use crate::utils::{api_response, constants};
use crate::utils::{api_response::ApiResponse, app_state, jwt::Claims};
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_multipart::form::MultipartForm;
use actix_web::{get, post, web};
use chrono::{NaiveDateTime, Utc};
use sea_orm::{ActiveModelTrait, TransactionTrait};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(MultipartForm)]
pub struct CreatePost {
    title: Text<String>,
    text: Text<String>,
    file: TempFile,
}

#[derive(Deserialize, Serialize)]
pub struct PostModel {
    pub id: i32,
    pub title: String,
    pub text: String,
    pub uuid: Uuid,
    pub image: String,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    user: Option<UserModel>,
}
#[derive(Deserialize, Serialize)]

struct UserModel {
    id: i32,
    name: String,
    email: String,
}

/// Creates a new post in the database.
///
/// This function handles the request to create a new post, requiring the user to be authenticated.
/// It generates a new UUID for the post and timestamps it with the current time. The post is
/// associated with the authenticated user.
///
/// # Parameters
/// - `app_state`: The application state containing the database connection.
/// - `claim`: The claims extracted from the JWT, containing user information.
/// - `post_model`: The JSON payload containing the post's title and text.
///
/// # Returns
/// - Returns a 200 status code with a success message if the post is created successfully.
/// - Returns a 500 status code if there is a database error during post creation.
#[post("/create")]
pub async fn create_post(
    app_state: web::Data<app_state::AppState>,
    claim: Claims,
    post_model: MultipartForm<CreatePost>,
) -> Result<ApiResponse, ApiResponse> {
    let file_name = post_model
        .file
        .file_name
        .clone()
        .unwrap_or("null".to_string())
        .to_owned();
    let max_file_size = (constants::MaxFileSize).clone();

    match &file_name[file_name.len() - 4..] {
        ".png" | ".jpg" | ".jpeg" => {}
        _ => return Err(ApiResponse::new(400, "Invalid file format".to_owned(),"Invalid file format".to_string(),false)),
    }

    match post_model.file.size {
        0 => {
            return Err(ApiResponse::new(400, "File is empty".to_owned(),"Error".to_string(),false));
        }
        length if length > max_file_size as usize => {
            return Err(ApiResponse::new(400, "File is too large".to_owned(),"Error".to_string(),false));
        }
        _ => {}
    }

    let txn = app_state
        .db
        .begin()
        .await
        .map_err(|e| ApiResponse::new(500, format!("Failed to create transaction: {}", e),"Error".to_string(),false))?;

    let post_entity = entity::post::ActiveModel {
        title: Set(post_model.title.clone()),
        text: Set(post_model.text.clone()),
        uuid: Set(Uuid::new_v4()),
        user_id: Set(claim.id),
        image: Set("".to_string()),
        created_at: Set(chrono::Utc::now().naive_local()),
        ..Default::default()
    };

    let mut created_entity = post_entity
        .save(&txn)
        .await
        .map_err(|e| ApiResponse::new(500, format!("Failed to create post: {}", e),"Error".to_string(),false))?;
    let temp_file = post_model.file.file.path();

    let file_name = post_model
        .file
        .file_name
        .as_ref()
        .map(|s| s.to_owned())
        .unwrap_or("null".to_string());

    let timestamp: i64 = Utc::now().timestamp();

    let mut file_path = PathBuf::from("./public");
    let new_file_name = format!("{}-{}", timestamp, file_name);
    file_path.push(&new_file_name);

    match std::fs::copy(temp_file, file_path) {
        Ok(_) => {
            created_entity.image = Set(new_file_name.clone());
            created_entity
                .save(&txn)
                .await
                .map_err(|e| ApiResponse::new(500, format!("Failed to update post: {}", e),"Error".to_string(),false))?;

            txn.commit().await.map_err(|e| {
                ApiResponse::new(500, format!("Failed to commit transaction: {}", e),"Error".to_string(),false)
            })?;

            std::fs::remove_file(temp_file).unwrap_or_default();

            Ok(ApiResponse::new(
                200,
                "Post created successfully".to_string(),"success".to_string(),true
            ))
        }
        Err(e) => {
            txn.rollback().await.map_err(|e| {
                ApiResponse::new(500, format!("Failed to rollback transaction: {}", e),"Error".to_string(),false)
            })?;
            return Err(ApiResponse::new(500, format!("internal error: {}", e),"Error".to_string(),false));
        }
    }
}

/// Returns all posts created by the user that is currently logged in.
///
/// Requires the user to be authenticated.
///
/// # Example
/// Request:
///
/// GET /my-posts
///
/// Response:
///
/// {
///     "status_code": 200,
///     "body": "[
///         {
///             'id': 1,
///             'title': 'My Post',
///             'text': 'This is my post.',
///             'uuid': '00000000-0000-0000-0000-000000000000',
///             'image': '',
///             'user_id': 1,
///             'created_at': '2022-01-01 00:00:00'
///         },
///         ...
///     ]"
/// }
///
/// Returns a 500 status code if there is a database error.

#[get("my-posts")]
pub async fn my_posts(
    app_state: web::Data<app_state::AppState>,
    claim: Claims,
) -> Result<api_response::ApiResponse, api_response::ApiResponse> {
    let posts: Vec<PostModel> = entity::post::Entity::find()
        .filter(entity::post::Column::UserId.eq(claim.id))
        .all(&app_state.db)
        .await
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string(),"Error".to_string(),false))?
        .into_iter()
        .map(|post| PostModel {
            id: post.id,
            title: post.title,
            text: post.text,
            uuid: post.uuid,
            image: post.image,
            user_id: post.user_id,
            created_at: post.created_at,
            user: None,
        })
        .collect();
    let res_str = serde_json::to_string(&posts)
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string(),"Error".to_string(),false))?;

    Ok(api_response::ApiResponse::new(200, res_str.to_owned(),"success".to_string(),true))
}

/// Returns all posts in the database.
///
/// This function handles the request to retrieve all posts stored in the database.
/// It does not require user authentication.
///
/// # Example
/// Request:
///
/// GET /all-posts
///
/// Response:
///
/// {
///     "status_code": 200,
///     "body": "[
///         {
///             'id': 1,
///             'title': 'Post Title',
///             'text': 'Post content.',
///             'uuid': '00000000-0000-0000-0000-000000000000',
///             'image': 'image_url',
///             'user_id': 1,
///             'created_at': '2022-01-01 00:00:00'
///         },
///         ...
///     ]"
/// }
///
/// Returns a 500 status code if there is a database error.

#[get("/all-posts")]
pub async fn all_posts(
    app_state: web::Data<app_state::AppState>,
) -> Result<ApiResponse, ApiResponse> {
    let posts: Vec<PostModel> = entity::post::Entity::find()
        .all(&app_state.db)
        .await
        .map_err(|e| ApiResponse::new(500, e.to_string(),"Error".to_string(),false))?
        .into_iter()
        .map(|post| PostModel {
            id: post.id,
            title: post.title,
            text: post.text,
            uuid: post.uuid,
            image: post.image,
            user_id: post.user_id,
            created_at: post.created_at,
            user: None,
        })
        .collect();
    let res_str =
        serde_json::to_string(&posts).map_err(|e| ApiResponse::new(500, e.to_string(),"Error".to_string(),false))?;
    Ok(ApiResponse::new(200, res_str.to_owned(),"success".to_string(),true))
}

/// Retrieves a single post along with its associated user information from the database
/// using the post's UUID. This endpoint does not require user authentication.
///
/// # Parameters
/// - `app_state`: The application state containing the database connection.
/// - `uuid`: The UUID of the post to retrieve.
///
/// # Returns
/// - Returns a 200 status code with the post and user information in the response body if found.
/// - Returns a 404 status code if the post is not found.
/// - Returns a 500 status code if there is a database error.
///
/// # Example
/// Request:
///
/// GET /post/{post_uuid}
///
/// Response:
///
/// {
///     "status_code": 200,
///     "body": {
///         "id": 1,
///         "title": "Post Title",
///         "text": "Post content.",
///         "uuid": "00000000-0000-0000-0000-000000000000",
///         "image": "image_url",
///         "user_id": 1,
///         "created_at": "2022-01-01 00:00:00",
///         "user": {
///             "id": 1,
///             "name": "User Name",
///             "email": "user@example.com"
///         }
///     }
/// }

#[get("/post/{post_uuid}")]
pub async fn one_posts(
    app_state: web::Data<app_state::AppState>,
    uuid: web::Path<Uuid>,
) -> Result<ApiResponse, ApiResponse> {
    let posts: PostModel = entity::post::Entity::find()
        .filter(entity::post::Column::Uuid.eq(uuid.clone()))
        .find_also_related(entity::user::Entity)
        .one(&app_state.db)
        .await
        .map_err(|e| ApiResponse::new(500, e.to_string(),"Error".to_string(),false))?
        .map(|post| PostModel {
            id: post.0.id,
            title: post.0.title,
            text: post.0.text,
            uuid: post.0.uuid,
            image: post.0.image,
            user_id: post.0.user_id,
            created_at: post.0.created_at,
            user: post.1.map(|user| UserModel {
                id: user.id,
                name: user.name,
                email: user.email,
            }),
        })
        .ok_or(ApiResponse::new(404, "post not found".to_string(),"Error".to_string(),false))?;

    let res_str =
        serde_json::to_string(&posts).map_err(|e| ApiResponse::new(500, e.to_string(),"Error".to_string(),false))?;
    Ok(ApiResponse::new(200, res_str.to_owned(),"success".to_string(),true))
}
