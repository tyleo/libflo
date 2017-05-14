use error::*;
use std::env;
use std::path::PathBuf;
use string;

pub fn root_module_directory() -> Result<PathBuf> {
    let root_module_path = env::current_exe().chain_err(|| ErrorKind::NoCurrentExe)?;

    let mut root_module_path =
        match root_module_path.parent() {
            Some(some) => some.to_path_buf(),
            None => {
                return Err(ErrorKind::NoExeDirectoryPath(root_module_path.clone()).into());
            },
        };

    root_module_path.push(string::root_module_directory());

    if !root_module_path.is_absolute() {
        Err(ErrorKind::RootModulePathIsNotAbsolute(root_module_path).into())
    } else if !root_module_path.exists() {
        Err(ErrorKind::NoRootModule(root_module_path).into())
    } else if !root_module_path.is_dir() {
        Err(ErrorKind::RootModulePathIsNotADir(root_module_path).into())
    } else {
        Ok(root_module_path)
    }
}
