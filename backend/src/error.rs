use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    LoginDoesntExist,
    LoginBadPassword,
    LoginFailedToGenerateToken,
    AuthMissingCookie,
    AuthBadToken,
    AuthNoAccess,
    SQLFail,
    SQLEntityNotFound { entity_type: &'static str, id: i32 },
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        error!(" -- {:?}", self);
        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED CLIENT ERROR").into_response()
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        error!("{value:?}");
        Error::SQLFail
    }
}
