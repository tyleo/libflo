use std::error::Error;
use error::Error as AppscrapsError;
use error::ErrorKind as AppscrapsErrorKind;
use error::Result as AppscrapsResult;

pub trait WrapError {
    fn wrap_error_to_err<TResult>(self) -> AppscrapsResult<TResult>;

    fn wrap_error_to_error(self) -> AppscrapsError;
}

impl <TError> WrapError for TError
    where TError: Error + Send + 'static {
    fn wrap_error_to_err<TResult>(self) -> AppscrapsResult<TResult> {
        Err(self.wrap_error_to_error())
    }

    fn wrap_error_to_error(self) -> AppscrapsError {
        let err = Box::new(self);
        AppscrapsErrorKind::ErrorWrapper(err).into()
    }
}
