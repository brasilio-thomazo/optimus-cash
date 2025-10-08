use std::pin::Pin;

use actix_web::{
    HttpMessage,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
};

use crate::{config, security};

pub struct JwtMiddleware;

pub struct AuthService<S> {
    service: S,
}

impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Transform = AuthService<S>;
    type InitError = ();
    type Future = std::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        std::future::ready(Ok(AuthService { service }))
    }
}

impl<S, B> Service<ServiceRequest> for AuthService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let open_routes = config::open_routes();
        if open_routes.contains(&req.path()) {
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            });
        }
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok());
        if token.is_none() {
            return Box::pin(
                async move { Err(actix_web::error::ErrorUnauthorized("Unauthorized")) },
            );
        }
        let token = token.unwrap().replace("Bearer ", "");
        match security::verify_jwt_token(&token) {
            Ok(data) => {
                req.extensions_mut().insert(data.clone());
            }
            Err(error) => {
                return Box::pin(
                    async move { Err(actix_web::error::ErrorUnauthorized(error.message)) },
                );
            }
        }
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
        // std::future::ready(Ok(fut))
    }
}
