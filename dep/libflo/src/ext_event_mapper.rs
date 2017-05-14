use libflo_event::{ Event, EventMapper };
use libflo_module::ModuleMapper;
use error::*;
use std::collections::HashMap;

pub struct ExtEventMapper<'a, 'b> {
    event_mapper: &'a EventMapper,
    module_mapper: &'b ModuleMapper,
}

impl <'a, 'b> ExtEventMapper<'a, 'b> {
    pub fn new(
        event_mapper: &'a EventMapper,
        module_mapper: &'b ModuleMapper,
    ) -> Self {
        ExtEventMapper {
            event_mapper: event_mapper,
            module_mapper: module_mapper,
        }
    }

    pub fn get(&self, event_id: usize) -> Result<&Event> {
        self.event_mapper.get(event_id).map_err(|err| err.into())
    }

    pub fn get_by_module_name<TStr0: AsRef<str>, TStr1: AsRef<str>>(&self, module_name: TStr0, event_name: TStr1) -> Result<&Event> {
        let module_id = self.module_mapper.get(module_name)?;
        self.get_by_name(module_id, event_name)
    }

    pub fn get_by_name<TStr: AsRef<str>>(&self, module_id: usize, event_name: TStr) -> Result<&Event> {
        let event_id = self.get_id(module_id, event_name)?;
        self.get(event_id)
    }

    pub fn get_id<TStr: AsRef<str>>(&self, module_id: usize, event_name: TStr) -> Result<usize> {
        self.event_mapper.get_id(module_id, event_name).map_err(|err| err.into())
    }

    pub fn get_id_by_module_name<TStr0: AsRef<str>, TStr1: AsRef<str>>(&self, module_name: TStr0, event_name: TStr1) -> Result<usize> {
        let module_id = self.module_mapper.get(module_name)?;
        self.get_id(module_id, event_name)
    }

    pub fn get_raw_list(&self) -> &Vec<Event> {
        self.event_mapper.get_raw_list()
    }

    pub fn get_raw_map(&self) -> &Vec<Option<HashMap<String, usize>>> {
        self.event_mapper.get_raw_map()
    }
}
