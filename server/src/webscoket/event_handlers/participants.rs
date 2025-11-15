use crate::{
    store::{ClientMap, ClientMetadata, ConnId, RoomMap},
    webscoket::{events::WsOutboundEvents, utils::close_connection},
};
use tracing::{warn};

pub async fn handle_list_participants(
    conn_id: ConnId,
    client_map: ClientMap,
    room_map: RoomMap,
    metadata_map: ClientMetadata,
) {
    let user_meta = if let Some(meta) = metadata_map.get(&conn_id) {
        meta.clone()
    } else {
        warn!(
            "Could not find metadata for conn_id {}. Closing connection.",
            conn_id
        );
        close_connection(conn_id, &client_map, &metadata_map, &room_map);
        return;
    };

    let room_id = if let Some(id) = user_meta.room_id {
        id
    } else {
        warn!(
            "User {} is not in a room, but requested participants. Closing connection.",
            conn_id
        );
        close_connection(conn_id, &client_map, &metadata_map, &room_map);
        return;
    };

    let room_participants_conn_ids = if let Some(conns) = room_map.get(&room_id) {
        conns.clone()
    } else {
        // This shouldn't happen if metadata is correct
        warn!(
            "Room {} not found for user {}. Closing connection.",
            room_id, conn_id
        );
        close_connection(conn_id, &client_map, &metadata_map, &room_map);
        return;
    };

    let mut participants_user_ids = Vec::new();
    for participant_conn_id in room_participants_conn_ids.iter() {
        if let Some(meta) = metadata_map.get(participant_conn_id) {
            participants_user_ids.push(meta.user_id.clone());
        }
    }

    if let Some(sender) = client_map.get(&conn_id) {
        let event = WsOutboundEvents::Participants {
            users: participants_user_ids,
        };
        let msg = match serde_json::to_string(&event) {
            Ok(m) => m,
            Err(e) => {
                warn!("Failed to serialize Participants event: {}", e);
                return;
            }
        };

        if sender.send(msg).await.is_err() {
            warn!("Failed to send participants list to {}", conn_id);
        }
    }
}
