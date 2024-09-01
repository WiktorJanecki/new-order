use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(FromRow, Clone)]
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
    pub checked: bool,
}

#[derive(Clone)]
pub struct ItemForCreate {
    pub quantity: String,
    pub name: String,
    pub value: i32,
    pub additional_info: Option<String>,
}
#[derive(Clone)]
pub struct ItemForUpdate {
    pub quantity: Option<String>,
    pub name: Option<String>,
    pub value: Option<i32>,
    pub additional_info: Option<String>,
    pub checked: Option<bool>,
}

#[derive(Serialize, Debug)]
pub struct ItemResponseBasic {
    pub id: i32,
    pub order_id: i32,
    pub time_created: chrono::NaiveDateTime,
    pub quantity: String, // for example 1l or 5kg
    pub name: String,     // for example czerwona farba
    pub value: i32,       // for example 100_00 = 100PLN
    pub checked: bool,
    pub additional_info: Option<String>,
}
