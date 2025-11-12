use axum::extract::ws::Message;
use nanoid::nanoid;
use tokio::sync::mpsc::Sender;

use crate::{comms::web_socket::message::WsResponse, error::WsError};

pub async fn handle_create(sender: Sender<Message>) -> Result<(), WsError> {
    let room_id = nanoid!();
    let response = WsResponse::CreateOK { room_id };
    let json_response = serde_json::to_string(&response).map_err(|e| {
        tracing::error!("Failed to serialize CreateOK response: {}", e);
        WsError::Serialization("Failed to serialize CreateOK response".to_string())
    })?;

    sender
        .send(Message::Text(json_response.into()))
        .await
        .map_err(|e| {
            tracing::error!("Failed to send CreateOK response: {}", e);
            WsError::SendFailed("Failed to send CreateOK response".to_string())
        })?;

    Ok(())
}
