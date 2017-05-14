use { AnyAction, ParseFnBox };
use libflo_std::WrapErr;
use std::error::Error;

pub fn parse_fn<TOut, TErr, TFn>(func: TFn) -> ParseFnBox
    where TOut: AnyAction,
          TErr: Error + Send + 'static,
          TFn: Fn(&str) -> Result<TOut, TErr> + Send + Sync + 'static {
    Box::new(
        move |arg| {
            let action = func(arg).wrap_err_to_err()?;
            Ok(Box::new(action))
        }
    )
}
