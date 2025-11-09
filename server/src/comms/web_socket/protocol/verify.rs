use tokio::sync::mpsc::Sender;

use axum::extract::ws::Message;

use crate::{
    comms::{store::ROOMS, web_socket::message::WsResponse},
    error::WsError,
};

pub async fn handle_verify_room(room: String, sender: Sender<Message>) -> Result<(), WsError> {
    let room_exists = ROOMS.contains_key(&room);

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
