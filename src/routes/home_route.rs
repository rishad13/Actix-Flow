use super::handlers;
use actix_web::web;

/// Configure the actix-web route services for the home page.
///
/// Registers the route services for the home page at the `/home` path.
/// The services registered include the `index` and `test` handlers.
///
pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/home")
            .service(handlers::home_handler::index)
            .service(handlers::home_handler::index)
            .service(handlers::home_handler::test),
    );
}
