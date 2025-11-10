use crate::{
    comms::{
        store::{ConnId, RoomId},
        web_socket::{message::WsResponse, room::broadcast_to_room},
    },
    error::WsError,
    AppState,
    redis,
};
use axum::extract::ws::Message;
use nanoid::nanoid;
use tokio::sync::mpsc::Sender;
use anyhow::Result;
use chrono::Utc;

pub async fn handle_join(
    conn_id: ConnId,
    room: Option<String>,
    user_id: String,
    sender: Sender<Message>,
    state: AppState,
) -> Result<(), WsError> {
    let room_id: RoomId = room.unwrap_or_else(|| nanoid!());

    let mut meta = redis::get_conn_metadata(state.redis.clone(), &conn_id)
        .await
        .map_err(|e| WsError::Redis(e.to_string()))?
        .ok_or(WsError::InternalServerError)?;

    meta.room_id = room_id.clone();
    meta.user_id = user_id.clone();
    meta.last_verified_at = Utc::now();

    redis::set_conn_metadata(state.redis.clone(), &conn_id, &meta)
        .await
        .map_err(|e| WsError::Redis(e.to_string()))?;

    redis::add_to_room(state.redis.clone(), &room_id, &conn_id)
        .await
        .map_err(|e| WsError::Redis(e.to_string()))?;

    let response = WsResponse::JoinOk {
        room: room_id.clone(),
    };
    let response_json = serde_json::to_string(&response)
        .map_err(|e| WsError::Serialization(format!("Failed to serialize response: {}", e)))?;

    sender
        .send(Message::Text(response_json.into()))
        .await
        .map_err(|e| WsError::SendFailed(e.to_string()))?;

    let peer_joined_msg = WsResponse::PeerJoined { user_id };
    let peer_joined_json = serde_json::to_string(&peer_joined_msg)
        .map_err(|e| WsError::Serialization(format!("Failed to serialize peer joined: {}", e)))?;

    broadcast_to_room(room_id, Message::Text(peer_joined_json.into()), state)
        .await
        .unwrap_or_else(|e| {
            tracing::warn!("Failed to broadcast peer joined message: {}", e);
        });

    Ok(())
}
