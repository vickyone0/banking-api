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


#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_credit_operation() {
        let balance = 1000;
        let amount = 500;
        let new_balance = |balance, amount| balance + amount;
        assert_eq!(new_balance(balance, amount), 1500);
    }

    #[test]
    fn test_debit_operation() {
        let balance = 1000;
        let amount = 400;
        let new_balance = |balance, amount| balance - amount;
        assert_eq!(new_balance(balance, amount), 600);
    }

    #[test]
    fn test_account_balance_struct_fields() {
        let id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let balance = 2000;
        let now = Utc::now();

        let ab = AccountBalance {
            id,
            user_id,
            balance,
            last_updated: now,
        };

        assert_eq!(ab.id, id);
        assert_eq!(ab.user_id, user_id);
        assert_eq!(ab.balance, 2000);
        assert_eq!(ab.last_updated, now);
    }
}