use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // User ID
    pub exp: usize,   // Expiration time
    pub iat: usize,   // Issued at
}

pub struct JwtConfig {
    pub secret: String,
    pub expiration_hours: i64,
}

impl JwtConfig {
    pub fn new(secret: String) -> Self {
        Self {
            secret,
            expiration_hours: 24,  // 24 hours expiration
        }
    }

    pub fn generate_token(&self, user_id: Uuid) -> Result<String, String> {
        let now = Utc::now();
        let exp = (now + chrono::Duration::hours(self.expiration_hours)).timestamp() as usize;

        let claims = Claims {
            sub: user_id.to_string(),
            exp,
            iat: now.timestamp() as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
        .map_err(|e| format!("Failed to create token: {}", e))
    }

    pub fn validate_token(&self, token: &str) -> Result<Uuid, String> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|e| format!("Invalid token: {}", e))?;

        Uuid::parse_str(&token_data.claims.sub)
            .map_err(|_| "Invalid user ID in token".to_string())
    }
}