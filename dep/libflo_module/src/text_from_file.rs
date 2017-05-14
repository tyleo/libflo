use error::*;
use std::fs::File;
use std::io::Read;

pub fn text_from_file(mut file: File) -> Result<String> {
    let mut result = String::new();
    match file.read_to_string(&mut result) {
        Err(err) => {
            Err(Error::from(ErrorKind::IoError(err))).chain_err(|| ErrorKind::TextFromFileFailure(file))
        },
        Ok(_) => {
            Ok(result)
        },
    }
}
