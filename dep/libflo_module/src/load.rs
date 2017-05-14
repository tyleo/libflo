use { file_funcs, ModuleMapper, PathResolver, Payload, string };
use error::*;
use serialization::{ DependencySerde, FileDependencySerde, PackageDependencySerde };
use std::collections::{ HashMap, HashSet };
use std::path::PathBuf;

pub fn load(
    root_path: PathBuf,
    exe_path: Option<PathBuf>,
    search_paths: Option<Vec<PathBuf>>,
) -> Result<Payload> {
    Loader::load(root_path, exe_path, search_paths)
}

struct Loader {
    loaded_file_dependency_paths: HashSet<PathBuf>,
    path_resolver: PathResolver,
    module_name_to_id: HashMap<String, usize>,
    module_id_to_name: Vec<String>,
    current_id: usize,
}

impl Loader {
    pub fn load(
        root_path: PathBuf,
        exe_path: Option<PathBuf>,
        search_paths: Option<Vec<PathBuf>>,
    ) -> Result<Payload> {
        let path_resolver = PathResolver::new(root_path, exe_path, search_paths)?;

        let root_module_file_name = string::module_file_name();
        let root_module_file_path = path_resolver.find_path(root_module_file_name)?;

        let mut loader =
            Loader {
                loaded_file_dependency_paths: HashSet::new(),
                path_resolver: path_resolver,
                module_name_to_id: HashMap::new(),
                module_id_to_name: Vec::new(),
                current_id: 0,
            };

        loader.load_from_root_file_path(root_module_file_path)?;

        let result = loader.to_result();

        Ok(result)
    }

    fn load_dependencies(&mut self, mut dependency_stack: Vec<DependencySerde>) -> Result<()> {
        while let Some(dependency) = dependency_stack.pop() {
            self.load_dependency(dependency, &mut dependency_stack)?;
        }

        Ok(())
    }

    fn load_dependency(&mut self, dependency: DependencySerde, dependency_stack: &mut Vec<DependencySerde>) -> Result<()> {
        match dependency {
            DependencySerde::File(file) => self.load_file_dependency(file, dependency_stack),
            DependencySerde::Package(package) => self.load_package_dependency(package, dependency_stack),
        }
    }

    fn load_dependency_serial(
        &mut self,
        module_id: usize,
        name: String,
        dependencies: Option<Vec<DependencySerde>>,
        dependency_stack: &mut Vec<DependencySerde>
    ) -> Result<()> {
        if let Some(_) = self.module_name_to_id.insert(name.clone(), module_id) {
            return Err(ErrorKind::ModuleLoadNameCollision(name).into());
        }
        self.module_id_to_name.push(name.clone());

        let dependencies =
            match dependencies {
                Some(some) => some.clone(),
                None => Vec::new(),
            };
        dependency_stack.reserve(dependencies.len());
        for dependency in dependencies  {
            dependency_stack.push(dependency);
        }

        Ok(())
    }

    fn load_file_dependency(&mut self, file_dependency: FileDependencySerde, dependency_stack: &mut Vec<DependencySerde>) -> Result<()> {
        let dependency_path = self.path_resolver.find_path(file_dependency.get_path())?;
        if self.loaded_file_dependency_paths.contains(&dependency_path) {
            Ok(())
        } else {
            if !self.loaded_file_dependency_paths.insert(dependency_path.clone()) {
                Err(ErrorKind::ModuleLoadPathCollision(dependency_path).into())
            }
            else
            {
                self.load_file_dependency_from_path(dependency_path, dependency_stack)
            }
        }
    }

    fn load_file_dependency_from_path(
        &mut self,
        dependency_path: PathBuf,
        dependency_stack: &mut Vec<DependencySerde>
    ) -> Result<()> {
        let dependency_module_file_path = PathResolver::create_file_path_from_temp_directory_path(
            dependency_path.clone(),
            string::module_file_name(),
        )?;

        let dependency_module = file_funcs::module_from_path(&dependency_module_file_path)?;
        let (name, dependencies, version) = dependency_module.destructure();
        let name =
            match name {
                Some(some) => some,
                None => {
                    return Err(ErrorKind::DependencyHasNoName(dependency_module_file_path).into());
                },
            };

        if self.module_name_to_id.contains_key(&name) {
            let version =
                match version {
                    Some(some) => some,
                    None => {
                        return Err(ErrorKind::DependencyHasNoVersion(name, dependency_module_file_path).into());
                    },
                };
            self.version_test_dependency(name, version)
        } else {
            let module_id = self.current_id;
            self.current_id += 1;
            self.path_resolver.add_module_path(dependency_path, module_id, name.clone())?;
            self.load_dependency_serial(module_id, name, dependencies, dependency_stack)
        }
    }

    fn load_from_root_file_path(&mut self, root_module_path: PathBuf) -> Result<()> {
        let root_module = file_funcs::module_from_path(root_module_path)?;
        let (_, dependencies, _) = root_module.destructure();
        let dependency_stack = dependencies.unwrap_or(Vec::new());
        self.load_dependencies(dependency_stack)
    }

    #[allow(unused_variables)]
    fn load_package_dependency(&mut self, package_dependency: PackageDependencySerde, dependency_stack: &mut Vec<DependencySerde>) -> Result<()> {
        // TODO: allow packages.
        Err(
            ErrorKind::NotImplemented(
                "Packages are not implemented.".to_string(),
                "Packages are not implemented.".to_string()
            ).into()
        )
    }

    fn to_result(self) -> Payload {
        let module_mapper = ModuleMapper::new(self.module_name_to_id, self.module_id_to_name);

        Payload::new(
            module_mapper,
            self.path_resolver,
        )
    }

    #[allow(unused_variables)]
    fn version_test_dependency(&self, name: String, version: String) -> Result<()> {
        // TODO: verision_test_dependency always passes.
        Ok(())
    }
}
