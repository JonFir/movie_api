use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header::HeaderMap,
    Error, HttpMessage, ResponseError,
};
use futures_util::future::LocalBoxFuture;
use std::sync::Arc;
use std::{
    future::{ready, Ready},
    rc::Rc,
};

use crate::app::{
    self,
    database::entity::User,
    error_response::{ErrorMeta, ErrorResponse},
    state::AppState,
};

pub struct AuthService<S> {
    service: Rc<S>,
    app_state: Arc<AppState>,
}

impl<S> AuthService<S> {
    fn extract_token_from_headers(headers: &HeaderMap) -> Result<String, app::errors::Error> {
        let token_parts = headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .split(" ")
            .collect::<Vec<&str>>();

        if token_parts.len() != 2 || !token_parts.first().unwrap_or(&"").eq(&"Bearer") {
            return Err(app::errors::Error::MissingAuthToken);
        }
        let token = token_parts[1].to_owned();
        Ok(token)
    }

    async fn user_from_token(
        token: Result<String, app::errors::Error>,
        app_state: Arc<AppState>,
    ) -> Result<User, app::errors::Error> {
        let token = token?;
        app_state
            .database
            .find_by_token(&token)
            .await
            .map_err(|e| e.into())
            .and_then(|user| user.ok_or(app::errors::Error::UserNotFound))
    }
}

impl<S, B> Service<ServiceRequest> for AuthService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let app_state = self.app_state.clone();
        let service = self.service.clone();
        Box::pin(async move {
            let token = Self::extract_token_from_headers(request.headers());
            let user = Self::user_from_token(token, app_state).await;

            match user {
                Ok(value) => {
                    request.extensions_mut().insert(value);
                    let result = service.call(request).await?.map_into_left_body();
                    return Ok(result);
                }
                Err(e) => {
                    let meta = match e {
                        app::errors::Error::MissingAuthToken => ErrorMeta::ACCESS_TOKEN_MISSING,
                        app::errors::Error::UserNotFound => ErrorMeta::USER_NOT_FOUND,
                        _ => ErrorMeta::INTERNAL,
                    };
                    let (request, _) = request.into_parts();
                    let response = ErrorResponse {
                        meta: meta,
                        parent: e.into(),
                    }
                    .error_response()
                    .map_into_right_body();
                    return Ok(ServiceResponse::new(request, response));
                }
            }
        })
    }
}

pub struct Middleware {
    pub app_state: Arc<AppState>,
}

impl<S: 'static, B> Transform<S, ServiceRequest> for Middleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = AuthService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        let service = AuthService {
            service: Rc::new(service),
            app_state: self.app_state.clone(),
        };
        ready(Ok(service))
    }
}
