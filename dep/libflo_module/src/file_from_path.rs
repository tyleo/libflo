use error::*;
use std::fs::File;
use std::path::Path;

pub fn file_from_path<TPath>(path: TPath) -> Result<File>
    where TPath: AsRef<Path> {
    let path = path.as_ref();
    File::open(path)
        .map_err(|err| Error::from(ErrorKind::IoError(err)))
        .chain_err(|| ErrorKind::FileFromPathFailure(path.to_path_buf()))
}
