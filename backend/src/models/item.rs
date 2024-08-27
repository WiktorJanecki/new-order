use std::time::SystemTime;

pub struct Item {
    pub id: i32,
    pub order_id: i32,
    pub creator_id: i32,
    pub time_created: SystemTime,

    pub quantity: String, // for example 1l or 5kg
    pub name: String,     // for example czerwona farba
    pub value: i32,       // for example 100_00 = 100PLN
    pub additional_info: Option<String>,
}
