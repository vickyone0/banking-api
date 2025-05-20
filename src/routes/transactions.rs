use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use crate::models::transaction::{Transaction, TransactionType};
use crate::error::AppError;
use crate::auth::AuthenticatedUser;

#[derive(serde::Deserialize)]
pub struct CreateTransactionRequest {
    amount: String,  // String to avoid floating point precision issues
    transaction_type: TransactionType,
    description: Option<String>,
}

pub async fn create_transaction(
    user: AuthenticatedUser,
    payload: web::Json<CreateTransactionRequest>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<impl Responder, AppError> {
    let amount = payload.amount.parse().map_err(|_| AppError::ValidationError("Invalid amount".into()))?;
    
    let transaction = Transaction::create(
        user.user_id,
        amount,
        payload.transaction_type,
        payload.description.clone(),
        &pool,
    ).await?;

    Ok(HttpResponse::Ok().json(transaction))
}

pub async fn get_user_transactions(
    user_id: web::ReqData<Uuid>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<impl Responder, AppError> {
    let transactions = Transaction::get_by_user(user_id.into_inner(), &pool).await?;
    Ok(HttpResponse::Ok().json(transactions))
}