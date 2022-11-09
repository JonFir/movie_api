use std::fmt;

#[derive(Debug)]
pub enum Error {
    Auth(crate::app::features::auth::errors::Error),
    Image(crate::app::features::image::errors::Error),
    Movie(crate::app::features::movie::errors::Error),
}
impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "code: {:}", self,)
    }
}
