use axum::extract::ws::Message;
use chrono::Utc;
use tokio::sync::mpsc::Sender;

use crate::{
    AppState,
    auth::validate_jwt_token,
    comms::{
        store::{CONN_METADATA, ConnId, ConnMetaData},
        web_socket::message::WsResponse,
    },
    error::WsError,
};

pub async fn handle_refresh_token(
    conn_id: ConnId,
    token: String,
    sender: Sender<Message>,
    state: &AppState,
) -> Result<(), WsError> {
    match validate_jwt_token(&token, state).await {
        Ok(claims) => {
            let mut meta = CONN_METADATA
                .entry(conn_id)
                .or_insert_with(ConnMetaData::default);

            meta.user_id = claims.user_id().to_string();
            meta.last_verified_at = Utc::now();

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
