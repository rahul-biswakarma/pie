use crate::{
    error::{logger, LogType},
    store::{ClientMap, ClientMetadata, ConnId, RoomMap, WsMetadata},
    webscoket::events::WsOutboundEvents,
};

pub async fn handle_join(
    conn_id: ConnId,
    room: String,
    user_id: String,
    client_map: ClientMap,
    room_map: RoomMap,
    metadata_map: ClientMetadata,
) {
    let client_map_guard = client_map.lock().await;
    let sender = if let Some(s) = client_map_guard.get(&conn_id) {
        s
    } else {
        // close connection
        return;
    };

    // if room map exists, add the conn_id else create room entry and add
    let mut room_map_guard = room_map.lock().await;
    match room_map_guard.get_mut(&room) {
        Some(conns_vec) => {
            conns_vec.push(conn_id);
        }
        None => {
            room_map_guard.insert(room.clone(), vec![conn_id]);
        }
    }

    metadata_map
        .lock()
        .await
        .insert(conn_id, WsMetadata { user_id });

    if sender
        .send(serde_json::to_string(&WsOutboundEvents::JoinOk { room }).unwrap())
        .await
        .is_err()
    {
        logger(LogType::Error, "sending join_ok failed");
    }
}
