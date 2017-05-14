use ActionMapper;
use libflo_std::{ impl_any_input_fn, Result as FuncResult };
use std::any::Any;
use std::error::Error;
use std::sync::Arc;

pub fn impl_construct<TErr, TFn>(arg: &Any, func: TFn) -> FuncResult<()>
    where TErr: Error + Send + 'static,
          TFn: FnOnce(&Arc<ActionMapper>) -> Result<(), TErr> {
    impl_any_input_fn(arg, func)
}
