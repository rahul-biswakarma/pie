use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WsEvents {
    Join { room: String, user_id: String },
}
