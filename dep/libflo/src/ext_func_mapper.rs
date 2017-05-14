use error::*;
use libflo_func::{ Func, FuncMapper };
use libflo_module::ModuleMapper;
use std::collections::HashMap;

pub struct ExtFuncMapper<'a, 'b> {
    func_mapper: &'a FuncMapper,
    module_mapper: &'b ModuleMapper,
}

impl <'a, 'b> ExtFuncMapper<'a, 'b> {
    pub fn new(
        func_mapper: &'a FuncMapper,
        module_mapper: &'b ModuleMapper,
    ) -> Self {
        ExtFuncMapper {
            func_mapper: func_mapper,
            module_mapper: module_mapper,
        }
    }

    pub fn get(&self, func_id: usize) -> Result<&Func> {
        self.func_mapper.get(func_id).map_err(|err| err.into())
    }

    pub fn get_by_module_name<TStr0: AsRef<str>, TStr1: AsRef<str>>(&self, module_name: TStr0, func_name: TStr1) -> Result<&Func> {
        let module_id = self.module_mapper.get(module_name)?;
        self.get_by_name(module_id, func_name)
    }

    pub fn get_by_name<TStr: AsRef<str>>(&self, module_id: usize, func_name: TStr) -> Result<&Func> {
        let func_id = self.get_id(module_id, func_name)?;
        self.get(func_id)
    }

    pub fn get_id<TStr: AsRef<str>>(&self, module_id: usize, func_name: TStr) -> Result<usize> {
        self.func_mapper.get_id(module_id, func_name).map_err(|err| err.into())
    }

    pub fn get_id_by_module_name<TStr0: AsRef<str>, TStr1: AsRef<str>>(&self, module_name: TStr0, func_name: TStr1) -> Result<usize> {
        let module_id = self.module_mapper.get(module_name)?;
        self.get_id(module_id, func_name)
    }

    pub fn get_raw_list(&self) -> &Vec<Box<Func>> {
        self.func_mapper.get_raw_list()
    }

    pub fn get_raw_map(&self) -> &Vec<Option<HashMap<String, usize>>> {
        self.func_mapper.get_raw_map()
    }
}
