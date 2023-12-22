use axum::{http::StatusCode, response::IntoResponse};
use bcrypt::BcryptError;
use diesel::result::DatabaseErrorKind as DbErrorKind;
use diesel::result::Error as DieselError;

pub enum AppError {
    // Error for getting connection to database
    Deadpool,
    // Diesel error
    Diesel(DieselError),
    // Bcrypt
    Bcrypt(BcryptError),
    // Status
    Status(StatusCode),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Status(status) => status,
            Self::Diesel(err) => match err {
                DieselError::DatabaseError(err, _) => match err {
                    // Check if it is a key issue and then send back conflict
                    DbErrorKind::UniqueViolation => StatusCode::CONFLICT,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                },
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
        .into_response()
    }
}
