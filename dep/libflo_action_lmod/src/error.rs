use libflo_std::{ event, libflo, module, mut_static };
use libflo_action_std::{ libflo_action, NumberOrString };
use serde_json;

error_chain! {
    types { }

    links {
        LibfloActionError(libflo_action::Error, libflo_action::ErrorKind);
        LibfloError(libflo::Error, libflo::ErrorKind);
        LibfloEventError(event::Error, event::ErrorKind);
        LibfloModuleError(module::Error, module::ErrorKind);
        MutStaticError(mut_static::Error, mut_static::ErrorKind);
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

        ParserReturnedIncorrectType(action_type: NumberOrString) {
            description("A parser returned a value of the incorrect type. A parser must return a value of type libflo_action_type::ParseOutput.")
            display(
                "{}{:?}{}",
                "A parser for type, '",
                action_type,
                "' returned a value of the incorrect type. A parser must return a value of type libflo_action_type::ParseOutput."
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
    }
}
