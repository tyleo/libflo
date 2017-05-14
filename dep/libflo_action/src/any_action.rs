use Action;
use std::any::Any;

pub trait AnyAction : Action + Any {
    fn as_any(&self) -> &Any;
}

impl <T> AnyAction for T
    where T: Any + Action {
    fn as_any(&self) -> &Any {
        self as &Any
    }
}
