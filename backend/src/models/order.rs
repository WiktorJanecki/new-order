use std::time::SystemTime;

pub struct Order {
    pub id: i32,
    pub creator_id: i32,
    pub time_created: SystemTime,
    pub receiver: String,
    pub additional_info: Option<String>,
}
