use serialization::EventHandlerSerde;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct EventHandlersSerde {
    handlers: Vec<EventHandlerSerde>,
}

impl EventHandlersSerde {
    pub fn destructure(self) -> Vec<EventHandlerSerde> {
        self.handlers
    }
}
