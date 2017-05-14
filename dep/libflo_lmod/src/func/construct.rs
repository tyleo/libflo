use libflo_std::{ impl_construct, LIBFLO, Result as FuncResult };
use std::any::Any;

#[no_mangle]
pub extern fn construct(arg: &Any) -> FuncResult<()> {
    impl_construct(arg, |arg| LIBFLO.set(arg.clone()))
}
