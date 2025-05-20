use crate::error::AppError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Postgres, Transaction};
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct AccountBalance {
    pub id: Uuid,
    pub user_id: Uuid,
    pub balance: i64, // Stored in cents/pence
    pub last_updated: DateTime<Utc>,
}

impl AccountBalance {
    pub async fn get_balance(user_id: Uuid, pool: &sqlx::PgPool) -> Result<Self, AppError> {
        let balance = sqlx::query_as!(
            Self,
            "SELECT * FROM account_balances WHERE user_id = $1",
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok(balance)
    }

    async fn update_balance(
        user_id: Uuid,
        amount: i64,
        operation: impl Fn(i64, i64) -> i64,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<Self, AppError> {
        let current = sqlx::query_as::<_, Self>(
            "SELECT * FROM account_balances WHERE user_id = $1 FOR UPDATE",
        )
        .bind(user_id)
        .fetch_one(&mut **tx)
        .await?;

        let new_balance = operation(current.balance, amount);

        if new_balance < 0 {
            return Err(AppError::InsufficientFunds);
        }

        let updated = sqlx::query_as::<_, Self>(
            r#"
            UPDATE account_balances
            SET balance = $1, last_updated = NOW()
            WHERE user_id = $2
            RETURNING *
            "#,
        )
        .bind(new_balance)
        .bind(user_id)
        .fetch_one(&mut **tx)
        .await?;

        Ok(updated)
    }

    pub async fn credit(
        user_id: Uuid,
        amount: i64,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<Self, AppError> {
        Self::update_balance(user_id, amount, |balance, amount| balance + amount, tx).await
    }

    pub async fn debit(
        user_id: Uuid,
        amount: i64,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<Self, AppError> {
        Self::update_balance(user_id, amount, |balance, amount| balance - amount, tx).await
    }
}
