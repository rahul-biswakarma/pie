use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WsMessage {
    Join {
        room: Option<String>,
        user_id: String,
    },
    Offer {
        sdp: String,
    },
    IceCandidate {
        candidate: String,
    },
    RefreshToken {
        token: String,
    },
    VerifyRoom {
        room: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WsResponse {
    VerifySuccess { room: String },
    VerifyError { error: String },

    AuthOk,
    AuthFailed,

    JoinOk { room: String },
    PeerJoined { user_id: String },

    Offer { sdp: String },
    Answer { sdp: String },
    IceCandidate { candidate: String },

    Error { message: String },
}
