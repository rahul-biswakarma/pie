use axum::{
    extract::{
        ws::{Message, WebSocket},
        Query, State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    error::{logger, LogType},
    store::{ClientMap, ConnId, RoomMap},
    webscoket::{event_handlers::handle_join, events::WsInboundEvents},
    AppState,
};

#[derive(Debug, Deserialize)]
pub struct AuthParams {
    token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenClaims {
    // 'sub' stands for "Subject".
    // In Supabase, this is the user's unique ID (UUID).
    sub: String,

    // 'aud' stands for "Audience".
    // In Supabase, this is "authenticated" for logged-in users.
    aud: String,

    // 'exp' stands for "Expiration Time".
    // The jsonwebtoken crate automatically uses this
    // to check if the token is expired.
    exp: usize,
}

pub async fn handle_ws_upgrade(
    ws: WebSocketUpgrade,
    Query(params): Query<AuthParams>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let token = match params.token {
        Some(token) => token,
        None => {
            let msg = "auth token missing";
            logger(LogType::Error, msg);
            return (StatusCode::NON_AUTHORITATIVE_INFORMATION, msg).into_response();
        }
    };

    let decoding_key = DecodingKey::from_secret(state.jwt_secret.as_bytes());

    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_audience(&["authenticated"]);

    match decode::<TokenClaims>(&token, &decoding_key, &validation) {
        Ok(_claims) => ws.on_upgrade(|socket| handle_socket(socket, state)),
        Err(e) => {
            let error_message = format!("JWT validation failed: {}", e);
            logger(LogType::Error, &error_message);

            (StatusCode::UNAUTHORIZED, error_message).into_response()
        }
    }
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    let conn_id = Uuid::new_v4();

    let (mut ws_sender, mut ws_receiver) = socket.split();
    let (tx_outbound, mut rx_outbound) = tokio::sync::mpsc::channel::<String>(156);

    let tx_inbound_clone = state.tx_inbound.clone();

    state
        .client_map
        .lock()
        .await
        .insert(conn_id, tx_outbound.clone());

    // precessing pre-client queue
    tokio::spawn(async move {
        while let Some(message) = rx_outbound.recv().await {
            if ws_sender.send(Message::Text(message.into())).await.is_err() {
                logger(LogType::Error, "Sending message to client failed");
            }
        }
    });

    // registering ws_message into global mpsc channel
    while let Some(Ok(ws_message)) = ws_receiver.next().await {
        match ws_message {
            Message::Text(txt_message) => {
                if txt_message == "ping" {
                    if tx_outbound.send("pong".into()).await.is_err() {
                        logger(LogType::Error, "pong failed");
                    }
                } else if tx_inbound_clone
                    .send((conn_id, txt_message.to_string()))
                    .await
                    .is_err()
                {
                    logger(LogType::Error, "Setting message to inbonud queue failed");
                };
            }
            Message::Ping(_ping_payload) => {}
            _ => {}
        }
    }
}

pub async fn handle_text_message(
    conn_id: ConnId,
    message: String,
    client_map: ClientMap,
    room_map: RoomMap,
) {
    let parsed_message: Result<WsInboundEvents, serde_json::Error> = serde_json::from_str(&message);
    match parsed_message {
        Ok(message) => match message {
            WsInboundEvents::Join { room, user_id } => {
                handle_join(conn_id, room, user_id, client_map, room_map).await;
            }
        },
        Err(_e) => {
            logger(LogType::Error, "Failed parsing client message json");
        }
    }
}
