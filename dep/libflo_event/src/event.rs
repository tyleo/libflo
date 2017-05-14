use error::*;
use libflo_func::{ Func, Input, Output };

pub struct Event {
    event_handlers: Vec<Box<Func>>,
}

impl Event {
    pub fn new(event_handlers: Vec<Box<Func>>) -> Self {
        Event { event_handlers: event_handlers }
    }

    pub unsafe fn call(&self, input: Input) -> Result<Vec<Output>> {
        let event_handlers = &self.event_handlers;

        let mut oks = Vec::new();
        let mut errs = Vec::new();

        for result in {
            event_handlers
                .into_iter()
                .map(|event_handler| { event_handler.call(input.clone()) })
        } {
            match result {
                Ok(ok) => oks.push(ok),
                Err(err) => errs.push(err),
            }
        }

        if errs.len() == 0 {
            Ok(oks)
        } else {
            Err(ErrorKind::EventError(errs).into())
        }

    }
}
