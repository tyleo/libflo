#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct EventHandlerEventSerde {
    module: String,
    name: String,
}

impl EventHandlerEventSerde {
    pub fn destructure(self) -> (String, String) {
        (
            self.module,
            self.name,
        )
    }
}
