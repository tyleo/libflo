use error::*;
use libflo_error::{ Result as FuncResult, WrapErr };
use std::any::Any;
use std::error::Error as StdError;
use std::result::Result as StdResult;

pub fn impl_any_input_any_output_fn<TIn, TOut, TErr, TFn>(arg: &Any, func: TFn) -> FuncResult<Box<Any>>
    where TIn: Any,
          TOut: Any,
          TErr: StdError + Send + 'static,
          TFn: FnOnce(&TIn) -> StdResult<TOut, TErr> {
    let input = arg
        .downcast_ref()
        .ok_or(Error::from(ErrorKind::InputDowncastFailure))
        .wrap_err_to_err()?;
    let result = func(input).wrap_err_to_err()?;
    Ok(Box::new(result))
}
