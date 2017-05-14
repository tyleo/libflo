use error::*;
use libflo_error::{ Result as FuncResult, WrapErr };
use std::any::Any;
use std::error::Error as StdError;
use std::result::Result as StdResult;

pub fn impl_any_input_fn<TIn, TOut, TErr, TFn>(arg: &Any, func: TFn) -> FuncResult<TOut>
    where TIn: Any,
          TErr: StdError + Send + 'static,
          TFn: FnOnce(&TIn) -> StdResult<TOut, TErr> {
    let input = arg
        .downcast_ref()
        .ok_or(Error::from(ErrorKind::InputDowncastFailure))
        .wrap_err_to_err()?;
    func(input).wrap_err_to_err()
}
