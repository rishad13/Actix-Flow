use crate::utils::api_response::ApiResponse; // Assuming your ApiResponse is here
use crate::utils::jwt::decode_jwt;
use actix_web::body::MessageBody;
use actix_web::middleware::Next;
use actix_web::HttpMessage;
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    error::{ErrorInternalServerError, ErrorUnauthorized},
    http::header::AUTHORIZATION,
    Error,
}; // Assuming decode_jwt is your JWT decoding function

pub async fn check_auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // Check for the Authorization header
    let auth_header = req.headers().get(AUTHORIZATION);
    if auth_header.is_none() {
        // Return 401 Unauthorized if no Authorization header is found
        return Err(ErrorUnauthorized(ApiResponse::new(
            401,
            "Unauthorized".to_string(),
        )));
    }

    // Extract and decode the token
    // Extract and decode the token
    let token = match auth_header.unwrap().to_str() {
        Ok(t) => t.to_string(), // Convert &str to String
        Err(_) => {
            return Err(ErrorUnauthorized(ApiResponse::new(
                401,
                "Invalid Authorization header".to_string(),
            )));
        }
    };

    // Validate the token
    let claim = decode_jwt(token).unwrap();

    next.call(req)
        .await
        .map_err(|e: Error| ErrorInternalServerError(ApiResponse::new(500, e.to_string())))
}
