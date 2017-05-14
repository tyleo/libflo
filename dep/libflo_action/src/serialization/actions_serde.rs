#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct ActionsSerde {
    actions: Vec<String>,
}

impl ActionsSerde {
    pub fn destructure(self) -> Vec<String> {
        self.actions
    }
}
