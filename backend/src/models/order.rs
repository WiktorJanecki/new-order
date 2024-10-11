use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use super::item::{Item, ItemResponseBasic};

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

#[derive(Clone)]
pub struct OrderForCreate {
    pub receiver: String,
    pub additional_info: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct OrderResponseBasic {
    pub id: i32,
    pub time_created: chrono::NaiveDateTime,
    pub receiver: String,
    pub additional_info: Option<String>,
    pub items: Vec<ItemResponseBasic>,
}
#[derive(Serialize, Deserialize, FromRow)]
pub struct OrderResponseFull {
    pub id: i32,
    pub creator_id: i32,
    pub time_created: chrono::NaiveDateTime,
    pub receiver: String,
    pub additional_info: Option<String>,
    pub deleted: bool,
    pub paid: bool,
    pub items: Vec<Item>,
}
#[derive(Deserialize)]
pub struct OrderListParams {
    #[serde(alias = "dateStart")]
    pub date_start: Option<chrono::NaiveDate>,
    #[serde(alias = "dateEnd")]
    pub date_end: Option<chrono::NaiveDate>,
}
