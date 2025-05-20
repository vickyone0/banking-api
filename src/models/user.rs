use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::AccountBalance;

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
    ) -> Result<(Self,AccountBalance), anyhow::Error> {

        // Start a transaction
        let mut tx = pool.begin().await?;

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
        .fetch_one(&mut *tx)
        .await?;  

         // Create the account balance with initial 0 balance
        let account_balance = sqlx::query_as!(
            AccountBalance,
            r#"
            INSERT INTO account_balances (user_id, balance, last_updated)
            VALUES ($1, 0, NOW())
            RETURNING *
            "#,
            user.id
        )
        .fetch_one(&mut *tx)
        .await?;

        // Commit the transaction
        tx.commit().await?;
    
        Ok((user,account_balance))
    }

    // User authentication
    pub async fn authenticate(
        email: String,
        password: String,
        pool: &sqlx::PgPool,
    ) -> Result<Self, anyhow::Error> {
        let user = sqlx::query_as!(Self, "SELECT * FROM users WHERE email = $1", email)
            .fetch_optional(pool)
            .await?
            .ok_or(anyhow::anyhow!("User not found"))?;

        if verify(password, &user.password_hash)? {
            Ok(user)
        } else {
            Err(anyhow::anyhow!("Invalid credentials"))
        }
    }

    pub async fn get_by_id(user_id: &Uuid, pool: &sqlx::PgPool) -> Result<Self, anyhow::Error> {
        let user = sqlx::query_as!(Self, "SELECT * FROM users WHERE id = $1", user_id)
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


#[cfg(test)]
mod tests {
    use super::*;
    use bcrypt::{hash, verify, DEFAULT_COST};

    #[test]
    fn test_password_hash_and_verify() {
        let password = "mysecretpassword";
        let hashed = hash(password, DEFAULT_COST).unwrap();
        assert!(verify(password, &hashed).unwrap());
        assert!(!verify("wrongpassword", &hashed).unwrap());
    }

    #[test]
    fn test_user_struct_fields() {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let user = User {
            id,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "hashed".to_string(),
            created_at: now,
            updated_at: now,
        };

        assert_eq!(user.id, id);
        assert_eq!(user.username, "testuser");
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.password_hash, "hashed");
        assert_eq!(user.created_at, now);
        assert_eq!(user.updated_at, now);
    }
}