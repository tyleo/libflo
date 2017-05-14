use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Eq, PartialEq)]
pub struct ModuleMapperSerde<'a, 'b> {
    module_map: &'a HashMap<String, usize>,
    module_list: &'b Vec<String>
}

impl <'a, 'b> ModuleMapperSerde<'a, 'b> {
    pub fn new(
        module_map: &'a HashMap<String, usize>,
        module_list: &'b Vec<String>
    ) -> Self {
        ModuleMapperSerde {
            module_map: module_map,
            module_list: module_list
        }
    }
}
