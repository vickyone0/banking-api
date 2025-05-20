use actix_web::{HttpResponse, ResponseError};
use std::fmt;

#[derive(Debug)]
pub enum AuthError {
    InvalidToken,
    TokenExpired,
    TokenCreation,
    MissingToken,
    Unauthorized,
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AuthError::InvalidToken => write!(f, "Invalid token"),
            AuthError::TokenExpired => write!(f, "Expired token"),
            AuthError::TokenCreation => write!(f, "Token creation error"),
            AuthError::MissingToken => write!(f, "Missing authorization header"),
            AuthError::Unauthorized => write!(f, "Unauthorized"),
        }
    }
}

impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AuthError::InvalidToken => HttpResponse::Unauthorized().json("Invalid token"),
            AuthError::TokenExpired => HttpResponse::Unauthorized().json("Token expired"),
            AuthError::TokenCreation => {
                HttpResponse::InternalServerError().json("Token creation error")
            }
            AuthError::MissingToken => HttpResponse::Unauthorized().json("Missing token"),
            AuthError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
        }
    }
}

// impl From<AuthError> for ActixError {
//     fn from(err: AuthError) -> Self {
//         ActixError::from(err)
//     }
// }