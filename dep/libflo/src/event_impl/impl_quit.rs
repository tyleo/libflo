use libflo_error::{ Result as FuncResult };
use libflo_func::impl_fn;
use std::error::Error;

pub fn impl_quit<TErr, TFn>(func: TFn) -> FuncResult<()>
    where TErr: Error + Send + 'static,
          TFn: FnOnce() -> Result<(), TErr> {
    impl_fn((), |_| func())
}
