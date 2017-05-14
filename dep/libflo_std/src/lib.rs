#[macro_use]
extern crate lazy_static;
pub extern crate libflo;
extern crate libflo_error;
extern crate libflo_event;
extern crate libflo_func;
extern crate libflo_module;
pub extern crate mut_static;

pub mod error {
    pub use libflo_error::*;
}

pub mod event {
    pub use libflo_event::*;
}

pub mod func {
    pub use libflo_func::*;
}

pub mod module {
    pub use libflo_module::*;
}

pub mod serialization {
    pub use libflo_module::serialization::ModuleMapperSerde;
}

pub use error::{ Result, WrapErr, WrapError };
pub use event::{ Event, EventMapper };
pub use func::{ Func, FuncMapper, Input, impl_any_input_fn, impl_any_input_any_output_fn, impl_any_output_fn, impl_fn, Output, Parameter };
pub use libflo::{ ExtEventMapper, ExtFuncMapper, impl_construct, impl_post_construct, impl_quit, impl_receive, impl_send, Libflo, string };
pub use module::{ file_from_path, ModuleMapper, PathResolver, text_from_file };
pub use mut_static::MutStatic;
pub use serialization::*;

use std::sync::Arc;

lazy_static! {
    pub static ref LIBFLO: MutStatic<Arc<Libflo>> = {
        MutStatic::new()
    };
}
