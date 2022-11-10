use std::fmt;

use actix_web::error::BlockingError;

#[derive(Debug)]
pub enum Error {
    DBOther(sqlx::Error),
    DBKeyDublicate(sqlx::Error),
    IncorectLogin,
    IncorectPassword,
    MissingAuthToken,
    UserNotFound,
    NotFound,
    CorruptedFSPath,
    FSError(std::io::Error),
    PasswordHashingFail(argon2::Error),
    PasswordVerifyFail(argon2::Error),
    ActixBlockError(BlockingError),
    IncorrectFiletype,
}
impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "code: {:}", self,)
    }
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        let e = error.as_database_error().and_then(|e| e.code());
        match e {
            Some(err) if err.eq("2067") => Error::DBKeyDublicate(error),
            _ => Error::DBOther(error),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::FSError(error)
    }
}

impl From<BlockingError> for Error {
    fn from(error: BlockingError) -> Self {
        Error::ActixBlockError(error)
    }
}
