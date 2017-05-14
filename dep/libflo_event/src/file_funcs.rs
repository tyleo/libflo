use error::*;
use libflo_module::{ file_from_path, text_from_file };
use serde_json;
use serialization::{ EventsSerde, EventHandlersSerde };
use std::path::Path;

pub fn event_handlers_from_path<TPath>(path: TPath) -> Result<EventHandlersSerde>
    where TPath: AsRef<Path> {
    let file = file_from_path(path)?;
    let text = text_from_file(file)?;

    event_handlers_from_text(text)
}

pub fn event_handlers_from_text(text: String) -> Result<EventHandlersSerde> {
    serde_json::from_str::<EventHandlersSerde>(&text)
        .map_err(|err| Error::from(ErrorKind::SerdeJsonError(err)))
        .chain_err(|| ErrorKind::EventHandlerDeserializationFailure(text))
}

pub fn events_from_path<TPath>(path: TPath) -> Result<EventsSerde>
    where TPath: AsRef<Path> {
    let file = file_from_path(path)?;
    let text = text_from_file(file)?;

    events_from_text(text)
}

pub fn events_from_text(text: String) -> Result<EventsSerde> {
    serde_json::from_str::<EventsSerde>(&text)
        .map_err(|err| Error::from(ErrorKind::SerdeJsonError(err)))
        .chain_err(|| ErrorKind::EventDeserializationFailure(text))
}
