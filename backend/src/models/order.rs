use std::time::SystemTime;

pub struct Order {
    id: i32,
    creator_id: i32,
    time_created: SystemTime,
    receiver: String,
    additional_info: Option<String>,
}
