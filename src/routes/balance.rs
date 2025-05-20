use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use crate::models::account_balance::AccountBalance;
use crate::error::AppError;

pub async fn get_balance(
    user_id: web::ReqData<Uuid>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<impl Responder, AppError> {
    let balance = AccountBalance::get_balance(user_id.into_inner(), &pool).await?;
    Ok(HttpResponse::Ok().json(balance))
}

