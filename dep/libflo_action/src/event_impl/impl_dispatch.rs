use AnyAction;
use libflo_std::{ impl_any_input_fn, Result as FuncResult };
use std::any::Any;
use std::error::Error;

pub fn impl_dispatch<TErr, TFn>(arg: &Any, func: TFn) -> FuncResult<()>
    where TErr: Error + Send + 'static,
          TFn: FnOnce(&&AnyAction) -> Result<(), TErr> {
    impl_any_input_fn(arg, |arg| func(arg))
}
