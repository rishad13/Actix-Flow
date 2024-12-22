use actix_web::{middleware::from_fn, web};

use super::{handlers, middleware};

pub fn config(config: &mut web::ServiceConfig) {
    config
        .service(
            web::scope("/secure/post")
                .wrap(from_fn(middleware::auth_middleware::check_auth_middleware))
                .service(handlers::post_handler::create_post)
                .service(handlers::post_handler::my_posts),
        )
        .service(
            web::scope("/post")
                .service(handlers::post_handler::all_posts)
                .service(handlers::post_handler::one_posts),
        );
}
