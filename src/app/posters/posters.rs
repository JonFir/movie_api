use std::{fs, io::Write, path::Path};

use crate::app::errors::Error;

pub fn make_poster_path(id: &str, folder_path: &str) -> Result<String, Error> {
    let path = Path::new(".")
        .join(folder_path)
        .join(&id)
        .to_str()
        .ok_or(Error::CorruptedFSPath)?
        .to_owned();
    Ok(path)
}

pub fn safe_poster(path: &str, bytes: &[u8]) -> Result<(), Error> {
    let mut file = fs::File::create(path)?;
    file.write(&bytes)?;
    Ok(())
}

pub fn is_exist(path: &str) -> bool {
    fs::metadata(path).is_ok()
}
