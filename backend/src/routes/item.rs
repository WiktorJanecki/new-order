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
        .route(
            "/orders/:order_id/items/:item_id",
            delete(handler_delete).patch(handler_update),
        )
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

#[derive(Deserialize)]
struct UpdatePayload {
    pub quantity: Option<String>,
    pub name: Option<String>,
    pub value: Option<i32>,
    pub additional_info: Option<String>,
    pub checked: Option<bool>,
}

async fn handler_update(
    session: Session,
    AppState { db, .. }: AppState,
    Path((order_id, item_id)): Path<(i32, i32)>,
    payload: Json<UpdatePayload>,
) -> Result<()> {
    trace!(" -- HANDLER UPDATE /orders/{order_id}/items/{item_id}");
    let item_fu = ItemForUpdate {
        quantity: payload.quantity.to_owned(),
        name: payload.name.to_owned(),
        value: payload.value,
        additional_info: payload.additional_info.to_owned(),
        checked: payload.checked,
    };
    controllers::item::update(session, item_id, item_fu, db).await?;
    Ok(())
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
