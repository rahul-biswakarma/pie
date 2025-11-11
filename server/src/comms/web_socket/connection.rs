use std::collections::HashMap;

use anyhow::Result;
use axum::{
    extract::{
        Query, State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{
    AppState,
    auth::{Claims, validate_jwt_token},
    comms::{
        store::{ConnId, ConnMetaData, SFU_PEERS, SOCKET_CONNS},
        web_socket::{
            message::{WsMessage, WsResponse},
            protocol::route_message,
        },
    },
    redis,
};

use axum::http::StatusCode;

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    Query(params): Query<HashMap<String, String>>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    let token = params.get("token").ok_or_else(|| {
        tracing::warn!("WebSocket connection attempt without token");
        StatusCode::UNAUTHORIZED
    })?;

    let claims = validate_jwt_token(token, &state).await.map_err(|e| {
        tracing::error!("JWT validation failed: {}", e);
        StatusCode::UNAUTHORIZED
    })?;

    tracing::info!(
        "WebSocket connection established for user: {}",
        claims.user_id()
    );

    Ok(ws.on_upgrade(move |socket| handle_upgrade(socket, claims, state)))
}

async fn handle_upgrade(socket: WebSocket, claims: Claims, state: AppState) {
    let conn_id = Uuid::new_v4();
    let (tx, mut rx) = mpsc::channel::<Message>(156);

    SOCKET_CONNS.insert(conn_id, tx.clone());

    let conn_meta_data = ConnMetaData {
        user_id: claims.user_id().to_string(),
        room_id: "".to_string(), // Default empty room_id, will be updated on join
        last_verified_at: chrono::Utc::now(),
    };

    if let Err(e) = redis::set_conn_metadata(state.redis.clone(), &conn_id, &conn_meta_data).await {
        tracing::error!("Failed to set connection metadata in Redis: {}", e);
        // Handle error, perhaps close connection or log more severely
    }

    let (mut ws_sender, mut ws_receiver) = socket.split();
    let state_clone = state.clone();

    let conn_id_clone = conn_id;
    let reader = tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_receiver.next().await {
            if let Message::Text(text) = msg {
                let message: WsMessage = match serde_json::from_str(&text) {
                    Ok(msg) => msg,
                    Err(e) => {
                        let txt_string: String = text.to_string();
                        if txt_string == "ping" {
                            let _ = tx.send(Message::Text("pong".into())).await;
                            continue;
                        }
                        tracing::error!("Failed to parse WebSocket message: {}", e);
                        let error_response = WsResponse::Error {
                            message: format!("Invalid message format: {}", e),
                        };
                        if let Ok(error_json) = serde_json::to_string(&error_response) {
                            let _ = tx.send(Message::Text(error_json.into())).await;
                        }
                        continue; // Skip to the next message
                    }
                };

                if let Err(e) =
                    route_message(message, conn_id_clone, tx.clone(), &state_clone).await
                {
                    tracing::error!("Error handling message: {}", e);
                    let error_response = WsResponse::Error {
                        message: e.to_string(),
                    };
                    if let Ok(error_json) = serde_json::to_string(&error_response) {
                        let _ = tx.send(Message::Text(error_json.into())).await;
                    }
                }
            }
        }

        cleanup_connection(conn_id_clone, state_clone).await;
    });

    while let Some(msg) = rx.recv().await {
        if ws_sender.send(msg).await.is_err() {
            break;
        }
    }

    let _ = reader.await;
    cleanup_connection(conn_id, state).await;
}

async fn cleanup_connection(conn_id: ConnId, state: AppState) {
    SOCKET_CONNS.remove(&conn_id);
    SFU_PEERS.remove(&conn_id);

    if let Err(e) = redis::remove_connection(state.redis.clone(), &conn_id).await {
        tracing::error!(
            "Failed to remove connection from Redis during cleanup: {}",
            e
        );
    }

    tracing::info!("Cleaned up connection: {}", conn_id);
}
