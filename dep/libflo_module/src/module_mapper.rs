use error::*;
use serialization::ModuleMapperSerde;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct ModuleMapper {
    module_map: HashMap<String, usize>,
    module_list: Vec<String>
}

impl ModuleMapper {
    pub fn new(module_map: HashMap<String, usize>, module_list: Vec<String>) -> Self {
        ModuleMapper {
            module_map: module_map,
            module_list: module_list
        }
    }

    pub fn get<TName>(&self, module_name: TName) -> Result<usize>
        where TName: AsRef<str> {
        let module_name = module_name.as_ref();
        match self.module_map.get(module_name) {
            Some(some) => Ok(some.clone()),
            None => {
                Err(ErrorKind::ModuleNotFoundInModuleMap(module_name.to_string()).into())
            },
        }
    }

    pub fn get_name(&self, module_id: usize) -> Result<&String> {
        match self.module_list.get(module_id) {
            Some(some) => Ok(some),
            None => Err(ErrorKind::ModuleNotFoundInList(module_id).into()),
        }
    }

    pub fn get_raw_map(&self) -> &HashMap<String, usize> {
        &self.module_map
    }

    pub fn make_serde(&self) -> ModuleMapperSerde {
        ModuleMapperSerde::new(&self.module_map, &self.module_list)
    }
}
