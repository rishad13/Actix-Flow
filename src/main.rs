use actix_web::{middleware::Logger, web, App, HttpServer};
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
mod routes;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv::dotenv().ok();
    env_logger::init();

    let (_address, _port, _db) = (
        utils::constants::address.clone(),
        utils::constants::port.clone(),
        utils::constants::db_url.clone(),
    );

    let db: DatabaseConnection = Database::connect(_db).await.unwrap();
    Migrator::up(&db, None).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(utils::app_state::AppState {
                db: db.clone(),
            }))
            .configure(routes::home_route::config)
            .configure(routes::auth_route::config)
    })
    .bind((_address, _port))?
    .run()
    .await
    .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "method not allowed"))
}
