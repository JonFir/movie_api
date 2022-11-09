use std::sync::Arc;

use actix_web::{
    post,
    web::{self, Data, Json},
    Responder,
};
use serde::{Deserialize, Serialize};

use crate::app::{
    error_response::{ErrorMeta, ErrorResponse},
    posters,
    response_payload::ResponsePayload,
    state::AppState,
};

use super::{entity::Movie, errors::Error};

#[post("/")]
pub async fn handler(
    payload: Json<Payload>,
    data: Data<Arc<AppState>>,
) -> Result<impl Responder, ErrorResponse> {
    let result = make_response(payload, data).await;
    match result {
        Ok(r) => {
            let respose = ResponsePayload::succes("Movie was created", r);
            Ok(respose)
        }
        Err(e @ Error::PosterNotFound) => {
            let r = ErrorResponse {
                meta: ErrorMeta::make_bad_request("Poster with this id not found".into()),
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
    payload: Json<Payload>,
    data: Data<Arc<AppState>>,
) -> Result<Response, Error> {
    let payload = payload.into_inner();

    let path = posters::posters::make_poster_path(
        &payload.movie.poster_id,
        &data.environment.posters_path,
    )
    .ok_or(Error::CorruptedPosterPath)?;

    let poster_exist = web::block(move || posters::posters::is_exist(&path))
        .await
        .map_err(|e| Error::BlockError(e))?;
    if !poster_exist {
        return Err(Error::PosterNotFound);
    }

    let movie = data.database.create_movie(payload.movie.into()).await?;
    let response = Response {
        movie: movie.into(),
    };
    Ok(response)
}

#[derive(Deserialize, Debug)]
pub struct Payload {
    movie: Movie,
}

#[derive(Serialize)]
struct Response {
    movie: Movie,
}
