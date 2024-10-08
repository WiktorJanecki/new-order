use axum::{
    extract::Path,
    routing::{delete, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::convert::Into;
use tracing::trace;

use crate::{
    controllers,
    models::item::{ItemForCreate, ItemForUpdate},
    session::Session,
    AppState, Result,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/orders/:order_id/items", post(handler_create))
        .route("/orders/:order_id/items/:item_id", delete(handler_delete))
}

#[derive(Deserialize)]
struct CreatePayload {
    quantity: String,
    name: String,
    value: i32,
    additional_info: Option<String>,
}
async fn handler_create(
    session: Session,
    AppState { db, .. }: AppState,
    Path(order_id): Path<i32>,
    payload: Json<CreatePayload>,
) -> Result<Json<Value>> {
    trace!(" -- HANDLER CREATE /orders/{order_id}/items");
    let item_fc = ItemForCreate {
        quantity: payload.quantity.clone(),
        name: payload.name.clone(),
        value: payload.value,
        additional_info: payload.additional_info.clone(),
    };
    let output = controllers::item::create(session, item_fc, order_id, db).await?;
    let json = json!({
        "id": output
    });
    Ok(Json(json))
}

async fn handler_delete(
    session: Session,
    AppState { db, .. }: AppState,
    Path((order_id, item_id)): Path<(i32, i32)>,
) -> Result<()> {
    trace!(" -- HANDLER DELETE /orders/{order_id}/items/{item_id}");
    controllers::item::delete(session, item_id, db).await?;
    Ok(())
}
