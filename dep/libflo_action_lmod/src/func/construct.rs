use error::*;
use libflo_action_std::{ ActionMapper, construct as construct_event };
use libflo_std::{ impl_construct, LIBFLO, Result as FuncResult };
use std::any::Any;
use std::sync::Arc;

#[no_mangle]
pub unsafe extern fn construct(arg: &Any) -> FuncResult<()> {
    impl_construct(
        arg,
        |arg| -> Result<_> {
            LIBFLO.set(arg.clone())?;

            let action_mapper = Arc::new(ActionMapper::load(arg.get_module_mapper(), arg.get_path_resolver())?);
            Ok(construct_event(arg, &action_mapper)?)
        }
    )
}
