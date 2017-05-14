#![recursion_limit="200"]

#[macro_use]
extern crate error_chain;
extern crate libflo_std;

pub mod error;
mod program;
mod root_module_directory;
mod string;
#[cfg(test)]
mod test;


pub use error::*;
use libflo_std::Libflo;
use program::Program;
use std::io;

fn unwrap<T>(result: Result<T>) -> T {
    match result {
        Err(err) => {
            let mut msg = "Fatal Error:\n".to_string();
            msg = format!("{}Error Debug:\n{:?}\nError Display(0):\n{}\n", msg, err, err);

            let mut i = 1;
            for err in err.iter().skip(1) {
                msg = format!("{}Error Display({}):\n{}\n", msg, i, err);
                i += 1;
            }
            msg = format!("{}Backtrace: {:?}\n", msg, err.backtrace());
            panic!(msg);
        },
        Ok(ok) => ok,
    }
}

fn main() {
    let root_module_directory = unwrap(root_module_directory::root_module_directory());
    let libflo = unwrap(
        unsafe {
            Libflo::load(root_module_directory, None, None, Some(Box::new(io::stderr())), Some(Box::new(io::stdout()))).map_err(|err| Error::from(err))
        }
    );

    let program = Program::new(
        io::stdin(),
        libflo
    );
    unwrap( unsafe { program.run() });
}
