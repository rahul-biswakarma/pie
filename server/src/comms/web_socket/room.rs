use axum::extract::ws::Message;
use tracing::warn;

use crate::{
    comms::store::{ROOMS, RoomId, SOCKET_CONNS},
    error::WsError,
};

pub async fn broadcast_to_room(room_id: RoomId, message: Message) -> Result<(), WsError> {
    let room = ROOMS.get(&room_id).ok_or_else(|| WsError::RoomNotFound)?;

    let mut failed_connections = Vec::new();

    for conn_id in room.iter() {
        if let Some(sender) = SOCKET_CONNS.get(&conn_id) {
            if sender.send(message.clone()).await.is_err() {
                failed_connections.push(*conn_id);
            }
        }
    }

    for conn_id in failed_connections {
        room.remove(&conn_id);
        warn!(
            "Removed failed connection {} from room {}",
            conn_id, room_id
        );
    }

    Ok(())
}
