use axum::{
    extract::{Path, Query},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::trace;

use crate::{
    controllers,
    models::order::{OrderForCreate, OrderListParams, OrderResponseBasic},
    session::Session,
    AppState, Result,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/orders", post(create).get(list))
        .route("/orders/:id", get(read).delete(delete))
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

async fn list(
    session: Session,
    AppState { db, .. }: AppState,
    params: Option<Query<OrderListParams>>,
) -> Result<Json<Vec<OrderResponseBasic>>> {
    trace!(" -- HANDLER GET /orders");
    if let Some(Query(params)) = params {
        let output = controllers::order::list_with_params(session, params, db).await?;
        return Ok(Json(output));
    }
    let output = controllers::order::list(session, db).await?;
    Ok(Json(output))
}

async fn delete(
    session: Session,
    AppState { db, .. }: AppState,
    Path(id): Path<i32>,
) -> Result<()> {
    trace!(" -- HANDLER DELETE /orders/{}", id);

    controllers::order::delete(session, id, db).await?;
    Ok(())
}
