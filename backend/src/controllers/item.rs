use tracing::trace;

use crate::{
    models::item::{Item, ItemForCreate},
    session::Session,
    Db, Error, Result,
};

pub async fn create(
    session: Session,
    item_fc: ItemForCreate,
    order_id: i32,
    db: Db, /**/
) -> Result<i32> {
    trace!(" -- CONTROLLER item::create");
    let creator_id = session.id();
    let time_created = chrono::Local::now().naive_local();

    let res: (i32,) = sqlx::query_as(
        "
            INSERT INTO items
                (order_id,creator_id,
                time_created,quantity,
                name,value,
                additional_info,deleted)
            VALUES
                ($1,$2,$3,$4,$5,$6,$7,false)
            RETURNING id
        ",
    )
    .bind(order_id)
    .bind(creator_id)
    .bind(time_created)
    .bind(item_fc.quantity)
    .bind(item_fc.name)
    .bind(item_fc.value)
    .bind(item_fc.additional_info)
    .fetch_one(&db)
    .await?;

    Ok(res.0)
}

pub async fn read_where_order_id(_session: Session, order_id: i32, db: Db) -> Result<Vec<Item>> {
    trace!(" -- CONTROLLER item::read_where_order_id");
    let result: Vec<Item> = sqlx::query_as(
        "
            SELECT * FROM items WHERE deleted=false AND order_id=$1 ORDER BY id
        ",
    )
    .bind(order_id)
    .fetch_all(&db)
    .await?;
    Ok(result)
}

pub async fn delete(_session: Session, item_id: i32, db: Db) -> Result<()> {
    trace!(" -- CONTROLLER item::delete");
    let result = sqlx::query(
        "
            UPDATE items
            SET deleted = true
            WHERE id=$1
        ",
    )
    .bind(item_id)
    .execute(&db)
    .await?;

    if result.rows_affected() == 0 {
        let err = Error::SQLEntityNotFound {
            entity_type: "item",
            id: item_id,
        };
        return Err(err);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::models::item::{Item, ItemForCreate};
    use anyhow::Result;

    use super::*;

    #[sqlx::test]
    async fn item_create(pool: Db) -> Result<()> {
        // create new item
        let item_fc = ItemForCreate {
            quantity: "15kg".to_owned(),
            name: "bejca".to_owned(),
            value: 13000,
            additional_info: None,
        };
        let id = create(Session::BASIC(), item_fc, 0, pool.clone()).await?;
        assert_eq!(id, 1); // on empty db first item should have 1 id

        // fetch item
        let item: Option<Item> = sqlx::query_as("SELECT * FROM items WHERE id = $1")
            .bind(id)
            .fetch_optional(&pool)
            .await?;

        assert!(item.is_some());

        // created and fetched should be the same
        let found_id = item.ok_or(anyhow::anyhow!("unwrap"))?.id;

        assert_eq!(found_id, id);

        Ok(())
    }

    #[sqlx::test]
    async fn item_delete(pool: Db) -> Result<()> {
        // create new item
        let item_fc = ItemForCreate {
            quantity: "15kg".to_owned(),
            name: "bejca".to_owned(),
            value: 13000,
            additional_info: None,
        };
        let id = create(Session::BASIC(), item_fc, 0, pool.clone()).await?;

        // fetch it
        let item: Item = sqlx::query_as("SELECT * FROM items WHERE id = $1")
            .bind(id)
            .fetch_one(&pool)
            .await?;

        // should not be deleted
        assert!(!item.deleted);

        // delete it
        delete(Session::BASIC(), id, pool.clone()).await?;

        // fetch it
        let item: Item = sqlx::query_as("SELECT * FROM items WHERE id = $1")
            .bind(id)
            .fetch_one(&pool)
            .await?;

        // should not be deleted
        assert!(item.deleted);
        Ok(())
    }

    #[sqlx::test]
    async fn item_delete_entity_not_found(pool: Db) -> Result<()> {
        let id = 4;
        let result = delete(Session::BASIC(), id, pool.clone()).await;
        assert_eq!(
            result,
            Err(crate::Error::SQLEntityNotFound {
                entity_type: "item",
                id
            })
        );
        Ok(())
    }

    #[sqlx::test]
    async fn item_read_where_order_id(pool: Db) -> Result<()> {
        // create 3 items
        let names = ["bejca", "lakier", "klej"];
        let objects: Vec<ItemForCreate> = names
            .iter()
            .map(|name| ItemForCreate {
                name: name.to_string(),
                quantity: "15kg".to_owned(),
                value: 13000,
                additional_info: None,
            })
            .collect();
        // push them to db with different order_id
        let id1_order1 = create(Session::BASIC(), objects[0].clone(), 1, pool.clone()).await?;
        let id2_order1 = create(Session::BASIC(), objects[1].clone(), 1, pool.clone()).await?;
        let id3_order2 = create(Session::BASIC(), objects[2].clone(), 2, pool.clone()).await?;

        // assert if filter works
        let res = read_where_order_id(Session::BASIC(), 1, pool.clone()).await?;
        assert_eq!(res.len(), 2);
        assert_eq!(res[0].id, id1_order1);
        assert_eq!(res[1].id, id2_order1);

        let res = read_where_order_id(Session::BASIC(), 2, pool.clone()).await?;
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].id, id3_order2);
        Ok(())
    }
}
