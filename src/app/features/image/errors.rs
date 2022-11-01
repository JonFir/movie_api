use std::fmt;

use actix_web::error::BlockingError;

use crate::app::{self, database};

#[derive(Debug)]
pub enum Error {
    CorruptedPath,
    IncorrectFiletype,
    FileNotFound,
    FSError(std::io::Error),
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
        app::errors::Error::Image(self)
    }
}

impl Into<Option<app::errors::Error>> for Error {
    fn into(self) -> Option<app::errors::Error> {
        Some(app::errors::Error::Image(self))
    }
}

// impl From<database::errors::Error> for Error {
//     fn from(error: database::errors::Error) -> Self {
//         match error {
//             database::errors::Error::UserAlreadyExist(_) => Error::UserAlreadyExist(error),
//             database::errors::Error::UserNotFound(_) => Error::QueryFail(error),
//             database::errors::Error::Other(_) => Error::QueryFail(error),
//         }
//     }
// }
