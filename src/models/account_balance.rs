use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Postgres, Transaction};
use uuid::Uuid;
use crate::error::AppError;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct AccountBalance {
    pub id: Uuid,
    pub user_id: Uuid,
    pub balance: i64,
    pub last_updated: DateTime<Utc>,
}

impl AccountBalance {
    pub async fn get_balance(user_id: Uuid, pool: &sqlx::PgPool) -> Result<Self, AppError> {
        let balance = sqlx::query_as!(
            Self,
            r#"
            SELECT * FROM account_balances
            WHERE user_id = $1
            "#,
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
        // Use the transaction directly (no &mut needed)
        let current = sqlx::query_as!(
            Self,
            r#"
            SELECT * FROM account_balances
            WHERE user_id = $1
            FOR UPDATE
            "#,
            user_id
        )
        .fetch_one(&mut *tx)  // Note: single dereference
        .await?;

        let new_balance = operation(current.balance, amount);

        if new_balance < 0 {
            return Err(AppError::InsufficientFunds);
        }

        let updated = sqlx::query_as!(
            Self,
            r#"
            UPDATE account_balances
            SET balance = $1, last_updated = NOW()
            WHERE user_id = $2
            RETURNING *
            "#,
            new_balance,
            user_id
        )
        .fetch_one(&mut *tx)
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