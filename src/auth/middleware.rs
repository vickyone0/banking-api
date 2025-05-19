use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform}, Error};
use futures_util::future::{ok, Ready, LocalBoxFuture};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use std::task::{Context, Poll};
use std::rc::Rc;
use std::future::Future;
use std::pin::Pin;
use actix_web::FromRequest;
pub struct JwtMiddleware;

impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtMiddlewareService { service: Rc::new(service) })
    }
}

pub struct JwtMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        Box::pin(async move {
            let bearer = BearerAuth::extract(req.request()).await.map_err(actix_web::error::ErrorUnauthorized)?;
            // ...validate token, insert user_id, etc...
            service.call(req).await
        })
    }
}