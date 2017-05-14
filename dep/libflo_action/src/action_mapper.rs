use { ActionInfo, load };
use error::*;
use libflo_std::{ ModuleMapper, PathResolver };
use serialization::ActionMapperSerde;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ActionMapper {
    action_map: Vec<Option<HashMap<String, usize>>>,
    action_list: Vec<ActionInfo>
}

impl ActionMapper {
    pub fn new(
        action_map: Vec<Option<HashMap<String, usize>>>,
        action_list: Vec<ActionInfo>
    ) -> Self {
        ActionMapper {
            action_map: action_map,
            action_list: action_list
        }
    }

    pub fn get<TStr>(&self, module_id: usize, action_name: TStr) -> Result<usize>
        where TStr: AsRef<str> {
        let action_name = action_name.as_ref();
        match self.action_map.get(module_id) {
            Some(&Some(ref some)) => {
                match some.get(action_name) {
                    Some(some) => Ok(some.clone()),
                    None => Err(ErrorKind::ActionNotFoundInMap(module_id, action_name.to_string()).into())
                }
            },
            None | Some(&None) => Err(ErrorKind::ActionNotFoundInMap(module_id, action_name.to_string()).into()),
        }
    }

    pub fn get_info(&self, action_id: usize) -> Result<&ActionInfo> {
        match self.action_list.get(action_id) {
            Some(some) => Ok(some),
            None => Err(ErrorKind::ActionNotFoundInList(action_id).into()),
        }
    }

    pub fn get_raw_map(&self) -> &Vec<Option<HashMap<String, usize>>> {
        &self.action_map
    }

    pub fn make_serde(&self) -> ActionMapperSerde {
        ActionMapperSerde::new(&self.action_map, self.action_list.iter().map(|action_info| action_info.make_serde()).collect())
    }

    pub fn load(module_mapper: &ModuleMapper, path_resolver: &PathResolver) -> Result<ActionMapper> {
        load::load(module_mapper, path_resolver)
    }
}
