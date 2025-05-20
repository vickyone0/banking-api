use chrono::{DateTime, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

use crate::auth::errors::AuthError;


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims{
    pub sub: Uuid,
    pub exp: usize,
    pub iat: usize,
    pub email: String,
}

#[derive(Debug, Clone)]
pub struct JwtService {
    pub secret: String,
    pub expiration_hours: i64,
}

impl JwtService {
    pub fn from_env() -> Self {
        JwtService { secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"), expiration_hours: 24 }
    }

    pub fn generate_token(&self, user_id: Uuid, email: &str) -> Result<String, AuthError> {

        let now = Utc::now().timestamp() as usize;
        let expiration = (Utc::now() + chrono::Duration::hours(self.expiration_hours)).timestamp() as usize;

        let claims = Claims {
            sub: user_id,
            exp: expiration,
            iat: now,
            email: email.to_owned(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
        .map_err(|_| AuthError::TokenCreation)
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims, AuthError> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|_| AuthError::InvalidToken)
        .map(|data| data.claims)
    }
        
    }

