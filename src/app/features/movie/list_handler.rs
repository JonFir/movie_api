use std::sync::Arc;

use actix_web::{get, web, Responder};
use serde::{Deserialize, Serialize};

use crate::app::{
    error_response::{ErrorMeta, ErrorResponse},
    errors::Error,
    response_payload::ResponsePayload,
    state::AppState,
};

use super::entity::Movie;
#[get("/")]
pub async fn handler(
    payload: web::Query<Payload>,
    data: web::Data<Arc<AppState>>,
) -> Result<impl Responder, ErrorResponse> {
    let result = make_response(payload, data).await;
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

async fn make_response(
    payload: web::Query<Payload>,
    data: web::Data<Arc<AppState>>,
) -> Result<Response, Error> {
    let movies = data
        .database
        .all_movies(payload.cursor, payload.count)
        .await?
        .into_iter()
        .map(|m| Movie::from(m))
        .collect::<Vec<Movie>>();
    let cursor = movies.last().and_then(|m| m.id);
    let response = Response { cursor, movies };

    Ok(response)
}

#[derive(Deserialize)]
pub struct Payload {
    cursor: Option<i64>,
    count: i64,
}

#[derive(Serialize)]
struct Response {
    cursor: Option<i64>,
    movies: Vec<Movie>,
}
