use std::sync::Arc;

use actix_web::{
    post,
    web::{Data, Json},
    Responder, Result,
};
use serde::Deserialize;

use crate::app::{
    error_response::{ErrorMeta, ErrorResponse},
    errors::Error,
    response_payload::ResponsePayload,
    state::AppState,
};

use super::password_hash;

#[post("/register")]
pub async fn register_handler(
    payload: Json<Payload>,
    data: Data<Arc<AppState>>,
) -> Result<impl Responder, ErrorResponse> {
    let result = make_response(payload, data).await;
    match result {
        Ok(_) => {
            let respose = ResponsePayload::succes_and_empty("User did registered");
            Ok(respose)
        }
        Err(e @ Error::DBKeyDublicate(_)) => {
            let r = ErrorResponse {
                meta: ErrorMeta::USER_EXIST,
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

async fn make_response(payload: Json<Payload>, data: Data<Arc<AppState>>) -> Result<(), Error> {
    let hash = password_hash::new(&payload.password)?;
    data.database
        .create_user(&payload.login, &hash, &payload.email)
        .await?;
    Ok(())
}

#[derive(Deserialize, Debug)]
pub struct Payload {
    login: String,
    password: String,
    email: String,
}
