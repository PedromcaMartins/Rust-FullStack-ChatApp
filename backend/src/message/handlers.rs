use chrono::Utc;

use crate::message::dtos::{MessageResponseDTO, NewMessageRequestDTO};

use super::repository::{insert_message_to_db, load_messages_from_db};

pub fn get_messages(
    conn: &mut diesel::SqliteConnection
) -> Vec<MessageResponseDTO> {
    load_messages_from_db(conn)
        .unwrap()
        .iter()
        .map(|m| m.clone().into())
        .collect()
}

pub fn post_message(
    conn: &mut diesel::SqliteConnection, 
    new_message: NewMessageRequestDTO
) -> MessageResponseDTO {
    insert_message_to_db(
        conn, 
        Utc::now().to_rfc3339(), 
        new_message.content
    )
        .unwrap()
        .into()
}