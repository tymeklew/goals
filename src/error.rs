use axum::{http::StatusCode, response::IntoResponse};

pub enum AppError {
    // Error for getting connection to database
    Deadpool,
    // Diesel error
    Diesel,
    // Status
    Status(StatusCode),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Deadpool => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            Self::Diesel => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            Self::Status(status) => status.into_response(),
        }
    }
}
