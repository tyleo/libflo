use { AnyAction, DispatchFnBox };
use error::*;
use libflo_std::WrapErr;
use std::error::Error as StdError;
use std::result::Result as StdResult;

pub fn dispatch_fn<TIn, TErr, TFn>(func: TFn) -> DispatchFnBox
    where TIn: AnyAction,
          TErr: StdError + Send + 'static,
          TFn: Fn(&TIn) -> StdResult<(), TErr> + Send + Sync + 'static {
    Box::new(
        move |arg| {
            let input = arg
                .as_any()
                .downcast_ref()
                .ok_or(Error::from(ErrorKind::InputDowncastFailure))
                .wrap_err_to_err()?;
            func(input).wrap_err_to_err()
        }
    )
}
