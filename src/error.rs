use axum::{http::StatusCode, response::IntoResponse};
use bcrypt::BcryptError;
use diesel::result::DatabaseErrorKind as DbErrorKind;
use diesel::result::Error as DieselError;
use diesel_async::pooled_connection::PoolError;
use lettre::transport::smtp::Error as LettreError;
use thiserror::Error;
use tracing::error;
use validator::ValidationErrors;
use validator::ValidationErrorsKind;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database connection error : {0}")]
    Bb8(#[from] bb8::RunError<PoolError>),
    // Diesel error
    #[error("Diesel database error : {0}")]
    Diesel(#[from] DieselError),
    // Lettre
    #[error("Lettre Error : {0}")]
    Lettre(#[from] LettreError),
    // Bcrypt
    #[error("Bcrypt error : {0}")]
    Bcrypt(#[from] BcryptError),
    // Status
    #[error("Status Code : {0}")]
    Status(StatusCode),
    // Validation
    #[error("Failed to validate input")]
    Validate(#[from] ValidationErrors),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        error!("{self}");

        match self {
            Self::Status(status) => status.into_response(),
            Self::Validate(err) => {
                let errors = err.into_errors().iter().fold(String::new(), |acc, new| {
                    acc + &format!("{} : {:#?}", new.0, display_validation_error(new.1.clone()))
                });

                (StatusCode::BAD_REQUEST, errors).into_response()
            }
            // Voileted unique key so sending back conflict instead
            Self::Diesel(DieselError::DatabaseError(DbErrorKind::UniqueViolation, _)) => {
                StatusCode::CONFLICT.into_response()
            }
            _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

fn display_validation_error(error: ValidationErrorsKind) -> String {
    match error {
        ValidationErrorsKind::Field(errors) => {
            if errors.is_empty() {
                return String::new();
            }

            return errors.first().unwrap().code.to_string() + "\n";
        }
        _ => String::new(),
    }
}

impl From<StatusCode> for AppError {
    fn from(value: StatusCode) -> Self {
        Self::Status(value)
    }
}
