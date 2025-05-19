use crate::auth::jwt::JwtConfig;
use crate::auth::middleware::JwtMiddleware;
use actix_web::web;
use std::env;

pub mod user;

pub fn configure(cfg: &mut web::ServiceConfig) {
    let jwt_config = JwtConfig::new(env::var("JWT_SECRET").expect("JWT_SECRET must be set"));
    cfg.app_data(web::Data::new(jwt_config)).service(
        web::scope("/api")
            .service(web::resource("/register").route(web::post().to(user::register)))
            .service(web::resource("/login").route(web::post().to(user::login)))
            .service(
                web::resource("/profile")
                    .wrap(JwtMiddleware)
                    .route(web::get().to(user::get_profile))
                    .route(web::put().to(user::update_profile)),
            ),
    );
}
