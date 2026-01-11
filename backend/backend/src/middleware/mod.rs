use actix_web::{HttpMessage, dev::Extensions, http::header::{HeaderName, HeaderValue}};
use std::{
    future::{Ready, ready},
    pin::Pin, str::FromStr,
};

use actix_web::{
    Error, HttpResponse, dev::{Service, ServiceRequest, ServiceResponse, Transform}, web
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


        let auth_header = req.headers().get("Authorization");

        if let Some(bearer) = auth_header {
            let token: Vec<&str> = bearer.to_str().unwrap().split("Bearer ").collect();
            let data = req.app_data::<web::Data<AppState>>();
            if token.len() == 2 {
                let token = token.get(1).unwrap();
                if let Some(data) = data {
                    match Claims::decode_jwt(token, &data.env.jwt_secret) {
                        Ok(claims) => {
                            req.extensions_mut().insert(claims.claims);
                        },
                        Err(_) => {}
                    }
                }
            }
        } else {

        }
        let future = self.service.call(req);
        Box::pin(async move {
            let res = match future.await {
                Ok(response) => response,
                Err(error) => panic!("Unable to process middleware: {}", error),
            };
            Ok(res)
        })
    }
}
