use std::env;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref address: String = set_address();
    pub static ref port: u16 = set_port();
    pub static ref db_url: String = set_db_url();
    pub static ref jwt_secret: String = set_jwt_secret();
}

/// Retrieves the JWT secret from the environment variables.
///
/// Loads environment variables from a `.env` file, if available, and
/// attempts to fetch the value of the `jwtSecret` variable. If the
/// variable is not set, the function will panic.

fn set_jwt_secret() -> String {
    dotenv::dotenv().ok();
    env::var("jwtSecret").unwrap()
}

/// Retrieves the database connection URL from the environment variables.
///
/// Loads environment variables from a `.env` file, if available, and
/// attempts to fetch the value of the `DATABASE_URL` variable. If the
/// variable is not set, the function will panic.
fn set_db_url() -> String {
    dotenv::dotenv().ok();
    env::var("DATABASE_URL").unwrap()
}

/// Retrieves the server address from the environment variables.
///
/// Loads environment variables from a `.env` file, if available, and
/// attempts to fetch the value of the `ADDRESS` variable. If the
/// variable is not set, the function will panic.

fn set_address() -> String {
    dotenv::dotenv().ok();
    env::var("ADDRESS").unwrap()
}

/// Retrieves the server port from the environment variables.
///
/// Loads environment variables from a `.env` file, if available, and attempts
/// to fetch the value of the `PORT` variable. If the variable is not set,
/// defaults to "8080".
///
/// # Returns
///
/// * A `u16` containing the server port.
fn set_port() -> u16 {
    dotenv::dotenv().ok();
    env::var("PORT").unwrap().parse().unwrap()
}
