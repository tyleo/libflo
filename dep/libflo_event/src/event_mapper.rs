use error::*;
use event::Event;
use libflo_func::FuncMapper;
use libflo_module::{ ModuleMapper, PathResolver };
use load;
use std::collections::HashMap;

pub struct EventMapper {
    event_map: Vec<Option<HashMap<String, usize>>>,
    event_list: Vec<Event>,
}

impl EventMapper {
    pub fn new(
        event_map: Vec<Option<HashMap<String, usize>>>,
        event_list: Vec<Event>,
    ) -> Self {
        EventMapper {
            event_map: event_map,
            event_list: event_list,
        }
    }

    pub fn get(&self, event_id: usize) -> Result<&Event> {
        match self.event_list.get(event_id) {
            Some(some) => Ok(some),
            None => Err(ErrorKind::EventNotFoundInList(event_id, self.event_list.len()).into())
        }
    }

    pub fn get_id<TStr>(&self, module_id: usize, event_name: TStr) -> Result<usize>
        where TStr: AsRef<str> {
        let event_name = event_name.as_ref();
        match self.event_map.get(module_id) {
            Some(&Some(ref some)) => {
                match some.get(event_name) {
                    Some(some) => Ok(some.clone()),
                    None => Err(ErrorKind::EventNotFoundInMap(module_id, event_name.to_string()).into())
                }
            },
            None | Some(&None) => Err(ErrorKind::EventNotFoundInMap(module_id, event_name.to_string()).into())
        }
    }

    pub fn get_raw_list(&self) -> &Vec<Event> {
        &self.event_list
    }

    pub fn get_raw_map(&self) -> &Vec<Option<HashMap<String, usize>>> {
        &self.event_map
    }

    pub fn load(
        func_mapper: &FuncMapper,
        module_mapper: &ModuleMapper,
        path_resolver: &PathResolver
    ) -> Result<EventMapper> {
        load::load(func_mapper, module_mapper, path_resolver)
    }
}
