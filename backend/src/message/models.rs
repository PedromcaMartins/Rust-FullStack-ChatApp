use diesel::{deserialize::Queryable, Selectable};
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Queryable, Selectable)]
#[diesel(table_name = crate::schema::message)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Message {
    pub id: Option<i32>,
    pub timestamp: String,
    pub content: String,
}