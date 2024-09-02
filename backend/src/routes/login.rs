use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use jwt_simple::{
    claims::Claims,
    prelude::{Duration, MACLike},
};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use tracing::trace;

use crate::{
    error::Result, models::user::User, session::Session, AppState, Error, JWTClaims,
    AUTH_COOKIE_KEY,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/token", get(token))
}

// just verify the token
async fn token(session: Session) -> Result<Json<Value>> {
    trace!(" -- HANDLER GET /token");
    Ok(Json(json!({
        "privileges": session.privileges()
    })))
}

#[derive(Deserialize)]
struct LoginPaylod {
    login: String,
    password: String,
}

async fn login(
    State(state): State<AppState>,
    cookies: Cookies,
    payload: Json<LoginPaylod>,
) -> Result<()> {
    trace!(
        " -- HANDLER POST /login ({} {})",
        &payload.login,
        &payload.password
    );

    let output: User = sqlx::query_as("SELECT * FROM users WHERE name=$1")
        .bind(&payload.login)
        .fetch_one(&state.db)
        .await
        .map_err(|_| Error::LoginDoesntExist)?;

    // TODO: password hashing (not really necessary)
    if output.password != payload.password {
        return Err(Error::LoginBadPassword);
    }

    let claims_content = JWTClaims {
        id: output.id,
        privileges: output.privileges,
    };

    let claims = Claims::with_custom_claims(claims_content, Duration::from_hours(2));
    let token = state
        .jwt_key
        .authenticate(claims)
        .map_err(|_| Error::LoginFailedToGenerateToken)?;

    // create cookie
    cookies.add(Cookie::new(AUTH_COOKIE_KEY, token));

    Ok(())
}
