use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(FromRow)]
pub struct Item {
    pub id: i32,
    pub order_id: i32,
    pub creator_id: i32,
    pub time_created: chrono::NaiveDateTime,

    pub quantity: String, // for example 1l or 5kg
    pub name: String,     // for example czerwona farba
    pub value: i32,       // for example 100_00 = 100PLN
    pub additional_info: Option<String>,
    pub deleted: bool,
}

#[derive(Clone)]
pub struct ItemForCreate {
    pub quantity: String,
    pub name: String,
    pub value: i32,
    pub additional_info: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct ItemResponseBasic {
    pub id: i32,
    pub order_id: i32,
    pub time_created: chrono::NaiveDateTime,
    pub quantity: String, // for example 1l or 5kg
    pub name: String,     // for example czerwona farba
    pub value: i32,       // for example 100_00 = 100PLN
    pub additional_info: Option<String>,
}
