use libflo_std::{ libflo, module, mut_static };
use serde_json;
use std::num;
use std::io;

error_chain! {
    types { }

    links {
        LibfloError(libflo::Error, libflo::ErrorKind);
        LibfloModuleError(module::Error, module::ErrorKind);
        MutStaticError(mut_static::Error, mut_static::ErrorKind);
    }

    foreign_links {
        IoError(io::Error);
        ParseIntError(num::ParseIntError);
        SerdeJsonError(serde_json::Error);
    }

    errors {
        ConstructEventNotPassedLibflo {
            description("The object passed to the construct event cannot be cast into a 'Libflo'.")
            display(
                "{}",
                "The object passed to the construct event cannot be cast into an 'Libflo'."
            )
        }
    }
}
