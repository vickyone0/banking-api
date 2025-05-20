use actix_web::web;
use crate::auth::middleware::JwtMiddleware;
use crate::auth::jwt::JwtConfig;
use std::env;

pub mod user;
pub mod transactions;
pub mod balance;

pub fn configure(cfg: &mut web::ServiceConfig) {
    let jwt_config = JwtConfig::new(
        env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
    );

    cfg.app_data(web::Data::new(jwt_config))
    .service(
        web::scope("/api")
            // Public routes
            .service(
                web::resource("/register")
                    .route(web::post().to(user::register)),
            )
            .service(
                web::resource("/login")
                    .route(web::post().to(user::login)),
            )
            
            // Protected routes
            .service(
                web::scope("")
                    .wrap(JwtMiddleware)
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
                    .service(
                        web::resource("/balance")
                            .route(web::get().to(balance::get_balance)),
                    ),
            ),
    );
}