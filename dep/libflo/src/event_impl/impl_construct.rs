use Libflo;
use libflo_error::{ Result as FuncResult };
use libflo_func::{ impl_any_input_fn };
use std::any::Any;
use std::error::Error;
use std::sync::Arc;

pub fn impl_construct<TErr, TFn>(arg: &Any, func: TFn) -> FuncResult<()>
    where TErr: Error + Send + 'static,
          TFn: FnOnce(&Arc<Libflo>) -> Result<(), TErr> {
    impl_any_input_fn(arg, func)
}
