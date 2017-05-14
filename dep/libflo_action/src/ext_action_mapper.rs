use { ActionInfo, ActionMapper };
use error::*;
use libflo_std::ModuleMapper;
use number_or_string::NumberOrString;
use serialization::ActionMapperSerde;
use std::collections::HashMap;

pub struct ExtActionMapper<'a, 'b> {
    action_mapper: &'a ActionMapper,
    module_mapper: &'b ModuleMapper,
}

impl <'a, 'b> ExtActionMapper<'a, 'b> {
    pub fn new(
        action_mapper: &'a ActionMapper,
        module_mapper: &'b ModuleMapper,
    ) -> Self {
        ExtActionMapper {
            action_mapper: action_mapper,
            module_mapper: module_mapper
        }
    }

    pub fn get<TStr: AsRef<str>>(&self, module_id: usize, action_name: TStr) -> Result<usize> {
        self.action_mapper.get(module_id, action_name).map_err(|err| err.into())
    }

    pub fn get_by_module_name<TStr0: AsRef<str>, TStr1: AsRef<str>>(&self, module_name: TStr0, action_name: TStr1) -> Result<usize> {
        let module_id = self.module_mapper.get(module_name)?;
        self.get(module_id, action_name)
    }

    pub fn get_info(&self, action_id: usize) -> Result<&ActionInfo> {
        self.action_mapper.get_info(action_id)
    }


    pub fn get_raw_map(&self) -> &Vec<Option<HashMap<String, usize>>> {
        self.action_mapper.get_raw_map()
    }

    pub fn get_type_string(&self, action_type: &NumberOrString) -> Result<String> {
        match action_type {
            &NumberOrString::String(ref result) => Ok(result.clone()),
            &NumberOrString::Number(action_id) => {
                let action_info = self.get_info(action_id as usize)?;
                let module_name = self.module_mapper.get_name(action_info.get_module_id())?;
                Ok(format!("{} {}", module_name, action_info.get_action_name()))
            }
        }
    }

    pub fn make_serde(&self) -> ActionMapperSerde {
        self.action_mapper.make_serde()
    }
}
