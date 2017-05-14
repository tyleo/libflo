use Action;
use number_or_string::NumberOrString;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BasicAction {
    #[serde(rename = "type")]
    action_type: NumberOrString,
}

impl BasicAction {
    pub fn destructure(self) -> NumberOrString {
        self.action_type
    }
}

impl Action for BasicAction {
    fn get_type(&self) -> &NumberOrString {
        &self.action_type
    }
}
