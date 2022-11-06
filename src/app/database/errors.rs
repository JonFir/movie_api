use std::fmt;

#[derive(Debug)]
pub enum Error {
    Other(sqlx::error::Error),
    AlreadyExist(sqlx::error::Error),
    NotFound(sqlx::error::Error),
}
impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:}", self,)
    }
}
