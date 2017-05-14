use std::error::Error as StdError;

error_chain! {
    types { }

    links { }

    foreign_links { }

    errors {
        ErrorWrapper(cause: Box<StdError + Send>) {
            description(cause.description())
            display("{}", cause)
        }
    }
}
