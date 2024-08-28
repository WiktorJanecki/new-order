use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginDoesntExist,
    LoginBadPassword,
    LoginFailedToGenerateToken,
    AuthMissingCookie,
    AuthBadToken,
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
