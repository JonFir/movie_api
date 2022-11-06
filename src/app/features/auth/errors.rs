use std::fmt;

use crate::app;

#[derive(Debug)]
pub enum Error {
    MissingToken,
    PasswordHashingFail(argon2::Error),
    PasswordVerifyFail(argon2::Error),
    IncorectLogin,
    IncorectPassword,
    UserNotFound,
    UserAlreadyExist(app::database::errors::Error),
    QueryFail(app::database::errors::Error),
}
impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:}", self,)
    }
}

impl Into<app::errors::Error> for Error {
    fn into(self) -> app::errors::Error {
        app::errors::Error::Auth(self)
    }
}

impl Into<Option<app::errors::Error>> for Error {
    fn into(self) -> Option<app::errors::Error> {
        Some(app::errors::Error::Auth(self))
    }
}

impl From<app::database::errors::Error> for Error {
    fn from(error: app::database::errors::Error) -> Self {
        match error {
            app::database::errors::Error::AlreadyExist(_) => Error::UserAlreadyExist(error),
            app::database::errors::Error::NotFound(_) => Error::QueryFail(error),
            app::database::errors::Error::Other(_) => Error::QueryFail(error),
        }
    }
}
