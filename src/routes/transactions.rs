use crate::auth::AuthenticatedUser;
use crate::error::AppError;
use crate::models::transaction::{Transaction, TransactionType};
use actix_web::{HttpResponse, Responder, web};
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct CreateTransactionRequest {
    amount: i64, // String to avoid floating point precision issues
    transaction_type: TransactionType,
    description: Option<String>,
}

pub async fn create_transaction(
    user: AuthenticatedUser,
    payload: web::Json<CreateTransactionRequest>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<impl Responder, AppError> {
    //let amount = payload.amount.parse().map_err(|_| AppError::ValidationError("Invalid amount".into()))?;
    let amount = payload.amount;
    let transaction = Transaction::create(
        user.user_id,
        amount,
        payload.transaction_type,
        payload.description.clone(),
        &pool,
    )
    .await?;

    Ok(HttpResponse::Ok().json(transaction))
}

pub async fn get_user_transactions(
    user: AuthenticatedUser,
    pool: web::Data<sqlx::PgPool>,
) -> Result<impl Responder, AppError> {
    let user_id = user.user_id;
    let transactions = Transaction::get_by_user(user_id, &pool).await?;
    Ok(HttpResponse::Ok().json(transactions))
}
