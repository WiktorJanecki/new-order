use axum::{extract::Path, routing::get, Json, Router};
use serde::Deserialize;

use crate::{controllers, models::order::OrderResponseFull, session::Session, AppState, Result};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/admin/order/:id", get(read).patch(pay))
        .route("/admin/order", get(list))
}

async fn read(
    session: Session,
    AppState { db, .. }: AppState,
    Path(order_id): Path<i32>,
) -> Result<Json<OrderResponseFull>> {
    let ord = controllers::admin::order::read(session, order_id, db).await?;
    Ok(Json(ord))
}

#[derive(Deserialize)]
struct PayPayload {
    paid: bool,
}

async fn pay(
    session: Session,
    AppState { db, .. }: AppState,
    Path(order_id): Path<i32>,
    Json(payload): Json<PayPayload>,
) -> Result<()> {
    controllers::admin::order::pay(session, order_id, payload.paid, db).await?;
    Ok(())
}

async fn list(
    session: Session,
    AppState { db, .. }: AppState,
) -> Result<Json<Vec<OrderResponseFull>>> {
    let out = controllers::admin::order::list(session, db).await?;
    Ok(Json(out))
}
