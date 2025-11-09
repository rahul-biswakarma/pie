use webrtc::ice_transport::ice_candidate::RTCIceCandidateInit;

use crate::{
    comms::store::{ConnId, SFU_PEERS},
    error::WsError,
};

pub async fn handle_ice_candidate(conn_id: ConnId, candidate: String) -> Result<(), WsError> {
    let pc = SFU_PEERS
        .get(&conn_id)
        .ok_or(WsError::ConnectionNotFound)?
        .clone();

    let ice: RTCIceCandidateInit = serde_json::from_str(&candidate)
        .map_err(|e| WsError::InvalidMessage(format!("Invalid ICE candidate: {}", e)))?;

    pc.add_ice_candidate(ice)
        .await
        .map_err(|e| WsError::WebRTC(format!("Failed to add ICE candidate: {}", e)))?;

    Ok(())
}
