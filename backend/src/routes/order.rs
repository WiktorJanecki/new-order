use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::trace;

use crate::{
    controllers,
    models::order::{Order, OrderForCreate, OrderForUpdate, OrderResponseBasic},
    session::Session,
    AppState, Result,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/orders", post(create).get(read_all))
        .route("/orders/:id", get(read).patch(update))
}

// POST /orders
// GET /orders
// GET /orders/:id
// PUT /orders/:id
// DELETE /orders/:id

#[derive(Deserialize)]
struct CreatePayload {
    receiver: String,
    additional_info: Option<String>,
}

async fn create(
    session: Session,
    AppState { db, .. }: AppState,
    payload: Json<CreatePayload>,
) -> Result<Json<Value>> {
    trace!(" -- HANDLER POST /orders");
    let orderfc = OrderForCreate {
        receiver: payload.receiver.clone(),
        additional_info: payload.additional_info.clone(),
    };
    let output = controllers::order::create(session, orderfc, db).await?;
    let json = json!({
        "id": output
    });
    Ok(Json(json))
}

async fn read(
    session: Session,
    AppState { db, .. }: AppState,
    Path(id): Path<i32>,
) -> Result<Json<OrderResponseBasic>> {
    trace!(" -- HANDLER GET /orders/{}", id);
    let output = controllers::order::read(session, id, db).await?;
    Ok(Json(output))
}

async fn read_all(
    session: Session,
    AppState { db, .. }: AppState,
) -> Result<Json<Vec<OrderResponseBasic>>> {
    trace!(" -- HANDLER GET /orders");
    let output = controllers::order::list(session, db).await?;
    Ok(Json(output))
}

#[derive(Deserialize)]
struct UpdatePayload {
    receiver: Option<String>,
    additional_info: Option<String>,
}

async fn update(
    session: Session,
    AppState { db, .. }: AppState,
    Path(id): Path<i32>,
    payload: Json<UpdatePayload>,
) -> Result<()> {
    trace!(" -- HANDLER PATCH /orders/{}", id);
    let orderfu = OrderForUpdate {
        receiver: payload.receiver.clone(),
        additional_info: payload.additional_info.clone(),
    };
    controllers::order::update(session, id, orderfu, db).await?;
    Ok(())
}
