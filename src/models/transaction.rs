use crate::error::AppError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::AccountBalance;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub user_id: Uuid,
    pub amount: i64,
    pub transaction_type: TransactionType,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "transaction_type", rename_all = "lowercase")]

#[derive(PartialEq)]
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
        //validate amount is positive
        if amount <= 0 {
            return Err(AppError::ValidationError("Amount must be positive".into()));
        }

        let mut tx = pool.begin().await?;

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
        .fetch_one(&mut *tx)
        .await?;

        match transaction_type {
            TransactionType::Credit => {
                AccountBalance::credit(user_id, amount, &mut tx).await?;
            }
            TransactionType::Debit => {
                AccountBalance::debit(user_id, amount, &mut tx).await?;
            }
        }
        tx.commit().await?;

        Ok(transaction)
    }

    pub async fn get_by_user(user_id: Uuid, pool: &sqlx::PgPool) -> Result<Vec<Self>, AppError> {
        let transactions = sqlx::query_as::<_, Self>(
            "
            SELECT * FROM transactions
            WHERE user_id = $1 ORDER BY created_at DESC
            ",
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

        Ok(transactions)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_type_serialization() {
        let credit = TransactionType::Credit;
        let debit = TransactionType::Debit;

        // Serialize to string (serde_json)
        let credit_str = serde_json::to_string(&credit).unwrap();
        let debit_str = serde_json::to_string(&debit).unwrap();

        assert_eq!(credit_str, "\"credit\"");
        assert_eq!(debit_str, "\"debit\"");

        // Deserialize from string
        let credit2: TransactionType = serde_json::from_str("\"credit\"").unwrap();
        let debit2: TransactionType = serde_json::from_str("\"debit\"").unwrap();

        assert_eq!(credit2, TransactionType::Credit);
        assert_eq!(debit2, TransactionType::Debit);
    }

    #[test]
    fn test_transaction_struct_fields() {
        let id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let amount = 1000;
        let transaction_type = TransactionType::Credit;
        let description = Some("Test".to_string());
        let now = chrono::Utc::now();

        let tx = Transaction {
            id,
            user_id,
            amount,
            transaction_type,
            description: description.clone(),
            created_at: now,
        };

        assert_eq!(tx.id, id);
        assert_eq!(tx.user_id, user_id);
        assert_eq!(tx.amount, 1000);
        assert_eq!(tx.transaction_type, TransactionType::Credit);
        assert_eq!(tx.description, description);
        assert_eq!(tx.created_at, now);
    }
}