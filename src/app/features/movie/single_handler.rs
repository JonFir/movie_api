use std::sync::Arc;

use actix_web::{get, web, Responder};
use serde::Serialize;

use crate::app::{
    error_response::{ErrorMeta, ErrorResponse},
    errors::Error,
    response_payload::ResponsePayload,
    state::AppState,
};

use super::entity::Movie;

#[get("/{id}")]
pub async fn handler(
    path: web::Path<i64>,
    data: web::Data<Arc<AppState>>,
) -> Result<impl Responder, ErrorResponse> {
    let id = path.into_inner();
    let result = make_response(id, data).await;
    match result {
        Ok(r) => {
            let respose = ResponsePayload::succes("Movie was loaded", r);
            Ok(respose)
        }
        Err(e @ Error::NotFound) => {
            let r = ErrorResponse {
                meta: ErrorMeta::NOT_FOUND,
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

async fn make_response(id: i64, data: web::Data<Arc<AppState>>) -> Result<Response, Error> {
    let movie = data
        .database
        .find_movie_by_id(id)
        .await?
        .ok_or(Error::NotFound)?;
    let response = Response {
        movie: movie.into(),
    };

    Ok(response)
}

#[derive(Serialize)]
struct Response {
    movie: Movie,
}
