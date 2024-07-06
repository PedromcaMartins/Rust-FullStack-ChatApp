use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: i64,
    pub timestamp: String,
    pub content: String,
}