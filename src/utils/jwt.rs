use std::future;

use super::constants;
use actix_web::{FromRequest, HttpMessage};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub email: String,
    pub id: i32,
}

impl FromRequest for Claims {
    type Error = actix_web::Error;
    type Future = future::Ready<Result<Claims, Self::Error>>;

    /// Extracts the Claims from the request extensions if it exists, otherwise returns 400 BadRequest.
    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> std::future::Ready<Result<Claims, actix_web::Error>> {
        let _ = payload;
        match req.extensions().get::<Claims>() {
            Some(claims) => future::ready(Ok(claims.clone())),
            None => future::ready(Err(actix_web::error::ErrorBadRequest("Bad claim"))),
        }
    }
}

/// Encodes a JWT token with the given email and id.
///
/// The token contains the email and id as claims and is signed with the secret
/// defined in the `JWT_SECRET` environment variable. The token is valid for 24 hours.
///
/// # Errors
///
/// If encoding the token fails, an `Error` is returned.
pub fn encode_token(email: String, id: i32) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let exp = Duration::hours(24);
    let claims = Claims {
        exp: (now + exp).timestamp() as usize,
        iat: now.timestamp() as usize,
        email,
        id,
    };

    let secret = (constants::jwt_secret).clone();
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

/// Decodes a JWT token and returns the contained claims.
///
/// # Errors
///
/// If decoding the token fails, an `Error` is returned.
pub fn decode_jwt(token: &String) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let secret = constants::jwt_secret.clone();
    let claim_data: Result<TokenData<Claims>, jsonwebtoken::errors::Error> = decode(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    );
    claim_data
}
