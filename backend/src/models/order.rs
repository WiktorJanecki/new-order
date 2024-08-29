use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use super::item::ItemResponseBasic;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Order {
    pub id: i32,
    pub creator_id: i32,
    pub time_created: chrono::NaiveDateTime,
    pub receiver: String,
    pub additional_info: Option<String>,
    pub deleted: bool,
    pub paid: bool,
}

pub struct OrderForCreate {
    pub receiver: String,
    pub additional_info: Option<String>,
}

pub struct OrderForUpdate {
    pub receiver: Option<String>,
    pub additional_info: Option<String>,
}

#[derive(Serialize)]
pub struct OrderResponseBasic {
    pub id: i32,
    pub time_created: chrono::NaiveDateTime,
    pub receiver: String,
    pub additional_info: Option<String>,
    pub items: Vec<ItemResponseBasic>,
}
