use actix_web::{middleware::Logger, web, App, HttpServer};
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use std::{error::Error, fmt::Display};
mod routes;
mod utils;

#[derive(Debug)]
struct MainError {
    message: String,
}

impl Display for MainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}

impl Error for MainError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        &self.message
    }

    /// Returns the underlying cause of this error, if any.
    ///
    /// This method delegates to the `source` method to provide compatibility
    /// with older error APIs that use `cause`.

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

#[actix_web::main]
async fn main() -> Result<(), MainError> {
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

    let db: DatabaseConnection = Database::connect(_db).await.map_err(|_| MainError {
        message: "Database connection error".to_string(),
    })?;
    Migrator::up(&db, None).await.map_err(|e| MainError {
        message: format!("Database migration error: {}", e),
    })?;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(utils::app_state::AppState {
                db: db.clone(),
            }))
            .configure(routes::home_route::config)
            .configure(routes::auth_route::config)
            .configure(routes::user_route::config)
    })
    .bind((_address, _port))
    .map_err(|_| MainError {
        message: "Server binding error".to_string(),
    })?
    .run()
    .await
    .map_err(|_| MainError {
        message: "Server run error".to_string(),
    })
}
