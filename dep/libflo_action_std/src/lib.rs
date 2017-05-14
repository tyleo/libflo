#[macro_use]
extern crate lazy_static;
pub extern crate libflo_action;
extern crate mut_static;
extern crate number_or_string;

pub mod error {
    pub use libflo_action::error::*;
}

pub mod serialization {
    pub use libflo_action::serialization::*;

    pub use number_or_string::*;
}

pub use error::*;
pub use libflo_action::{ Action, ActionInfo, ActionMapper, action_str_id, ActionTypeMap, AnyAction, BasicAction, construct, dispatch, dispatch_fn, DispatchFnBox, DispatchMap, ExtActionMapper, impl_construct, impl_dispatch, impl_parse, parse, parse_fn, ParseFnBox, ParseMap, string };
pub use serialization::*;

use mut_static::MutStatic;
use std::sync::Arc;

lazy_static! {
    pub static ref ACTION_MAPPER: MutStatic<Arc<ActionMapper>> = {
        MutStatic::new()
    };
}

lazy_static! {
    pub static ref DISPATCH_MAP: MutStatic<DispatchMap> = {
        MutStatic::new()
    };
}

lazy_static! {
    pub static ref PARSE_MAP: MutStatic<ParseMap> = {
        MutStatic::new()
    };
}
