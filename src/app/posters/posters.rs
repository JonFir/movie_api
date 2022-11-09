use std::{fs, io::Write, path::Path};

use super::errors::Error;

pub fn make_poster_path(id: &str, folder_path: &str) -> Option<String> {
    Path::new(".")
        .join(folder_path)
        .join(&id)
        .to_str()
        .map(|p| p.to_owned())
}

pub fn safe_poster(path: &str, bytes: &[u8]) -> Result<(), Error> {
    let mut file = fs::File::create(path).map_err(|e| Error::FSError(e))?;
    file.write(&bytes).map_err(|e| Error::FSError(e))?;
    Ok(())
}

pub fn is_exist(path: &str) -> bool {
    fs::metadata(path).is_ok()
}
