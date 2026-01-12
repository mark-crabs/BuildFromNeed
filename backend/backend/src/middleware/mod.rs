use actix_web::{
    HttpMessage,
};
use std::{
    future::{Ready, ready},
    pin::Pin,
};

use actix_web::{
    Error, HttpResponse,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    web,
};
use utils::config::AppState;

use utils::dto::Claims;

pub struct AuthorizationBase;

impl<S, B> Transform<S, ServiceRequest> for AuthorizationBase
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthorizationMiddlware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthorizationMiddlware { service }))
    }
}

pub struct AuthorizationMiddlware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthorizationMiddlware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;
    actix_web::dev::forward_ready!(service);
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let claims = extract_claims(&req);

        req.extensions_mut().insert(claims);

        let fut = self.service.call(req);

        Box::pin(async move { fut.await })
    }
}

fn extract_claims(req: &ServiceRequest) -> Option<Claims> {
    let auth_header = req.headers().get(actix_web::http::header::AUTHORIZATION)?;
    let auth_str = auth_header.to_str().ok()?;

    let token = auth_str.strip_prefix("Bearer ")?;

    let data = req.app_data::<web::Data<AppState>>()?;

    let decoded = Claims::decode_jwt(token, &data.env.jwt_secret).ok()?;

    Some(decoded.claims)
}
