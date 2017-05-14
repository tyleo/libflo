use { ActionInfo, ActionMapper, file_funcs, string };
use error::*;
use libflo_std::{ ModuleMapper, PathResolver };
use serialization::ActionsSerde;
use std::collections::HashMap;

pub fn load(
    module_mapper: &ModuleMapper,
    path_resolver: &PathResolver
) -> Result<ActionMapper> {
    load_events(
        module_mapper,
        path_resolver,
    )
}

fn create_actions(
    module_name: &String,
    module_id: usize,
    actions_json: ActionsSerde,
    pre_action_map: &mut Vec<Option<HashMap<String, usize>>>,
    pre_action_list: &mut Vec<ActionInfo>,
    current_id: &mut usize,
) -> Result<()> {
    for action in actions_json.destructure() {
        let action_id = *current_id;
        *current_id = *current_id + 1;

        if let Some(&None) = pre_action_map.get(module_id) {
            pre_action_map[module_id] = Some(HashMap::new());
        }

        if let Some(&mut Some(ref mut inner_action_map)) = pre_action_map.get_mut(module_id) {
            if let Some(_) = inner_action_map.insert(action.clone(), action_id) {
                return Err(ErrorKind::ActionLoadNameCollision(action, module_name.clone()).into());
            }
            pre_action_list.push(ActionInfo::new(module_id, action.clone()));
        }
    }

    Ok(())
}

fn load_events(
    module_mapper: &ModuleMapper,
    path_resolver: &PathResolver,
) -> Result<ActionMapper> {
    let my_id = module_mapper.get(string::module())?;
    let action_file_name = string::action_file_name();

    let mut pre_action_map = module_mapper.get_raw_map().into_iter().map(|_| None).collect();
    let mut pre_action_list = Vec::new();
    let mut current_id = 0;

    for (module_name, module_id) in module_mapper.get_raw_map() {
        if let Some(actions_json_path) = path_resolver.try_find_submodule_file_path(
                action_file_name,
                *module_id,
                my_id
            )? {
            let actions_json = file_funcs::actions_from_path(actions_json_path)?;
            create_actions(module_name, *module_id, actions_json, &mut pre_action_map, &mut pre_action_list, &mut current_id)?;
        }
    }

    let result = ActionMapper::new(pre_action_map, pre_action_list);
    Ok(result)
}
