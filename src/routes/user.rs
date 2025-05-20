use actix_web::{HttpResponse, Responder, web};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::AuthenticatedUser;
use crate::auth::JwtService;
use crate::models::User;

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
    jwt_config: web::Data<JwtService>,
) -> impl Responder {
    match User::authenticate(payload.email.clone(), payload.password.clone(), &pool).await {
        Ok(user) => match jwt_config.generate_token(user.id, &user.email) {
            Ok(token) => HttpResponse::Ok().json(json!({
                "user": user,
                "user_id": user.id,
                "token": token,
            })),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        },
        Err(e) => HttpResponse::Unauthorized().body(e.to_string()),
    }
}

pub async fn get_profile(
    //user_id: Option<web::ReqData<Uuid>>,
    user: AuthenticatedUser,
    pool: web::Data<PgPool>,
) -> impl Responder {
    // let user_id = match user_id {
    //     Some(id) => id.into_inner(),
    //     None => return HttpResponse::Unauthorized().body("Unauthorized"),
    // };
    let user_id = user.user_id;
    match User::get_by_id(&user_id, &pool).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::NotFound().body(e.to_string()),
    }
}
//eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI4ZmM0NGM1Ni05YzliLTQ0MjAtODM3OS0yMzNiZGNhNTk5MzYiLCJleHAiOjE3NDc4MTI2NDYsImlhdCI6MTc0NzcyNjI0NiwiZW1haWwiOiIxMjM0LmNvbSJ9.MAXcECF89sW6YGgvPrxRSDZ9eKD_RgnYi2NBcXX0HvU
pub async fn update_profile(
    //user_id: Option<web::ReqData<Uuid>>,
    user: AuthenticatedUser,
    payload: web::Json<UpdateProfileRequest>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    //let user_id = user_id.unwrap().into_inner();
    let user_id = user.user_id;
    match User::get_by_id(&user_id, &pool).await {
        Ok(user) => {
            match user
                .update_profile(payload.username.clone(), payload.email.clone(), &pool)
                .await
            {
                Ok(updated_user) => HttpResponse::Ok().json(updated_user),
                Err(e) => HttpResponse::BadRequest().body(e.to_string()),
            }
        }
        Err(e) => HttpResponse::NotFound().body(e.to_string()),
    }
}
