#![recursion_limit="200"]

#[macro_use]
extern crate error_chain;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod error;
mod file_funcs;
mod file_from_path;
mod load;
mod module_mapper;
mod path_resolver;
mod payload;
pub mod serialization;
mod string;
#[cfg(test)]
mod test;
mod text_from_file;

pub use error::*;
pub use file_from_path::*;
pub use load::*;
pub use module_mapper::*;
pub use path_resolver::*;
pub use payload::*;
pub use string::self_module_name;
pub use text_from_file::*;
