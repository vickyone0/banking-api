use actix_web::web;

pub mod user;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::resource("/register")
                    .route(web::post().to(user::register)),
            )
            .service(
                web::resource("/login")
                    .route(web::post().to(user::login)),
            )
            .service(
                web::resource("/profile")
                    //.route(web::get().to(user::get_profile))
                    .route(web::put().to(user::update_profile)),
            ),
    );
}