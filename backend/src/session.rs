use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
    RequestPartsExt,
};
use jwt_simple::prelude::MACLike;
use tower_cookies::Cookies;
use tracing::trace;

use crate::{models::user::Privileges, AppState, Error, JWTClaims, Result, AUTH_COOKIE_KEY};

// id and privileges read only
pub struct Session {
    id: i32,
    privileges: Privileges,
}

impl Session {
    pub fn new(id: i32, privileges: Privileges) -> Self {
        Self { id, privileges }
    }
    pub fn id(self) -> i32 {
        self.id
    }
    pub fn privileges(self) -> Privileges {
        self.privileges
    }
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Session
where
    AppState: FromRef<S>,
{
    type Rejection = Error;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        trace!(" -- EXTRACTOR session");

        let state = AppState::from_ref(state);
        let cookies = parts
            .extract::<Cookies>()
            .await
            .map_err(|_| Error::AuthMissingCookie)?;

        let cookie = cookies
            .get(AUTH_COOKIE_KEY)
            .ok_or(Error::AuthMissingCookie)?;

        let token = cookie.value();
        let token_claims = state
            .jwt_key
            .verify_token::<JWTClaims>(
                token,
                None, // Some(VerificationOptions {
                     //     time_tolerance: Some(Duration::from_hours(2)),
                     //     ..Default::default()
                     // }),
            )
            .map_err(|_| Error::AuthBadToken)?;
        let session_id = token_claims.custom.id;
        let privileges = token_claims.custom.privileges;
        Ok(Session::new(session_id, privileges))
    }
}
