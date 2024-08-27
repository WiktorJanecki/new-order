use std::time::SystemTime;

pub struct Item {
    id: i32,
    order_id: i32,
    creator_id: i32,
    time_created: SystemTime,

    quantity: String, // for example 1l or 5kg
    name: String,     // for example czerwona farba
    value: i32,       // for example 100_00 = 100PLN
    additional_info: Option<String>,
}
