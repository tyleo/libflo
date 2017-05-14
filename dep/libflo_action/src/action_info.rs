use serialization::ActionInfoSerde;

#[derive(Debug)]
pub struct ActionInfo {
    module_id: usize,
    action_name: String
}

impl ActionInfo {
    pub fn new(
        module_id: usize,
        action_name: String
    ) -> Self {
        ActionInfo { module_id: module_id, action_name: action_name }
    }

    pub fn get_module_id(&self) -> usize {
        self.module_id
    }

    pub fn get_action_name(&self) -> &String {
        &self.action_name
    }

    pub fn make_serde(&self) -> ActionInfoSerde {
        ActionInfoSerde::new(self.module_id, &self.action_name)
    }
}
