use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WsInboundEvents {
    Join { room: String },
    ListParticipants,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WsOutboundEvents {
    JoinOk { room: String },
    Participants { users: Vec<String> },
    Error { message: String },
}
