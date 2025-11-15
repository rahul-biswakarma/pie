use crate::store::{ClientMap, ClientMetadata, ConnId, RoomMap};
use tracing::{info, warn};

pub fn close_connection(
    conn_id: ConnId,
    client_map: &ClientMap,
    metadata_map: &ClientMetadata,
    room_map: &RoomMap,
) {
    info!("Force closing connection for {}", conn_id);

    // Removing from client_map will trigger the drop of the sender, which in turn should
    // close the websocket connection from the server side.
    if client_map.remove(&conn_id).is_none() {
        warn!("Client {} not found in client_map for closing.", conn_id);
    }

    if let Some((_, metadata)) = metadata_map.remove(&conn_id) {
        if let Some(room_id) = metadata.room_id {
            if let Some(mut room) = room_map.get_mut(&room_id) {
                room.retain(|id| *id != conn_id);
                info!("Removed {} from room {}", conn_id, room_id);
            } else {
                warn!(
                    "Room {} not found for cleanup for user {}",
                    room_id, conn_id
                );
            }
        }
    } else {
        warn!("Metadata not found for client {} for cleanup.", conn_id);
    }
}