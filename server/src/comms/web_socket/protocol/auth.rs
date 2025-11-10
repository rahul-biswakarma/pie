use axum::extract::ws::Message;
use chrono::Utc;
use tokio::sync::mpsc::Sender;
use anyhow::Result;

use crate::{
    AppState,
    auth::validate_jwt_token,
    comms::{
        store::{ConnId, ConnMetaData},
        web_socket::message::WsResponse,
    },
    error::WsError,
    redis,
};

pub async fn handle_refresh_token(
    conn_id: ConnId,
    token: String,
    sender: Sender<Message>,
    state: &AppState,
) -> Result<(), WsError> {
    match validate_jwt_token(&token, state).await {
        Ok(claims) => {
            let mut meta = redis::get_conn_metadata(state.redis.clone(), &conn_id)
                .await
                .map_err(|e| WsError::Redis(e.to_string()))?
                .unwrap_or_else(|| ConnMetaData {
                    user_id: "".to_string(), // Will be overwritten
                    room_id: "".to_string(),
                    last_verified_at: Utc::now(), // Will be overwritten
                });

            meta.user_id = claims.user_id().to_string();
            meta.last_verified_at = Utc::now();

            redis::set_conn_metadata(state.redis.clone(), &conn_id, &meta)
                .await
                .map_err(|e| WsError::Redis(e.to_string()))?;

            let response = WsResponse::AuthOk;
            let response_json = serde_json::to_string(&response)
                .map_err(|e| WsError::Serialization(format!("Failed to serialize response: {}", e)))?;

            sender
                .send(Message::Text(response_json.into()))
                .await
                .map_err(|e| WsError::SendFailed(e.to_string()))?;
        }
        Err(e) => {
            let response = WsResponse::AuthFailed;
            let response_json = serde_json::to_string(&response)
                .map_err(|e| WsError::Serialization(format!("Failed to serialize response: {}", e)))?;

            let _ = sender.send(Message::Text(response_json.into())).await;

            return Err(WsError::Authentication(e));
        }
    }

    Ok(())
}
