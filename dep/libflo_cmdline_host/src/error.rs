use libflo_std::{ event, libflo, module, mut_static, Parameter };
use std::io;
use std::path::PathBuf;
use std::string;

error_chain! {
    types { }

    links {
        LibfloError(libflo::Error, libflo::ErrorKind);
        LibfloEventError(event::Error, event::ErrorKind);
        LibfloModuleError(module::Error, module::ErrorKind);
        MutStaticError(mut_static::Error, mut_static::ErrorKind);
    }

    foreign_links {
        IoError(io::Error);
        StringFromUtf8Error(string::FromUtf8Error);
    }

    errors {
        ConstructEventWrongType(module: String, name: String) {
            description("Error calling construct event. The construct event is the wrong type. It must be of type, 'NativeEventInput'.")
            display(
                "{}{}{}{}{}",
                "Error calling construct event. The construct event with name, '",
                name,
                "', from module, '",
                module,
                "' is the wrong type. It must be of type, 'NativeEventInput'.",
            )
        }

        ErrorReadingInputStream {
            description("Error reading input stream.")
            display(
                "{}",
                "Error reading input stream.",
            )
        }

        ErrorWritingErrorStream {
            description("Error writing error stream.")
            display(
                "{}",
                "Error writing error stream."
            )
        }

        ErrorWritingOutputStream {
            description("Error writing output stream.")
            display(
                "{}",
                "Error writing output stream.",
            )
        }

        FuncOutputTypeIncorrect(given_type: Parameter, expected_type: Parameter) {
            description("The output type of the function is incorrect.")
            display(
                "{}{:?}{}{:?}{}",
                "The output type of the function is incorrect. The type, '",
                given_type,
                "', was given but the expected type is,'",
                expected_type,
                "'."
            )
        }

        InvalidCommandLine(command_line: String) {
            description("Error executing command. The command is invalid.")
            display(
                "{}{}",
                "Error executing command. The command is invalid. Command contents:\n",
                command_line,
            )
        }

        NoCurrentExe {
            description("Error locating execution path. No exe could be found in the environment.")
            display(
                "{}",
                "Error locating execution path. No exe could be found in the environment.",
            )
        }

        NoExeDirectoryPath(path_to_exe: PathBuf) {
            description("Error locating execution path. The exe you are running is not in a directory.")
            display(
                "{}{}{}",
                "Error locating execution path. The exe you are running at path, '",
                path_to_exe.to_string_lossy(),
                "', is not in a directory.",
            )
        }

        NoRootModule(root_module_path: PathBuf) {
            description("Error locating root module. No root module at path.")
            display(
                "{}{}{}",
                "Error locating root module. No root module at path, '",
                root_module_path.to_string_lossy(),
                "'.",
            )
        }

        NonStringFuncCannotExecute(func_id: usize, arg: String) {
            description("Error attempting to execute func. The func is not a string func.")
            display(
                "{}{}{}{}",
                "Error attempting to execute func. The func with id, '",
                func_id,
                "', is not a string func. The args to the func were:\n",
                arg,
            )
        }

        RootModulePathIsNotADir(root_module_path: PathBuf) {
            description("Error with root module path. The path is not a directory.")
            display(
                "{}{}{}",
                "Error with root module path. The path, '",
                root_module_path.to_string_lossy(),
                "' is not a directory.",
            )
        }

        RootModulePathIsNotAbsolute(root_module_path: PathBuf) {
            description("Error with root module path. The path is not absolute.")
            display(
                "{}{}{}",
                "Error with root module path. The path, '",
                root_module_path.to_string_lossy(),
                "' is not absolute.",
            )
        }
    }
}
