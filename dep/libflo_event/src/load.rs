use { EventMapper, string };
use error::*;
use event::Event;
use event_handler_map::EventHandlerMap;
use file_funcs;
use libflo_func::{ FuncMapper, Parameter, Result as FuncResult };
use libflo_func::serialization::ParameterSerde;
use libflo_module::{ ModuleMapper, PathResolver, self_module_name };
use serialization::{ EventHandlersSerde, EventsSerde };
use std::collections::HashMap;

pub fn load(
    func_mapper: &FuncMapper,
    module_mapper: &ModuleMapper,
    path_resolver: &PathResolver
) -> Result<EventMapper> {
    let event_handlers = load_event_handlers(
        func_mapper,
        module_mapper,
        path_resolver
    )?;

    load_events(
        module_mapper,
        path_resolver,
        event_handlers
    )
}

fn create_event_handlers(
    func_mapper: &FuncMapper,
    module_mapper: &ModuleMapper,
    event_handlers_json: EventHandlersSerde,
    event_handler_map: &mut EventHandlerMap
) -> Result<()> {
    for event_handler in event_handlers_json.destructure() {
        let (event, function, functions) = event_handler.destructure();
        let (handler_module_name, event_name) = event.destructure();
        let handler_module_id = module_mapper.get(handler_module_name)?;

        if let Some(function) = function {
            if let Some(_) = functions {
                return Err(ErrorKind::FunctionAndFunctionsBothDefined.into());
            } else {
                let (function_module_name, function_name) = function.destructure();
                let function_module_id = module_mapper.get(function_module_name)?;
                let function_id = func_mapper.get_id(function_module_id, function_name)?;
                let function = func_mapper.get(function_id)?;

                event_handler_map.add_function(handler_module_id, event_name.clone(), function.clone_into_box())?;
            }
        } else if let Some(functions) = functions {
            for function in functions {
                let (function_module_name, function_name) = function.destructure();
                let function_module_id = module_mapper.get(function_module_name)?;
                let function_id = func_mapper.get_id(function_module_id, function_name)?;
                let function = func_mapper.get(function_id)?;

                event_handler_map.add_function(handler_module_id, event_name.clone(), function.clone_into_box())?;
            }
        } else {
            return Err(ErrorKind::FunctionOrFunctionsNotDefined.into());
        }
    }

    Ok(())
}

fn create_events(
    module_name: &String,
    module_id: usize,
    events_json: EventsSerde,
    event_handlers: &mut EventHandlerMap,
    pre_event_list: &mut Vec<Event>,
    pre_event_map: &mut Vec<Option<HashMap<String, usize>>>,
) -> Result<()> {
    for event in events_json.destructure() {
        let (input, output, name) = event.destructure();

        let input = input.unwrap_or(ParameterSerde::None);
        let output = output.unwrap_or(ParameterSerde::None);
        let input =  FuncResult::<Parameter>::from(&input)?;
        let output =  FuncResult::<Parameter>::from(&output)?;
        let event_id = pre_event_list.len();

        if let Some(&None) = pre_event_map.get(module_id) {
            pre_event_map[module_id] = Some(HashMap::new());
        }

        if let Some(&mut Some(ref mut inner_event_map)) = pre_event_map.get_mut(module_id) {
            if let Some(_) = inner_event_map.insert(name.clone(), event_id) {
                return Err(ErrorKind::EventLoadNameCollision(name, module_name.clone()).into());
            }
        }

        let functions = event_handlers.take_functions(module_id, name);
        for function in &functions {
            let func_type = function.get_type();
            let event_type = (input, output);
            if func_type != event_type {
                return Err(ErrorKind::EventTypeFailure(func_type, event_type).into());
            }
        }

        let event = Event::new(functions);
        pre_event_list.push(event);
    }

    Ok(())
}

fn load_event_handlers(
    func_mapper: &FuncMapper,
    module_mapper: &ModuleMapper,
    path_resolver: &PathResolver,
) -> Result<EventHandlerMap> {
    let my_id = module_mapper.get(self_module_name())?;
    let event_handlers_file_name = string::event_handlers_file_name();

    let mut event_handlers = EventHandlerMap::new();

    for (_, module_id) in module_mapper.get_raw_map() {
        if let Some(event_handlers_json_path) = path_resolver.try_find_submodule_file_path(
            event_handlers_file_name,
            *module_id,
            my_id
        )? {
            let event_handlers_json = file_funcs::event_handlers_from_path(event_handlers_json_path)?;
            create_event_handlers(
                func_mapper,
                module_mapper,
                event_handlers_json,
                &mut event_handlers
            )?
        }
    }

    Ok(event_handlers)
}

fn load_events(
    module_mapper: &ModuleMapper,
    path_resolver: &PathResolver,
    mut event_handlers: EventHandlerMap
) -> Result<EventMapper> {
    let my_id = module_mapper.get(self_module_name())?;
    let events_file_name = string::events_file_name();

    let mut pre_event_list = Vec::new();
    let mut pre_event_map = module_mapper.get_raw_map().into_iter().map(|_| None).collect();

    for (module_name, module_id) in module_mapper.get_raw_map() {
        if let Some(events_json_path) = path_resolver.try_find_submodule_file_path(
            events_file_name,
            *module_id,
            my_id
        )? {
            let events_json = file_funcs::events_from_path(events_json_path)?;
            create_events(module_name, *module_id, events_json, &mut event_handlers, &mut pre_event_list, &mut pre_event_map)?;
        }
    }

    let event_mapper = EventMapper::new(pre_event_map, pre_event_list);
    Ok(event_mapper)
}
