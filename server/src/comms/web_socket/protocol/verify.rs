use tokio::sync::mpsc::Sender;

use axum::extract::ws::Message;
use anyhow::Result;

use crate::{
    comms::{web_socket::message::WsResponse},
    error::WsError,
    AppState,
    redis,
};

pub async fn handle_verify_room(room: String, sender: Sender<Message>, state: AppState) -> Result<(), WsError> {
    let room_members = redis::get_room_members(state.redis.clone(), &room).await?;
    let room_exists = !room_members.is_empty();

    let response = if room_exists {
        WsResponse::VerifySuccess { room }
    } else {
        WsResponse::VerifyError {
            error: "Room does not exist".to_string(),
        }
    };

    let response_json = serde_json::to_string(&response)
        .map_err(|e| WsError::Serialization(format!("Failed to serialize response: {}", e)))?;

    sender
        .send(Message::Text(response_json.into()))
        .await
        .map_err(|e| WsError::SendFailed(e.to_string()))?;

    Ok(())
}
