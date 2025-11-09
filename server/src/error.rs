use axum::response::IntoResponse;
use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Authentication failed: {0}")]
    Authentication(String),

    #[error("Invalid message format: {0}")]
    InvalidMessage(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("WebRTC error: {0}")]
    WebRTC(String),

    #[error("Connection not found")]
    ConnectionNotFound,

    #[error("Room not found")]
    RoomNotFound,

    #[error("Failed to send message: {0}")]
    SendFailed(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Network error: {0}")]
    Network(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            AppError::Authentication(_) => StatusCode::UNAUTHORIZED,
            AppError::InvalidMessage(_) => StatusCode::BAD_REQUEST,
            AppError::ConnectionNotFound | AppError::RoomNotFound => StatusCode::NOT_FOUND,
            AppError::Configuration(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, self.to_string()).into_response()
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Internal(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::InvalidMessage(err.to_string())
    }
}

pub type WsError = AppError;
