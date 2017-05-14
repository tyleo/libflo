use AnyAction;
use libflo_std::{ Result };

pub type ParseFnBox = Box<Fn(&str) -> Result<Box<AnyAction>> + Send + Sync>;
