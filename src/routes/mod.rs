use crate::auth::jwt::JwtService;
use crate::auth::middleware::jwt_validator;
use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;
use std::env;

pub mod balance;
pub mod transactions;
pub mod user;

pub fn configure(cfg: &mut web::ServiceConfig) {
    let jwt_config = JwtService::from_env();
    let auth = HttpAuthentication::bearer(jwt_validator);

    cfg.app_data(web::Data::new(jwt_config)).service(
        web::scope("/api")
            // Public routes
            .service(web::resource("/register").route(web::post().to(user::register)))
            .service(web::resource("/login").route(web::post().to(user::login)))
            // Protected routes
            .service(
                web::scope("")
                    .wrap(auth)
                    .service(
                        web::resource("/profile")
                            .route(web::get().to(user::get_profile))
                            .route(web::put().to(user::update_profile)),
                    )
                    .service(
                        web::resource("/transactions")
                            .route(web::post().to(transactions::create_transaction))
                            .route(web::get().to(transactions::get_user_transactions)),
                    )
                    // .service(
                    //     web::resource("/transactions/{id}")
                    //         .route(web::get().to(transactions::get_transaction)),
                    // )
                    .service(web::resource("/balance").route(web::get().to(balance::get_balance))),
            ),
    );
}
