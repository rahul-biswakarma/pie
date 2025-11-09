use axum::extract::ws::Message;
use tokio::sync::mpsc::Sender;

use crate::{
    AppState,
    comms::{store::ConnId, web_socket::message::WsMessage},
    error::WsError,
};

mod auth;
mod ice;
mod join;
mod offer;
mod verify;

pub use auth::handle_refresh_token;
pub use ice::handle_ice_candidate;
pub use join::handle_join;
pub use offer::handle_offer;
pub use verify::handle_verify_room;

pub async fn route_message(
    message: WsMessage,
    conn_id: ConnId,
    sender: Sender<Message>,
    state: &AppState,
) -> Result<(), WsError> {
    match message {
        WsMessage::Join { room, user_id } => handle_join(conn_id, room, user_id, sender).await,
        WsMessage::VerifyRoom { room } => handle_verify_room(room, sender).await,
        WsMessage::Offer { sdp } => handle_offer(conn_id, sdp, sender).await,
        WsMessage::IceCandidate { candidate } => handle_ice_candidate(conn_id, candidate).await,
        WsMessage::RefreshToken { token } => {
            handle_refresh_token(conn_id, token, sender, state).await
        }
    }
}
