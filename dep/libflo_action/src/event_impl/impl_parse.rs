use AnyAction;
use libflo_std::{ impl_any_input_any_output_fn, Result as FuncResult };
use number_or_string::NumberOrString;
use std::any::Any;
use std::error::Error;

pub fn impl_parse<TErr, TFn>(arg: &Any, func: TFn) -> FuncResult<Box<Any>>
    where TErr: Error + Send + 'static,
          TFn: FnOnce(&(&NumberOrString, &str)) -> Result<Option<Box<AnyAction>>, TErr> {
    impl_any_input_any_output_fn(arg, |arg| func(arg))
}
