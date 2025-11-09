use std::collections::HashMap;

use axum::{
    extract::{
        Query, State, WebSocketUpgrade,
        ws::{Message, Utf8Bytes, WebSocket},
    },
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{
    AppState,
    auth::{validate_jwt_token, Claims},
    comms::{
        store::{CONN_METADATA, ConnId, SOCKET_CONNS},
        web_socket::{message::{WsMessage, WsResponse}, protocol::route_message},
    },
    error::WsError,
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

    CONN_METADATA.insert(
        conn_id,
        crate::comms::store::ConnMetaData {
            user_id: claims.user_id().to_string(),
            last_verified_at: chrono::Utc::now(),
            ..Default::default()
        },
    );

    let (mut ws_sender, mut ws_receiver) = socket.split();
    let state_clone = state.clone();

    let conn_id_clone = conn_id;
    let reader = tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_receiver.next().await {
            if let Message::Text(text) = msg {
                if let Err(e) =
                    handle_message(text, conn_id_clone, tx.clone(), state_clone.clone()).await
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

        cleanup_connection(conn_id_clone);
    });

    while let Some(msg) = rx.recv().await {
        if ws_sender.send(msg).await.is_err() {
            break;
        }
    }

    let _ = reader.await;
    cleanup_connection(conn_id);
}

async fn handle_message(
    text: Utf8Bytes,
    conn_id: ConnId,
    tx: mpsc::Sender<Message>,
    state: AppState,
) -> Result<(), WsError> {
    let message: WsMessage = serde_json::from_str(&text)?;
    route_message(message, conn_id, tx, &state).await
}

fn cleanup_connection(conn_id: ConnId) {
    SOCKET_CONNS.remove(&conn_id);
    CONN_METADATA.remove(&conn_id);
    crate::comms::store::SFU_PEERS.remove(&conn_id);

    use crate::comms::store::ROOMS;
    ROOMS.iter_mut().for_each(|room| {
        room.value().remove(&conn_id);
    });

    tracing::info!("Cleaned up connection: {}", conn_id);
}
