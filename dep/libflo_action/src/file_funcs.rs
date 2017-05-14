use error::*;
use libflo_std::{ file_from_path, text_from_file };
use serde_json;
use serialization::ActionsSerde;
use std::path::Path;

pub fn actions_from_path<TPath>(path: TPath) -> Result<ActionsSerde>
    where TPath: AsRef<Path> {
    let file = file_from_path(path)?;
    let text = text_from_file(file)?;

    actions_from_text(text)
}

pub fn actions_from_text(text: String) -> Result<ActionsSerde> {
    serde_json::from_str::<ActionsSerde>(&text)
        .map_err(|err| Error::from(ErrorKind::SerdeJsonError(err)))
        .chain_err(|| ErrorKind::ActionsDeserializationFailure(text))
}
