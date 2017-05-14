use libflo_error::{ Result as FuncResult, WrapErr };
use std::error::Error as StdError;
use std::result::Result as StdResult;

pub fn impl_fn<TIn, TOut, TErr, TFn>(arg: TIn, func: TFn) -> FuncResult<TOut>
    where TErr: StdError + Send + 'static,
          TFn: FnOnce(TIn) -> StdResult<TOut, TErr> {
    func(arg).wrap_err_to_err()
}
