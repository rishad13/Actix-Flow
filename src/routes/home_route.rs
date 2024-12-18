use super::handlers;
use actix_web::web;

/// Configure the routes for the home route.
///
/// The home route is responsible for providing the homepage of the application.
/// It provides two endpoints: `/` and `/test`.
///
/// The `/` endpoint returns a simple "Hello, Rustacean!" message.
///
/// The `/test` endpoint returns a simple "Hello, Rustacean! test" message.
///
/// This function is a part of the `actix_web` framework, and is used to configure
/// the application's routes.
pub fn config(config: &mut web::ServiceConfig) {
    config
        .service(handlers::home_handler::index)
        .service(handlers::home_handler::test);
}
