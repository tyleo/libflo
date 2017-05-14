use serialization::FuncSerde;
use serialization::PlatformSerde;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct LibSerde {
    functions: Vec<FuncSerde>,
    platforms: Vec<PlatformSerde>,
}

impl LibSerde {
    pub fn destructure(self) -> (Vec<FuncSerde>, Vec<PlatformSerde>) {
        (self.functions, self.platforms)
    }
}
