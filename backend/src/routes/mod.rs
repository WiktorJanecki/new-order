use axum::Router;

use crate::AppState;

mod login;
mod ping;

pub fn routes() -> Router<AppState> {
    Router::new().merge(ping::routes()).merge(login::routes())
}
