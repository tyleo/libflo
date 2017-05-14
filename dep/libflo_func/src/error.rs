use { libflo_error, libflo_module, serde_json };
use func::Parameter;
use serialization::OsSerde;
use std::fmt::Debug;
use std::path::PathBuf;


error_chain! {
    types { }

    links {
        LibfloDllError(libflo_error::Error, libflo_error::ErrorKind);
        LibfloModuleError(libflo_module::Error, libflo_module::ErrorKind);
    }

    foreign_links {
        SerdeJsonError(serde_json::Error);
    }

    errors {
        DllCouldNotLoad(path: PathBuf, module_id: usize) {
            description("Error loading dll. The dll could not be loaded.")
            display(
                "{}{}{}{}{}",
                "Error loading dll at path, '",
                path.to_string_lossy(),
                "', from module with id, '",
                module_id,
                "'. The dll could not be loaded.",
            )
        }

        DllDeserializationFailure(dll_text: String) {
            description("Error deserializing dll text. The dll text could not be deserialized.")
            display(
                "{}{}{}",
                "Error deserializing dll text:\n",
                dll_text,
                "\nThe dll text could not be deserialized.",
            )
        }

        EventNotFoundInEventList(event_id: usize, event_list_max: usize) {
            description("Error locating func in func list. The func could not be found.")
            display(
                "{}{}{}{}{}",
                "Error locating func with id, '",
                event_id,
                "', in func list. The func could not be found. The list contains only, '",
                event_list_max,
                "', funcs.",
            )
        }

        FuncLoadNameCollision(name: String, module_name: String) {
            description("Error loading func. An func with the specified name has already been loaded from the module.")
            display(
                "{}{}{}{}{}",
                "Error loading func with name, '",
                name,
                "', from module, '",
                module_name,
                "'. A func with the specified name has already been loaded from the module.",
            )
        }

        FuncNotFoundInFuncMap(module_id: usize, event_name: String) {
            description("Error locating func in func map. The func could not be found.")
            display(
                "{}{}{}{}{}",
                "Error locating func with name, '",
                event_name,
                "', for module with id, '",
                module_id,
                "', in func map. The func could not be found.",
            )
        }

        FuncParameterIncorrect(required: Parameter, given: Parameter) {
            description("Func::call was passed the incorrect parameters for this func's type.")
            display(
                "{}{:?}{}{:?}{}",
                "A func with input type, '",
                required,
                "', was passed input of type, '",
                given,
                "'. These types must match for the function call to succeed."
            )
        }

        FuncTypeCouldNotBeMatched(func_type: (Box<Debug + Send>, Box<Debug + Send>)) {
            description("new_func was passed an invalid func type. Perhaps, you forgot to register it.")
            display(
                "{}{:?}{}{:?}{}",
                "new_func was passed type, '(",
                func_type.0,
                ", ",
                func_type.1,
                ")', but it does not accept this type. Perhaps, you forgot to register it."
            )
        }

        InputDowncastFailure {
            description("Cannot call func. The input cannot by downcast to the correct type.")
            display("Cannot call func. The input cannot by downcast to the correct type.")
        }

        LibAndLibsBothDefined {
            description("Error using deserialized libs. 'lib', and 'libs' are both defined. Only one can be defined.")
            display("Error using deserialized libs. 'lib', and 'libs' are both defined. Only one can be defined.")
        }

        LibOrLibsNotDefined {
            description("Error using deserialized libs. 'lib', or 'libs' are not defined. One of these must be defined.")
            display("Error using deserialized libs. 'lib', or 'libs' are not defined. One of these must be defined.")
        }

        ModuleInfo(module: String) {
            description("Error occured in module.")
            display(
                "{}{}{}",
                "Error occurred in module, '",
                module,
                "'."
            )
        }

        PlatformNotSupported(os: OsSerde) {
            description("The current os not supported for this library.")
            display(
                "{}{:?}{}",
                "The current os, '",
                os,
                "', is not supported for this library."
            )
        }

        SymbolCouldNotLoad(symbol: String) {
            description("Error loading symbol. The symbol could not be loaded.")
            display(
                "{}{}{}",
                "Error loading symbol, '",
                symbol,
                "'. The symbol could not be loaded.",
            )
        }
    }
}
