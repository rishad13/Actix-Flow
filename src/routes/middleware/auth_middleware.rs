use crate::utils::api_response::{self};
use crate::utils::jwt::decode_jwt;
use actix_web::body::MessageBody;
use actix_web::middleware::Next;
use actix_web::HttpMessage;
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
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
        return Err(Error::from(api_response::ApiResponse::new(
            401,
            "Unauthorized".to_string(),
        )));
    }

    // Extract and decode the token
    let token = auth_header
        .unwrap()
        .to_str()
        .unwrap()
        .replace("Bearer ", "");
    // Validate the token
    let claim = decode_jwt(&token.to_string()).map_err(|_| {
        Error::from(api_response::ApiResponse::new(
            401,
            "Unauthorized".to_string(),
        ))
    });

    req.extensions_mut().insert(claim.unwrap().claims);

    next.call(req).await.map_err(|e: Error| {
        Error::from(api_response::ApiResponse::new(
            500,
            e.to_string().to_string(),
        ))
    })
}
