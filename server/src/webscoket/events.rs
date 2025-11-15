use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WsInboundEvents {
    Join { room: String, user_id: String },
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WsOutboundEvents {
    JoinOk { room: String },
}
