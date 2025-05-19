use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use crate::models::user::User;
use sqlx::PgPool;

// Request payloads
#[derive(serde::Deserialize)]
pub struct RegisterRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(serde::Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(serde::Deserialize)]
pub struct UpdateProfileRequest {
    username: Option<String>,
    email: Option<String>,
}

// Route handlers
pub async fn register(
    payload: web::Json<RegisterRequest>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    match User::register(
        payload.username.clone(),
        payload.email.clone(),
        payload.password.clone(),
        &pool,
    )
    .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

pub async fn login(
    payload: web::Json<LoginRequest>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    match User::authenticate(
        payload.email.clone(),
        payload.password.clone(),
        &pool,
    )
    .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::Unauthorized().body(e.to_string()),
    }
}

pub async fn get_profile(
    user_id: web::ReqData<Uuid>,  // Assuming you have auth middleware
    pool: web::Data<PgPool>,
) -> impl Responder {
    match User::get_by_id(&user_id, &pool).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::NotFound().body(e.to_string()),
    }
}

pub async fn update_profile(
    user_id: web::ReqData<Uuid>,
    payload: web::Json<UpdateProfileRequest>,
    pool: web::Data<PgPool>,
) -> impl Responder {
     match User::get_by_id(&user_id, &pool).await {
        Ok(user) => {
            match user.update_profile(
                payload.username.clone(),
                payload.email.clone(),
                &pool,
            ).await {
                Ok(updated_user) => HttpResponse::Ok().json(updated_user),
                Err(e) => HttpResponse::BadRequest().body(e.to_string()),
            }
        }
        Err(e) => HttpResponse::NotFound().body(e.to_string()),
    }
}