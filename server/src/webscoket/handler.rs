use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::Response,
};
use futures_util::{SinkExt, StreamExt};
use uuid::Uuid;

use crate::{
    error::{logger, LogType},
    store::{ClientMap, ConnId, RoomMap},
    webscoket::{event_handlers::handle_join, events::WsEvents},
    AppState,
};

pub async fn handle_ws_upgrade(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    let conn_id = Uuid::new_v4();

    let (mut ws_sender, mut ws_receiver) = socket.split();
    let (tx_outbound, mut rx_outbound) = tokio::sync::mpsc::channel::<String>(156);

    let tx_inbound_clone = state.tx_inbound.clone();

    state.client_map.lock().await.insert(conn_id, tx_outbound);

    // registering ws_message into global mpsc channel
    while let Some(Ok(ws_message)) = ws_receiver.next().await {
        match ws_message {
            Message::Text(txt_message) => {
                if tx_inbound_clone
                    .send((conn_id, txt_message.to_string()))
                    .await
                    .is_err()
                {
                    logger(LogType::Error, "Setting message to inbonud queue failed");
                }
            }
            Message::Ping(_ping_payload) => {}
            _ => {}
        }
    }

    // precessing pre-client queue
    tokio::spawn(async move {
        while let Some(message) = rx_outbound.recv().await {
            if ws_sender.send(Message::Text(message.into())).await.is_err() {
                logger(LogType::Error, "Sending message to client failed");
            }
        }
    });
}

pub fn handle_text_message(
    conn_id: ConnId,
    message: String,
    client_map: ClientMap,
    room_map: RoomMap,
) {
    let parsed_message: Result<WsEvents, serde_json::Error> = serde_json::from_str(&message);
    match parsed_message {
        Ok(message) => match message {
            WsEvents::Join { room, user_id } => {
                handle_join(conn_id, room, user_id, client_map, room_map);
            }
        },
        Err(_e) => {
            logger(LogType::Error, "Failed parsing client message json");
        }
    }
}
