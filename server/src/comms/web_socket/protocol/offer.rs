use std::sync::Arc;

use axum::extract::ws::Message;
use tokio::sync::mpsc::Sender;
use webrtc::peer_connection::sdp::session_description::RTCSessionDescription;

use crate::{
    comms::{
        create_peer_connection,
        store::{ConnId, SFU_PEERS},
        web_socket::message::WsResponse,
    },
    error::WsError,
};

pub async fn handle_offer(
    conn_id: ConnId,
    sdp: String,
    sender: Sender<Message>,
) -> Result<(), WsError> {
    let pc = match SFU_PEERS.get(&conn_id) {
        Some(existing_pc) => existing_pc.clone(),
        None => {
            let new_pc = Arc::new(
                create_peer_connection(sender.clone())
                    .await
                    .map_err(|e| WsError::WebRTC(e.to_string()))?,
            );
            SFU_PEERS.insert(conn_id, new_pc.clone());
            new_pc
        }
    };

    let offer = RTCSessionDescription::offer(sdp)
        .map_err(|e| WsError::WebRTC(format!("Invalid offer: {}", e)))?;

    pc.set_remote_description(offer)
        .await
        .map_err(|e| WsError::WebRTC(format!("Failed to set remote description: {}", e)))?;

    let answer = pc
        .create_answer(None)
        .await
        .map_err(|e| WsError::WebRTC(format!("Failed to create answer: {}", e)))?;

    pc.set_local_description(answer.clone())
        .await
        .map_err(|e| WsError::WebRTC(format!("Failed to set local description: {}", e)))?;

    let response = WsResponse::Answer { sdp: answer.sdp };
    let response_json = serde_json::to_string(&response)
        .map_err(|e| WsError::Serialization(format!("Failed to serialize response: {}", e)))?;

    sender
        .send(Message::Text(response_json.into()))
        .await
        .map_err(|e| WsError::SendFailed(e.to_string()))?;

    Ok(())
}
