use axum::Router;

use crate::AppState;

mod admin;
mod item;
mod login;
mod order;
mod ping;

pub fn routes() -> Router<AppState> {
    Router::new()
        .merge(ping::routes())
        .merge(login::routes())
        .merge(order::routes())
        .merge(admin::routes())
        .merge(item::routes())
}
