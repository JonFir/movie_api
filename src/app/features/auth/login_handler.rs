use std::sync::Arc;

use actix_web::{post, web, Responder};
use serde::{Deserialize, Serialize};

use crate::app::{
    error_response::{ErrorMeta, ErrorResponse},
    errors::Error,
    response_payload::ResponsePayload,
    state::AppState,
};

use super::{enity::User, password_hash, random_string};

#[post("/login")]
pub async fn login_handler(
    payload: web::Json<Payload>,
    data: web::Data<Arc<AppState>>,
) -> Result<impl Responder, ErrorResponse> {
    let result = make_response(payload, data).await;
    match result {
        Ok(response) => {
            let response = ResponsePayload::succes("Login was success", response);
            Ok(response)
        }
        Err(e @ Error::IncorectLogin) | Err(e @ Error::IncorectPassword) => {
            let r = ErrorResponse {
                meta: ErrorMeta::USER_NOT_FOUND,
                parent: e.into(),
            };
            Err(r)
        }
        Err(e) => {
            let r = ErrorResponse {
                meta: ErrorMeta::INTERNAL,
                parent: e.into(),
            };
            Err(r)
        }
    }
}

async fn make_response(
    payload: web::Json<Payload>,
    data: web::Data<Arc<AppState>>,
) -> Result<Response, Error> {
    let user = data
        .database
        .find_by_username(&payload.login)
        .await?
        .ok_or(Error::IncorectLogin)?;

    if !password_hash::verify(&user.hash, &payload.password)? {
        return Err(Error::IncorectPassword);
    }

    let token = random_string::new(255);
    data.database.set_remember_tokens(&user.id, &token).await?;
    let user = User::from(user);
    let response = Response { token, user };

    Ok(response)
}

#[derive(Deserialize)]
pub struct Payload {
    login: String,
    password: String,
}

#[derive(Serialize)]
struct Response {
    token: String,
    user: User,
}
