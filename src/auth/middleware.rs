use actix_web::{
    Error as ActixError, FromRequest, HttpMessage, HttpRequest,
    dev::{Payload, ServiceRequest},
    web,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use futures::future::{Ready, ready};
use uuid::Uuid;

use crate::auth::jwt::{Claims, JwtService};

pub async fn jwt_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (ActixError, ServiceRequest)> {
    //let jwt_service = req.app_data::<JwtService>().expect("JwtService not found in app data");
    let jwt_service = req
        .app_data::<web::Data<JwtService>>()
        .expect("JwtService not found in app data");

    match jwt_service.validate_token(credentials.token()) {
        Ok(claims) => {
            req.extensions_mut().insert(claims);
            Ok(req)
        }
        Err(e) => Err((e.into(), req)),
    }
}

#[derive(Debug)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
    pub email: String,
}

impl FromRequest for AuthenticatedUser {
    type Error = ActixError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let extensions = req.extensions();
        let claims = extensions
            .get::<Claims>()
            .expect("Claims not found in request extensions");

        ready(Ok(AuthenticatedUser {
            user_id: claims.sub,
            email: claims.email.clone(),
        }))
    }
}
