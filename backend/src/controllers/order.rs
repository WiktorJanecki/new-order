use axum::Json;
use serde_json::{json, Value};

use crate::{
    models::{
        order::{Order, OrderForCreate, OrderForUpdate},
        user::Privileges,
    },
    session::Session,
    Db, Error, Result,
};

pub async fn create(session: Session, payload: OrderForCreate, db: Db) -> Result<Json<Value>> {
    let creator_id = session.id();
    let time_created = chrono::Local::now();
    let res: (i32, ) = sqlx::query_as("INSERT INTO orders (creator_id,time_created,receiver,additional_info) VALUES ($1,$2,$3,$4) RETURNING id")
        .bind(creator_id)
        .bind(time_created)
        .bind(payload.receiver)
        .bind(payload.additional_info)
        .fetch_one(&db).await.map_err(|_|Error::SQLFail)?;

    let output = json!({
        "id": res.0
    });
    Ok(Json(output))
}

pub async fn read(_session: Session, payload: i32, db: Db) -> Result<Json<Order>> {
    let res: Order = sqlx::query_as("SELECT * FROM orders WHERE id = $1")
        .bind(payload)
        .fetch_one(&db)
        .await
        .map_err(|_| Error::SQLFail)?;
    Ok(Json(res))
}

pub async fn read_all(_session: Session, db: Db) -> Result<Json<Vec<Order>>> {
    let res: Vec<Order> = sqlx::query_as("SELECT * FROM orders")
        .fetch_all(&db)
        .await
        .map_err(|_| Error::SQLFail)?;
    Ok(Json(res))
}

pub async fn update(_session: Session, id: i32, payload: OrderForUpdate, db: Db) -> Result<()> {
    sqlx::query(
        "
            UPDATE users SET 
                receiver=COALESCE($1,receiver),
                additional_info=COALESCE($2, additional_info)
            WHERE id=$3
        ",
    )
    .bind(payload.receiver)
    .bind(payload.additional_info)
    .bind(id)
    .execute(&db)
    .await
    .map_err(|_| Error::SQLFail)?;
    Ok(())
}

// only full privileges can hard delete orders
pub async fn delete(session: Session, id: i32, db: Db) -> Result<()> {
    // return early if privileges are not full
    if !matches!(session.privileges(), Privileges::Full) {
        return Err(Error::AuthNoAccess);
    }
    sqlx::query(
        "
            DELETE FROM orders WHERE id=$1
        ",
    )
    .bind(id)
    .execute(&db)
    .await
    .map_err(|_| Error::SQLFail)?;
    Ok(())
}
