use serialization::EventSerde;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct EventsSerde {
    events: Vec<EventSerde>,
}

impl EventsSerde {
    pub fn destructure(self) -> Vec<EventSerde> {
        self.events
    }
}
