use actix_web::{middleware::Logger, App, HttpServer};
mod routes;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv::dotenv().ok();
    env_logger::init();

    let (_address, _port) = (
        utils::constants::address.clone(),
        utils::constants::port.clone(),
    );

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(routes::home_route::config)
    })
    .bind((_address, _port))?
    .run()
    .await
    .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "method not allowed"))
}
