use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)] // Never expose password hash in responses
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    // User registration
    pub async fn register(
        username: String,
        email: String,
        password: String,
        pool: &sqlx::PgPool,
    ) -> Result<Self, anyhow::Error> {
        let hashed_password = hash(password, DEFAULT_COST)?;
        let user = sqlx::query_as!(
            Self,
            r#"
            INSERT INTO users (username, email, password_hash)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            username,
            email,
            hashed_password
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    // User authentication
    pub async fn authenticate(
        email: String,
        password: String,
        pool: &sqlx::PgPool,
    ) -> Result<Self, anyhow::Error> {
        let user = sqlx::query_as!(
            Self,
            "SELECT * FROM users WHERE email = $1",
            email
        )
        .fetch_optional(pool)
        .await?
        .ok_or(anyhow::anyhow!("User not found"))?;

        if verify(password, &user.password_hash)? {
            Ok(user)
        } else {
            Err(anyhow::anyhow!("Invalid credentials"))
        }
    }

    pub async fn get_by_id(
        user_id: &Uuid,
        pool: &sqlx::PgPool,
    ) -> Result<Self, anyhow::Error> {
        let user = sqlx::query_as!(
            Self,
            "SELECT * FROM users WHERE id = $1",
            user_id
        )
        .fetch_one(pool)
        .await?;
        Ok(user)
    }

    // Update profile
    pub async fn update_profile(
        &self,
        new_username: Option<String>,
        new_email: Option<String>,
        pool: &sqlx::PgPool,
    ) -> Result<Self, anyhow::Error> {
        let username = new_username.unwrap_or(self.username.clone());
        let email = new_email.unwrap_or(self.email.clone());

        let user = sqlx::query_as!(
            Self,
            r#"
            UPDATE users
            SET username = $1, email = $2, updated_at = NOW()
            WHERE id = $3
            RETURNING *
            "#,
            username,
            email,
            self.id
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }
}