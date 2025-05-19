use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
//use bigdecimal::BigDecimal;
use crate::error::AppError;
use crate::models::AccountBalance;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub user_id: Uuid,
    pub amount: i64,
    pub transaction_type: TransactionType,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "transaction_type", rename_all = "lowercase")]
pub enum TransactionType {
    Debit,
    Credit,
}

impl Transaction {
    pub async fn create(
        user_id: Uuid,
        amount: i64,
        transaction_type: TransactionType,
        description: Option<String>,
        pool: &sqlx::PgPool,
    ) -> Result<Self, AppError> {
        // Start database transaction
        let mut tx = pool.begin().await?;

        // 1. Create the transaction record
        let transaction = sqlx::query_as::<_, Self>(
            r#"
    INSERT INTO transactions (user_id, amount, transaction_type, description)
    VALUES ($1, $2, $3, $4)
    RETURNING *
    "#,
        )
        .bind(user_id)
        .bind(amount)
        .bind(transaction_type)
        .bind(description)
        .fetch_one(&mut tx)
        .await?;
        // let transaction = sqlx::query_as!(
        //     Self,
        //     r#"
        //     INSERT INTO transactions (user_id, amount, transaction_type, description)
        //     VALUES ($1, $2, $3, $4)
        //     RETURNING *
        //     "#,
        //     user_id,
        //     amount,
        //     transaction_type as TransactionType,
        //     description
        // )
        // .fetch_one(&mut tx)
        // .await?;

        // 2. Update account balance
        match transaction_type {
            TransactionType::Credit => {
                AccountBalance::credit(user_id, amount, &mut tx).await?;
            }
            TransactionType::Debit => {
                AccountBalance::debit(user_id, amount, &mut tx).await?;
            }
        }

        // Commit both operations
        tx.commit().await?;
        Ok(transaction)
    }

    pub async fn get_by_user(user_id: Uuid, pool: &sqlx::PgPool) -> Result<Vec<Self>, AppError> {
        let transactions = sqlx::query_as::<_, Self>(
            r#"
    SELECT * FROM transactions 
    WHERE user_id = $1
    ORDER BY created_at DESC
    "#,
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;
        Ok(transactions)
    }
}
