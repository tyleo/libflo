use { file_from_path, text_from_file };
use error::*;
use serde_json;
use serialization::ModuleSerde;
use std::path::Path;

pub fn module_from_path<TPath>(path: TPath) -> Result<ModuleSerde>
    where TPath: AsRef<Path> {
    let file = file_from_path(path)?;
    let text = text_from_file(file)?;
    module_from_text(text)
}

pub fn module_from_text(text: String) -> Result<ModuleSerde> {
    serde_json::from_str::<ModuleSerde>(&text)
        .map_err(|err| Error::from(ErrorKind::SerdeJsonError(err)))
        .chain_err(|| ErrorKind::ModuleDeserializationFailure(text))
}
