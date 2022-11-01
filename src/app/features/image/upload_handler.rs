use std::{io::Write, path::Path, sync::Arc};

use actix_web::{
    post,
    web::{self, Bytes, Data},
    Responder,
};
use file_format::FileFormat;
use serde::Serialize;

use crate::app::{
    error_response::{ErrorMeta, ErrorResponse},
    response_payload::ResponsePayload,
    state::AppState,
};

use super::errors::Error;

#[post("/upload")]
pub async fn upload_handler(
    body: Bytes,
    data: Data<Arc<AppState>>,
) -> Result<impl Responder, ErrorResponse> {
    let result = make_response(body, data).await;
    match result {
        Ok(response) => {
            let respose = ResponsePayload::succes("Poster was upload successful", response);
            Ok(respose)
        }
        Err(e @ Error::IncorrectFiletype) => {
            let r = ErrorResponse {
                meta: ErrorMeta::make_bad_request(
                    "Unsupported file type. Only jpg and png accepted".into(),
                ),
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

async fn make_response(body: Bytes, data: Data<Arc<AppState>>) -> Result<Response, Error> {
    let bytes = body.to_vec();
    let file_format = FileFormat::from_bytes(&bytes);

    match file_format {
        FileFormat::PortableNetworkGraphics | FileFormat::JointPhotographicExpertsGroup => (),
        _ => return Err(Error::IncorrectFiletype),
    }

    let id = uuid::Uuid::new_v4().to_string();
    let path = Path::new(".")
        .join(&data.environment.posters_path)
        .join(&id)
        .to_str()
        .ok_or(Error::CorruptedPath)?
        .to_owned();

    let mut file = web::block(|| std::fs::File::create(path))
        .await
        .map_err(|e| Error::BlockError(e))?
        .map_err(|e| Error::FSError(e))?;
    let result = web::block(move || file.write(&bytes))
        .await
        .map_err(|e| Error::BlockError(e))?
        .map(|_| Response { poster_id: id })
        .map_err(|e| Error::FSError(e));
    result
}

#[derive(Serialize)]
struct Response {
    poster_id: String,
}
