use crate::error::AppError;
use crate::models::account_balance::AccountBalance;
use actix_web::{HttpResponse, Responder, web};
use crate::auth::AuthenticatedUser;

pub async fn get_balance(
    user: AuthenticatedUser,
    pool: web::Data<sqlx::PgPool>,
) -> Result<impl Responder, AppError> {
    let user_id = user.user_id;
    let balance = AccountBalance::get_balance(user_id, &pool).await?;
    Ok(HttpResponse::Ok().json(balance))
}
