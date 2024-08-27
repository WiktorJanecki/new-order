// User, UserForCreate

#[derive(sqlx::Type)]
pub enum Privileges {
    // Add new orders, see orders
    Basic,

    // See all data
    Full,
}

pub struct User {
    id: i32,
    name: String,
    password: String,
    privileges: Privileges,
}
