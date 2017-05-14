use { libflo_func, libflo_module, serde_json };
use libflo_func::Parameter;

error_chain! {
    types { }

    links {
        LibfloFuncError(libflo_func::Error, libflo_func::ErrorKind);
        LibfloModuleError(libflo_module::Error, libflo_module::ErrorKind);
    }

    foreign_links {
        SerdeJsonError(serde_json::Error);
    }

    errors {
        EventDeserializationFailure(event_text: String) {
            description("Error deserializing event text.")
            display(
                "{}{}",
                "Error deserializing event text:\n",
                event_text,
            )
        }

        EventError(errs: Vec<libflo_func::Error>) {
            description("One or more event handlers failed executing.")
            display(
                "{}{}",
                "One or more event handlers failed executing. The failures include:",
                {
                    let mut result = String::new();
                    for error in errs {
                        result = format!("{}\n{}", result, error);
                    }
                    result.to_string()
                }
            )
        }

        EventHandlerDeserializationFailure(event_handler_text: String) {
            description("Error deserializing event handler text. The event handler text could not be deserialized.")
            display(
                "{}{}",
                "Error deserializing event handler text:\n",
                event_handler_text,
            )
        }

        EventLoadNameCollision(event_name: String, module_name: String) {
            description("Error loading event. An event with the same name has already been loaded from the module.")
            display(
                "{}{}{}{}{}",
                "Error loading event with name, '",
                event_name,
                "', from module, '",
                module_name,
                "'. A event with the same name has already been loaded from the module.",
            )
        }

        EventNotFoundInList(event_id: usize, event_list_max: usize) {
            description("Error locating event in event list. The event could not be found.")
            display(
                "{}{}{}{}{}",
                "Error locating event with id, '",
                event_id,
                "', in event list. The event could not be found. The list contains only, '",
                event_list_max,
                "', funcs.",
            )
        }

        EventNotFoundInMap(module_id: usize, event_name: String) {
            description("Error getting event in event map. The event could not be found.")
            display(
                "{}{}{}{}{}",
                "Error locating event with name, '",
                event_name,
                "', for module with id, '",
                module_id,
                "', in event map. The event could not be found.",
            )
        }

        EventTypeFailure(func_type: (Parameter, Parameter), event_type: (Parameter, Parameter)) {
            description("Error adding an event handler to an event. The event handler is the wrong type.")
            display(
                "{}{:?}{}{:?}{}",
                "Error adding an event handler to an event. The event handler is of type,'",
                func_type,
                "', but the event is of type,'",
                event_type,
                "'."
            )
        }

        FailureFindingEventHandler(module: usize, event: String) {
            description("Error adding an event handler to an event. The event handler could not be found.")
            display(
                "{}{}{}{}{}",
                "Error adding a function from module, '",
                module,
                ", to the event,",
                event,
                ". The event handler could not be found.",
            )
        }

        FunctionAndFunctionsBothDefined {
            description("Error using deserialized event handler. 'function', and 'functions' are both defined. Only one can be defined.")
            display("Error using deserialized event handler. 'function', and 'functions' are both defined. Only one can be defined.")
        }

        FunctionOrFunctionsNotDefined {
            description("Error using deserialized event handler. 'function', or 'functions' are not defined. One of these must be defined.")
            display("Error using deserialized event handler. 'function', or 'functions' are not defined. One of these must be defined.")
        }
    }
}
