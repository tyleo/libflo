use serialization::EventHandlerEventSerde;
use serialization::FuncSerde;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct EventHandlerSerde {
    event: EventHandlerEventSerde,
    function: Option<FuncSerde>,
    functions: Option<Vec<FuncSerde>>,
}

impl EventHandlerSerde {
    pub fn destructure(self) -> (EventHandlerEventSerde, Option<FuncSerde>, Option<Vec<FuncSerde>>) {
        (
            self.event,
            self.function,
            self.functions,
        )
    }
}
