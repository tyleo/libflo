use serialization::ActionInfoSerde;
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Eq, PartialEq)]
pub struct ActionMapperSerde<'a, 'b> {
    action_map: &'a Vec<Option<HashMap<String, usize>>>,
    action_list: Vec<ActionInfoSerde<'b>>
}

impl <'a, 'b> ActionMapperSerde<'a, 'b> {
    pub fn new(
        action_map: &'a Vec<Option<HashMap<String, usize>>>,
        action_list: Vec<ActionInfoSerde<'b>>
    ) -> Self {
        ActionMapperSerde {
            action_map: action_map,
            action_list: action_list,
        }
    }
}
