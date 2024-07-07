use rocket::{get, post, serde::json::Json, State};

use crate::{message::dtos::{MessageResponseDTO, NewMessageRequestDTO}, message::handlers, AppState};

#[get("/messages")]
pub async fn get_messages(state: &State<AppState>) -> Json<Vec<MessageResponseDTO>> {
    Json(handlers::get_messages(state).await)
}

#[post("/messages", data="<new_message>")]
pub async fn post_message(state: &State<AppState>, new_message: Json<NewMessageRequestDTO>) -> Json<MessageResponseDTO> {
    Json(handlers::post_message(state, new_message.into_inner()).await)
}