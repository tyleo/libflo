use error::*;
use func::Func;
use libflo_module::{ ModuleMapper, PathResolver };
use load;
use std::collections::HashMap;

pub struct FuncMapper {
    func_map: Vec<Option<HashMap<String, usize>>>,
    func_list: Vec<Box<Func>>,
}

impl FuncMapper {
    pub fn new(
        func_map: Vec<Option<HashMap<String, usize>>>,
        func_list: Vec<Box<Func>>
    ) -> Self {
        FuncMapper {
            func_map: func_map,
            func_list: func_list,
        }
    }

    pub fn get(&self, func_id: usize) -> Result<&Func> {
        match self.func_list.get(func_id) {
            Some(some) => Ok(some.as_ref()),
            None => Err(ErrorKind::EventNotFoundInEventList(func_id, self.func_list.len()).into())
        }
    }

    pub fn get_id<TStr>(&self, module_id: usize, func_name: TStr) -> Result<usize>
        where TStr: AsRef<str> {
        let func_name = func_name.as_ref();
        match self.func_map.get(module_id) {
            Some(&Some(ref some)) => {
                match some.get(func_name) {
                    Some(some) => Ok(some.clone()),
                    None => Err(ErrorKind::FuncNotFoundInFuncMap(module_id, func_name.to_string()).into())
                }
            },
            None | Some(&None) => Err(ErrorKind::FuncNotFoundInFuncMap(module_id, func_name.to_string()).into())
        }
    }

    pub fn get_raw_list(&self) -> &Vec<Box<Func>> {
        &self.func_list
    }

    pub fn get_raw_map(&self) -> &Vec<Option<HashMap<String, usize>>> {
        &self.func_map
    }

    pub unsafe fn load(module_mapper: &ModuleMapper, path_resolver: &PathResolver) -> Result<FuncMapper> {
        load::load(module_mapper, path_resolver)
    }
}
