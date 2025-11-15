use crate::{
    store::{ConnId, WsMetadata},
    webscoket::{
        event_handlers::{handle_join, handle_list_participants},
        events::{WsInboundEvents, WsOutboundEvents},
    },
    AppState,
};
use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    http::HeaderMap,
    response::{IntoResponse, Response},
};
use futures_util::{SinkExt, StreamExt};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct TokenClaims {
    sub: String,
    aud: String,
    exp: usize,
}

pub async fn handle_ws_upgrade(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Response {
    let protocols = headers
        .get("Sec-WebSocket-Protocol")
        .and_then(|v| v.to_str().ok());

    let token = if let Some(protocols) = protocols {
        // The header can be a comma-separated list of protocols.
        // We are looking for the one that is a JWT.
        // In our case, the client sends only one.
        protocols.split(',').next().unwrap_or("").trim()
    } else {
        let msg = "auth token missing in subprotocol";
        error!(msg);
        return (StatusCode::UNAUTHORIZED, msg).into_response();
    };

    if token.is_empty() {
        let msg = "auth token is empty";
        error!(msg);
        return (StatusCode::UNAUTHORIZED, msg).into_response();
    }

    let decoding_key = DecodingKey::from_secret(state.jwt_secret.as_bytes());
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_audience(&["authenticated"]);

    match decode::<TokenClaims>(token, &decoding_key, &validation) {
        Ok(claims) => {
            let user_id = claims.claims.sub;
            ws.on_upgrade(move |socket| handle_socket(socket, state, user_id))
        }
        Err(e) => {
            let error_message = format!("JWT validation failed: {}", e);
            error!(error_message);
            (StatusCode::UNAUTHORIZED, error_message).into_response()
        }
    }
}

async fn handle_socket(socket: WebSocket, state: AppState, user_id: String) {
    let conn_id = Uuid::new_v4();
    info!("New connection: {} ({})", conn_id, user_id);

    let (mut ws_sender, mut ws_receiver) = socket.split();
    let (tx_outbound, mut rx_outbound) = tokio::sync::mpsc::channel::<String>(156);

    state.client_map.insert(conn_id, tx_outbound.clone());
    state.metadata_map.insert(
        conn_id,
        WsMetadata {
            user_id,
            ..Default::default()
        },
    );

    // Outbound message task
    tokio::spawn(async move {
        while let Some(message) = rx_outbound.recv().await {
            if ws_sender.send(Message::Text(message.into())).await.is_err() {
                warn!(
                    "Sending message to client {} failed, connection might be closed.",
                    conn_id
                );
                break;
            }
        }
    });

    // Inbound message loop
    while let Some(Ok(ws_message)) = ws_receiver.next().await {
        match ws_message {
            Message::Text(txt_message) => {
                if txt_message == "ping" {
                    if tx_outbound.send("pong".to_string()).await.is_err() {
                        warn!("pong failed for client {}", conn_id);
                    }
                    continue;
                }

                let state_clone = state.clone();
                tokio::spawn(async move {
                    handle_text_message(conn_id, txt_message.to_string(), state_clone).await;
                });
            }
            Message::Close(_) => {
                info!("Connection closed by client {}", conn_id);
                break;
            }
            _ => {}
        }
    }

    // Cleanup
    info!("Cleaning up connection: {}", conn_id);
    state.client_map.remove(&conn_id);

    if let Some((_, metadata)) = state.metadata_map.remove(&conn_id) {
        if let Some(room_id) = metadata.room_id {
            if let Some(mut room) = state.room_map.get_mut(&room_id) {
                room.retain(|id| *id != conn_id);
                info!("Removed {} from room {}", conn_id, room_id);
            }
        }
    }
}

pub async fn handle_text_message(conn_id: ConnId, message: String, state: AppState) {
    let parsed_message: Result<WsInboundEvents, serde_json::Error> = serde_json::from_str(&message);
    match parsed_message {
        Ok(message) => match message {
            WsInboundEvents::Join { room } => {
                handle_join(
                    conn_id,
                    room,
                    state.client_map,
                    state.room_map,
                    state.metadata_map,
                )
                .await;
            }
            WsInboundEvents::ListParticipants => {
                handle_list_participants(
                    conn_id,
                    state.client_map,
                    state.room_map,
                    state.metadata_map,
                )
                .await;
            }
        },
        Err(e) => {
            error!("Failed parsing client message json: {}", e);
            if let Some(sender) = state.client_map.get(&conn_id) {
                let event = WsOutboundEvents::Error {
                    message: "Invalid message format".to_string(),
                };
                if let Ok(msg) = serde_json::to_string(&event) {
                    if sender.send(msg).await.is_err() {
                        warn!("Failed to send error message to client {}", conn_id);
                    }
                }
            }
        }
    }
}
