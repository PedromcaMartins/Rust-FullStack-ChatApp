use serde::{Deserialize, Serialize};

use crate::message::models::Message;

#[derive(Deserialize)]
pub struct NewMessageRequestDTO {
    pub content: String,
}

#[derive(Serialize)]
pub struct MessageResponseDTO {
    pub timestamp: String,
    pub content: String,
}

impl From<Message> for MessageResponseDTO {
    fn from(value: Message) -> Self {
        MessageResponseDTO { 
            timestamp: value.timestamp, 
            content: value.content 
        }
    }
}