use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Serialize, Debug, Deserialize)]
pub struct OrderResponseBasic {
    pub id: i32,
    pub time_created: chrono::NaiveDateTime,
    pub receiver: String,
    pub additional_info: Option<String>,
    pub items: Vec<ItemResponseBasic>,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Serialize, Deserialize, Debug)]
pub struct ItemResponseBasic {
    pub id: i32,
    pub order_id: i32,
    pub time_created: chrono::NaiveDateTime,
    pub quantity: String, // for example 1l or 5kg
    pub name: String,     // for example czerwona farba
    pub value: i32,       // for example 100_00 = 100PLN
    pub additional_info: Option<String>,
}
