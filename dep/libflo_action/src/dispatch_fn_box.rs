use AnyAction;
use libflo_std::Result;

pub type DispatchFnBox = Box<Fn(&AnyAction) -> Result<()> + Send + Sync>;
