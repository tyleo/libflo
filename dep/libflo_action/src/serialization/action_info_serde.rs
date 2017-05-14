#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ActionInfoSerde<'a> {
    pub module_id: usize,
    pub action_name: &'a String
}

impl <'a> ActionInfoSerde<'a> {
    pub fn new(
        module_id: usize,
        action_name: &'a String
    ) -> Self {
        ActionInfoSerde { module_id: module_id, action_name: action_name }
    }
}
