use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Insufficient funds")]
    InsufficientFunds,

    #[error("Validation error: {0}")]
    ValidationError(String),
    // Add other error variants as needed
}

impl actix_web::error::ResponseError for AppError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            AppError::DatabaseError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InsufficientFunds => actix_web::http::StatusCode::BAD_REQUEST,
            AppError::ValidationError(_) => actix_web::http::StatusCode::BAD_REQUEST,
        }
    }
}
