use tracing::trace;

use crate::{
    controllers,
    models::{
        item::{Item, ItemResponseBasic},
        order::{Order, OrderForCreate, OrderForUpdate, OrderResponseBasic},
    },
    session::Session,
    Db, Error, Result,
};

pub async fn create(session: Session, payload: OrderForCreate, db: Db) -> Result<i32> {
    trace!(" -- CONTROLLER order::create");
    let creator_id = session.id();
    let time_created = chrono::Local::now().naive_local();
    let res: (i32, ) = sqlx::query_as("INSERT INTO orders (creator_id,time_created,receiver,additional_info) VALUES ($1,$2,$3,$4) RETURNING id")
        .bind(creator_id)
        .bind(time_created)
        .bind(payload.receiver)
        .bind(payload.additional_info)
        .fetch_one(&db).await?;

    Ok(res.0)
}

// TODO: UNIT TEST ALL FUNCTIONS HERE
// unit test this TODO:
fn order_and_items_into_response(res: Order, items: Vec<Item>) -> OrderResponseBasic {
    let mapped_items = items
        .iter()
        .map(|item| ItemResponseBasic {
            id: item.id,
            order_id: item.order_id,
            time_created: item.time_created,
            quantity: item.quantity.to_string(),
            name: item.name.to_string(),
            value: item.value,
            additional_info: item.additional_info.to_owned(),
        })
        .collect::<Vec<_>>();

    // combine output

    let output = OrderResponseBasic {
        id: res.id,
        time_created: res.time_created,
        receiver: res.receiver,
        additional_info: res.additional_info,
        items: mapped_items,
    };
    output
}
pub async fn read(session: Session, payload: i32, db: Db) -> Result<OrderResponseBasic> {
    trace!(" -- CONTROLLER order::read");

    // get order data
    let res: Order = sqlx::query_as("SELECT * FROM orders WHERE id = $1")
        .bind(payload)
        .fetch_optional(&db)
        .await?
        .ok_or(Error::SQLEntityNotFound {
            entity_type: "order",
            id: payload,
        })?;
    // get its items
    let items = controllers::item::read_where_order_id(session, res.id, db.clone()).await?;

    // combine
    let combined = order_and_items_into_response(res, items);

    // return
    Ok(combined)
}

pub async fn list(session: Session, db: Db) -> Result<Vec<OrderResponseBasic>> {
    trace!(" -- CONTROLLER order::read_all");
    let res: Vec<Order> = sqlx::query_as("SELECT * FROM orders")
        .fetch_all(&db)
        .await?;
    let mut mapped = vec![];
    for order in res {
        let items =
            controllers::item::read_where_order_id(session.clone(), order.id, db.clone()).await?;
        mapped.push(order_and_items_into_response(order, items));
    }
    Ok(mapped)
}

pub async fn update(_session: Session, id: i32, payload: OrderForUpdate, db: Db) -> Result<()> {
    trace!(" -- CONTROLLER order::update");
    sqlx::query(
        "
            UPDATE orders SET 
                receiver=COALESCE($1,receiver),
                additional_info=COALESCE($2, additional_info)
            WHERE id=$3
        ",
    )
    .bind(payload.receiver)
    .bind(payload.additional_info)
    .bind(id)
    .execute(&db)
    .await?;
    Ok(())
}
