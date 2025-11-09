use axum::extract::ws::Message;
use tokio::sync::mpsc::Sender;
use webrtc::ice_transport::ice_candidate::RTCIceCandidate;

use crate::comms::web_socket::message::WsResponse;

pub async fn send_ice_candidate_to_client(tx: Sender<Message>, candidate: RTCIceCandidate) {
    if let Ok(json_candidate) = candidate.to_json() {
        let response = WsResponse::IceCandidate {
            candidate: serde_json::to_string(&json_candidate).unwrap_or_default(),
        };

        if let Ok(payload) = serde_json::to_string(&response) {
            let _ = tx.send(Message::Text(payload.into())).await;
        }
    }
}
