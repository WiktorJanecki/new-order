use axum::{routing::get, Router};
use tracing::trace;

use crate::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/ping", get(ping))
}

async fn ping() -> String {
    trace!(" -- HANDLER /ping");
    "pong".to_owned()
}
