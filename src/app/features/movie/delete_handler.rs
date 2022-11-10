use std::sync::Arc;

use actix_web::{delete, web, Responder};
use serde::Serialize;

use crate::app::{
    error_response::{ErrorMeta, ErrorResponse},
    errors::Error,
    response_payload::ResponsePayload,
    state::AppState,
};

#[delete("/{id}")]
pub async fn handler(
    path: web::Path<i64>,
    data: web::Data<Arc<AppState>>,
) -> Result<impl Responder, ErrorResponse> {
    let id = path.into_inner();
    let result = make_response(id, data).await;
    match result {
        Ok(r) => {
            let respose = ResponsePayload::succes("", r);
            Ok(respose)
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

async fn make_response(id: i64, data: web::Data<Arc<AppState>>) -> Result<Response, Error> {
    let deleted = data.database.delete_movie(id).await?;
    let response = Response { deleted };

    Ok(response)
}

#[derive(Serialize)]
struct Response {
    deleted: u64,
}
