use error::*;
use libflo_module::{ file_from_path, text_from_file };
use serde_json;
use serialization::LibsSerde;
use std::path::Path;

pub fn libs_from_path<TPath>(path: TPath) -> Result<LibsSerde>
    where TPath: AsRef<Path> {
    let file = file_from_path(path)?;
    let text = text_from_file(file)?;
    libs_from_text(text)
}

pub fn libs_from_text(text: String) -> Result<LibsSerde> {
    serde_json::from_str::<LibsSerde>(&text)
        .map_err(|err| Error::from(ErrorKind::SerdeJsonError(err)))
        .chain_err(|| ErrorKind::DllDeserializationFailure(text))
}
