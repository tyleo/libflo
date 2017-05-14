use { AnyAction, ActionMapper, ActionTypeMap, ParseFnBox };
use error::*;
use libflo_std::{ ModuleMapper };
use number_or_string::NumberOrString;

pub struct ParseMap {
    type_to_delegate_map: ActionTypeMap<ParseFnBox>,
}

impl ParseMap {
    pub fn new<'a, 'b, TIter>(module_mapper: &ModuleMapper, action_mapper: &ActionMapper, iter: TIter) -> Result<Self>
        where TIter: IntoIterator<Item = (&'a str, &'b str, ParseFnBox)> {
        Ok(ParseMap { type_to_delegate_map: ActionTypeMap::new(module_mapper, action_mapper, iter)? })
    }

    pub fn parse(&self, arg: &(&NumberOrString, &str), module_mapper: &ModuleMapper, action_mapper: &ActionMapper) -> Result<Option<Box<AnyAction>>> {
        let &(action_type, action_str) = arg;

        let delegate_option = self.type_to_delegate_map.get(action_type, module_mapper, action_mapper)?;

        if let Some(delegate) = delegate_option {
            let action = delegate(action_str)?;
            Ok(Some(action))
        } else {
            Ok(None)
        }
    }
}
