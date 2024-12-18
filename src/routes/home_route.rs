use super::handlers;
use actix_web::web;

///
/// Registers routes for the "home" scope.
///
/// The "home" scope includes the following routes:
///
/// - `GET /home`: Returns a message with a Rust mascot.
/// - `GET /`: Returns a message with a Rust mascot.
/// - `GET /test`: Returns a message with a Rust mascot.
///
/// # Parameters
///
/// * `config`: An instance of `actix_web::web::ServiceConfig` to configure.
///
/// # Returns
///
/// * None
pub fn config(config: &mut web::ServiceConfig) {
    config.service(web::scope("/home").service(handlers::home_handler::index))
        .service(handlers::home_handler::index)
        .service(handlers::home_handler::test);
}
