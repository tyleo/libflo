use error::*;
use libflo_func::Func;
use std::collections::BTreeMap;

pub struct EventHandlerMap {
    module_and_event_to_funcs: BTreeMap<(usize, String), Vec<Box<Func>>>,
}

impl EventHandlerMap {
    pub fn new() -> Self {
        EventHandlerMap {
            module_and_event_to_funcs: BTreeMap::new(),
        }
    }

    pub fn add_function(&mut self, module: usize, event: String, function: Box<Func>) -> Result<()> {
        let key = (module, event);
        if !self.module_and_event_to_funcs.contains_key(&key) {
            self.module_and_event_to_funcs.insert(key.clone(), Vec::new());
        }

        if let Some(ref mut function_list) = self.module_and_event_to_funcs.get_mut(&key) {
            function_list.push(function);
            Ok(())
        } else {
            Err(ErrorKind::FailureFindingEventHandler(key.0, key.1).into())
        }
    }

    pub fn take_functions(&mut self, module: usize, event: String) -> Vec<Box<Func>> {
        let key = (module, event);
        self.module_and_event_to_funcs.remove(&key).unwrap_or(Vec::new())
    }
}
