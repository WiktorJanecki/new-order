use axum::async_trait;
use axum::extract::FromRef;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
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
use tracing::trace;

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

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for AppState
where
    AppState: FromRef<S>,
{
    type Rejection = Error;
    async fn from_request_parts(_: &mut Parts, state: &S) -> Result<Self> {
        trace!(" -- EXTRACTOR AppState");
        let state = AppState::from_ref(state);
        Ok(state.to_owned())
    }
}
