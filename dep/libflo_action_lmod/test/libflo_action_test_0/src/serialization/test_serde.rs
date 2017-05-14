use libflo_action_std::{ Action, NumberOrString };

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TestSerde {
    #[serde(rename = "type")]
    pub action_type: NumberOrString,
    pub value: usize
}

impl Action for TestSerde {
    fn get_type(&self) -> &NumberOrString {
        &self.action_type
    }
}
