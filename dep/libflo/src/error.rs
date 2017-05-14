use { libflo_event, libflo_func, libflo_module };
use std::io;

error_chain! {
    types { }

    links {
        LibfloEventError(libflo_event::Error, libflo_event::ErrorKind);
        LibfloFuncError(libflo_func::Error, libflo_func::ErrorKind);
        LibfloModuleError(libflo_module::Error, libflo_module::ErrorKind);
    }

    foreign_links {
        IoError(io::Error);
    }

    errors {
        DefaultErrLockError(error_text: String) {
            description("Error locking default_err.")
            display(
                "{}{}",
                "Error locking default_err: ",
                error_text
            )
        }

        DefaultSendLockError(error_text: String) {
            description("Error locking default_send.")
            display(
                "{}{}",
                "Error locking default_send: ",
                error_text
            )
        }

        IsRunningLockError(error_text: String) {
            description("Error locking is_running.")
            display(
                "{}{}",
                "Error locking is_running: ",
                error_text
            )
        }
    }
}
