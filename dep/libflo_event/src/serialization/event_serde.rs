use libflo_func::serialization::ParameterSerde;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct EventSerde {
    input: Option<ParameterSerde>,
    output: Option<ParameterSerde>,
    name: String,
}

impl EventSerde {
    pub fn destructure(self) -> (Option<ParameterSerde>, Option<ParameterSerde>, String) {
        (
            self.input,
            self.output,
            self.name,
        )
    }
}
