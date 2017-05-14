use libflo_error::{ Result as FuncResult, WrapErr };
use std::any::Any;
use std::error::Error as StdError;
use std::result::Result as StdResult;

pub fn impl_any_output_fn<TIn, TOut, TErr, TFn>(arg: TIn, func: TFn) -> FuncResult<Box<Any>>
    where TOut: Any,
          TErr: StdError + Send + 'static,
          TFn: FnOnce(TIn) -> StdResult<TOut, TErr> {
    let result = func(arg).wrap_err_to_err()?;
    Ok(Box::new(result))
}
