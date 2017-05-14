use { AnyAction, string };
use error::*;
use libflo_std::{ Input, Libflo };
use std::mem;

pub unsafe fn dispatch(libflo: &Libflo, arg: &AnyAction) -> Result<()> {
    let event_mapper = libflo.get_event_mapper();
    let dispatch_event = event_mapper.get_by_module_name(string::module(), string::dispatch_event())?;

    let static_arg: &'static AnyAction = mem::transmute(arg);
    dispatch_event.call(Input::Any(&static_arg))?;
    Ok(())
}
