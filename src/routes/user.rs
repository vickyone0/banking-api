use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use sqlx::PgPool;
use serde_json::json;


use crate::models::user::User;
use crate::auth::jwt::JwtConfig;


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
    jwt_config: web::Data<JwtConfig>,
) -> impl Responder {
    match User::authenticate(
        payload.email.clone(),
        payload.password.clone(),
        &pool,
    )
    .await
    {
        Ok(user) => {
             match jwt_config.generate_token(user.id) {
                Ok(token) => HttpResponse::Ok().json(json!({
                    "user": user,
                    "token": token,
                })),
                Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
            }
        },
        Err(e) => HttpResponse::Unauthorized().body(e.to_string()),
    }
}

pub async fn get_profile(
    user_id: Option<web::ReqData<Uuid>>,  // Assuming you have auth middleware
    pool: web::Data<PgPool>,
) -> impl Responder {
    let user_id = user_id.unwrap().into_inner();
    match User::get_by_id(&user_id, &pool).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::NotFound().body(e.to_string()),
    }
}

pub async fn update_profile(
    user_id: Option<web::ReqData<Uuid>>,
    payload: web::Json<UpdateProfileRequest>,
    pool: web::Data<PgPool>,
) -> impl Responder {
        let user_id = user_id.unwrap().into_inner();
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