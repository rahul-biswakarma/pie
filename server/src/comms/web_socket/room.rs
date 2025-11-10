use axum::extract::ws::Message;
use tracing::warn;
use anyhow::Result;
use uuid::Uuid;

use crate::{
    comms::store::{RoomId, SOCKET_CONNS},
    error::WsError,
    AppState,
    redis,
};

pub async fn broadcast_to_room(room_id: RoomId, message: Message, state: AppState) -> Result<(), WsError> {
    let room_members = redis::get_room_members(state.redis.clone(), &room_id).await.map_err(|_| WsError::RoomNotFound)?;

    let mut failed_connections = Vec::new();

    for conn_id_str in room_members.iter() {
        let conn_id = Uuid::parse_str(conn_id_str).map_err(|_| WsError::InternalServerError)?;
        if let Some(sender) = SOCKET_CONNS.get(&conn_id) {
            if sender.send(message.clone()).await.is_err() {
                failed_connections.push(conn_id);
            }
        }
    }

    for conn_id in failed_connections {
        if let Err(e) = redis::remove_from_room(state.redis.clone(), &room_id, &conn_id).await {
            warn!("Failed to remove connection {} from room {} in Redis: {}", conn_id, room_id, e);
        }
        warn!(
            "Removed failed connection {} from room {}",
            conn_id, room_id
        );
    }

    Ok(())
}
