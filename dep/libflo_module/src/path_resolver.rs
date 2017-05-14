use error::*;
use std::path::{ Path, PathBuf };
use string;

#[derive(Clone, Debug)]
pub struct PathResolver {
    module_names: Vec<String>,
    module_paths: Vec<PathBuf>,
    search_paths: Vec<PathBuf>,
}

impl PathResolver {
    pub fn new(root_path: PathBuf, exe_path: Option<PathBuf>, search_paths: Option<Vec<PathBuf>>) -> Result<Self> {
        let root_path_vec = vec![root_path.clone()];
        let search_paths = search_paths.unwrap_or(Vec::new());

        let result_paths = root_path_vec.into_iter().chain(exe_path).chain(search_paths).collect();

        for result_path in &result_paths {
            try!(Self::verify_search_path(result_path));
        }

        let result =
            PathResolver{
                module_names: Vec::new(),
                module_paths: Vec::new(),
                search_paths: result_paths,
            };

        Ok(result)
    }

    pub fn add_module_path(&mut self, path: PathBuf, module_id: usize, module_name: String) -> Result<()> {
        try!(Self::verify_search_path(&path));

        if self.module_paths.len() == module_id && self.module_names.len() == module_id {
            self.module_paths.push(path);
            self.module_names.push(module_name);
            Ok(())
        } else {
            Err(
                ErrorKind::ModulePathIdIncorrect(
                    path,
                    module_id,
                    self.module_paths.len()
                ).into()
            )
        }
    }

    pub fn create_file_path_from_temp_directory_path<TPath>(
        mut directory_path: PathBuf,
        file_path: TPath
    ) -> Result<PathBuf>
        where TPath: AsRef<Path> {
        try!(Self::verify_search_path(&directory_path));
        directory_path.push(file_path);

        try!(Self::verify_file_path(&directory_path));
        Ok(directory_path)
    }

    pub fn find_path<TPath>(&self, path: TPath) -> Result<PathBuf>
        where TPath: AsRef<Path> {
        let path = path.as_ref();
        if path.is_absolute() {
            Self::find_absolute_path(path)
        } else {
            self.find_relative_path(path)
        }
    }

    pub fn find_module_path(&self, module_id: usize) -> Result<&Path> {
        match self.module_paths.get(module_id) {
            Some(some) => Ok(some),
            None => {
                if let Some(module_name) = self.module_names.get(module_id) {
                    Err(ErrorKind::ModuleFolderNotFound(Some(module_name.to_string()), module_id).into())
                } else {
                    Err(ErrorKind::ModuleFolderNotFound(None, module_id).into())
                }
            },
        }
    }

    pub fn try_find_module_path(&self, module_id: usize) -> Option<&PathBuf> {
        self.module_paths.get(module_id)
    }

    pub fn find_module_file_path<TPath>(&self, path: TPath, module_id: usize) -> Result<PathBuf>
        where TPath: AsRef<Path> {
        let path = path.as_ref();
        let module_path = self.find_module_path(module_id).chain_err(|| ErrorKind::ModulePathNotFound(path.to_path_buf(), module_id));
        let mut module_path = try!(module_path).to_path_buf();

        module_path.push(path);

        if module_path.exists() {
            Ok(module_path)
        } else {
            let file = path.to_path_buf();
            Err(ErrorKind::FileNotFoundInModule(file, module_path).into())
        }
    }

    pub fn try_find_module_file_path<TPath>(&self, path: TPath, module_id: usize) -> Result<Option<PathBuf>>
        where TPath: AsRef<Path> {
        let path = path.as_ref();
        let module_path = self.find_module_path(module_id).chain_err(|| ErrorKind::ModulePathNotFound(path.to_path_buf(), module_id));
        let mut module_path = try!(module_path).to_path_buf();

        module_path.push(path);

        if module_path.exists() {
            Ok(Some(module_path))
        } else {
            Ok(None)
        }
    }

    pub fn has_submodule(&self, module_id: usize, submodule_id: usize) -> Result<bool> {
        let mut module_path = try!(self.find_module_path(module_id)).to_path_buf();

        if let Some(some) = self.module_names.get(submodule_id) {
            module_path.push(string::submodule_path());
            module_path.push(some);
            Ok(module_path.exists())
        } else {
            Err(ErrorKind::SubmoduleNotExist(module_id, submodule_id).into())
        }
    }

    pub fn find_submodule_path(&self, module_id: usize, submodule_id: usize) -> Result<PathBuf> {
        let mut module_path = try!(self.find_module_path(module_id)).to_path_buf();

        if let Some(some) = self.module_names.get(submodule_id) {
            module_path.push(string::submodule_path());
            module_path.push(some);
            Ok(module_path)
        } else {
            Err(ErrorKind::SubmoduleNotExist(module_id, submodule_id).into())
        }
    }

    pub fn find_submodule_file_path<TPath>(&self, path: TPath, module_id: usize, submodule_id: usize) -> Result<PathBuf>
        where TPath: AsRef<Path> {
        let mut submodule_path = try!(self.find_submodule_path(module_id, submodule_id));
        submodule_path.push(path);

        Ok(submodule_path)
    }

    pub fn try_find_submodule_file_path<TPath>(&self, path: TPath, module_id: usize, submodule_id: usize) -> Result<Option<PathBuf>>
        where TPath: AsRef<Path> {
        let mut submodule_path = try!(self.find_submodule_path(module_id, submodule_id));
        submodule_path.push(path);

        if submodule_path.exists() {
            Ok(Some(submodule_path))
        } else {
            Ok(None)
        }
    }

    fn find_absolute_path<TPath>(path: TPath) -> Result<PathBuf>
        where TPath: AsRef<Path> {
        let path = path.as_ref();
        if path.exists() {
            let result = path.to_path_buf();
            Ok(result)
        } else {
            Err(ErrorKind::AbsolutePathNotFound(path.to_path_buf()).into())
        }
    }

    fn find_relative_path<TPath>(&self, path: TPath) -> Result<PathBuf>
        where TPath: AsRef<Path> {
        let path = path.as_ref();
        let self_search_paths = &self.search_paths;

        for search_path in self_search_paths {
            let mut current_path = search_path.to_path_buf();
            current_path.push(path);
            if current_path.exists() {
                return Ok(current_path);
            }
        }

        let search_paths =
            self_search_paths
            .into_iter()
            .map(
                |search_path| {
                    let mut search_path_buf = search_path.to_path_buf();
                    search_path_buf.push(path);
                    search_path_buf
                }
            ).collect();

        Err(ErrorKind::RelativePathNotFound(path.to_path_buf(), search_paths).into())
    }

    fn verify_file_path(file_path: &PathBuf) -> Result<()> {
        if !file_path.is_absolute() {
            Err(ErrorKind::PathVerificationNotAbsolute(file_path.clone()).into())
        } else if !file_path.exists() {
            Err(ErrorKind::PathVerificationNotExist(file_path.clone()).into())
        } else if !file_path.is_file() {
            Err(ErrorKind::PathVerificationNotFile(file_path.clone()).into())
        } else {
            Ok(())
        }
    }

    fn verify_search_path(search_path: &PathBuf) -> Result<()> {
        if !search_path.is_absolute() {
            Err(ErrorKind::PathVerificationNotAbsolute(search_path.clone()).into())
        } else if !search_path.exists() {
            Err(ErrorKind::PathVerificationNotExist(search_path.clone()).into())
        } else if !search_path.is_dir() {
            Err(ErrorKind::PathVerificationNotDirectory(search_path.clone()).into())
        } else {
            Ok(())
        }
    }
}
