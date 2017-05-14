use { ActionMapper, string };
use error::*;
use libflo_std::{ Input, Libflo };
use std::sync::Arc;

pub unsafe fn construct(libflo: &Libflo, arg: &Arc<ActionMapper>) -> Result<()> {
    let event_mapper = libflo.get_event_mapper();
    let construct_event = event_mapper.get_by_module_name(string::module(), string::construct_event())?;
    construct_event.call(Input::Any(arg))?;
    Ok(())
}
