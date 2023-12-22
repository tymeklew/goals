use axum::{http::StatusCode, response::IntoResponse};
use bcrypt::BcryptError;
use diesel::result::DatabaseErrorKind as DbErrorKind;
use diesel::result::Error as DieselError;
use lettre::transport::smtp::Error as LettreError;
use log2::error;
use validator::ValidationErrors;
use validator::ValidationErrorsKind;

pub enum AppError {
    // Error for getting connection to database
    Deadpool,
    // Diesel error
    Diesel(DieselError),
    // Lettre
    Lettre(LettreError),
    // Bcrypt
    Bcrypt(BcryptError),
    // Status
    Status(StatusCode),
    // Validation
    Validate(ValidationErrors),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        error!("Error : {self}");
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

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Deadpool => write!(f, "Diesel error idk"),
            Self::Diesel(err) => err.fmt(f),
            Self::Bcrypt(err) => err.fmt(f),
            _ => write!(f, ""),
        }
    }
}
