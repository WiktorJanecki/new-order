pub mod order {
    use tracing::trace;

    use crate::{
        models::{
            item::Item,
            order::{Order, OrderResponseFull},
            user::Privileges,
        },
        session::Session,
        Db, Error, Result,
    };

    pub async fn list(session: Session, db: Db) -> Result<Vec<OrderResponseFull>> {
        trace!(" -- CONTROLLER admin::order::list");
        if matches!(session.privileges(), Privileges::Basic) {
            return Err(Error::AuthNoAccess);
        }
        let res: Vec<Order> = sqlx::query_as("SELECT * FROM orders ORDER BY id")
            .fetch_all(&db)
            .await?;
        let mut mapped = vec![];
        for order in res {
            let its: Vec<Item> =
                sqlx::query_as("SELECT * FROM items WHERE order_id=$1 ORDER BY id")
                    .bind(order.id)
                    .fetch_all(&db)
                    .await?;

            let response = OrderResponseFull {
                id: order.id,
                creator_id: order.creator_id,
                time_created: order.time_created,
                receiver: order.receiver,
                additional_info: order.additional_info,
                deleted: order.deleted,
                paid: order.paid,
                items: its,
            };
            mapped.push(response);
        }
        Ok(mapped)
    }
    pub async fn pay(session: Session, order_id: i32, payload: bool, db: Db) -> Result<()> {
        trace!(" -- CONTROLLER admin::order::pay");
        if matches!(session.privileges(), Privileges::Basic) {
            return Err(Error::AuthNoAccess);
        }
        let output = sqlx::query!("UPDATE orders SET paid=$1 WHERE id = $2", payload, order_id)
            .execute(&db)
            .await?;
        if output.rows_affected() == 0 {
            return Err(Error::SQLEntityNotFound {
                entity_type: "order",
                id: order_id,
            });
        }
        Ok(())
    }
    pub async fn read(session: Session, order_id: i32, db: Db) -> Result<OrderResponseFull> {
        trace!(" -- CONTROLLER admin::order::read");
        if matches!(session.privileges(), Privileges::Basic) {
            return Err(Error::AuthNoAccess);
        }
        let order: Option<Order> = sqlx::query_as("SELECT * FROM orders WHERE id=$1")
            .bind(order_id)
            .fetch_optional(&db)
            .await?;
        if order.is_none() {
            return Err(Error::SQLEntityNotFound {
                entity_type: "order",
                id: order_id,
            });
        }
        let order = order.unwrap();
        let items: Vec<Item> = sqlx::query_as("SELECT * FROM items WHERE order_id = $1")
            .bind(order_id)
            .fetch_all(&db)
            .await?;
        let mapped_order = OrderResponseFull {
            id: order.id,
            creator_id: order.creator_id,
            time_created: order.time_created,
            receiver: order.receiver,
            additional_info: order.additional_info,
            deleted: order.deleted,
            paid: order.paid,
            items,
        };
        Ok(mapped_order)
    }
}
#[cfg(test)]
mod tests {
    use crate::*;
    use anyhow::Result;
    use models::{
        item::ItemForCreate,
        order::{OrderForCreate, OrderResponseFull},
    };
    use session::Session;

    #[sqlx::test]
    async fn order_list_no_access(pool: Db) -> Result<()> {
        let output = controllers::admin::order::list(Session::BASIC(), pool).await;
        assert!(matches!(output, Err(crate::Error::AuthNoAccess)));
        Ok(())
    }
    #[sqlx::test]
    async fn order_list(pool: Db) -> Result<()> {
        let output = controllers::admin::order::list(Session::FULL(), pool.clone()).await?;
        assert_eq!(output.len(), 0);
        let payload = OrderForCreate {
            receiver: "Eryk".to_string(),
            additional_info: None,
        };
        let itemed_id =
            controllers::order::create(Session::BASIC(), payload.clone(), pool.clone()).await?;
        let _ = controllers::order::create(Session::BASIC(), payload.clone(), pool.clone()).await?;
        let deleted_id =
            controllers::order::create(Session::BASIC(), payload, pool.clone()).await?;

        controllers::order::delete(Session::BASIC(), deleted_id, pool.clone()).await?;
        let item = ItemForCreate {
            quantity: "foidaj".to_string(),
            name: "oije".to_string(),
            value: 2000,
            additional_info: None,
        };
        controllers::item::create(Session::BASIC(), item.clone(), itemed_id, pool.clone()).await?;
        controllers::item::create(Session::BASIC(), item, itemed_id, pool.clone()).await?;

        let output = controllers::admin::order::list(Session::FULL(), pool.clone()).await?;
        assert_eq!(output.len(), 3);
        assert_eq!(output[0].items.len(), 2);
        assert_eq!(output[0].items[0].value, 2000);

        Ok(())
    }
    #[sqlx::test]
    async fn order_pay_no_access(pool: Db) -> Result<()> {
        let output = controllers::admin::order::pay(Session::BASIC(), 0, true, pool).await;
        assert!(matches!(output, Err(crate::Error::AuthNoAccess)));
        Ok(())
    }
    #[sqlx::test]
    async fn order_pay_entity_not_found(pool: Db) -> Result<()> {
        let output = controllers::admin::order::pay(Session::FULL(), 0, true, pool).await;
        let _err = Error::SQLEntityNotFound {
            entity_type: "order",
            id: 0,
        };
        assert!(matches!(output, Err(_err)));
        Ok(())
    }
    #[sqlx::test]
    async fn order_pay(pool: Db) -> Result<()> {
        let payload = OrderForCreate {
            receiver: "Eryk".to_string(),
            additional_info: None,
        };
        let id =
            controllers::order::create(Session::BASIC(), payload.clone(), pool.clone()).await?;
        // default paid should be true
        let order: OrderResponseFull =
            controllers::admin::order::read(Session::FULL(), id, pool.clone()).await?;
        assert_eq!(order.paid, false);

        controllers::admin::order::pay(Session::FULL(), id, true, pool.clone()).await?;
        let order: OrderResponseFull =
            controllers::admin::order::read(Session::FULL(), id, pool.clone()).await?;
        assert_eq!(order.paid, true);
        Ok(())
    }
    #[sqlx::test]
    async fn order_read_no_access(pool: Db) -> Result<()> {
        let output = controllers::admin::order::read(Session::BASIC(), 0, pool).await;
        assert!(matches!(output, Err(crate::Error::AuthNoAccess)));
        Ok(())
    }
    #[sqlx::test]
    async fn order_read_entity_not_found(pool: Db) -> Result<()> {
        let output = controllers::admin::order::read(Session::FULL(), 0, pool).await;
        let _err = Error::SQLEntityNotFound {
            entity_type: "order",
            id: 0,
        };
        assert!(matches!(output, Err(_err)));
        Ok(())
    }
    #[sqlx::test]
    async fn order_read(pool: Db) -> Result<()> {
        let payload = OrderForCreate {
            receiver: "Eryk".to_string(),
            additional_info: None,
        };
        let id =
            controllers::order::create(Session::BASIC(), payload.clone(), pool.clone()).await?;

        let item = ItemForCreate {
            quantity: "foidaj".to_string(),
            name: "oije".to_string(),
            value: 2000,
            additional_info: None,
        };
        controllers::item::create(Session::BASIC(), item.clone(), id, pool.clone()).await?;
        controllers::item::create(Session::BASIC(), item, id, pool.clone()).await?;

        let output = controllers::admin::order::read(Session::FULL(), id, pool.clone()).await?;

        assert_eq!(output.receiver, "Eryk".to_owned());
        assert_eq!(output.additional_info, None);
        assert_eq!(output.paid, false);
        assert_eq!(output.deleted, false);
        assert_eq!(output.items.len(), 2);
        assert_eq!(output.items[0].value, 2000);
        Ok(())
    }
}
