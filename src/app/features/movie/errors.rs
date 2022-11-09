use std::fmt;

use actix_web::error::BlockingError;

use crate::app;

#[derive(Debug)]
pub enum Error {
    PosterNotFound,
    CorruptedPosterPath,
    AlreadyExist(app::database::errors::Error),
    NotFound(app::database::errors::Error),
    QueryFail(app::database::errors::Error),
    BlockError(BlockingError),
}
impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:}", self,)
    }
}

impl Into<app::errors::Error> for Error {
    fn into(self) -> app::errors::Error {
        app::errors::Error::Movie(self)
    }
}

impl Into<Option<app::errors::Error>> for Error {
    fn into(self) -> Option<app::errors::Error> {
        Some(app::errors::Error::Movie(self))
    }
}

impl From<app::database::errors::Error> for Error {
    fn from(error: app::database::errors::Error) -> Self {
        match error {
            app::database::errors::Error::AlreadyExist(_) => Error::AlreadyExist(error),
            app::database::errors::Error::NotFound(_) => Error::NotFound(error),
            app::database::errors::Error::Other(_) => Error::QueryFail(error),
        }
    }
}
