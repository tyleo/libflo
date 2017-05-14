use libflo_error::{ Result as FuncResult };
use libflo_func::impl_fn;
use std::error::Error;

pub fn impl_receive<TErr, TFn>(arg: &str, func: TFn) -> FuncResult<()>
    where TErr: Error + Send + 'static,
          TFn: FnOnce(&str) -> Result<(), TErr> {
    impl_fn(arg, |arg| func(arg))
}
