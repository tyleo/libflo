#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct FuncSerde {
    module: String,
    name: String,
}

impl FuncSerde {
    pub fn destructure(self) -> (String, String) {
        (
            self.module,
            self.name
        )
    }
}
