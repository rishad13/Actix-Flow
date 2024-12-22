use crate::utils::api_response;
use crate::utils::{api_response::ApiResponse, app_state, jwt::Claims};
use actix_web::{get, post, web};
use chrono::NaiveDateTime;
use sea_orm::ActiveModelTrait;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct CreatePost {
    title: String,
    text: String,
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
    post_model: web::Json<CreatePost>,
) -> Result<ApiResponse, ApiResponse> {
    let post_entity = entity::post::ActiveModel {
        title: Set(post_model.title.clone()),
        text: Set(post_model.text.clone()),
        uuid: Set(Uuid::new_v4()),
        user_id: Set(claim.id),
        image: Set("".to_string()),
        created_at: Set(chrono::Utc::now().naive_local()),
        ..Default::default()
    };

    match post_entity.insert(&app_state.db).await {
        Ok(_) => Ok(ApiResponse::new(200, "success".to_owned())),
        Err(e) => Err(ApiResponse::new(
            500,
            format!("Failed to create post: {}", e),
        )),
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
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?
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
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;

    Ok(api_response::ApiResponse::new(200, res_str.to_owned()))
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
        .map_err(|e| ApiResponse::new(500, e.to_string()))?
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
        serde_json::to_string(&posts).map_err(|e| ApiResponse::new(500, e.to_string()))?;
    Ok(ApiResponse::new(200, res_str.to_owned()))
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
        .map_err(|e| ApiResponse::new(500, e.to_string()))?
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
        .ok_or(ApiResponse::new(404, "post not found".to_string()))?;

    let res_str =
        serde_json::to_string(&posts).map_err(|e| ApiResponse::new(500, e.to_string()))?;
    Ok(ApiResponse::new(200, res_str.to_owned()))
}
