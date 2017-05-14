use { ActionMapper, ActionTypeMap, AnyAction, DispatchFnBox };
use error::*;
use libflo_std::ModuleMapper;

pub struct DispatchMap {
    type_to_delegate_map: ActionTypeMap<DispatchFnBox>,
}

impl DispatchMap {
    pub fn new<'a, 'b, TIter>(module_mapper: &ModuleMapper, action_mapper: &ActionMapper, iter: TIter) -> Result<Self>
        where TIter: IntoIterator<Item = (&'a str, &'b str, DispatchFnBox)> {
        Ok(DispatchMap { type_to_delegate_map: ActionTypeMap::new(module_mapper, action_mapper, iter)? })
    }

    pub fn dispatch(&self, arg: &&AnyAction, module_mapper: &ModuleMapper, action_mapper: &ActionMapper) -> Result<()> {
        let action_type = arg.get_type();
        let action = *arg;

        let delegate_option = self.type_to_delegate_map.get(action_type, module_mapper, action_mapper)?;

        if let Some(delegate) = delegate_option {
            delegate(action)?;
        }

        Ok(())
    }
}
