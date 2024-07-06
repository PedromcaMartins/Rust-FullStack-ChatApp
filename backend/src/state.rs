use std::sync::Arc;

use rocket::tokio::sync::Mutex;

use crate::models::message::Message;

pub struct AppState {
    pub messages: Arc<Mutex<Vec<Message>>>,
    pub messages_counter: i64,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            messages: Arc::new(Mutex::new(vec![])),
            messages_counter: 0,
        }
    }

    pub fn get_new_message_id(&self) -> Option<i64> {
        self.messages_counter.checked_add(1)
    }
}