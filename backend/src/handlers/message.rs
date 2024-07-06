use chrono::Utc;

use crate::{dtos::message::{MessageResponseDTO, NewMessageRequestDTO}, models::message::Message, AppState};

pub async fn get_messages(state: &AppState) -> Vec<MessageResponseDTO> {
    let messages = state.messages.lock().await;
    messages.iter().map(|message| message.clone().into()).collect()
}

pub async fn post_message(state: &AppState, new_message: NewMessageRequestDTO) -> MessageResponseDTO {
    let mut messages = state.messages.lock().await;
    let message = Message {
        id: state.get_new_message_id().unwrap(), 
        timestamp: Utc::now().to_rfc3339(),
        content: new_message.content
    };
    messages.push(message.clone());
    message.into()
}