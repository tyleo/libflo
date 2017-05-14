use serialization::ParameterSerde;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct FuncSerde {
    input: Option<ParameterSerde>,
    output: Option<ParameterSerde>,
    symbol: String,
    name: Option<String>,
}

impl FuncSerde {
    pub fn destructure(self) -> (Option<ParameterSerde>, Option<ParameterSerde>, String, Option<String>) {
        (
            self.input,
            self.output,
            self.symbol,
            self.name,
        )
    }
}
