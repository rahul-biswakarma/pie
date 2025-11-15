use crate::{
    store::{ClientMap, ClientMetadata, ConnId, RoomMap},
    webscoket::events::WsOutboundEvents,
};
use tracing::{error, warn};

pub async fn handle_join(
    conn_id: ConnId,
    room_id: String,
    client_map: ClientMap,
    room_map: RoomMap,
    metadata_map: ClientMetadata,
) {
    // Add conn_id to room
    room_map.entry(room_id.clone()).or_default().push(conn_id);

    // Update metadata
    if let Some(mut meta) = metadata_map.get_mut(&conn_id) {
        meta.room_id = Some(room_id.clone());
    }

    // Send confirmation to client
    if let Some(sender) = client_map.get(&conn_id) {
        let msg = match serde_json::to_string(&WsOutboundEvents::JoinOk { room: room_id }) {
            Ok(m) => m,
            Err(e) => {
                error!("Failed to serialize JoinOk message: {}", e);
                return;
            }
        };
        if sender.send(msg).await.is_err() {
            warn!("sending join_ok to {} failed", conn_id);
        }
    }
}
