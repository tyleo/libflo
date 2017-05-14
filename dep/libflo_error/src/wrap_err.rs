use std::error::Error;
use error::Result as AppscrapsResult;
use WrapError;

pub trait WrapErr<TResult> {
    fn wrap_err_to_err(self) -> AppscrapsResult<TResult>;
}

impl <TResult, TError> WrapErr<TResult> for Result<TResult, TError>
    where TError: Error + Send + 'static {
    fn wrap_err_to_err(self) -> AppscrapsResult<TResult> {
        match self  {
            Ok(ok) => Ok(ok),
            Err(error) => error.wrap_error_to_err(),
        }
    }
}
