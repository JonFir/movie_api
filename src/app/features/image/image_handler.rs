use std::{fs, path::Path, sync::Arc};

use actix_web::{
    get,
    http::{header, StatusCode},
    web::{self, Data},
    HttpResponse, HttpResponseBuilder, Responder,
};
use file_format::FileFormat;

use crate::app::{
    error_response::{ErrorMeta, ErrorResponse},
    posters,
    state::AppState,
};

use super::errors::Error;

#[get("/{id}")]
pub async fn handler(
    path: web::Path<String>,
    data: Data<Arc<AppState>>,
) -> Result<impl Responder, ErrorResponse> {
    let id = path.into_inner();
    let result = make_response(id, data).await;
    match result {
        Ok(response) => Ok(response),
        Err(e @ Error::FileNotFound) => {
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

async fn make_response(id: String, data: Data<Arc<AppState>>) -> Result<HttpResponse, Error> {
    let path = posters::posters::make_poster_path(&id, &data.environment.posters_path)
        .ok_or(Error::CorruptedPath)?;

    let file = web::block(|| {
        if posters::posters::is_exist(&path) {
            fs::read(path).map_err(|e| Error::FSError(e))
        } else {
            Err(Error::FileNotFound)
        }
    })
    .await
    .map_err(|e| Error::BlockError(e))??;
    let file_format = FileFormat::from_bytes(&file);
    let response = HttpResponseBuilder::new(StatusCode::OK)
        .insert_header((header::CONTENT_TYPE, file_format.media_type()))
        .body(file);
    Ok(response)
}
