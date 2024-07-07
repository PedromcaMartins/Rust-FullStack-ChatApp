use rocket::{get, post, serde::json::Json};

use crate::{message::{dtos::{MessageResponseDTO, NewMessageRequestDTO}, handlers}, DbConn};

#[get("/messages")]
pub async fn get_messages(
    conn: DbConn
) -> Json<Vec<MessageResponseDTO>> {
    Json(
        conn.run(handlers::get_messages).await
    )
}

#[post("/messages", data="<new_message>")]
pub async fn post_message(
    conn: DbConn, 
    new_message: Json<NewMessageRequestDTO>
) -> Json<MessageResponseDTO> {
    Json(
        conn.run(|c| 
            handlers::post_message(c, new_message.into_inner())
        ).await
    )
}