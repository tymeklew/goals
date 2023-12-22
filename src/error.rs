use axum::{http::StatusCode, response::IntoResponse};

pub enum AppError {
    // Error for getting connection to database
    Deadpool,
    // Diesel error
    Diesel(diesel::result::Error),
    // Bcrypt
    Bcrypt(bcrypt::BcryptError),
    // Status
    Status(StatusCode),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Status(status) => status.into_response(),
            _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
