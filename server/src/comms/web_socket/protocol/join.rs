use crate::{
    comms::{
        store::{CONN_METADATA, ConnId, ROOMS, RoomId},
        web_socket::{message::WsResponse, room::broadcast_to_room},
    },
    error::WsError,
};
use axum::extract::ws::Message;
use nanoid::nanoid;
use tokio::sync::mpsc::Sender;

pub async fn handle_join(
    conn_id: ConnId,
    room: Option<String>,
    user_id: String,
    sender: Sender<Message>,
) -> Result<(), WsError> {
    let room_id: RoomId = room.unwrap_or_else(|| nanoid!());

    ROOMS
        .entry(room_id.clone())
        .or_insert_with(|| dashmap::DashSet::new())
        .insert(conn_id);

    let mut meta = CONN_METADATA
        .entry(conn_id)
        .or_insert_with(crate::comms::store::ConnMetaData::default);

    meta.room_id = room_id.clone();
    meta.user_id = user_id.clone();

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

    broadcast_to_room(room_id, Message::Text(peer_joined_json.into()))
        .await
        .unwrap_or_else(|e| {
            tracing::warn!("Failed to broadcast peer joined message: {}", e);
        });

    Ok(())
}
