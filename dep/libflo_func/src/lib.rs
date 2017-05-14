#![recursion_limit="200"]

#[macro_use]
extern crate error_chain;
extern crate libflo_error;
extern crate libflo_module;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate sharedlib;

#[macro_use]
mod register_funcs;
pub mod error;
mod file_funcs;
mod func;
mod func_mapper;
mod impl_any_input_fn;
mod impl_any_input_any_output_fn;
mod impl_any_output_fn;
mod impl_fn;
mod load;
pub mod serialization;
mod string;
#[cfg(test)]
mod test;

pub use error::*;
pub use func::*;
pub use func_mapper::*;
pub use impl_any_input_fn::*;
pub use impl_any_input_any_output_fn::*;
pub use impl_any_output_fn::*;
pub use impl_fn::*;
