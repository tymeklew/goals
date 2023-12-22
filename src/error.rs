use axum::{http::StatusCode, response::IntoResponse};
use bcrypt::BcryptError;
use diesel::result::DatabaseErrorKind as DbErrorKind;
use diesel::result::Error as DieselError;
use validator::ValidationErrors;
use validator::ValidationErrorsKind;

pub enum AppError {
    // Error for getting connection to database
    Deadpool,
    // Diesel error
    Diesel(DieselError),
    // Bcrypt
    Bcrypt(BcryptError),
    // Status
    Status(StatusCode),
    // Validation
    Validate(ValidationErrors),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Status(status) => status.into_response(),
            Self::Validate(err) => {
                let errors = err
                    .into_errors()
                    .iter()
                    .map(|val| {
                        format!("{} : {:#?}", val.0, display_validation_error(val.1.clone()))
                    })
                    .collect::<String>();

                (StatusCode::BAD_REQUEST, errors).into_response()
            }
            Self::Diesel(err) => match err {
                DieselError::DatabaseError(err, _) => match err {
                    // Check if it is a key issue and then send back conflict
                    DbErrorKind::UniqueViolation => StatusCode::CONFLICT.into_response(),
                    _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
                },
                _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            },
            _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

fn display_validation_error(error: ValidationErrorsKind) -> String {
    match error {
        ValidationErrorsKind::Field(errors) => {
            if errors.len() <= 0 {
                return String::new();
            }

            return errors.first().unwrap().code.to_string() + "\n";
        }
        _ => String::new(),
    }
}
