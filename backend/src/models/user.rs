// User, UserForCreate

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, sqlx::Type)]
pub enum Privileges {
    // Add new orders, see orders
    Basic,

    // See all data
    Full,
}

#[derive(FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub password: String,
    pub privileges: Privileges,
}
