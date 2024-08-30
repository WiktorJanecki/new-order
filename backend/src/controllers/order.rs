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
    let res: (i32, ) = sqlx::query_as("INSERT INTO orders (creator_id,time_created,receiver,additional_info,deleted,paid) VALUES ($1,$2,$3,$4, false,false) RETURNING id")
        .bind(creator_id)
        .bind(time_created)
        .bind(payload.receiver)
        .bind(payload.additional_info)
        .fetch_one(&db).await?;

    Ok(res.0)
}

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

    OrderResponseBasic {
        id: res.id,
        time_created: res.time_created,
        receiver: res.receiver,
        additional_info: res.additional_info,
        items: mapped_items,
    }
}
pub async fn read(session: Session, payload: i32, db: Db) -> Result<OrderResponseBasic> {
    trace!(" -- CONTROLLER order::read");

    // get order data
    let res: Order = sqlx::query_as("SELECT * FROM orders WHERE id = $1 and deleted=false")
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
    let res: Vec<Order> = sqlx::query_as("SELECT * FROM orders WHERE deleted=false")
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

pub async fn delete(_session: Session, id: i32, db: Db) -> Result<()> {
    trace!(" -- CONTROLLER order::delete");

    let result = sqlx::query(
        "
        UPDATE orders SET deleted=true WHERE id=$1
    ",
    )
    .bind(id)
    .execute(&db)
    .await?;

    if result.rows_affected() == 0 {
        return Err(Error::SQLEntityNotFound {
            entity_type: "order",
            id,
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::models::item::ItemForCreate;

    use super::*;
    use anyhow::Result;
    use chrono::NaiveDateTime;

    #[sqlx::test]
    async fn order_create(pool: Db) -> Result<()> {
        let order_fc = OrderForCreate {
            receiver: "tomek".to_owned(),
            additional_info: None,
        };
        let id = controllers::order::create(Session::BASIC(), order_fc, pool.clone()).await?;
        assert_eq!(id, 1); // first db item id should be 1

        let order: Order = sqlx::query_as("SELECT * FROM orders")
            .fetch_one(&pool)
            .await?;
        assert_eq!(order.id, id);
        assert_eq!(order.receiver, "tomek");
        assert_eq!(order.creator_id, Session::BASIC().id());
        assert_eq!(order.deleted, false);
        assert_eq!(order.paid, false);

        Ok(())
    }

    #[sqlx::test]
    async fn order_read(pool: Db) -> Result<()> {
        let item_fc = ItemForCreate {
            quantity: "1".to_owned(),
            name: "1".to_owned(),
            value: 1,
            additional_info: None,
        };
        let id1 =
            controllers::item::create(Session::BASIC(), item_fc.clone(), 1, pool.clone()).await?;
        let _id2 = controllers::item::create(Session::BASIC(), item_fc, 1, pool.clone()).await?;

        let order_fc = OrderForCreate {
            receiver: "tomek".to_owned(),
            additional_info: None,
        };
        let order_id_1 =
            controllers::order::create(Session::BASIC(), order_fc.clone(), pool.clone()).await?;

        let order_id_2 =
            controllers::order::create(Session::BASIC(), order_fc, pool.clone()).await?;

        assert_eq!(order_id_1, 1); // first_item should have id of 1
        assert_eq!(order_id_2, 2);

        // fetch

        let order1 = controllers::order::read(Session::BASIC(), order_id_1, pool.clone()).await?;
        let order2 = controllers::order::read(Session::BASIC(), order_id_2, pool.clone()).await?;

        assert_eq!(order1.id, order_id_1);
        assert_eq!(order1.items.len(), 2);
        assert_eq!(order2.items.len(), 0);
        assert_eq!(order1.items[0].id, id1);
        assert_eq!(order1.receiver, "tomek");

        let time = NaiveDateTime::UNIX_EPOCH;
        // test deleted user
        let id: (i32,) = sqlx::query_as(
            "
            INSERT INTO orders
                (creator_id,time_created,receiver,additional_info,deleted,paid)
            VALUES
                (0,$1,'tomek',NULL,true,false) RETURNING id
                ",
        )
        .bind(time)
        .fetch_one(&pool)
        .await?;

        let should_err = controllers::order::read(Session::BASIC(), id.0, pool.clone()).await;
        let _err = Error::SQLEntityNotFound {
            entity_type: "order",
            id: id.0,
        };

        assert!(matches!(should_err, _err));

        Ok(())
    }

    #[sqlx::test]
    async fn order_read_not_found(pool: Db) -> Result<()> {
        let result = controllers::order::read(Session::BASIC(), 0, pool).await;

        let _err = Error::SQLEntityNotFound {
            entity_type: "order",
            id: 1,
        };
        assert!(matches!(result, _err));

        Ok(())
    }

    #[sqlx::test]
    async fn order_list(pool: Db) -> Result<()> {
        let empty = controllers::order::list(Session::BASIC(), pool.clone()).await?;
        assert_eq!(empty.len(), 0);

        // add one
        let order_fc = OrderForCreate {
            receiver: "tomek".to_owned(),
            additional_info: None,
        };
        let order_id_1 =
            controllers::order::create(Session::BASIC(), order_fc.clone(), pool.clone()).await?;
        let one = controllers::order::list(Session::BASIC(), pool.clone()).await?;
        assert_eq!(one.len(), 1);
        assert_eq!(one[0].id, order_id_1);

        let order_id_2 =
            controllers::order::create(Session::BASIC(), order_fc, pool.clone()).await?;
        let both = controllers::order::list(Session::BASIC(), pool.clone()).await?;
        assert_eq!(both.len(), 2);
        assert_eq!(both[1].id, order_id_2);
        Ok(())
    }

    #[sqlx::test]
    async fn order_list_deleted(pool: Db) -> Result<()> {
        // TODO
        Ok(())
    }

    #[sqlx::test]
    async fn order_update_not_found(pool: Db) -> Result<()> {
        let orderfu = OrderForUpdate {
            receiver: None,
            additional_info: None,
        };
        let should_err =
            controllers::order::update(Session::BASIC(), 0, orderfu, pool.clone()).await;
        let _err = Error::SQLEntityNotFound {
            entity_type: "order",
            id: 0,
        };
        assert!(matches!(should_err, _err));
        Ok(())
    }

    #[sqlx::test]
    async fn order_update(pool: Db) -> Result<()> {
        let order_fc = OrderForCreate {
            receiver: "tomek".to_owned(),
            additional_info: None,
        };
        let id = controllers::order::create(Session::BASIC(), order_fc, pool.clone()).await?;

        let fetched = controllers::order::read(Session::BASIC(), id, pool.clone()).await?;
        assert_eq!(fetched.additional_info, None);

        let order_fu = OrderForUpdate {
            receiver: None,
            additional_info: Some("Actually".to_owned()),
        };

        controllers::order::update(Session::BASIC(), id, order_fu, pool.clone()).await?;

        let fetched = controllers::order::read(Session::BASIC(), id, pool.clone()).await?;
        assert_eq!(fetched.additional_info, Some("Actually".to_owned()));

        Ok(())
    }
    #[sqlx::test]
    async fn order_delete_not_found(pool: Db) -> Result<()> {
        let should_err = controllers::order::delete(Session::BASIC(), 0, pool.clone()).await;
        let _err = Error::SQLEntityNotFound {
            entity_type: "order",
            id: 0,
        };
        assert!(matches!(should_err, _err));
        Ok(())
    }

    #[sqlx::test]
    async fn order_delete(pool: Db) -> Result<()> {
        let order_fc = OrderForCreate {
            receiver: "tomek".to_owned(),
            additional_info: None,
        };
        let id = controllers::order::create(Session::BASIC(), order_fc, pool.clone()).await?;

        let fetched = controllers::order::read(Session::BASIC(), id, pool.clone()).await?;
        assert_eq!(fetched.additional_info, None);

        controllers::order::delete(Session::BASIC(), id, pool.clone()).await?;

        let list = controllers::order::list(Session::BASIC(), pool.clone()).await?;
        assert_eq!(list.len(), 0);

        let get = controllers::order::read(Session::BASIC(), id, pool.clone()).await;
        let _err = Error::SQLEntityNotFound {
            entity_type: "order",
            id,
        };
        assert!(matches!(get, _err));

        Ok(())
    }

    #[test]
    fn order_helper_mapper() -> Result<()> {
        // order_and_items_into_response
        let fx_order = Order {
            id: 0,
            creator_id: 1,
            time_created: NaiveDateTime::UNIX_EPOCH,
            receiver: "wujek".to_owned(),
            additional_info: Some("Actually info".to_owned()),
            deleted: true,
            paid: false,
        };
        let fx_item1 = Item {
            id: 1,
            order_id: 1,
            creator_id: 1,
            time_created: NaiveDateTime::UNIX_EPOCH,
            quantity: "1".to_owned(),
            name: "1".to_owned(),
            value: 1,
            additional_info: None,
            deleted: true,
        };

        let fx_item2 = Item {
            id: 2,
            order_id: 2,
            creator_id: 2,
            time_created: NaiveDateTime::UNIX_EPOCH,
            quantity: "2".to_owned(),
            name: "2".to_owned(),
            value: 2,
            additional_info: None,
            deleted: true,
        };

        let output = order_and_items_into_response(fx_order, vec![fx_item1, fx_item2]);
        assert_eq!(output.id, 0);
        assert_eq!(output.time_created, NaiveDateTime::UNIX_EPOCH);
        assert_eq!(output.receiver, "wujek");
        assert_eq!(output.items[0].id, 1);
        assert_eq!(output.items[1].id, 2);
        assert_eq!(output.items[0].quantity, "1");
        assert_eq!(output.items[1].quantity, "2");

        Ok(())
    }
}
