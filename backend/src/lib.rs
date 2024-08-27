use jwt_simple::prelude::HS256Key;
use serde::Deserialize;
use serde::Serialize;

pub mod controllers;
pub mod error;
pub mod middlewares;
pub mod models;
pub mod routes;
pub mod session;

pub use error::Error;
pub use error::Result;
use models::user::Privileges;

pub type Db = sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
    pub jwt_key: HS256Key,
}

const AUTH_COOKIE_KEY: &str = "AUTH_TOKEN";

#[derive(Serialize, Deserialize)]
pub struct JWTClaims {
    pub id: i32,
    pub privileges: Privileges,
}
