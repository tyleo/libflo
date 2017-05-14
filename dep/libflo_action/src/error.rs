use libflo_std::{ error, event, libflo, module };
use number_or_string::NumberOrString;
use serde_json;

error_chain! {
    types { }

    links {
        LibfloError(libflo::Error, libflo::ErrorKind);
        LibfloErrorError(error::Error, error::ErrorKind);
        LibfloEvent(event::Error, event::ErrorKind);
        LibfloModuleError(module::Error, module::ErrorKind);
    }

    foreign_links {
        SerdeJsonError(serde_json::Error);
    }

    errors {
        ActionsDeserializationFailure(actions_text: String) {
            description("Error deserializing actions text.")
            display(
                "{}{}",
                "Error deserializing actions text:\n",
                actions_text,
            )
        }

        ActionLoadNameCollision(action_name: String, module_name: String) {
            description("Error loading action. An action with the same name has already been loaded from the module.")
            display(
                "{}{}{}{}{}",
                "Error loading action with name, '",
                action_name,
                "', from module, '",
                module_name,
                "'. A action with the same name has already been loaded from the module.",
            )
        }

        ActionNotFoundInList(action_id: usize) {
            description("Error getting action in action list. The action could not be found.")
            display(
                "{}{}{}",
                "Error locating action with id, '",
                action_id,
                "', in action list. The action could not be found.",
            )
        }

        ActionNotFoundInMap(module_id: usize, action_name: String) {
            description("Error getting action in action map. The action could not be found.")
            display(
                "{}{}{}{}{}",
                "Error locating action with name, '",
                action_name,
                "', for module with id, '",
                module_id,
                "', in action map. The action could not be found.",
            )
        }

        ActionTypeIsEmpty {
            description("The action type is empty.")
            display(
                "{}",
                "The action type is empty."
            )
        }

        ActionIsAlreadyParsed(action_type: NumberOrString) {
            description("The action has already been parsed.")
            display(
                "{}{:?}{}",
                "An action of type, '",
                action_type,
                "', was parsed twice. Only one parser can exist per action type."
            )
        }

        ConstructEventNotPassedLibflo {
            description("The object passed to the construct event cannot be cast into a 'Libflo'.")
            display(
                "{}",
                "The object passed to the construct event cannot be cast into an 'Libflo'."
            )
        }

        DispatchArgIsIncorrectType {
            description("The arg sent to the parse function is not a DispatchInput.")
            display(
                "{}",
                "The arg sent to the parse function is not a DispatchInput."
            )
        }

        DowncastFailure(expected_type: String) {
            description("Failed to cast into the expected type.")
            display(
                "{}{}{}",
                "Failed to cast into the expected type, '",
                expected_type,
                "'."
            )
        }

        InputDowncastFailure {
            description("Cannot dispatch action. The input cannot by downcast to the correct type.")
            display("Cannot dispatch action. The input cannot by downcast to the correct type.")
        }

        ParseArgIsIncorrectType {
            description("The arg sent to the parse function is not a ParseInput.")
            display(
                "{}",
                "The arg sent to the parse function is not a ParseInput."
            )
        }

        ParserError(errs: Vec<Error>) {
            description("One or more parsers failed executing.")
            display(
                "{}{}",
                "One or more parsers failed executing. The failures include:",
                {
                    let mut result = String::new();
                    for error in errs {
                        result = format!("{}\n{}", result, error);
                    }
                    result.to_string()
                }
            )
        }

        ParserReturnedIncorrectType(action_type: NumberOrString) {
            description("A parser returned a value of the incorrect type. A parser must return a value of type libflo_action_type::ParseOutput.")
            display(
                "{}{:?}{}",
                "A parser for type, '",
                action_type,
                "' returned a value of the incorrect type. A parser must return a value of type libflo_action_type::ParseOutput."
            )
        }

        ParserReturnedMultipleActions(action_type: NumberOrString) {
            description("Multiple parsers returned actions.")
            display(
                "{}{:?}{}",
                "Multiple parsers returned actions for the action type, '",
                action_type,
                "'."
            )
        }

        NoParserFound(action_type: NumberOrString) {
            description("No parser could be found for the action type.")
            display(
                "{}{:?}{}",
                "No parser could be found for the action type, '",
                action_type,
                "'."
            )
        }

        TypeAlreadyMapped(module: String, action: String) {
            description("Error constructing action type map, attempted to map an action twice.")
            display(
                "{}{}{}{}{}",
                "Error constructing action type map, attempted to map the action, '",
                module,
                " ",
                action,
                "', twice."
            )
        }

        TypeIsEmpty {
            description("A type string is empty. A type string must contain a module followed by an event.")
            display(
                "{}",
                "A type string is empty. A type string must contain a module followed by an event."
            )
        }

        TypeOnlyContainsOneWord(word: String) {
            description("A type string only contains one word. A type string must contain a module followed by an event.")
            display(
                "{}{}{}",
                "A type string only contains one word, '",
                word,
                "'. A type string must contain a module followed by an event."
            )
        }

        TypeContainsMoreThanTwoWords(words: String) {
            description("A type string only contains more than 2 words. A type string must contain a module followed by an event.")
            display(
                "{}{}{}",
                "A type string only contains more than 2 words, '",
                words,
                "'. A type string must contain a module followed by an event."
            )
        }
    }
}
