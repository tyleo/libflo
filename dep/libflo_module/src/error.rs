use serde_json;
use std::fs::File;
use std::io;
use std::path::PathBuf;

error_chain! {
    types { }

    links { }

    foreign_links {
        IoError(io::Error);
        SerdeJsonError(serde_json::Error);
    }

    errors {
        AbsolutePathNotFound(path: PathBuf) {
            description("Error locating absolute path. The path does not exist.")
            display(
                "{}{}{}",
                "Error locating absolute path. The path, '",
                path.to_string_lossy(),
                "', does not exist.",
            )
        }

        DependencyHasNoName(path: PathBuf) {
            description("Error loading dependency. The dependency has no name.")
            display(
                "{}{}{}",
                "Error loading dependency at path, '",
                path.to_string_lossy(),
                "'. The dependnecy has no name.",
            )
        }

        DependencyHasNoVersion(name: String, path: PathBuf) {
            description("Error loading dependency. The dependency has no version.")
            display(
                "{}{}{}{}{}",
                "Error loading dependency, '",
                name,
                "' at path, '",
                path.to_string_lossy(),
                "'. The dependnecy has no version.",
            )
        }

        FileFromPathFailure(path: PathBuf) {
            description("Error opening file. The file could not be opened.")
            display(
                "{}{}{}",
                "Error opending file at path, '",
                path.to_string_lossy(),
                "'. The file could not be opended.",
            )
        }

        FileNotFoundInModule(file: PathBuf, full_path: PathBuf) {
            description("Error locating file in module. The file does not exist.")
            display(
                "{}{}{}{}{}",
                "Error locating file in module. The file, '",
                file.to_string_lossy(),
                "', does not exist at '",
                full_path.to_string_lossy(),
                "'.",
            )
        }

        ModuleNotFoundInList(module_id: usize) {
            description("Error getting module in module list. The module could not be found.")
            display(
                "{}{}{}",
                "Error locating module with id, '",
                module_id,
                "', in module list. The module could not be found.",
            )
        }

        NotImplemented(description: String, display: String) {
            description(description)
            display(
                "{}",
                display,
            )
        }

        ModuleFolderNotFound(module_name: Option<String>, module_id: usize) {
            description("The module folder could not be found.")
            display(
                "{}",
                {
                    if let &Some(ref module_name) = module_name {
                        format!("The module folder for module, '{}', could not be found.", module_name)
                    } else {
                        format!("The module folder for module with id, '{}', could not be found.", module_id)
                    }
                }
            )
        }

        PathVerificationNotAbsolute(path: PathBuf) {
            description("Error verifying path. To path is not absolute.")
            display(
                "{}{}{}",
                "Error verifying path. The path, '",
                path.to_string_lossy(),
                "', is not absolute.",
            )
        }

        PathVerificationNotDirectory(path: PathBuf) {
            description("Error verifying path. To path is not a directory.")
            display(
                "{}{}{}",
                "Error verifying path. The path, '",
                path.to_string_lossy(),
                "', is not a directory.",
            )
        }

        PathVerificationNotExist(path: PathBuf) {
            description("Error verifying path. To path is does not exist.")
            display(
                "{}{}{}",
                "Error verifying path. The path, '",
                path.to_string_lossy(),
                "', does not exist.",
            )
        }

        PathVerificationNotFile(path: PathBuf) {
            description("Error verifying path. To path is not a file.")
            display(
                "{}{}{}",
                "Error verifying path. The path, '",
                path.to_string_lossy(),
                "', is not a file.",
            )
        }

        ModuleDeserializationFailure(module_text: String) {
            description("Error deserializing module text. The module text could not be deserialized.")
            display(
                "{}{}{}",
                "Error deserializing module text:\n",
                module_text,
                "\nThe module text could not be deserialized.",
            )
        }

        ModuleLoadNameCollision(name: String) {
            description("Error loading module. A module with the specified name has already been loaded.")
            display(
                "{}{}{}",
                "Error loading module with name, '",
                name,
                "'. A module with the specified name has already been loaded.",
            )
        }

        ModuleLoadPathCollision(path: PathBuf) {
            description("Error loading module. A module at the specified path has already been loaded.")
            display(
                "{}{}{}",
                "Error loading module at path, '",
                path.to_string_lossy(),
                "'. A module at the specified path has already been loaded.",
            )
        }

        ModuleNotFoundInModuleMap(module_name: String) {
            description("Error locating module in module map. The module could not be found.")
            display(
                "{}{}{}",
                "Error locating module with name, '",
                module_name,
                "', in module map. The module could not be found.",
            )
        }

        ModulePathIdIncorrect(path: PathBuf, module_id: usize, next_id: usize) {
            description("Error adding module path. A module with the current id cannot be added.")
            display(
                "{}{}{}{}{}{}{}",
                "Error adding module path, '",
                path.to_string_lossy(),
                "'. A module with path with the id, '",
                module_id,
                "', cannot be added. The next module id should be, '",
                next_id,
                "'.",
            )
        }

        ModulePathNotFound(file: PathBuf, module_id: usize) {
            description("Error finding module path for file. A module the requested id does not exist.")
            display(
                "{}{}{}{}{}",
                "Error finding module path for file, '",
                file.to_string_lossy(),
                "'. A module with the requested id, '",
                module_id,
                "', does not exist.",
            )
        }

        RelativePathNotFound(path: PathBuf, search_paths: Vec<PathBuf>) {
            description("Error locating relative path. The path does not exist.")
            display(
                "{}{}{}{}",
                "Error locating relative path, '",
                path.to_string_lossy(),
                "', the path could not be found. The following paths were searched:",
                {
                    let mut result = String::new();
                    for search_path in search_paths {
                        let this_result =
                            format!(
                                "{}{}{}",
                                "\n'",
                                search_path.to_string_lossy(),
                                "'",
                            );
                        result.push_str(&this_result);
                    }
                    result
                },
            )
        }

        SubmoduleNotExist(module_id: usize, submodule_id: usize) {
            description("The submodule does not exist.")
            display(
                "{}{}{}{}{}",
                "Error locating with id, '",
                submodule_id,
                "', while searching in module with id, '",
                module_id,
                "'. The submodule does not exist.",
            )
        }

        TextFromFileFailure(file: File) {
            description("Error reading text from file. The file could not be read.")
            display(
                "{}{:?}{}",
                "Error reading text from file:\n",
                file,
                "\nThe file could not be read. Cause:\n",
            )
        }
    }
}
